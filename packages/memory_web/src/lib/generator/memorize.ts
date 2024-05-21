import type {  MemorizeData, StudySetWithCards } from "$lib/types";
import type { Question } from "./quiz";
import { rSkewNorm } from "./randomGen";

class MemorizeGenerator{
    public data: MemorizeData ;
    set: StudySetWithCards;
    public question: Question | null = null;
    public scores: {id:string,correct:number,wrong:number,struggling: boolean}[] = [];
    public last_seen: string[] = []
    constructor(data: MemorizeData | null, set: StudySetWithCards){
        this.data = data ?? {id: set.id, set_id: set.id, user_id: set.user_id, last_answered: new Date().toISOString(), answers: []};
        this.set = set;
        this.CalculateScore();
   
    }
    PercentCorrect(this: MemorizeGenerator, score:{correct:number,wrong:number}): number{
        return score.wrong/(score.correct+score.wrong);
    }
    CalculateScore(this: MemorizeGenerator){
        //used last_seen to calculate score
        const scores  = new Map<string,{id:string,correct:number,wrong:number}>();
    
        for (let i = 0; i < (this.data.answers?.length ?? 0); i++){
            const card_data = this.data!.answers[i];
            const card_score:{ id:string,correct:number,wrong:number} = scores.get(card_data.card_id) ?? { id: card_data.card_id,correct:0,wrong:0};
            if (card_data.correct){
                card_score.correct++;
            }else{
                card_score.wrong+=1;
            }
            scores.set(this.data!.answers[i].card_id, card_score);
        }
    
        for (let i = 0; i < this.set.cards.length; i++){
            if (!scores.has(this.set.cards[i].id)){
                scores.set(this.set.cards[i].id,{id:this.set.cards[i].id,correct:0,wrong:0});
            }
        }
    
        const scores_sorted = Array.from(scores.values()).sort(
            (a,b) =>  this.PercentCorrect(b) -this.PercentCorrect(a)
        ).map(score => {
            return {id: score.id, correct: score.correct, wrong: score.wrong, struggling: this.PercentCorrect(score) >0.3};
        });
        this.scores = scores_sorted;
        console.log(this.scores);
    }
    NextQuestion(this: MemorizeGenerator): void{


        // Make it so older answers are disregarded
        // Make it select randomly from the top 20% of cards for new card
        let index;
        if (Math.random() < 0.9){
            index = Math.floor(rSkewNorm(0,0,2,0,this.scores.length));
        }else{
            const indexOfHidden = this.scores.findIndex(score => score.correct == 0 && score.wrong == 0);
            const middle_of_hidden = indexOfHidden + (this.scores.length-indexOfHidden)/2;
            console.log(`middle of hidden: ${middle_of_hidden} length: ${this.scores.length} indexOfHidden: ${indexOfHidden}`); 
            index = Math.floor(rSkewNorm(0,middle_of_hidden,5,indexOfHidden,this.scores.length));
        }
        console.log(index)
        const card = this.set.cards.find(card => card.id == this.scores[index]?.id);
        if (card == undefined){
            throw new Error("card not found");
        }
        const question:Question = {
            card_id: card.id,
            question: card.front,
            answer: card.back,
            options: []
    
        };
        console.log(question)
        question.options.push(card.back);
        for(let i = 0; i < 3; i++){
            let option_card = this.set.cards[Math.floor(Math.random()*this.set.cards.length)].back;
            while (question.options.includes(option_card)){
                option_card = this.set.cards[Math.floor(Math.random()*this.set.cards.length)].back;
                
            }
            question.options.push(option_card);
            
        }
        question.options = question.options.sort(() => Math.random() - 0.5);
    
        this.question= question;
    }

    async SubmitAnswer(this: MemorizeGenerator, answer: string,  difficulty: number){
        this.last_seen.push(this.question!.card_id);
        const data = {card_id: this.question!.card_id, answer, correct:this.question!.answer == answer, difficulty};
        this.data.answers.push(data);
        this.CalculateScore();
        await fetch(`/api/memorize/${this.set.id.$oid}`, { 
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify([data])
        });
    }

    

}


export { MemorizeGenerator}
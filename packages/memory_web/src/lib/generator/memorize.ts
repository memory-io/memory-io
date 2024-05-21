import type {  MemorizeData, StudySetWithCards } from "$lib/types";
import type { Question } from "./quiz";

class MemorizeGenerator{
    public data: MemorizeData ;
    set: StudySetWithCards;
    public question: Question | null = null;
    public scores: Map<string,{id:string,correct:number,wrong:number}> | null = null;
    constructor(data: MemorizeData | null, set: StudySetWithCards){
        this.data = data ?? {id: set.id, set_id: set.id, user_id: set.user_id, last_answered: new Date().toISOString(), answers: []};
        this.set = set;
   
    }
    NextQuestion(this: MemorizeGenerator): void{
        
        const percent_correct: (a:{correct:number,wrong:number}) => number = (a:{correct:number,wrong:number}) => {return a.correct/(a.correct+a.wrong)};
    
        this.scores = new Map<string,{id:string,correct:number,wrong:number}>();
    
        for (let i = 0; i < (this.data.answers?.length ?? 0); i++){
            const card_data = this.data!.answers[i];
            const card_score:{ id:string,correct:number,wrong:number} = this.scores.get(card_data.card_id) ?? { id: card_data.card_id,correct:0,wrong:0};
            if (card_data.correct){
                card_score.correct++;
            }else{
                card_score.wrong+=1;
                card_score.wrong += card_data.difficulty -3;
            }
            this.scores.set(this.data!.answers[i].card_id, card_score);
        }
    
        for (let i = 0; i < this.set.cards.length; i++){
            if (!this.scores.has(this.set.cards[i].id)){
                this.scores.set(this.set.cards[i].id,{id:this.set.cards[i].id,correct:0,wrong:0});
            }
        }
    
        console.log(this.scores);
        const scores_sorted = Array.from(this.scores.entries()).sort(
            (a,b) =>  percent_correct(b[1]) -percent_correct(a[1])
        );
        console.log(scores_sorted);
        const index = Math.floor((Math.E**(-(Math.random()**2)/2)/Math.sqrt(2*Math.PI)) * scores_sorted.length);
        console.log(index)
        const card = this.set.cards.find(card => card.id == scores_sorted[index][0]);
        if (card == undefined){
            throw new Error("card not found");
        }
        const question:Question = {
            card_id: card.id,
            question: card.front,
            answer: card.back,
            options: []
    
        };
        question.options.push(card.back);
        for(let i = 0; i < 3; i++){
            let card = this.set.cards[Math.floor(Math.random()*this.set.cards.length)].back;
            while (question.options.includes(card)){
                card = this.set.cards[Math.floor(Math.random()*this.set.cards.length)].back;
                
            }
            question.options.push(card);
            
        }
        question.options = question.options.sort(() => Math.random() - 0.5);
    
        this.question= question;
    }

    async SubmitAnswer(this: MemorizeGenerator, answer: string,  difficulty: number){
        const data = {card_id: this.question!.card_id, answer, correct:this.question!.answer == answer, difficulty};
        this.data.answers.push(data);
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
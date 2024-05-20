import type { MemorizeCardQuestionData, MemorizeData, ObjectId, StudySetWithCards } from "$lib/types";
import { GenerateQuiz } from "./quiz";


 function GenerateRound(data: MemorizeData | null, set: StudySetWithCards,size: number){
    const cards = set.cards;
    if (data == null || data.answers.length == 0){
        const round = [];
        for (let i = 0; i < size; i++){
            round.push(cards[i]);
        }
        const quiz = GenerateQuiz(round,cards.map(card => card.back));
        return quiz;
    }

    const percent_correct: (a:{correct:number,wrong:number}) => number = (a:{correct:number,wrong:number}) => {return a.correct/(a.correct+a.wrong)};

    const scores: Map<string,{id:string,correct:number,wrong:number}> = new Map();
    for (let i = 0; i < (data?.answers?.length ?? 0); i++){
        const card_data = data!.answers[i];
        const card_score:{ id:string,correct:number,wrong:number} = scores.get(card_data.card_id) ?? { id: card_data.card_id,correct:0,wrong:0};
        if (card_data.correct){
            card_score.correct++;
        }else{
            card_score.wrong+=1;
            card_score.wrong += card_data.difficulty -3;
        }

       
        scores.set(data!.answers[i].card_id, card_score);
    }
    console.log(scores);
    const scores_sorted = Array.from(scores.entries()).sort(
        (a,b) => percent_correct(a[1]) - percent_correct(b[1])
    );
    console.log(scores_sorted);
    const round = [];
    for (let i = 0; i < Math.min(scores_sorted.length,size) ; i++){
        round.push(cards.find(card => card.id == scores_sorted[i][0])!);
    }

    const quiz = GenerateQuiz(round,cards.map(card => card.back));
    


    return quiz;

}
async function submitResults(results: MemorizeCardQuestionData[],set_id:ObjectId){
    await fetch(`/api/memorize/${set_id.$oid}`, { 
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(results)
    });


}

export { GenerateRound,submitResults}
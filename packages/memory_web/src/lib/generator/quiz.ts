import type { Card } from "$lib/types";


type Question = {
    card_id: string,
    question: string,
    answer: string,
    options: string[]
}


function GenerateQuiz(cards: Card[],extra_answers?: string[]): Question[]{
    const questions: Question[] = [];
    for (let i = 0; i < cards.length; i++){
        const card = cards[i];
        const question: Question = {
            card_id: card.id,
            question: card.front,
            answer: card.back,
            options: []
        }
        let options = cards.map(card => card.back);
        if (extra_answers){
            options = options.concat(extra_answers);
        }
        options = options.filter(option => option != card.back);
        options = options.sort(() => Math.random() - 0.5);
        question.options = options.slice(0,3);
        question.options.push(card.back);
        questions.push(question);
    }
    return questions;
}


export { GenerateQuiz,type Question }
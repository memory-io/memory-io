import type { StudySetWithCards } from "$lib/types";


type Question = {
    question: string,
    answer: string,
    options: string[]
}


function GenerateQuiz(set: StudySetWithCards): Question[]{
    const questions: Question[] = [];
    for (let i = 0; i < set.cards.length; i++){
        const card = set.cards[i];
        const question: Question = {
            question: card.front,
            answer: card.back,
            options: []
        }
        let options = set.cards.map(card => card.back);
        options = options.filter(option => option != card.back);
        options = options.sort(() => Math.random() - 0.5);
        question.options = options.slice(0,3);
        question.options.push(card.back);
        questions.push(question);
    }
    return questions;


    

}


export { GenerateQuiz,type Question }
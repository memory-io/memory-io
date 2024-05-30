interface ObjectId{
	$oid: string
}
interface User{
	id: ObjectId,
    username:string,
	email:string,
	paid_user:boolean,
	verified: boolean
}
interface StudySetWithCards{
	id: ObjectId
	title: string,
	description: string,
	user_id: ObjectId,
	visibility: string,
	cards: Card[]
}

interface StudySet{
	id: ObjectId
	title: string,
	description: string,
	user?: User,
	user_id: ObjectId,
	visibility: string,

}
interface Card{
	id: string,
	front: string,
	back: string
}

interface MemorizeData{
	id: ObjectId,
    set_id: ObjectId,
    user_id: ObjectId,
    last_answered: string,
    answers: MemorizeCardQuestionData[],
}
interface MemorizeCardQuestionData{
	card_id: string,
	answer: string,
	correct: boolean,
	difficulty: number ,
}

export type {StudySet,Card,StudySetWithCards,User,ObjectId,MemorizeData,MemorizeCardQuestionData}
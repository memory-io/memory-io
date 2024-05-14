interface ObjectId{
	$oid: string
}
interface User{
	id: ObjectId,
    username:string,
	email:string,
}
interface StudySetWithCards{
	id: ObjectId
	title: string,
	description: string,
	user_id: ObjectId,
	visibility: string,
	cards: Card[]
}
interface User{
	id: ObjectId,
	email: string,
	username: string,
	paid_user: boolean

}

interface StudySet{
	id: ObjectId
	title: string,
	description: string,
	user_id: ObjectId,
	visibility: string,

}
interface Card{
	id: string,
	front: string,
	back: string
}
export type {StudySet,Card,StudySetWithCards,User,ObjectId}
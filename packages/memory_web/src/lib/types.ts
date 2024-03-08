
interface StudySetWithCards{
	id: string
	title: string,
	user_id: string,
	visibility: string,
	cards: Card[]
}
interface User{
	id: string,
	email: string,
	username: string

}

interface StudySet{
	id: string
	title: string,
	user_id: string,
	visibility: string,
	cards: string[]

}
interface Card{
	id: string,
	front: string,
	back: string
}
export type {StudySet,Card,StudySetWithCards,User}
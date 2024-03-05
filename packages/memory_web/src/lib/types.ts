
interface StudySetWithCards{
	id: string
	title: string,
	visibility: string,
	cards: Card[]

}

interface StudySet{
	id: string
	title: string,
	visibility: string,
	cards: string[]

}
interface Card{
	id: string,
	front: string,
	back: string
}
export type {StudySet,Card,StudySetWithCards}
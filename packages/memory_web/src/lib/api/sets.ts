import type { Card } from "$lib/types";
import { redirect } from "@sveltejs/kit";


async function deleteCard(set_id: string, card_id: string) {
    const response = await fetch(`/api/sets/${set_id}`, { 
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ "RemoveCard":{"id":card_id }})
    }
    );
    if (response.status == 401){
        redirect(301,"/auth/login"); 
    }
    if (response.status !== 200) {
        return {
            error:await response.text()
        }
        
    }
    return {
    }

    
}

async function deleteSet(set_id: string) {
    const response = await fetch(`/api/sets/${set_id}`, { 
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',

        },
    }
    );
    if (response.status == 401){
        redirect(301,"/auth/login"); 
    }
    if (response.status !== 200) {
        return {
            error:await response.text()
        }
        
    }
    return {
    }

    
}

async function updateCard(set_id: string,card: Card) {
    const response = await fetch(`/api/sets/${set_id}`, { 
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ "UpdateCard":{id:card.id,"front":card.front ,"back":card.back }})
    }
    );
    if (response.status == 401){
        redirect(301,"/auth/login"); 
    }
    if (response.status !== 200) {
        return {
            error:await response.text()
        };
        
    }
    return {

    }
}


async function addCard(set_id: string,front: string,back: string) {
    const response = await fetch(`/api/sets/${set_id}`, { 
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ "AddCard":{"front":front ,"back":back }})
    }
    );
    if (response.status == 401){
        redirect(301,"/auth/login"); 
    }
    if (response.status !== 200) {
        return {
            error:await response.text()
        };
        
    }
    return {}
}


export {deleteCard,deleteSet,updateCard,addCard};
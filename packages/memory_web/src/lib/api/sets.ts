import type { Card, ObjectId, StudySet } from "$lib/types";
import { redirect } from "@sveltejs/kit";



async function deleteCard(set_id: ObjectId, card_id: string) {
    const response = await fetch(`/api/sets/${set_id.$oid}`, { 
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

async function deleteSet(set_id: ObjectId) {
    const response = await fetch(`/api/sets/${set_id.$oid}`, { 
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

async function updateCard(set_id: ObjectId,card: Card) {
    console.log(`updating card ${card.id} in set ${set_id.$oid}`);
    const response = await fetch(`/api/sets/${set_id.$oid}`, { 
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

async function updateSet(set:StudySet) {

    const response = await fetch(`/api/sets/${set.id.$oid}`, { 
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ "UpdateSet":{...set}})
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

async function createSet(title: string,description:string,visibility: string) {
    console.log(`creating set ${title}`);
    const response = await fetch('/api/sets/create', { 
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',

        },
        body: JSON.stringify({ "title":title,"description":description,"visibility": visibility })
    });
    if (response.status == 401){
        return {
            error:401
        }
    }
    if (response.status !== 200) {
        return {
            error:await response.text()
        }
        
    }
    return {
        set:await response.json() as StudySet
    }
    
}


async function addCard(set_id: ObjectId,front: string,back: string) {
    const response = await fetch(`/api/sets/${set_id.$oid}`, { 
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


export {deleteCard,deleteSet,updateCard,addCard,createSet,updateSet};
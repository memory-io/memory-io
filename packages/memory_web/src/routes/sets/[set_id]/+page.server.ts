import type {  StudySetWithCards } from "$lib/types.js";
import { fail, redirect } from "@sveltejs/kit";
export const ssr = true


/** @type {import('./$types').PageLoad} */
export async function load({params,fetch}) {
    const set_id = params.set_id;

    //request the url at localhost:8000/api/auth/signup
    const response = await fetch(`/api/sets/${set_id}`, { 
        method: 'GET',
    });
    if (response.status == 401){
        redirect(301,"/auth/login");
    }
    if (response.status !== 200) {
        console.log(response)
        return {
            error:"error loading sets"
        };
        
    }

    const data: StudySetWithCards = await response.json();
	return {
		set: data
	};
}



/** @type {import('./$types').Actions} */
export const actions = {
    add_card: async ({fetch,params}) => {
		// TODO log the user in

        const set_id = params.set_id;

        if (set_id === null || set_id === "") {
            return fail(400,{
                error:"set_id is missing"
            })
        }
        
        //request the url at localhost:8000/api/auth/signup
        const response = await fetch(`/api/sets/${set_id}`, { 
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ "AddCard":{"front":"Front Of Card" ,"back":"Back Of Card" }})
        }
        );
        if (response.status == 401){
            redirect(301,"/auth/login"); 
        }
        if (response.status !== 200) {
            return fail(400,{
                error:await response.text()
            });
            
        }
	},
    delete: async ({fetch,params }) => {
		// TODO log the user in
        const set_id = params.set_id;

        if (set_id === null || set_id === "") {
            return fail(400,{
                error:"set_id is missing"
            })
        }

        //request the url at localhost:8000/api/auth/signup
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
            return fail(400,{
                error:await response.text()
            });
            
        }
        redirect(301,"/sets"); 
	},
    delete_card: async ({request,params }) => {
		// TODO log the user in
        const data = await request.formData();
		const card_id = data.get('id');
        const set_id = params.set_id;

        if (set_id === null || set_id === "") {
            return fail(400,{
                error:"set_id is missing"
            })
        }
        
        //request the url at localhost:8000/api/auth/signup
        const response = await fetch(`/api/sets/${set_id}`, { 
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ "RemoveCard":{"card_id":card_id }})
        }
        );
        if (response.status == 401){
            redirect(301,"/auth/login"); 
        }
        if (response.status !== 200) {
            return fail(400,{
                error:await response.text()
            });
            
        }
	}
}
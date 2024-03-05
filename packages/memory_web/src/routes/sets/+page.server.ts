
import type { StudySet } from "$lib/types";
import { fail, redirect } from "@sveltejs/kit";
export const ssr = true

/** @type {import('./$types').PageLoad} */
export async function load({fetch}) {

    //request the url at localhost:8000/api/auth/signup
    const response = await fetch('/api/sets', { 
        method: 'GET',
        credentials: 'include',
        
    });
    if (response.status == 401){
        //cookies.set('id', "",{path:"/"});
        throw redirect(301,"/auth/login");
    }
    if (response.status !== 200) {
        return {
            error:"error loading sets"
        };
        
    }
    const data: StudySet[] = await response.json();
	return {
		sets: data
	};
}


/** @type {import('./$types').Actions} */
export const actions = {
	create: async ({request,fetch }) => {
		// TODO log the user in
        const data = await request.formData();
		const title = data.get('title');
		const visibility = data.get('visibility');
        if (title === null || visibility === null || visibility === "" || title === "") {
            return fail(400,{
                error:"title or visibility is missing"
            })
        }
  
        //request the url at localhost:8000/api/auth/signup
        const response = await fetch('/api/sets/create', { 
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',

            },
            body: JSON.stringify({ "title":title,"visibility": visibility })
        });
        if (response.status == 401){
            redirect(301,"/auth/login"); 
        }
        if (response.status !== 200) {
            return fail(400,{
                error:await response.text()
            });
            
        }

	},
    
    
};
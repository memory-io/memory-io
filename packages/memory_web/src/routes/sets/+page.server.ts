
import type {StudySet } from "$lib/types";
import {   redirect } from "@sveltejs/kit";

/** @type {import('./$types').PageLoad} */
export async function load({fetch,locals}) {
    if (locals.user == null){
        throw redirect(301,"/auth/login");
    }

    //request the url at localhost:8000/api/auth/signup
    const response = await fetch('/api/sets', { 
        method: 'GET',
        credentials: 'include',
        
    });
    if (response.status == 401 ){
        //cookies.set('id', "",{path:"/"});
        throw redirect(301,"/auth/login");
    }
    if (response.status !== 200) {
        return {
            error:"error loading sets"
        };
        
    }
    const data: StudySet[] = (await response.json());
	return {
		sets: data
	};
}


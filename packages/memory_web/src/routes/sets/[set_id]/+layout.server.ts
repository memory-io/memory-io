import type { StudySetWithCards } from "$lib/types";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({params ,fetch}) => {
    const set_id = params.set_id;

    //request the url at localhost:8000/api/auth/signup
    const response = await fetch(`/api/sets/${set_id}?includeCards=true`, { 
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
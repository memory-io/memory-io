import type { MemorizeData, StudySetWithCards } from "$lib/types";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({params ,fetch,locals}) => {
    const set_id = params.set_id;
    if (locals.user == null){
        throw redirect(301,"/auth/login");
    }
    //request the url at localhost:8000/api/auth/signup
    const set_response = await fetch(`/api/sets/${set_id}?includeCards=true`, { 
        method: 'GET',
        
    });
    if (set_response.status == 401){
        redirect(301,"/auth/login");
    }
    if (set_response.status !== 200) {
        console.log(set_response)
        console.log(set_id)
        return {
            error:"error loading set"
        };
        
    }
    const set: StudySetWithCards = await set_response.json();


    //request the url at localhost:8000/api/auth/signup
    const memorize_response = await fetch(`/api/memorize/${set_id}`, { 
        method: 'GET',
        
    });
    if (memorize_response.status == 401){
        redirect(301,"/auth/login");
    }
    if (memorize_response.status !== 200 && memorize_response.status !== 404) {
        console.log(memorize_response)
        console.log(set_id)
        return {
            error:"error loading set"
        };
        
    }
    let memorize_data: MemorizeData | null= null;
    if (memorize_response.status !== 404){
        memorize_data = await memorize_response.json();
    }
    console.log(memorize_data);

	return {
		set,
        memorize_data
	};
}

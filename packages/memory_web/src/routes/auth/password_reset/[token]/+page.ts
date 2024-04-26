import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch,params }) => {
    const token = params.token;

    const response = await fetch(`/api/users/password_reset/${token}`);
    if (response.ok){
        return {
            token: token
        };
    }else{
        redirect(301,"/auth/password_reset")
    }
	
};
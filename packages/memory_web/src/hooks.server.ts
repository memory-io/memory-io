import type { User } from '$lib/types';
import {  type Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
    if (event.url.pathname.startsWith('/auth') || event.url.pathname.startsWith('/api')) {
		const response = await resolve(event);
	    return response;
	}
    const data = await event.fetch("/api/users/me");
    if (data.ok){
        const user:User = await data.json();
	    event.locals.user = user;
        console.log(user)
    }
    
	const response = await resolve(event);

	return response;
};
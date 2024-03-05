import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
    const id = event.cookies.get('id');
    if (id === null || id == "" || id === undefined) {
        const response = await resolve(event);
        return response;
    }
    
	//event.locals.user = await getUser(id);
	const response = await resolve(event);

	return response;
};
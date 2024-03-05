import { fail, redirect } from '@sveltejs/kit';
export const ssr = true

/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ fetch,cookies, request }) => {
		// TODO log the user in
        const data = await request.formData();
		const email = data.get('email');
		const password = data.get('password');
        if (email === null || password === null) {
            return fail(401,{
                error:"Email or password is missing"
            })
        }
        //request the url at localhost:8000/api/auth/signup
        const response = await fetch('/api/users/login', { 
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                
            },
            body: JSON.stringify({ "email":email,"password": password })
        });
        if (response.status !== 200) {
            return fail(401,{
                error:await response.text()
            });
            
        }

        const cookie = response.headers.getSetCookie()[0].split(";")[0].split("=")[1];
        if (cookie !== null && cookie !== undefined) {
            const decoded = decodeURIComponent(cookie);
            
            cookies.set('id', decoded,{path:"/",sameSite:"lax",secure:true,httpOnly:true});
            return redirect(301,"/sets");
        }
        return fail(401,{
            error:"error logging in"
        });

	}
};
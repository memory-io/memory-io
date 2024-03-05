interface User{
	id: string,
    username:string,
	email:string,
}

export async function getUser(user_cookie:string): Promise<User> {
    return fetch('http://localhost:8000/users/get_user', { 
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Cookie': `id=${user_cookie};`
        }
    }).then(response => response.json())
}

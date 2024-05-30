



async function deleteUser(){
    return await fetch('/api/users/delete', { 
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        }
    });
}
export { deleteUser}

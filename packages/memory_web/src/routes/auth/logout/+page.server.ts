
//export const prerender = true

import { redirect } from '@sveltejs/kit';



/** @type {import('./$types').PageLoad} */
export async function load({cookies}) {
    cookies.delete("id",{path:"/"});
    throw redirect(301,"/");
}
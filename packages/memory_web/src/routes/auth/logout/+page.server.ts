
export const prerender = true



/** @type {import('./$types').PageLoad} */
export async function load({cookies}) {
    cookies.delete("id",{path:"/"});
}
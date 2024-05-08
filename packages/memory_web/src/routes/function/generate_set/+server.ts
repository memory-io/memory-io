import { anthropic } from '$lib/server/anthropic';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({url}) => {
    const content = url.searchParams.get('data')?.toString();

    console.log("Requesting Data")
    if (content === undefined) {   
        return new Response(null,{status:400,statusText:"Bad Request"});
        
    }

    const message = await anthropic.messages.create({
        system:"Take in the users data and generate flashcards from the data. Output the list flash cards in JSON with the following keys “front” and “back”. Do not output anything other JSON. ",
        messages: [{role: 'user', content: content },{role: 'assistant', content: "JSON:" }],
        model: 'claude-3-haiku-20240307',
        max_tokens: 2000,
    });
    console.log(message);
    // const sets: Omit<Card,"id">[] = JSON.parse(message.content[0].text);
    
    return new Response(message.content[0].text);
};
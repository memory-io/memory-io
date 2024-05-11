import { anthropic } from '$lib/server/anthropic';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({url,locals}) => {
    if (!locals.user.paid_user) {
        return new Response(null,{status:400,statusText:"Not included in the free tier. Please upgrade to use this feature."});
    }
    const content = url.searchParams.get('data')?.toString();

    console.log("Requesting Data")
    if (content === undefined ) {   
        return new Response(null,{status:400,statusText:"Bad Request"});
        
    }

    const message = await anthropic.messages.create({
        system:"Take in the users data and generate flashcards from the data. If the provided data does not provide the answer try to find the correct answer. Optionally generate more flashcards that would help the user to answer the provided questions or remember the data better. Do not output anything other than the flashcards as JSON. The output format should be JSON with  three keys. title: generate a title for the set. description: generate a concise 1-3 sentence description for the set. cards: list of the generated cards with 'front' and 'back' fields.",
        messages: [{role: 'user', content: content },{role: 'assistant', content: "JSON:" }],
        model: 'claude-3-haiku-20240307',
        max_tokens: 4096,
    });
    console.log(message);
    // const sets: Omit<Card,"id">[] = JSON.parse(message.content[0].text);
    
    return new Response(message.content[0].text);
};
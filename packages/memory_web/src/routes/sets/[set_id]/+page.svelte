


<script lang="ts">
    import { enhance } from "$app/forms";
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { flip } from 'svelte/animate';
	import SetCard from "./set-card.svelte";
    import { toast } from "svelte-sonner";
	import { user } from "$lib/store";
	import Formatter from "$lib/formatter.svelte";
	import { ChevronRight,ChevronLeft, DrillIcon } from "lucide-svelte";
	import { quintOut } from 'svelte/easing';
	import { deleteSet } from "$lib/api/sets";
	import type { TouchEventHandler } from "svelte/elements";
	import { swipe, tap } from "svelte-gestures";
	import SetCarosel from "./set_carosel.svelte";

    export let data;
    let own_set = data.set?.user_id == data.user?.id;
    



</script>

<section class="flex flex-col max-w-screen-md h-full gap-5 justify-items-center ">

{#if data.set != undefined}
    <Card.Root>
        <Card.Header>
            <Card.Title>{data.set?.title}</Card.Title>
            <Card.Description>{data.set?.visibility}</Card.Description>
        </Card.Header>
        <Card.Content>
            {#if data.set.cards != null }
                <SetCarosel cards={data.set.cards}/>
            
            {/if}

        </Card.Content>
        {#if  data.set != undefined}

        <Card.Footer class="flex justify-between gap-3">
            <span>
                <Button href={`${data.set.id}/quiz`} >Quiz</Button>
                <Button href={`${data.set.id}/learn`} >Learn</Button>
            </span>
            {#if own_set}
            <span>

                <Button type="submit">Edit</Button>
                <Button on:click={async () => {
                    let out = await deleteSet(data.set?.id ??"nol");
                    if (!out.error){
                        window.location.href = "/sets";
                    }else{
                        toast.error(out.error);
                    }
                }} variant="destructive">Delete</Button>
            </span>
            {/if}
        </Card.Footer>

        {/if}

    </Card.Root>
    {#if own_set &&  data.set != undefined}
    <Card.Root >
        <SetCard set_id={data.set.id} default_editable={true} owned={own_set} card={{
            id:"",
            front:"",
            back:""
        }}/>
    </Card.Root>
{/if}
    {#if data.set.cards != undefined}
        {#if data.set.cards.length == 0 }
            <p>It's empty</p>
        {/if}
    {   #each data.set.cards as card}
            <Card.Root >
                <SetCard owned={own_set} default_editable={false} set_id={data.set.id} card={card}/>
            </Card.Root>
        {/each}
    {/if}
{/if}

{#if data.set == undefined}
<p>No sets found</p>
{/if}



</section>
<style>
    .grayed{
        opacity: 0.1;
    }
    .wrap{
        overflow-wrap: anywhere;
    }



</style>

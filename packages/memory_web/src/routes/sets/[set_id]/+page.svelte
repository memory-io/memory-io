


<script lang="ts">
    import { enhance } from "$app/forms";
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { flip } from 'svelte/animate';
	import SetCard from "./set-card.svelte";
    import { toast } from "svelte-sonner";
	import { user } from "$lib/store";
	import Formatter from "$lib/formatter.svelte";
	import { ChevronRight,ChevronLeft } from "lucide-svelte";
	import { quintOut } from 'svelte/easing';

   
    export let data;
    export let form;
    let own_set = data.set?.user_id == data.user.id;

    $: {
        if (form?.error) {
            toast.error(form.error, {
           
            });
        }else if (form?.status) {
            toast.success(form.status, {
            
            });
        }
    }
    let card_index = 0;
    let front = true;
    $:cards_len = data.set?.cards.length ?? 0;

    $: {
        if (data.set != undefined && cards_len <= card_index){
            card_index = cards_len-1;
        }
    }



</script>

<section class="flex flex-col max-w-screen-md h-full gap-5 justify-items-center ">
{#if data.set != undefined}
<Card.Root>
    
    <Card.Header>
        
        <Card.Title>{data.set?.title}</Card.Title>
        <Card.Description>{data.set?.visibility}</Card.Description>
    </Card.Header>
    <Card.Content>
        {#if data.set.cards[card_index] != undefined }
        
        <div id={data.set.cards[card_index].id} class=" h-[300px] bg-secondary rounded-md flex flex-row">
            
            <button  class:grayed={card_index == 0} on:click={() =>{
                if (card_index > 0){
                    card_index-=1;front=true;
                }
            }} class="transition-opacity w-16 h-full flex flex-col justify-center items-center border-r border-secondary-foreground border-opacity-40">
                <ChevronLeft />
            </button>
            <button  on:click={() => {front=!front}} class="relative p-10 flex justify-center items-center w-full" >
                
                <div class ="absolute left-2 top-2 text-secondary-foreground text-sm opacity-50">
                    {`Card ${card_index+1} of ${cards_len}`}
                </div>
                <span class="wrap">
                    <Formatter data={front ? data.set.cards[card_index].front:data.set.cards[card_index].back}/>
                </span>
            </button>

            <button class:grayed={card_index == cards_len-1} on:click={() => {
                if (cards_len >  (card_index+1)){
                    card_index+=1;front=true;
                }
            }
            } class={" transition-opacity w-16 h-full flex flex-col justify-center items-center border-l border-secondary-foreground border-opacity-40"}>
                <ChevronRight />
            </button>
        
        </div>
        {/if}

    </Card.Content>
    {#if own_set}
    <form method="POST" use:enhance={() => {
        return async ({ update }) => {
            update({ reset: false });
        };
    }} >
    <input name="id" value={data.set.id} hidden>
    <Card.Footer class="flex justify-between gap-3">
        <span>
            <Button href={`${data.set.id}/learn`} >Learn</Button>
        </span>
        <span>
            <Button type="submit">Edit</Button>
            <Button formaction="?/delete" type="submit" variant="destructive">Delete</Button>
        </span>
   
    </Card.Footer>
    </form>
    {/if}

</Card.Root>
{#if data.set.cards != undefined}
{#each data.set.cards as card}
<Card.Root >
    <form id={card.id} method="POST" use:enhance={() => {
        return async ({ update }) => {
            update({ reset: false });
        };
    }}>
    <input name="id" value={card.id} hidden>
    <SetCard default_editable={false} card={card}/>
    
    

    </form>

</Card.Root>

{/each}

{#if own_set}
    <Card.Root >
        <form id="new-card"  method="POST" use:enhance>
            <SetCard default_editable={true} card={null}/>
        </form>
    </Card.Root>
{/if}

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

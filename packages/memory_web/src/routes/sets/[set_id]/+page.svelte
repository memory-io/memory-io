


<script lang="ts">
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";

    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
	import SetCard from "./set-card.svelte";
    import { toast } from "svelte-sonner";
	import { deleteSet, updateSet } from "$lib/api/sets";
	import SetCarosel from "../../../lib/ucomponents/set_carosel.svelte";
	import { MoreVertical } from "lucide-svelte";
	import { invalidate, invalidateAll } from "$app/navigation";
	import SetActions from "./set-actions.svelte";
	import { Input } from "$lib/components/ui/input";

    export let data;
    if (data.set == undefined ){
        throw new Error("No set found");
    }

    let title = data.set.title;
    let description = data.set.description;
    let edit_set = false;

    let own_set = data.set.user_id.$oid == data.user?.id.$oid ;

    async function updateSetValue(){
        if (data.set == undefined ){
            throw new Error("No set found");
        }
        
        let out = await updateSet({
            ...data.set,
            title: title,
            description: description
        });
        if (!out.error){
            toast.success("Updated");
            
        }else{
            toast.error(out.error);
        }
    }

</script>

<section class="flex flex-col max-w-screen-md h-full gap-5 justify-items-center ">

{#if data.set != undefined}
    <Card.Root>
        <Card.Header class="flex-row justify-between">
            {#if edit_set}
            <div class="w-full">
                <Card.Title>
                <input class="bg-inherit w-full" bind:value={title} on:focusout={
                    updateSetValue
                } />
                </Card.Title>
                <Card.Description>
                <input class="bg-inherit w-full " bind:value={description} on:focusout={
                    updateSetValue
                } />
                </Card.Description>
                
            </div>
            
            
            {:else}
            <div>
                <Card.Title>{data.set?.title}</Card.Title>
                <Card.Description>{data.set?.description}</Card.Description>
            </div>

            {/if}
            <div>
                <SetActions set={data.set} own_set={own_set} bind:edit_set={edit_set} />
            </div>
            
        
        </Card.Header>
        <Card.Content>
            {#if data.set.cards != null }
                <SetCarosel cards={data.set.cards}/>
            
            {/if}

        </Card.Content>
        {#if  data.set != undefined}

        <Card.Footer class="flex justify-between gap-3">
            {#if data.set.cards.length != 0}
            <span>
                <Button variant="outline" href={`${data.set.id.$oid}/quiz`} >Quiz</Button>
                <Button variant="outline" href={`${data.set.id.$oid}/memorize`} >Memorize</Button>
            </span>
            {:else}
            <span></span>
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

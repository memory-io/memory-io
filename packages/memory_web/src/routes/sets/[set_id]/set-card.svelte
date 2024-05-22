<script lang="ts">
    //https://www.shadcn-svelte.com/docs/components/card
    import { Button } from "$lib/components/ui/button";
	import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import type { Card, ObjectId } from "$lib/types";
	import { MoreVertical } from "lucide-svelte";

	import { addCard, deleteCard, updateCard } from "$lib/api/sets";
	import { invalidate, invalidateAll } from "$app/navigation";
	import { toast } from "svelte-sonner";
	import CardEditor from "$lib/ucomponents/CardEditor.svelte";
	import Formatter from "$lib/ucomponents/formatter.svelte";
    export let card:Card ;
    export let set_id:ObjectId ;
    export let default_editable: boolean;
    export let owned: boolean;

    const original_card = {
        front: card.front,
        back: card.back,
        id: card.id
    }

    let editable = default_editable;

    
</script>

<div class="p-2">
<div class="card relative">
        
    <div class="card-side" id="front">
        {#if editable && owned}
        <CardEditor disabled={!editable}   bind:value={card.front} />

        {/if}
        {#if !editable && card.id != ""}
            <span class="pr-2 overflow-auto">
                <Formatter data={card?.front}/>
            </span>
        {/if}
    </div>
    <div class="card-side">
        {#if editable && owned}
            <CardEditor disabled={!editable}   bind:value={card.back} />
        {/if}
        {#if !editable && card.id != ""}
            <span class="pl-4 text-wrap ">
                <Formatter data={card?.back}/>
            </span>
        {/if}
    </div>
    {#if card.id != "" && owned}
    <div class="m-w-8">
    <DropdownMenu.Root>
        <DropdownMenu.Trigger>
            <MoreVertical />
        </DropdownMenu.Trigger>
        <DropdownMenu.Content class="w-56">
            <DropdownMenu.Label>Card Actions</DropdownMenu.Label>
            <DropdownMenu.Separator />
            <DropdownMenu.Group>
                {#if editable}
                    <DropdownMenu.CheckboxItem on:click={async ()=>{
                        let out = await updateCard(set_id,card);
                        if (!out.error){
                            toast.success("Card Updated");    
                            invalidateAll();                    
                        }else{
                            toast.error(out.error);
                        }
                        
                    }} bind:checked={editable}>
                        Save
                    </DropdownMenu.CheckboxItem>
                {/if}
                {#if !editable }
                    <DropdownMenu.CheckboxItem bind:checked={editable}>
                        Editable
                    </DropdownMenu.CheckboxItem>
                   
                {/if}
                <DropdownMenu.CheckboxItem on:click={async ()=>{
                    let out = await deleteCard(set_id,card.id);
                    if (!out.error){
                        toast.success("Card Deleted");
                        invalidateAll();                    
                    }else{
                        toast.error(out.error);
                    }
                }}>
                    Delete
                </DropdownMenu.CheckboxItem>
            </DropdownMenu.Group>
        </DropdownMenu.Content>
    </DropdownMenu.Root>
    </div>
    {/if}
</div>
{#if editable}
    {#if card.id == ""}
        <Button on:click={async () => {
            let out = await addCard(set_id,card.front,card.back);
            if (!out.error){
                invalidateAll(); 
                toast.success("Card Added");                   
            }else{
                toast.error(out.error);
            }
            
        }} type="submit" variant="outline" >Add Card</Button>
    {/if}
{/if}
</div>

<style>
    /* .card:hover .popover{
        display: block;
    }
    .popover{
        display: none;
    } */
    .card{
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        min-height: 150px;
        padding: 10px;
        
    }
    #front{
        border-right: 2px solid #fff;
        align-items: flex-center;
        width: 30%;
        min-width: 30%;
    }


    .card-side{
        padding-left: 5px;
        padding-right: 5px;
        width: 100%;
        overflow-wrap: anywhere;
        
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        gap: 10px;
    }
</style>
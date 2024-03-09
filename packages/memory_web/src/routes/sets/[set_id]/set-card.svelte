<script lang="ts">
    //https://www.shadcn-svelte.com/docs/components/card
    import { Button } from "$lib/components/ui/button";
	import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import type { Card } from "$lib/types";
	import { MoreVertical } from "lucide-svelte";
	import Formatter from "../../../lib/formatter.svelte";
    export let card:Card | null;
    export let default_editable: boolean;

    let editable = default_editable;
    let deleteButton: HTMLElement | null | undefined;

    
</script>

<div class="p-2">
<div class="card relative">
        
    <div class="card-side" id="front">
        {#if editable}
        <Textarea disabled={!editable} class="h-full" placeholder="Front of card"  value={card?.front} name="front" />
        {/if}
        {#if !editable && card != null}
        <span class="pr-2 overflow-auto">
            <Formatter data={card?.front}/>
        </span>
        {/if}
    </div>
    <div class="card-side">
        {#if editable}
        <Textarea disabled={!editable} class="h-full" placeholder="Back of card"  value={card?.back} name="back" />
        {/if}
        {#if !editable && card != null}
        <span class="pl-4 text-wrap ">
        <Formatter data={card?.back}/>
        </span>
        {/if}
    </div>
    {#if card != null}
    <div class="m-w-8">
    <DropdownMenu.Root>
        <DropdownMenu.Trigger>
            <MoreVertical />

            
        </DropdownMenu.Trigger>
        <DropdownMenu.Content class="w-56">
            <DropdownMenu.Label>Card Actions</DropdownMenu.Label>
            <DropdownMenu.Separator />
        
            <DropdownMenu.Group>
                <DropdownMenu.CheckboxItem bind:checked={editable}>
                    Editable
                </DropdownMenu.CheckboxItem>
                <DropdownMenu.CheckboxItem on:click={()=>{
                    let e = document.getElementById(card?.id ?? "new-card");
                    if (!(e instanceof HTMLFormElement)) 
                        throw new Error(`Expected e to be an HTMLScriptElement, was ${e && e.constructor && e.constructor.name || e}`);
                    e.requestSubmit(deleteButton)
                }}>
                    <input bind:this={deleteButton} type="submit" form={card.id}  formaction="?/delete_card" hidden>

                    Delete
                </DropdownMenu.CheckboxItem>
               
       
            </DropdownMenu.Group>
            
        </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>
    {/if}
    
    
  
</div>
{#if editable}
    {#if card != null}
        <Button form={card.id}  type="submit" variant="outline" formaction="?/update_card">Save</Button>
    {/if}
    {#if card == null}
        <Button form="new-card" type="submit" variant="outline" formaction="?/add_card">Add Card</Button>
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
    .card-side .card-content{
        width: 100%;
        height: 100%;
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
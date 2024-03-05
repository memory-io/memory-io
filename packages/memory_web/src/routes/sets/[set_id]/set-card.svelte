<script lang="ts">
    import { enhance } from "$app/forms";
    import * as Popover from "$lib/components/ui/popover";
    import SimpleLineIconsOptionsVertical from '~icons/simple-line-icons/options-vertical';
    //https://www.shadcn-svelte.com/docs/components/card
    import { Button } from "$lib/components/ui/button";
	import Input from "$lib/components/ui/input/input.svelte";
	import { Label } from "$lib/components/ui/label";
	import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import type { Card } from "$lib/types";
    export let card:Card | null;
    export let default_editable: boolean;

    let editable = default_editable;
    let deleteButton: HTMLElement | null | undefined;

    
</script>

<div class="p-2">
<div class="card relative">
        
    <div class="card-side" id="front">
        <Textarea disabled={!editable} class="h-full" placeholder="Front of card"  value={card?.front} name="front" />
    </div>
    <div class="card-side">
        <Textarea disabled={!editable} class="h-full" placeholder="Back of card" value={card?.back}  name="back" />
    </div>
    {#if card != null}
    <div class="popover absolute right-3 top-4 z-10">
    <DropdownMenu.Root>
        <DropdownMenu.Trigger>
            <SimpleLineIconsOptionsVertical />

            
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
        <Button form={card.id} type="submit" variant="outline" formaction="?/update_card">Save</Button>
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
        height: 150px;
        padding: 10px;
    }
    .card #front{
        border-right: 2px solid #fff;
        align-items: flex-center;
        width: 50%;
    
    }
    .card-side .card-content{
        width: 100%;
        height: 100%;
    }

    .card-side{
        padding-left: 5px;
        padding-right: 5px;
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        gap: 10px;
    }
</style>
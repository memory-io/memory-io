<script lang="ts">
import * as Dialog from "$lib/components/ui/dialog";
import * as Tabs from "$lib/components/ui/tabs";

import { Input } from "$lib/components/ui/input";
import { Label } from "$lib/components/ui/label";
import { Button } from "$lib/components/ui/button";
import { enhance } from "$app/forms";
import type { Card } from "$lib/types";
import SetCarosel from "../[set_id]/set_carosel.svelte";
import { Reload } from "svelte-radix";
import { goto } from "$app/navigation";

let dialogOpen = false;

let paste_data = "";
let generated_set: {
    cards: Card[];
    title: string;
    descripition: string;

} | null = null;
let loading = false;
let create_set_loading = false;
async function GenerateSet(content:string){
    console.log(content);
    loading= true;
    let response = await fetch(`/function/generate_set?data=${content}`, {
        method: "GET",
    });
    loading = false

    if (response.status != 200){
        console.log(response.statusText);
        return;
    }
    const data = await response.text();
  
    generated_set = JSON.parse(data);

}
async function CreateSet(){
    create_set_loading = true;
    let response = await fetch(`/api/sets/create`, {
        method: "POST",
        body: JSON.stringify({visibility: 'Private',...generated_set}),
        headers: {
            "Content-Type": "application/json",
        },
    });
    create_set_loading = false

    if (response.status != 200){
        console.log(response.statusText);
        return;
    }
    let body = await response.json();
    goto(`/sets/${body.id}`)
    


}
</script>

<Tabs.Root value="paste" class="w-[400px] min-h-10">
    <Tabs.List class="w-full">
        <Tabs.Trigger value="paste" class="w-full">Paste</Tabs.Trigger>
        <Tabs.Trigger value="file" class="w-full">File</Tabs.Trigger>
        <Tabs.Trigger value="website" class="w-full">Website</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="paste">
        <Label for="file">Paste comma seperated data here.</Label>
        <textarea name="text-data" bind:value={paste_data} class="w-full h-full"></textarea>
        {#if loading}
            <Button disabled>
                <Reload class="mr-2 h-4 w-4 animate-spin" />
                Generating
            </Button>
        {:else}
        <Button type="submit" on:click={(_) => GenerateSet(paste_data)}> Generate</Button>
        {/if}

        



    </Tabs.Content>
    
    <Tabs.Content value="file">
        <form >
        <Label for="file">Import a file.</Label>
        <Input name="file" type="file" placeholder="File" />
        {#if loading}
            <Button disabled>
                <Reload class="mr-2 h-4 w-4 animate-spin" />
                Generating
            </Button>
        {:else}
        <Button type="submit" >Generate</Button>
        {/if}
        </form>
    </Tabs.Content>
    <Tabs.Content value="website">
    </Tabs.Content>

        
    </Tabs.Root>
    {#if generated_set}
    <SetCarosel cards={generated_set.cards}/>
        {#if loading}
            <Button disabled>
                <Reload class="mr-2 h-4 w-4 animate-spin" />
                Creating Set..
            </Button>
        {:else}
        <Button on:click={()=> {CreateSet()}}>Save Generated Set</Button>
        {/if}
    {/if}
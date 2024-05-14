<script lang="ts">
	import { goto } from "$app/navigation";
	import Button from "$lib/components/ui/button/button.svelte";
    import { Label } from "$lib/components/ui/label";
	import { Separator } from "$lib/components/ui/separator";
	import type { Card } from "$lib/types";
	import SetCarosel from "$lib/ucomponents/set_carosel.svelte";
	import { Reload } from "svelte-radix";

    let paste_data = "";
    let generated_set: {
        cards: Omit<Card,'id'>[];
        title: string;
        descripition: string;

    } | null = null;
    let loading = false;
    let create_set_loading = false
    async function GenerateSet(){
        //parse comma seperated sets
        loading = true;
        let content = paste_data.split("\n").map((line)=>{
            let [front, back] = line.split(",");
            return {front, back}
        });
        loading = false;

        generated_set = {
            cards: content,
            title: "Generated Set",
            descripition: "This set was generated from a list of comma seperated values."
        }
        
        
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
        goto(`/sets/${body.id.$oid}`)
    }



    


    
</script>

<span class="flex flex-col gap-2 overflow-y-scroll">
    <Label for="file">Paste comma seperated data here.</Label>
    <textarea bind:value={paste_data} name="paste" class="w-full h-full"></textarea>
    {#if loading}
        <Button disabled>
            <Reload class="mr-2 h-4 w-4 animate-spin" />
            Creating Set..
        </Button>
    {:else}
        <Button variant="outline"  on:click={GenerateSet}>Generate</Button>
    {/if}
    {#if generated_set}
    <Separator />
    <h2>{generated_set.title}</h2>
    <p>{generated_set.descripition}</p>
    <SetCarosel cards={generated_set.cards}/>
        {#if create_set_loading}
            <Button disabled>
                <Reload class="mr-2 h-4 w-4 animate-spin" />
                Creating Set..
            </Button>
        {:else}
        <Button on:click={()=> {CreateSet()}}>Save Generated Set</Button>
        {/if}
    {/if}
</span>
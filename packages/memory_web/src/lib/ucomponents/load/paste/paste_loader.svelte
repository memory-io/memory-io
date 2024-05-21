<script lang="ts">
	import { goto } from "$app/navigation";
	import Button from "$lib/components/ui/button/button.svelte";
    import { Label } from "$lib/components/ui/label";
	import { Separator } from "$lib/components/ui/separator";
	import type { Card } from "$lib/types";
	import SetCarosel from "$lib/ucomponents/set_carosel.svelte";
	import { Reload } from "svelte-radix";
    import * as RadioGroup from "$lib/components/ui/radio-group";
	import Input from "$lib/components/ui/input/input.svelte";

    let card_term_seperator = "Comma";
    let card_seperator = "newline";
    let custom_card_term_separator = "";
    let custom_card_separator = "";

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
        let seperator: string = "";
        switch(card_term_seperator){
            case "Comma":
                seperator = ",";
                break;
            case "Tabs":
                seperator = "\t";
                break;
            case "Custom":
                seperator = custom_card_term_separator;
                break;
        }
        let card_sep:string = "";
        switch(card_seperator){
            case "semicolon":
                card_sep = ";";
                break;
            case "newline":
                card_sep = "\n";
                break;
            case "Custom":
                card_sep = custom_card_separator;
                break;
        }
        loading = true;
        let content = paste_data.split(card_sep).map((line)=>{
            let [front, back] = line.split(seperator);
          
            return {front, back}
        }).filter((a)=> a.front != undefined && a.back != undefined && a.front.length > 0 && a.back.length > 0);
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
    <div class="flex flex-row gap-3">
        <div>
            <Label>Card Term Seperator</Label>
        <RadioGroup.Root bind:value={card_term_seperator}>
            <div class="flex items-center space-x-2">
                <RadioGroup.Item value="Comma" id="Comma" />
                <Label for="Comma">Comma</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroup.Item value="Tabs" id="Tabs" />
                <Label for="Tabs">Tabs</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroup.Item value="Custom" id="Custom" />
                <Label for="Custom"><Input  bind:value={custom_card_term_separator}/></Label>
            </div>
        </RadioGroup.Root>
    </div>
    <div>
        <Label>Card Seperator</Label>
        <RadioGroup.Root bind:value={card_seperator}>
            <div class="flex items-center space-x-2">
                <RadioGroup.Item value="semicolon" id="semicolon" />
                <Label for="semicolon">;</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroup.Item value="newline" id="newline" />
                <Label for="newline">New Line</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroup.Item value="Custom" id="Custom" />
                <Label for="Custom"><Input class="w-min"  bind:value={custom_card_separator}/></Label>
            </div>
        </RadioGroup.Root>
        </div>
        
</div>

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
<script lang="ts">
	import * as Tooltip from "$lib/components/ui/tooltip";
	import { Circle } from "lucide-svelte";

 
    export let difficulty:number;


    const map =[
        {
            difficulty:5,
            title:"Very Hard",
            color:"#9e0000"
        },
        {
            difficulty:4,
            title:"Hard",
            color:"#9e3333"
        },
        {
            difficulty:3,
            title:"Normal",
            color:"white"
        },
        
        {
            difficulty:2,
            title:"Easy",
            color:"lightgreen"
        },
        
        {
            difficulty:1,
            title:"Very Easy",
            color:"green"
        }
    ]
</script>

<div class="flex flex-col justify-center text-center gap-2">

    <p>How difficult was this question?</p>
    <div class="flex flex-row justify-evenly " >
        {#each map as item}

            <button id={`${item.difficulty}`} on:click={() => difficulty = item.difficulty}><Tooltip.Root>
                <Tooltip.Trigger >
                    <Circle color={item.color} fill={difficulty == item.difficulty ? item.color: null}/>
                </Tooltip.Trigger>
                
                <Tooltip.Content>
                    <p>{item.title}</p>
                </Tooltip.Content>
                </Tooltip.Root>
            </button>
            
        {/each}
    </div>
</div>
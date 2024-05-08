
<script lang="ts">
	import Formatter from "$lib/formatter.svelte";
	import type { Card } from "$lib/types";
	import { ChevronLeft, ChevronRight } from "lucide-svelte";
	import { swipe, tap } from "svelte-gestures";

	
    
    export let cards:   Omit<Card,"id">[]  ;


    let card_index = 0;
    let front = true;
    $:cards_len = cards?.length ?? 0 ;

    $: {
        if (cards_len <= card_index){
            card_index = cards_len-1;
        }
    }
    
    function onCardSwipe(event:CustomEvent<{
    direction: "top" | "right" | "bottom" | "left";
    target: EventTarget;
    }>) {
        if (event.detail.direction == "left"){
            if (card_index < cards_len-1){
                card_index+=1;front=true;
            }
        }
        if (event.detail.direction == "right"){
            if (card_index > 0){
                card_index-=1;front=true;
            }
        }
    }
    function onCardTap(){
        front = !front;

    }
    
</script>


<div id={`${card_index}`} class=" h-[300px] bg-secondary rounded-md flex flex-row">
                
    <button class:grayed={card_index == 0} on:click={() =>{
        if (card_index > 0){
            card_index-=1;front=true;
        }
    }} class=" hidden sm:flex transition-opacity w-16 h-full  flex-col justify-center items-center border-r border-secondary-foreground border-opacity-40">
        <ChevronLeft />
    </button>
    <button use:tap={{ timeframe: 300 }} on:tap={onCardTap}  use:swipe={{ timeframe: 300, minSwipeDistance: 20, touchAction: 'pan-y' }} on:swipe={onCardSwipe}  class="relative p-10 flex justify-center items-center w-full" >
        
        <div class ="absolute left-2 top-2 text-secondary-foreground text-sm opacity-50">
            {`Card ${card_index+1} of ${cards_len}`}
        </div>
        <span class="wrap">
            <Formatter data={front ? cards[card_index].front:cards[card_index].back}/>
        </span>
    </button>

    <button class:grayed={card_index == cards_len-1} on:click={() => {
        if (cards_len >  (card_index+1)){
            card_index+=1;front=true;
        }
    }
    } class={"hidden sm:flex transition-opacity w-16 h-full flex-col justify-center items-center border-l border-secondary-foreground border-opacity-40"}>
        <ChevronRight />
    </button>

</div>
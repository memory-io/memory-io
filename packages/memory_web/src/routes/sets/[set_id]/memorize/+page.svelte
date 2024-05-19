<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card";
	import { GenerateRound } from "$lib/generator/learn";
	import Formatter from "$lib/ucomponents/formatter.svelte";
    import type { PageData } from './$types';
	import SetCarosel from "$lib/ucomponents/set_carosel.svelte";
	import MultipleChoice from "$lib/ucomponents/multiple-choice.svelte";

    export let data:PageData;
    let answered:string | null = null;
    

    let round = GenerateRound(data.memorize_data?? null, data.set!,5);

    let currentQuestion = 0;

    function selected(choice:string){
        console.log(choice);
    }

    function nextQuestion(){
        currentQuestion++;
        answered = null;
    }
    
</script>

<section class="flex flex-col gap-4">
    {#await round}
        <div>Generating your next round</div>
    {:then round} 
        <MultipleChoice question={round[currentQuestion].question} choices={round[currentQuestion].options} answer={round[currentQuestion].answer} selected={selected} answered={answered}/>
        {#if answered}
            <Button on:click={nextQuestion}>Next</Button>
        {/if}
    {/await}
    
</section>


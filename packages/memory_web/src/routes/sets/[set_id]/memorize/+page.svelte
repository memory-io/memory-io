<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card";
    
	import MultipleChoice from "$lib/ucomponents/multiple-choice.svelte";
	import {  MemorizeGenerator } from "$lib/generator/memorize.js";

	import Difficulty from "./difficulty.svelte";
    export let data;
    let generator = new MemorizeGenerator(data.memorize_data ?? null,data.set!);  
    generator.NextQuestion();
    
    let answered:string | null = null;
    if (data.set == undefined ){
        console.log("No set data")
        throw new Error("No set data")
        
    }
    let difficulty = 3;
    
    async function nextQuestion(){
        await generator.SubmitAnswer(answered!,difficulty);
        generator.NextQuestion();
        generator = generator;
        difficulty = 3;
        answered = null;
    }

    function selected(choice:string){
        answered = choice;
    }
    let results:Array<{question:string,correct:number,wrong:number}> | null = null;
    $: results = Array.from(generator.scores?.values()!).sort((a,b) => b.correct - a.correct).map((score) => {
        return {
            question:data.set.cards.find((a)=> a.id == score.id)!.front,
            correct:score.correct,
            wrong:score.wrong
        }
    });
    



    
    
</script>

<section class="flex flex-col gap-4">
    {#if generator.question && data.set != undefined }

    <MultipleChoice 
    question={generator.question.question} 
    choices={generator.question.options} 
    selected={selected}
    answered={answered}
    answer={generator.question.answer}
    />
    {#if answered != null}
    <div class="flex flex-row mx-4 justify-center gap-5">
        <div class="w-[20%]"></div>
        <div class="w-[60%] flex flex-row justify-center">
        <Difficulty bind:difficulty />
        </div>

      
        <div class="w-[20%] flex flex-row justify-end">
            <Button on:click={nextQuestion}>Next</Button>


        </div>
    </div>
    {/if}


    
    {/if}
    {#each results as score}
    {#if score.correct != 0 || score.wrong != 0}
    <Card.Root>
        <Card.Header>
            <Card.Title>{score.question}</Card.Title>
  
        </Card.Header>
        <Card.Content>
            <div class="flex flex-row max-w-full w-full">
                <div class="bg-green-600 h-6 flex flex-row justify-center" style="width: {score.correct * 10}%">
                    {#if score.correct != 0}
                    <p>{score.correct}</p>
                    {/if}
                </div>
                <div class="bg-red-600 h-6 flex flex-row justify-center" style="width: {score.wrong * 10}%">
                    {#if score.wrong != 0}
                    <p>{score.wrong}</p>
                    {/if}
                </div>
      
        </Card.Content>
   
    </Card.Root>
    {/if}
        
    {/each}

</section>


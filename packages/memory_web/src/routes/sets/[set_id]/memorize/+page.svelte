<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card";
    
	import MultipleChoice from "$lib/ucomponents/multiple-choice.svelte";
	import {  MemorizeGenerator } from "$lib/generator/memorize.js";

	import Difficulty from "./difficulty.svelte";
    export let data;
    let generator = new MemorizeGenerator(data.memorize_data ?? null,data.set!);  
    
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
    async function start(){
        generator.NextQuestion();
        generator = generator;
    }

    function selected(choice:string){
        answered = choice;
    }
    let results:Array<{question:string,correct:number,wrong:number,struggling: boolean,cooldown: number}> | null = null;
    $: results = Array.from(generator.scores?.values()!).map((score) => {
        return {
            question:data.set.cards.find((a)=> a.id == score.id)!.front,
            correct:score.correct,
            wrong:score.wrong,
            struggling: score.struggling,
            cooldown: generator.last_seen.findIndex((a) => a == score.id)
        }
    });
    



    
    
</script>

<section class="flex flex-col gap-4">
    {#if !generator.question}
    <div class="flex flex-row justify-center gap-4">
        <Button on:click={start}>Start</Button>
    </div>
    {/if}
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
    
    <Card.Root>
        <Card.Header>
            <Card.Title>Memorize Data</Card.Title>
        </Card.Header>
        <Card.Content class="flex flex-col gap-2">
            {#each results as score}
            {#if score.correct != 0 || score.wrong != 0}
            <div class="flex flex-row max-w-full w-full justify-between">
                <div class="overflow-hidden text-nowrap text-ellipsis">
                    <p>{score.question} {score.cooldown == -1 ? "":score.cooldown}</p>
                </div>

                <div class="w-96  h-6 flex-row flex justify-between">
                    <div class={'w-20 mr-2' + (score.struggling ? " text-red-500":"")}>
                        <p>{((score.correct/(score.wrong+score.correct))*100).toFixed(0)}%</p>
                    </div>
                    <div class="bg-green-600 h-6 flex flex-row justify-center rounded-md" style="width: {(score.correct/(score.wrong+score.correct))*100}%">
                        {#if score.correct != 0}
                        <p>{score.correct}</p>
                        {/if}
                    </div>
                    <div class="bg-red-600 h-6 flex flex-row justify-center rounded-md" style="width: {(score.wrong/(score.wrong+score.correct))*100}%">
                        {#if score.wrong != 0}
                        <p>{score.wrong}</p>
                        {/if}
                    </div>
                  
                </div>
            </div>
            {/if}
            {/each}
      
        </Card.Content>
   
    </Card.Root>
   

</section>


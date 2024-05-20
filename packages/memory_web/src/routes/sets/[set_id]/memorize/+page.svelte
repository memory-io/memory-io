<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card";
	import Formatter from "$lib/ucomponents/formatter.svelte";
	import { GenerateQuiz } from "$lib/generator/quiz.js";
	import MultipleChoice from "$lib/ucomponents/multiple-choice.svelte";
	import { GenerateRound, submitResults } from "$lib/generator/learn.js";
	import type { MemorizeCardQuestionData } from "$lib/types.js";
	import Slider from "$lib/components/ui/slider/slider.svelte";
	import Difficulty from "./difficulty.svelte";
    function shuffle(array: any[]) {
        let currentIndex = array.length;

        // While there remain elements to shuffle...
        while (currentIndex != 0) {

            // Pick a remaining element...
            let randomIndex = Math.floor(Math.random() * currentIndex);
            currentIndex--;

            // And swap it with the current element.
            [array[currentIndex], array[randomIndex]] = [
            array[randomIndex], array[currentIndex]];
        }
    }
    export let data;

    let currentQuestion = 0;
    let correct = 0;
    let wrong = 0;
    let results:MemorizeCardQuestionData[] = [];
    let answered:string | null = null;
    if (data.set == undefined ){
        console.log("No set data")
        throw new Error("No set data")
        
    }
    const quiz =  GenerateRound(data.memorize_data,data.set,data.set.cards.length/3);
    let difficulty = 3;
    let currentOptions;
    $:  {
        currentOptions = shuffle(quiz[currentQuestion].options);
        
    };
    async function nextQuestion(){
        results.push({
            card_id: quiz[currentQuestion].card_id,
            answer:quiz[currentQuestion].answer,
            difficulty: difficulty,
            correct:answered == quiz[currentQuestion].answer,
        });
        currentQuestion++;
        if (currentQuestion+1 >= quiz.length){
            console.log("submitting results")
            console.log(results);
            
            await submitResults(results,data.set!.id);
        }
        difficulty = 3;
        answered = null;
    }

    function selected(choice:string){
        answered = choice;
    }


    
    
</script>

<section class="flex flex-col gap-4">
    {#if quiz && data.set != undefined && currentQuestion+1 < quiz.length}

    <Card.Root>
        <Card.Header>
            <Card.Title class="flex justify-between">
                {data.set.title} Memorize                     
            </Card.Title>
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-3">
                <span>Question {currentQuestion + 1} of {quiz.length}</span>
                <span class="">
                </span>
            </div>
        </Card.Content>
    </Card.Root>

    <MultipleChoice 
    question={quiz[currentQuestion].question} 
    choices={quiz[currentQuestion].options} 
    selected={selected}
    answered={answered}
    answer={quiz[currentQuestion].answer}
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
    {#if quiz && data.set != undefined && currentQuestion+1 >= quiz.length}
    <Card.Root>
        <Card.Header>
            <Card.Title>Results</Card.Title>
  
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-3">
                <span>Correct: {correct}</span>
                <span>Wrong: {wrong}</span>
                <span>Percentage: {correct / (correct + wrong) * 100}%</span>
            </div>
            
        </Card.Content>
    </Card.Root>
    {/if}

</section>


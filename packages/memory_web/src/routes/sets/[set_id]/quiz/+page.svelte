<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card";
	import Formatter from "$lib/ucomponents/formatter.svelte";
	import { GenerateQuiz } from "$lib/generator/quiz.js";
	import QuizSettings from "./quiz-settings.svelte";
	import MultipleChoice from "$lib/ucomponents/multiple-choice.svelte";
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
    let answered:string | null = null;
    if (data.set == undefined ){
        throw new Error("No set data")
        
    }
    const quiz = GenerateQuiz(data.set.cards);
    
    let currentOptions;
    $:  {
        currentOptions = shuffle(quiz[currentQuestion].options);
        
    };
    function nextQuestion(){
        currentQuestion++;
        answered = null;
    }

    function selected(choice:string){
        answered = choice;
        if (choice == quiz[currentQuestion].answer){
            correct++;
            
        }else{
            wrong++;
            
        }

       
    }


    
    
</script>

<section class="flex flex-col gap-4">
    {#if quiz && data.set != undefined && currentQuestion+1 < quiz.length}

    <Card.Root>
        <Card.Header>
            <Card.Title class="flex justify-between">
                {data.set.title} Quiz                     
                <QuizSettings />
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
    <Button on:click={nextQuestion}>Next</Button>
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


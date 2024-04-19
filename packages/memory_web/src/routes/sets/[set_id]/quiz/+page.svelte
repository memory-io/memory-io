<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card";
	import Formatter from "$lib/formatter.svelte";
	import { GenerateQuiz } from "$lib/generator/quiz.js";
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
    if (data.set == undefined ){
        console.log("No set data")
        throw new Error("No set data")
        
    }
    const quiz = GenerateQuiz(data.set);
    let answered: string | null = null;
    
    let currentOptions;
    $:  {
        currentOptions = shuffle(quiz[currentQuestion].options);
        
    };


    
    
</script>

<section class="flex flex-col gap-4">
    {#if quiz && data.set != undefined && currentQuestion+1 < quiz.length}

    <Card.Root>
        <Card.Header>
            <Card.Title>{data.set.title} Quiz</Card.Title>
  
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-3">
                <div class="flex flex-row gap-3">
                    <span>Correct: {correct}</span>
                    <span>Wrong: {wrong}</span>
                </div>
                <span>Question {currentQuestion + 1} of {quiz.length}</span>
            </div>
            
        </Card.Content>
    </Card.Root>

    <Card.Root>
        <Card.Header>
            <span class="pl-4 text-wrap text-lg">
                <Formatter data={quiz[currentQuestion].question}/>
            </span>
  
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-3">
               
                <!-- Answer Choices -->
                <div class="grid grid-cols-2 gap-2 ">
                    {#each quiz[currentQuestion].options as choice}
                        <Button class={answered != null ?  choice == quiz[currentQuestion].answer? "bg-green-500 hover:bg-green-600" : "bg-red-500 hover:bg-red-600" : ""} on:click={()=>{
                            if (answered != null){
                                return;
                            }
                            
                            answered = choice
                            if (quiz != undefined && choice == quiz[currentQuestion].answer){
                                correct++;
                                
                            }else{
                                wrong++;
                            }
                        }}>
                            <Formatter data={choice}/>
                        </Button>
                    {/each}
                    

                </div> 
                {#if answered != null && currentQuestion + 1 != quiz.length}
                    <Button on:click={()=>{
                        answered = null;
                        currentQuestion++;
                    }}>
                        Next
                    </Button> 
                {/if}
                {#if answered != null && currentQuestion + 1 == quiz.length}
                    <Button on:click={()=>{
                        answered = null;
                        currentQuestion++;
                    }}>
                        Complete
                    </Button> 
                {/if}
               
            </div>
            
        </Card.Content>
    </Card.Root>
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


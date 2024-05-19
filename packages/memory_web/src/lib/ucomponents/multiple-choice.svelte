<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import Button from "$lib/components/ui/button/button.svelte";
    import Formatter from "$lib/ucomponents/formatter.svelte";
    export let question:string;
    export let choices:string[];
    export let selected: (choice:string) => void;
    export let answer:string;
    export let answered:string | null;
    
</script>

<Card.Root>
    <Card.Header>
        <span class="pl-4 text-wrap text-lg">
            <Formatter data={question}/>
        </span>

    </Card.Header>
    <Card.Content>
        <div class="flex flex-col gap-3">
               
            <!-- Answer Choices -->
            <div class="grid grid-cols-2 gap-2 ">
                {#each choices as choice}
                    <Button class={"h-24 text-primary bg-primary-foreground hover:bg-secondary "+(answered != null ? (answered == choice? "bg-green-800  hover:bg-green-700" : "bg-red-800 hover:bg-red-700" ): "")} on:click={()=>selected(choice)}>
                        <Formatter data={choice}/>
                    </Button>
                {/each}
                
        
            </div> 
            
           
        </div>
        
        
    </Card.Content>
</Card.Root>

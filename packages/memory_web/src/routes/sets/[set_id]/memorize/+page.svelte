<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card";
	import Formatter from "$lib/formatter.svelte";
    
    
</script>

<section class="flex flex-col gap-4">
    <Card.Root>
        <Card.Header>
            <Card.Title>Quiz</Card.Title>
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-3">
                <div class="flex flex-row gap-3">
                    <span>Correct: 0</span>
                    <span>Wrong: 0</span>
                </div>
                <span>Question 1 of 0</span>
            </div>
             
        </Card.Content>
        <Card.Footer>
            <Button>Next</Button>
        </Card.Footer>
    </Card.Root>
</section>


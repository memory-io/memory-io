<script lang="ts">
    import * as Card from "$lib/components/ui/card";

    export let data;
</script>

{#if data.set}
<section>
    <Card.Root>
        <Card.Header>
            <Card.Title>{data.set.title}</Card.Title>
            <Card.Description>{data.set.visibility}</Card.Description>
        </Card.Header>
        <Card.Content>
            <div class="flex flex-row">
                <div class="flex-1">
                    <Card.Title>{data.set.cards[0].front}</Card.Title>
                </div>
                <div class="flex-1">
                    <Card.Title>{data.set.cards[0].back}</Card.Title>
                </div>
            </div>
        </Card.Content>
    </Card.Root>
</section>
{/if}

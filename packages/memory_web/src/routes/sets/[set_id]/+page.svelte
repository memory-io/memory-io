


<script lang="ts">
    import { enhance } from "$app/forms";

    //https://www.shadcn-svelte.com/docs/components/card
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
   
    export let data;

</script>

<section class="flex flex-col w-8/12 h-full gap-5 justify-items-center ">
{#if data.set != undefined}
<Card.Root>
    <form method="POST" use:enhance={() => {
        return async ({ update }) => {
          update({ reset: false });
        };
      }}>
    <input name="id" value={data.set.id} hidden>
    <Card.Header>
        <Card.Title>{data.set?.title}</Card.Title>
        <Card.Description>{data.set?.visibility}</Card.Description>
    </Card.Header>
    <Card.Footer class="flex justify-end gap-3">
        <Button formaction="?/delete" type="submit" variant="outline">Delete</Button>
        <Button type="submit">Edit</Button>
        <Button formaction="?/add_card" type="submit">Add Card</Button>
    </Card.Footer>
    </form>

</Card.Root>
{#if data.set.cards != undefined}
{#each data.set.cards as card}
<Card.Root >
    <form method="POST" use:enhance={() => {
        return async ({ update }) => {
            update({ reset: false });
        };
    }}>
    <input name="id" value={card.id} hidden>
    <input name="set_id" value={data.set.id} hidden>
    <div class="card">
        <div class="card-side">
            <p class="card-content">{card.front}</p>
        </div>
        <div class="card-side">
            <p class="card-content">{card.back}</p>
        </div>
    </div>

    </form>

</Card.Root>
    
{/each}
{/if}
{/if}
{#if data.set == undefined}
<p>No sets found</p>
{/if}


</section>

<style>
    .card{
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        height: 150px;
        padding: 10px;
    }
    .card .card-side:nth-child(1){
        border-right: 2px solid #fff;
        align-items: flex-center;
        width: 50%;
    
    }
    .card-side .card-content{
        width: 100%;
        
        
        height: 100%;

    }

    .card-side{
        padding-left: 5px;
        padding-right: 5px;
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        gap: 10px;
    }
</style>
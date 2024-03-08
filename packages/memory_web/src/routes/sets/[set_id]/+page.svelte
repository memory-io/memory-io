


<script lang="ts">
    import { enhance } from "$app/forms";
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
	
	import SetCard from "./set-card.svelte";
    import { toast } from "svelte-sonner";
	import { user } from "$lib/store";

   
    export let data;
    export let form;
    let own_set = data.set?.user_id == $user?.id;

    $: {
        if (form?.error) {
            toast.error(form.error, {
            // description: "Sunday, December 03, 2023 at 9:00 AM",
            // action: {
            //     label: "Undo",
            //     onClick: () => console.log("Undo")
            // }
            });
        }else if (form?.status) {
            toast.success(form.status, {
            // description: "Sunday, December 03, 2023 at 9:00 AM",
            // action: {
            //     label: "Undo",
            //     onClick: () => console.log("Undo")
            // }
            });
        }
    }

</script>

<section class="flex flex-col max-w-screen-md h-full gap-5 justify-items-center ">
{#if data.set != undefined}
<Card.Root>
    <form method="POST" use:enhance={() => {
        return async ({ update }) => {
          update({ reset: false });
        };
      }} >
    <input name="id" value={data.set.id} hidden>
    <Card.Header>
        <Card.Title>{data.set?.title}</Card.Title>
        <Card.Description>{data.set?.visibility}</Card.Description>
    </Card.Header>
    <Card.Footer class="flex justify-end gap-3">
        
        <Button type="submit">Edit</Button>
        <Button formaction="?/delete" type="submit" variant="destructive">Delete</Button>
   
    </Card.Footer>
    </form>

</Card.Root>
{#if data.set.cards != undefined}
{#each data.set.cards as card}
<Card.Root >
    <form id={card.id} method="POST" use:enhance={() => {
        return async ({ update }) => {
            update({ reset: false });
        };
    }}>
    <input name="id" value={card.id} hidden>
    <SetCard default_editable={false} card={card}/>
    
    

    </form>

</Card.Root>

{/each}
<Card.Root >
<form id="new-card"  method="POST" use:enhance>
<SetCard default_editable={true} card={null}/>
</form>
</Card.Root>
{/if}
{/if}
{#if data.set == undefined}
<p>No sets found</p>
{/if}


</section>

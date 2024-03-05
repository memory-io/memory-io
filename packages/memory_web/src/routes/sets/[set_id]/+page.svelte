


<script lang="ts">
    import { enhance } from "$app/forms";
    import * as Popover from "$lib/components/ui/popover";
    import SimpleLineIconsOptionsVertical from '~icons/simple-line-icons/options-vertical';
    //https://www.shadcn-svelte.com/docs/components/card
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
	import Input from "$lib/components/ui/input/input.svelte";
	import { Label } from "$lib/components/ui/label";
	import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import SetCard from "./set-card.svelte";
    import { toast } from "svelte-sonner";

   
    export let data;
    export let form;

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

<section class="flex flex-col w-8/12 h-full gap-5 justify-items-center ">
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

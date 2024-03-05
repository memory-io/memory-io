


<script lang="ts">
    import { enhance } from "$app/forms";

    import * as Dialog from "$lib/components/ui/dialog";

    //https://www.shadcn-svelte.com/docs/components/card
    import * as Card from "$lib/components/ui/card";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";
    import * as Select from "$lib/components/ui/select";
	import type { Selected } from "bits-ui";
	import { slide } from "svelte/transition";
    import { toast } from "svelte-sonner";


    let selected: Selected<string> = {
        value:"Public",
        label:"Public"
    };
    export let data;
    export let form;
    let dialogOpen = false;
    if (form?.error){
        toast.success("Error creating Set", {
            description: form?.error,
            action: {
                label: "Undo",
                onClick: () => console.log("Undo")
            }
            }
        )
    }
    
</script>

<section class="flex flex-col gap-5 justify-items-center">
    <div class="w-min">
    <Button on:click={() => (dialogOpen = true)}>Create Set</Button>
    </div>

<Dialog.Root bind:open={dialogOpen}>
    
    <Dialog.Content>
        <Dialog.Header>
        <Dialog.Title>New Set</Dialog.Title>
        <Dialog.Description>
            Save your study set.
        </Dialog.Description>
        </Dialog.Header>
        <form method="POST" action="?/create" use:enhance={() => {
            return async ({ update }) => {
                update({ reset: false });
            };
            }}>
            <Label for="title">Title</Label>
            <Input name="title" type="text" placeholder="Title" />
            <br>
            <Label >Visibility</Label>
            <input name="visibility" value={selected.value} hidden>
            <Select.Root selected={selected} onSelectedChange={(v) => {
                v && (selected = v);
            }}>
                <Select.Trigger class="w-[180px]">
                    <Select.Value  placeholder="Visibility" />
                </Select.Trigger>
                <Select.Content>
                    <Select.Item value="Private">Private</Select.Item>
                    <Select.Item value="Public">Public</Select.Item>
                </Select.Content>
            </Select.Root>
            <Dialog.Footer>
                <Button variant="destructive" on:click={() => {dialogOpen =false}}>Cancel</Button>
                <Button type="submit" variant="default" on:click={() => {dialogOpen =false}}>Create</Button>
            </Dialog.Footer>

        </form>

    </Dialog.Content>
</Dialog.Root>

{#if data.sets != undefined}
{#each data.sets as set}
<Card.Root >
    <form method="post" use:enhance>

    <Card.Header>
        <a href={`/sets/${set.id}`}>
        <Card.Title>{set.title}</Card.Title>
        </a>
        <Card.Description>{set.visibility}</Card.Description>
    </Card.Header>
    <Card.Content>
        
    </Card.Content>
    <Card.Footer class="flex justify-between">
        
        <Button type="submit" formaction={`/sets/${set.id}?/delete`} variant="outline">Delete</Button>
        
    </Card.Footer>
    </form>


</Card.Root>
    
{/each}
{/if}
</section>
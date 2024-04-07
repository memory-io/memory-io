


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
	import { createSet, deleteSet } from "$lib/api/sets";
	import { error, redirect } from "@sveltejs/kit";
	import { invalidate, invalidateAll } from "$app/navigation";


    let selected: Selected<string> = {
        value:"Public",
        label:"Public"
    };
    export let data;
    let dialogOpen = false;

    async function formCreateSet(e: SubmitEvent & {
    currentTarget: EventTarget & HTMLFormElement;
}){
        const formData = new FormData(e.currentTarget);
        const data:{[id: string]:string} = {};
        for (let field of formData) {
            const [key, value] = field;
            data[key] = value.toString();
        }
        let out = await createSet(data["title"],data["visibility"]);
        if (out.error){
            if (out.error == 401){
                window.location.href = "/auth/login";
            }else {
                toast.error(out.error.toString());
            }
        }else if (out.set){
            dialogOpen = false;
            window.location.href = `/sets/${out.set.id}`;
        }
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
        <form on:submit|preventDefault={formCreateSet}>
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
                <Button type="submit" variant="default" >Create</Button>
            </Dialog.Footer>

        </form>

    </Dialog.Content>
</Dialog.Root>

{#if data.sets != undefined}
{#each data.sets as set}
<Card.Root >

    <Card.Header>
        <a href={`/sets/${set.id}`}>
        <Card.Title>{set.title}</Card.Title>
        </a>
        <Card.Description>{set.visibility}</Card.Description>
    </Card.Header>
    <Card.Content>
        
    </Card.Content>
    <Card.Footer class="flex justify-between">
        
        <Button type="submit" on:click={async ()=>{
            let out = await deleteSet(set.id);
            if (out.error){
                toast.error(out.error);
            }else{
                invalidate((url) => {
                    console.log(url.pathname);
                    return url.pathname === "/api/sets"
                });
            }

        }} variant="outline">Delete</Button>
        
    </Card.Footer>


</Card.Root>
    
{/each}
{/if}
</section>
<script lang="ts">

    import * as Dialog from "$lib/components/ui/dialog";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";
    import * as Select from "$lib/components/ui/select";
    import { toast } from "svelte-sonner";
	import { createSet } from "$lib/api/sets";
	import type { Selected } from "bits-ui";
    let dialogOpen = false;
    let selected: Selected<string> = {
        value:"Public",
        label:"Public"
    };

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
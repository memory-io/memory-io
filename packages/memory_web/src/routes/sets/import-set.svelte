<script lang="ts">

    import * as Dialog from "$lib/components/ui/dialog";
    import * as Tabs from "$lib/components/ui/tabs";

    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";
	import QuizletLoader from "$lib/ucomponents/load/quizlet/quizlet_loader.svelte";
	import PasteLoader from "$lib/ucomponents/load/paste/paste_loader.svelte";

    let dialogOpen = false;
    

</script>

<div class="w-min">
    <Button variant="link" on:click={() => (dialogOpen = true)}>Import Set</Button>
</div>

<Dialog.Root bind:open={dialogOpen}>
   
        
    <Dialog.Content>
        <Dialog.Header>
            <Dialog.Title>Import Set</Dialog.Title>
            <Dialog.Description>
                Import a study set from somewhere else.
            </Dialog.Description>
        </Dialog.Header>
    <Tabs.Root value="paste" class="w-[400px] h-min max-h-[80%]" >
        <Tabs.List class="w-full">
            <Tabs.Trigger value="paste" class="w-full">Paste</Tabs.Trigger>
            <Tabs.Trigger value="file" class="w-full">File</Tabs.Trigger>
            <Tabs.Trigger value="quizlet" class="w-full">Quizlet</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="paste">
           <PasteLoader/>
        </Tabs.Content>
        
        <Tabs.Content value="file">
            <Label for="file">Import a CSV File.</Label>
            <Input name="file" type="file" placeholder="File" />
        </Tabs.Content>
        <Tabs.Content value="quizlet">
            <QuizletLoader/>
        </Tabs.Content>
        </Tabs.Root>
    </Dialog.Content>
</Dialog.Root>
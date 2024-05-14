<script lang="ts">

    import * as Dialog from "$lib/components/ui/dialog";
    import * as Tabs from "$lib/components/ui/tabs";

    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";
	import { Settings, SettingsIcon } from "lucide-svelte";

    let dialogOpen = false;
    

</script>

<div class="w-min">
    <Button variant={"ghost"} size="icon" on:click={() => (dialogOpen = true)}> <SettingsIcon /></Button>

</div>

<Dialog.Root bind:open={dialogOpen}>
   
        
    <Dialog.Content>
        
   
    </Dialog.Content>
</Dialog.Root>
<script lang="ts">
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { MoreVertical } from "lucide-svelte";
    import { toast } from "svelte-sonner";
    import { deleteSet, updateSet } from "$lib/api/sets";
    import { invalidate } from "$app/navigation";
    export let set;
    export let own_set;
    
    let is_public = set.visibility == "Public"
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger>
        <MoreVertical />
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-56">
        <DropdownMenu.Label>Card Actions</DropdownMenu.Label>
        <DropdownMenu.Group>
            <DropdownMenu.Item>
                Fork
            </DropdownMenu.Item>
            <DropdownMenu.Item>
                Embed
            </DropdownMenu.Item>
        </DropdownMenu.Group>

        {#if own_set}
        <DropdownMenu.Separator />

        <DropdownMenu.Label>Owner Actions</DropdownMenu.Label>

        <DropdownMenu.Group>
            
            <DropdownMenu.Item on:click={async ()=>{
                let out = await deleteSet(set?.id ??"nol");
                if (!out.error){
                    window.location.href = "/sets";
                    invalidate("/api/sets")
                }else{
                    toast.error(out.error);
                }
            }}>
                Delete
            </DropdownMenu.Item>
            <DropdownMenu.CheckboxItem on:click={async ()=>{
                set.visibility = is_public ? "Private" : "Public";
                let out = await updateSet(set);
                if (!out.error){
                    invalidate("/api/sets")
                }else{
                    toast.error(out.error);
                }
            }} bind:checked={is_public}>
                Public
            </DropdownMenu.CheckboxItem>
        </DropdownMenu.Group>
        {/if}
    </DropdownMenu.Content>
</DropdownMenu.Root>
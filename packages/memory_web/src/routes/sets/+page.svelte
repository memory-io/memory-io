


<script lang="ts">


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
	import { invalidate } from "$app/navigation";
	import NewSet from "./new-set.svelte";
	import ImportSet from "./import-set.svelte";
    import * as Tooltip from "$lib/components/ui/tooltip";



    export let data;

   
   
    
</script>

<section class="flex flex-col gap-5 justify-items-center">
    <div class="flex flex-row gap-5 justify-start">
        <NewSet/>
        <ImportSet/>
        {#if data.user.paid_user}
        <Button variant="link" href="/sets/generation">Generate Set</Button>
        {:else}
        <Tooltip.Root>
        <Tooltip.Trigger>        
            <Button disabled variant="link"  >Generate Set</Button>
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p> Upgrade to <a href="/upgrade">premium</a></p>
        </Tooltip.Content>
        </Tooltip.Root>


        {/if}
    </div>

    

    

    {#if data.sets != undefined}
    {#each data.sets as set}
    <Card.Root >

        <Card.Header>
            <a href={`/sets/${set.id.$oid}`}>
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
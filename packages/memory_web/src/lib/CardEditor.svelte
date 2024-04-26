<script lang="ts">
	import { Sigma } from "lucide-svelte";
	import { Button } from "./components/ui/button";
	import Textarea from "./components/ui/textarea/textarea.svelte";
    import * as Card from "$lib/components/ui/card";
	import type { FormEventHandler } from "svelte/elements";

    export let value:string;
    export let disabled:boolean;

 
 
    
</script>

<textarea disabled={disabled} class="h-full" bind:value={value}  />


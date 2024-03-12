<script lang="ts">
    import markedKatex from "marked-katex-extension";
    import { marked } from 'marked'
    import { Render } from '@jill64/svelte-sanitize'
	import Avatar from "./components/ui/avatar/avatar.svelte";
    export let data: string;
    const options = {
    throwOnError: true
    };
    
    marked.use(markedKatex(options))

    $: rendered_data = marked.parse(data);

</script>


{#await rendered_data }
    <h1>Loading..</h1>
{:then cleaned} 
    <Render  html={cleaned}  />
    
{/await}



<style>

</style>



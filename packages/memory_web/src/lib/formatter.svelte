<script lang="ts">
    import markedKatex from "marked-katex-extension";
    import { marked } from 'marked'
    import { Render } from '@jill64/svelte-sanitize'
    export let data: string;
    const options = {
    throwOnError: false
    };
    marked.use(markedKatex(options));


    

</script>

{#await marked.parse(data)}
    <h1>Loading..</h1>
{:then cleaned} 
    <Render html={cleaned}  />
    
{/await}







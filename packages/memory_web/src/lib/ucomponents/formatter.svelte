<script lang="ts">
    import markedKatex from "marked-katex-extension";
    import { Marked } from 'marked'
    export let data: string;
    const options = {
    throwOnError: true
    };

    const marked = new Marked(
        markedKatex(options)
    );
    

    let html: string | Promise<string>;
    $: html = marked.parse(data);

</script>


{#await html }
    <h1>Loading..</h1>
{:then cleaned} 
    <markdown >
    {@html cleaned}
    </markdown>
{/await}



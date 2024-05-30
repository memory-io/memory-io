<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    export let data;
    console.log(data.recent_sets)
</script>

<section class="m-5 flex flex-col gap-x-32">
<h1>Explore</h1>

<Card.Root>
    <Card.Header>
        <Card.Title>Recently Added</Card.Title>
        <Card.Description>These our the most recent sets added.</Card.Description>
    </Card.Header>
    <Card.Content class="flex flex-row gap-3 flex-wrap">
        {#each data.recent_sets as set}
        <a href={`/sets/${set.id.$oid}`}>
            <Card.Root class="w-64" >
                <Card.Header >
                    <Card.Title >{set.title}</Card.Title>
                    <Card.Description>
                        {set.description}
                        {set.user?.username ?? "Unknown"}
                    </Card.Description>
                </Card.Header>
            </Card.Root>
        </a>
        {/each}
        
    </Card.Content>
</Card.Root>

<Card.Root>
    <Card.Header>
        <Card.Title>Friends Recent Sets</Card.Title>
        <Card.Description>These our the most recent sets added by your friends</Card.Description>
    </Card.Header>
    <Card.Content class="flex flex-row gap-3 flex-wrap">
        {#each data.recent_sets as set}
        <Card.Root class="w-64">
            <Card.Header>
                <Card.Title>{set.title}</Card.Title>
                <Card.Description>{set.description}</Card.Description>
            </Card.Header>
        </Card.Root>
        {/each}
    </Card.Content>
</Card.Root>

</section>
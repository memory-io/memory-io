<script lang="ts">
    import * as Card from "$lib/components/ui/card";
	import { Label } from "$lib/components/ui/label";


</script>

<section >
    <Card.Root class="flex flex-row w-full p-5 justify-between">
        <Card.Title tag="h1" class="title-gradient">Memory</Card.Title>
        <Card.Content>
            
            <Label>Email</Label>
            <Label>Password</Label>
        </Card.Content>
    </Card.Root>
</section>

<style>
    .header{
        border: solid;
        width: 100%;
    }
    .title-gradient{
        background: linear-gradient(90deg, #bd0000 0%, #0004ff 50%, #bd0000 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }
    

    section{
        display: flex;
        flex-direction: row;
        width: 100%;

      
    }

</style>
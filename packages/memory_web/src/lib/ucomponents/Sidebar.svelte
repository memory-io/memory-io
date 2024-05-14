<script lang="ts">
    import Label from "$lib/components/ui/label/label.svelte";
    import * as Card from "$lib/components/ui/card";
</script>

<section >
    <Card.Root >
        <Card.Header>
            <Card.Title tag="h1">Account</Card.Title>
        </Card.Header>
        <Card.Footer>
            <Label>Username</Label>
            <Label>Email</Label>
            <Label>Password</Label>
        </Card.Footer>
    </Card.Root>
</section>

<style>
   
    section{

        display: flex;
        flex-direction: column;
        width: 25%;
    }

</style>
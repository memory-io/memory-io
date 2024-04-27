<script>
    import * as Card from "$lib/components/ui/card";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";
	import { toast } from "svelte-sonner";
    import Reload from "svelte-radix/Reload.svelte";


    let email = "";
    let loading = false;
    async function reset(){
        loading = true;
        const res = await fetch("/api/users/password_reset", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ email }),
        });
        loading = false;

        if (res.ok) {
            toast.success("Check your email for a password reset link.");
        } else {
            toast.error(await res.text());
        }
    }
</script>


<div class="container">
  <Card.Root class="w-[350px]">
    <Card.Header>
      <Card.Title>Password Reset</Card.Title>
      <Card.Description>If you have an account we will send you an email to reset your password.</Card.Description>
   
    </Card.Header>
    
    <form on:submit|preventDefault>
        <Card.Content>
            <!-- Your signup form code here -->
            <Label for="email">Email for account</Label>
            <Input name="email" bind:value={email} type="email" placeholder="Email" />
        </Card.Content>
        <Card.Footer class="flex justify-between">
            {#if loading}
                <Button disabled>
                    <Reload class="mr-2 h-4 w-4 animate-spin" />
                    Please wait
                </Button>
            {:else}
                <Button on:click={() => reset()} >Reset</Button>
            {/if}
           
        </Card.Footer>
    </form>
  </Card.Root>
  
</div>


<style>
  .form{
    display: flex;
    flex-direction: column;
    gap: 1rem;
    background-color: var(--accent);
    border-radius: 10px;
    padding: 10px;
    width: 300px;

  }
  .container {
    display: flex;
    justify-content: center;
    
    align-items: center;
    height: 100%;
  }
</style>

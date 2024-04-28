<script>
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Button } from "$lib/components/ui/button";
	import { toast } from "svelte-sonner";
	import { Reload } from "svelte-radix";
  import { passwordStrength } from 'check-password-strength'



  let email = "";
  let password = "";
  let loading = false;

  $: password_stength = passwordStrength(password);
  console.log(password_stength)

  async function login(){
    loading = true;
    const res = await fetch("/api/users/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    });
    loading = false;

    if (res.ok) {
      toast.success("Logged in successfully redirecting shortly.");
      setTimeout(() => {
        window.location.href = "/";
      }, 1000);
    } else {
      toast.error(await res.text());
    }
  }
</script>


<div class="container">
  <Card.Root class="w-[350px]">
    <Card.Header>
      <Card.Title>Login to your account.</Card.Title>
      <Card.Description>Save your study sets.</Card.Description>
    </Card.Header>
    <form on:submit|preventDefault>
      <Card.Content>
          <!-- Your signup form code here -->
          <Label for="email">Email</Label>
          <Input name="email" bind:value={email} type="email" placeholder="Email" />
          <br>
          <Label for="password">Password</Label>
          <Input name="password" bind:value={password} type="password" placeholder="Password" />
      </Card.Content>
      <Card.Footer class="flex justify-between">
        <Button variant="outline" href="/auth/password_reset">Reset Password</Button>
        {#if loading}
                <Button disabled>
                    <Reload class="mr-2 h-4 w-4 animate-spin" />
                    Please wait
                </Button>
          {:else}
            <Button on:click={() => login()} type="submit">Login</Button>
          {/if}

      </Card.Footer>
    </form>
  </Card.Root>
  
</div>


<style>

  .container {
    display: flex;
    justify-content: center;
    
    align-items: center;
    height: 100%;
  }
</style>

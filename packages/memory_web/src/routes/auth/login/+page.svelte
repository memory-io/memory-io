<script>
	import { enhance } from "$app/forms";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Button } from "$lib/components/ui/button";
	import { toast } from "svelte-sonner";


  let email = "";
  let password = "";

  async function login(){
    const res = await fetch("/api/users/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    });

    if (res.ok) {
      window.location.href = "/";
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
        <Button variant="outline">Cancel</Button>
        <Button on:click={() => login()} type="submit">Login</Button>
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

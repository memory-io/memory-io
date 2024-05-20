<script lang="ts">
	import { enhance } from "$app/forms";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Button } from "$lib/components/ui/button";
	import { user } from "$lib/store";
	import { toast } from "svelte-sonner";

  let username: string = "";
  let email: string = "";
  let password: string = "";
  let username_error: string = "";
  let valid_username = false;
  $: {
    if (username.length < 3) {
      valid_username = false;
      username_error ="More chars please"
    }
    else if(username.length > 20){
      valid_username=false;
      username_error ="Not that many!"
    }else{
      check_username(username).then((res) => {
        valid_username = true;
        username_error ="";
      }).catch((e) => {
        valid_username = false;
        username_error = e.message;
      });
      
    }
  };

  async function signup(){
    const res = await fetch("/api/users/signup", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, username, password }),
    });

    if (res.ok) {
      toast.success("Succesfully signed up redirecting shortly.");
      setTimeout(() => {
        window.location.href = "/";
      }, 1000);
    } else {
      toast.error (await res.text());
    }
  }

  async function check_username(username: string){
    console.log(username)
    const response = await fetch(`/api/users/check_username/${username}`)
    if (response.ok) {
      console.log("valid")
      return true;
    } else if (response.status === 409) {
      throw new Error(await response.json());
      //return response.json();
    }
    throw new Error("Failed to connect to Server");
  }
</script>


<span class="signup-container">
  <Card.Root class="w-[350px]">
    <Card.Header>
      <Card.Title>Create an account</Card.Title>
      <Card.Description>Save your study sets.</Card.Description>
    </Card.Header>
    <form on:submit|preventDefault >
      <Card.Content>
          <!-- Your signup form code here -->
          <Label for="email">Email</Label>
          <Input bind:value={email} name="email" type="email" placeholder="Email" />
          <br>
          <Label for="username">Username <span class="text-rose-500">{username_error}</span></Label>
          
            <Input bind:value={username} class={valid_username ? "border-green-500": "border-rose-500"} name="username" type="username" placeholder="Username" />

          <br>
          <Label for="password">Password</Label>
          <Input bind:value={password} name="password" type="password" placeholder="Password" />
      </Card.Content>
      <Card.Footer class="flex justify-between">
        <Button variant="outline">Cancel</Button>
        <Button on:click={() => signup()} type="submit">Signup</Button>
      </Card.Footer>
    </form>
  </Card.Root>
  
</span>


<style>
  
  .signup-container {
    display: flex;
    justify-content: center;
    padding: 0;
    
    align-items: center;
    height: 100%;
  }
</style>

<script lang="ts">
    import { enhance } from "$app/forms";
    import * as Card from "$lib/components/ui/card";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";
	import { user } from "$lib/store";
	import { toast } from "svelte-sonner";
    export let data;
    const token = data.token;
    let password: string = "";
    let check_password: string = "";
    let valid_password = false;
    $: { if (password.length < 8 || password.length > 20 || password !== check_password) {
        valid_password= false;
        } else {
            valid_password= true;
        }}
    
    async function change_password(){
        
        const res = await fetch("/api/users/change_password", {
            method: "POST",
            headers: {
            "Content-Type": "application/json",
            },
            body: JSON.stringify({ token, password }),
            });
        if (res.ok) {
            toast.success("Password changed successfully.");
            window.location.href = "/auth/login";
        } else {
            toast.error(await res.text());
        }
    }
</script>


<span class="signup-container">
    <Card.Root class="w-[350px]">
        <Card.Header>
        <Card.Title>Reset your password</Card.Title>
        </Card.Header>
        <form on:submit|preventDefault >
            <Card.Content>
                <!-- Your signup form code here -->
                <Label for="password">New Password</Label>
                <Input bind:value={password} name="password" type="password" placeholder="Password" />
                <br>
                <Label for="check_password">Verify Password</Label>

                <Input  type="password" bind:value={check_password} class={valid_password ? "border-green-500": "border-rose-500"} name="check_password"  placeholder="Verify Password" />

            </Card.Content>
            <Card.Footer class="flex justify-between">
                <Button on:click={() => change_password()} type="submit">Change</Button>
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

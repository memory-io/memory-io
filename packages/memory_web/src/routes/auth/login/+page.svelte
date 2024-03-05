<script>
	import { enhance } from "$app/forms";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Button } from "$lib/components/ui/button";

</script>


<div class="container">
  <Card.Root class="w-[350px]">
    <Card.Header>
      <Card.Title>Login to your account.</Card.Title>
      <Card.Description>Save your study sets.</Card.Description>
    </Card.Header>
    <form method="POST" use:enhance>
      <Card.Content>
          <!-- Your signup form code here -->
          <Label for="email">Email</Label>
          <Input name="email" type="email" placeholder="Email" />
          <br>
          <Label for="password">Password</Label>
          <Input name="password" type="password" placeholder="Password" />
      </Card.Content>
      <Card.Footer class="flex justify-between">
        <Button variant="outline">Cancel</Button>
        <Button type="submit">Login</Button>
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

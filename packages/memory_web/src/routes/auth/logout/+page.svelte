<script>
	import { enhance } from "$app/forms";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Button } from "$lib/components/ui/button";
	import { invalidate } from "$app/navigation";

  invalidate((url) => url.pathname.includes("users"));
  setTimeout(() => {
    window.location.href = "/";
  }, 2000);

</script>


<span class="login-container">
  <Card.Root class="max-w-[300px] sm:p-0">
    <Card.Header>
      <Card.Title>Goodbye </Card.Title>
      <Card.Description>Save your study sets.</Card.Description>
    </Card.Header>
    
  </Card.Root>
  
</span>


<style>
 
  .login-container {
    display: flex;
    justify-content: center;
    padding: 0;
    
    align-items: center;
    height: 100%;
  }
</style>

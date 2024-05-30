<script lang="ts">
	import { deleteUser } from "$lib/api/user";
	import Button from "$lib/components/ui/button/button.svelte";
	import { toast } from "svelte-sonner";


</script>
<h1>Settings</h1>

<Button variant="destructive" on:click={async () => {
	if (confirm("Are you sure you want to delete your account?"))
	{
		let response = await deleteUser();
		if (response.ok) {
			toast.info("Your account has been deleted.");
			
		} else {
			toast.error("An error occurred while deleting your account.")
		}


	}}}>Delete account</Button>


<script lang="ts">
	import { cn } from "$lib/utils";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import { Button } from "$lib/components/ui/button";
	import { page } from '$app/stores';  
	import type { User } from "$lib/types";
	let className: string | undefined | null = undefined;
	export { className as class };
	$: current_url = "/"+$page.url.pathname.split("/")[1];

	
	const urls = new Map([
		["/", "Explore"],
		["/sets", "Your Sets"],
		["/a", "Create"],
	]);
</script>

<nav class={cn("flex  items-center space-x-4 lg:space-x-6", className)}>
	{#each urls as url }
		<a href={url[0]} class={"text-sm font-medium transition-colors hover:text-primary "+(url[0]!=current_url ? "text-muted-foreground" : "")}>
			{url[1]}
		</a>
	{/each}
	
</nav>
<div class="dropdown space-x-4 lg:space-x-6 mx-6">
<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		<Button variant="outline">{urls.get(current_url) == undefined ? urls.get("/") : urls.get(current_url)}</Button>

		
	</DropdownMenu.Trigger>
	<DropdownMenu.Content class="w-56">
		<DropdownMenu.Label>{urls.get(current_url) == undefined ? urls.get("/") : urls.get(current_url)}</DropdownMenu.Label>
		<DropdownMenu.Separator />
	
		<DropdownMenu.Group>
			{#each urls as url }
				<DropdownMenu.Item href={url[0]}>
					{url[1]}
				</DropdownMenu.Item>
			{/each}
		

		</DropdownMenu.Group>
		
	</DropdownMenu.Content>
</DropdownMenu.Root>
</div>

<style>
	nav {
		display: flex
	}
	.dropdown{
		display: none;
	}

	/* On screens that are 992px or less, set navbar to horizontal */
	@media screen and (max-width: 720px) {
	nav {
		display: none;
	}
	.dropdown{
		display: block;
	}
	}
	
</style>
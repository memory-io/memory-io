<script lang="ts">
	import { cn } from "$lib/utils";

	let className: string | undefined | null = undefined;
	export { className as class };
</script>

<nav class={cn("flex items-center space-x-4 lg:space-x-6", className)}>
	<a href="/" class="text-sm font-medium transition-colors hover:text-primary">
		Overview
	</a>


	<a
		href="/sets"
		class="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
	>
		Your Sets
	</a>
	<a
		href="/create-set"
		class="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
	>
		Create
	</a>
</nav>
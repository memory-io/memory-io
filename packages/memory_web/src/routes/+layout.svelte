<script>
	import Sidebar from '$lib/Sidebar.svelte';
	import Header from '$lib/Header.svelte';
	


	import '../app.pcss';
	import Search from './search.svelte';
	import UserNav from './user-nav.svelte';
	import MainNav from './main-nav.svelte';
</script>
<div class="hidden flex-col md:flex">
	<div class="border-b">
		<div class="flex h-16 items-center px-4">
			<h1 class="ml-10">Memory</h1>
			<MainNav class="mx-6 ml-20" />
			<div class="ml-auto flex items-center space-x-4">
				<Search />
				<UserNav />
			</div>
		</div>
	</div>
	<div class="flex-1 space-y-4 p-8 pt-6">
		<slot/>
		
	</div>
</div>


<style>
	
</style>

<script lang="ts">
	import '../app.pcss';
	import Search from './search.svelte';
	import UserNav from './user-nav.svelte';
	import MainNav from './main-nav.svelte';

	import { Toaster } from "$lib/components/ui/sonner";
	export let data;


</script>
<Toaster />
<div class="flex-col flex w-lvw h-lvh">
	<div class="border-b">
		<div class="flex h-16 items-center px-4">
			<h1 class="md:ml-10 " ><a href="/" >M3m0ry</a></h1>
			<MainNav  class="mx-6 ml-20 w-max min-w-max " />
			<span class="w-full"></span>
			<div class="w-full flex-row flex items-center justify-end mr-4 md:mr">
				<span class=" min-w-[200px] max-w-[300px] w-full sm:block hidden">
				<Search  />
				</span>
				<span class="pl-2">
				<UserNav user={data.user} />
				</span>
			</div>
		</div>
	</div>
	<div class="flex-1 ">
	
		<slot/>
		
	</div>
</div>


<style>
	
</style>

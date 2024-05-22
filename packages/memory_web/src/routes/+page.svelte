<script>
	import Button from "$lib/components/ui/button/button.svelte";

</script>
<section class ="bg-gradient-to-r from-slate-600 to-slate-900 w-full h-full flex flex-row justify-center">
    <div class="flex flex-col justify-center text-center gap-4">
        <p class="text-6xl">m3m0ry.io</p>
        <p>An open place to save your study sets.</p>
        <div class="flex flex-row justify-center">
            <Button href="/explore" variant="link">Explore</Button>
            <Button href="/auth/signup" variant="link">Sign Up</Button>
        </div>
    </div>
</section>
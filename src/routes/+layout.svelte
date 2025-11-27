<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { theme } from '$lib/theme-switcher';

	let { children } = $props();

	onMount(() => {
		theme.initializeTheme();
	});
</script>

{@render children()}

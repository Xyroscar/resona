<script lang="ts">
	onMount(() => {
		check_onboarding().then((response) => {
			if (response) {
				goto('/workspaces');
			} else {
				goto('/onboarding');
			}
		});
	});
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { check_onboarding } from '$lib/services/onboarding';
</script>

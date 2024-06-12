<script lang="ts">
	import {
		CardsAlertSettings,
		CardsScores,
		CardsAbout,
		CardsIosSettings,
		isIOSWithNotifications
	} from './(components)/index.js';
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Toaster } from '$lib/components/ui/sonner';
	let needs_pin = false;

	onMount(async () => {
		needs_pin = isIOSWithNotifications();
	});
</script>

<div class="container mx-auto px-4 sm:px-6 md:max-w-7xl md:px-8">
	<div
		class="items-start justify-center gap-x-5 rounded-lg p-4 sm:p-6 md:grid md:grid-cols-1 md:p-8 lg:grid-cols-2 xl:grid-cols-2"
	>
		<div class="col-span-1 mb-4 grid items-start gap-5 lg:col-span-2">
			<h1 class="text-xl font-medium md:mb-0">
				<img src="favicon-32x32.png" class="inline" alt="icon" />&nbsp; Footy Alerts
			</h1>
		</div>
		<div class="grid items-start lg:col-span-1">
			{#if needs_pin}
				<div class="mb-4 sm:mb-4">
					<CardsIosSettings />
				</div>
			{/if}
			<div class="mb-4 sm:mb-4">
				<CardsAlertSettings />
			</div>

			<div class="hidden lg:block">
				<CardsAbout />
			</div>
		</div>
		<div class="grid items-start lg:col-span-1">
			<div class="mb-4 sm:mb-4"><CardsScores /></div>
			<div class="lg:hidden">
				<CardsAbout />
			</div>
		</div>
	</div>
</div>

<Toaster richColors position="top-center" closeButton />

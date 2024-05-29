<script lang="ts">
	import { CardsAlertSettings, CardsScores, CardsAbout } from './(components)/index.js';
	import { Button } from '$lib/components/ui/button';
</script>

<div class="container mx-auto px-4 sm:px-6 md:max-w-7xl md:px-8">
	<div
		class="items-start justify-center gap-5 rounded-lg p-4 sm:p-6 md:grid md:grid-cols-1 md:p-8 lg:grid-cols-2 xl:grid-cols-2"
	>
		<div class="col-span-1 grid items-start gap-5 lg:col-span-2">
			<h1 class="mb-5 text-xl font-medium md:mb-0">
				<img src="whistle.png" class="inline" alt="icon" />&nbsp; Footy Alerts
			</h1>
		</div>
		<div class="grid items-start gap-5 lg:col-span-1">
			<CardsAlertSettings />
			<div class="hidden lg:block">
				<CardsAbout />
			</div>
		</div>
		<div class="grid items-start gap-5 lg:col-span-1">
			<CardsScores />
			<div class="lg:hidden">
				<CardsAbout />
			</div>
		</div>
	</div>
</div>

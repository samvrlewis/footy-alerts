<script lang="ts">
	import { CardsAlertSettings, CardsScores, CardsAbout } from './(components)/index.js';
	import { Button } from '$lib/components/ui/button';
</script>

<div class="container mx-auto md:max-w-7xl">
	<div
		class="items-start justify-center gap-5 rounded-lg p-8 md:grid lg:grid-cols-2 xl:grid-cols-2"
	>
		<div class="col-span-2 grid items-start gap-5 lg:col-span-2">
			<h1 class="text-xl font-medium">
				<img src="whistle.png" class="inline" alt="icon" />&nbsp; Footy Alerts
			</h1>
		</div>
		<div class="col-span-2 grid items-start gap-5 md:col-span-1 lg:col-span-1">
			<CardsAlertSettings />
			<CardsAbout />
		</div>
		<div class="col-span-2 grid items-start gap-5 md:col-span-1 lg:col-span-1">
			<CardsScores />
		</div>
	</div>
</div>

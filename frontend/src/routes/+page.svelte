<script lang="ts">
	import { CardsAlertSettings, CardsScores, CardsAbout } from './(components)/index.js';
	import { Button } from '$lib/components/ui/button';
</script>

<div class="md:max-w-8xl container mx-auto">
	<div
		class="items-start justify-center gap-5 rounded-lg p-8 md:grid lg:grid-cols-2 xl:grid-cols-2"
	>
		<div class="col-span-2 grid items-start gap-5 lg:col-span-2">
			<h1 class="text-lg font-medium">Footy Alerts</h1>
		</div>
		<div class="col-span-2 grid items-start gap-5 lg:col-span-1">
			<CardsAlertSettings />
			<CardsAbout />
		</div>
		<div class="col-span-2 grid items-start gap-5 lg:col-span-1">
			<CardsScores />
		</div>
	</div>
</div>

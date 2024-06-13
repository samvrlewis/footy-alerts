<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	import { PUBLIC_API_BASE_URL } from '$env/static/public';

	import { onMount } from 'svelte';
	import { writable, derived } from 'svelte/store';

	export const gamesApiData = writable<any[]>([]);

	export const games = derived(gamesApiData, ($gamesApiData) => {
		if (Array.isArray($gamesApiData)) {
			return $gamesApiData;
		}
		return [];
	});

	onMount(async () => {
		// todo: Run on a timer
		try {
			const response = await fetch(`${PUBLIC_API_BASE_URL}/games`);
			const data = await response.json();
			if (Array.isArray(data)) {
				gamesApiData.set(data);
			} else {
				console.error('Fetched data is not an array:', data);
				gamesApiData.set([]);
			}
		} catch (error) {
			console.error('Error fetching games data:', error);
			gamesApiData.set([]);
		}
	});

	function converToIconName(input: string): string {
		return input.toLowerCase().replace(/\s/g, '') + '.png';
	}

	function formatDateTime(date: string, tz: string, timestr: string, complete: number): string {
		let timestr_compare = timestr.toLowerCase();
		if (timestr_compare != 'not started') {
			if (timestr_compare == 'full time') {
				return timestr;
			} else {
				return timestr + ' (' + complete + '%)';
			}
		}

		const datetime = new Date(date + ' ' + tz);

		if (isNaN(datetime.getTime())) {
			return date;
		}

		const daysOfWeek = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
		const dayOfWeek = daysOfWeek[datetime.getDay()];
		let hour = datetime.getHours();
		const minute = datetime.getMinutes();
		const ampm = hour >= 12 ? 'PM' : 'AM';
		hour = hour % 12;
		hour = hour ? hour : 12;

		const timeString = `${hour}.${minute < 10 ? '0' : ''}${minute}${ampm}`;
		return `${dayOfWeek} ${timeString}`;
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Scores</Card.Title>
		<Card.Description>Latest round scores</Card.Description>
	</Card.Header>
	<Card.Content>
		<div>
			<div class="grid grid-cols-1 items-center gap-5">
				{#each $games as game}
					<div class="grid grid-cols-4 items-center gap-1 lg:grid-cols-3">
						<div class="col-span-2 lg:col-span-1">
							<p class="text-sm font-medium leading-none">
								<img
									src="team_icons/{converToIconName(game.home_team)}"
									class="inline"
									alt="icon"
								/>&nbsp;
								{game.home_team}
							</p>
						</div>
						<div class="flex justify-center">
							<p class="content-center text-sm leading-none">
								{game.home_score}
							</p>
						</div>
						<div class="row-span-2 flex justify-end text-right">
							<p class="text-sm text-muted-foreground">
								{formatDateTime(game.date, game.tz, game.timestr, game.complete)}
							</p>
						</div>
						<div class="col-span-2 lg:col-span-1">
							<p class="text-sm font-medium leading-none">
								<img
									src="team_icons/{converToIconName(game.away_team)}"
									class="inline"
									alt="icon"
								/>&nbsp;
								{game.away_team}
							</p>
						</div>
						<div class="flex justify-center">
							<p class="text-sm leading-none">
								{game.away_score}
							</p>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</Card.Content>
	<Card.Footer></Card.Footer>
</Card.Root>

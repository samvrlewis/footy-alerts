<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Switch } from '$lib/components/ui/switch/index.js';

	const games = [
		{
			id: 35754,
			round: 7,
			home_team: 'Richmond',
			away_team: 'Melbourne',
			complete: 100,
			winner: null,
			home_score: 42,
			away_score: 85,
			timestr: 'Full Time',
			year: 2024,
			date: '2024-04-24 19:25:00',
			tz: '+10:00'
		},
		{
			id: 35755,
			round: 7,
			home_team: 'Essendon',
			away_team: 'Collingwood',
			complete: 99,
			winner: null,
			home_score: 85,
			away_score: 85,
			timestr: 'Full Time',
			year: 2024,
			date: '2024-04-25 15:20:00',
			tz: '+10:00'
		},
		{
			id: 35756,
			round: 7,
			home_team: 'GWS',
			away_team: 'Brisbane',
			complete: 100,
			winner: null,
			home_score: 113,
			away_score: 59,
			timestr: 'Full Time',
			year: 2024,
			date: '2024-04-25 19:30:00',
			tz: '+10:00'
		},
		{
			id: 35757,
			round: 7,
			home_team: 'Port Adelaide',
			away_team: 'St Kilda',
			complete: 100,
			winner: null,
			home_score: 82,
			away_score: 72,
			timestr: 'Full Time',
			year: 2024,
			date: '2024-04-26 19:40:00',
			tz: '+10:00'
		},
		{
			id: 35758,
			round: 7,
			home_team: 'North Melbourne',
			away_team: 'Adelaide',
			complete: 24,
			winner: null,
			home_score: 14,
			away_score: 35,
			timestr: 'Q1 29:12',
			year: 2024,
			date: '2024-04-27 13:45:00',
			tz: '+10:00'
		},
		{
			id: 35759,
			round: 7,
			home_team: 'Geelong',
			away_team: 'Carlton',
			complete: 0,
			winner: null,
			home_score: 0,
			away_score: 0,
			timestr: 'Not started',
			year: 2024,
			date: '2024-04-27 16:35:00',
			tz: '+10:00'
		},
		{
			id: 35760,
			round: 7,
			home_team: 'Fremantle',
			away_team: 'Western Bulldogs',
			complete: 0,
			winner: null,
			home_score: 0,
			away_score: 0,
			timestr: 'Not started',
			year: 2024,
			date: '2024-04-27 19:30:00',
			tz: '+10:00'
		},
		{
			id: 35761,
			round: 7,
			home_team: 'Gold Coast',
			away_team: 'West Coast',
			complete: 0,
			winner: null,
			home_score: 0,
			away_score: 0,
			timestr: 'Not started',
			year: 2024,
			date: '2024-04-28 13:00:00',
			tz: '+10:00'
		},
		{
			id: 35762,
			round: 7,
			home_team: 'Hawthorn',
			away_team: 'Sydney',
			complete: 0,
			winner: null,
			home_score: 0,
			away_score: 0,
			timestr: 'Not started',
			year: 2024,
			date: '2024-04-28 16:00:00',
			tz: '+10:00'
		}
	];

	function converToIconName(input: string): string {
		// Convert the input string to lowercase and remove spaces using regex
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

		// Check if the parsed date is valid
		if (isNaN(datetime.getTime())) {
			// If parsing fails, return the original string
			return date;
		}

		// Create an array to map the days of the week
		const daysOfWeek = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

		// Get the day of the week from the Date object
		const dayOfWeek = daysOfWeek[datetime.getDay()];

		// Get the hour and minute from the Date object
		let hour = datetime.getHours();
		const minute = datetime.getMinutes();

		// Convert hour to 12-hour format
		const ampm = hour >= 12 ? 'PM' : 'AM';
		hour = hour % 12;
		hour = hour ? hour : 12; // Handle midnight

		// Format the time string
		const timeString = `${hour}.${minute < 10 ? '0' : ''}${minute}${ampm}`;

		// Combine day of the week and time
		return `${dayOfWeek} ${timeString}`;
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Scores</Card.Title>
		<Card.Description>Current round scores</Card.Description>
	</Card.Header>
	<Card.Content>
		<div>
			<div class="grid grid-cols-1 items-center gap-5">
				{#each games as game}
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

<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Switch } from '$lib/components/ui/switch/index.js';

	const games = [
		{
			home_team: 'Geelong',
			away_team: 'Hawthorn',
			home_icon: '/team_icon/geelong.png',
			away_icon: '/team_icon/hawthorn.png',
			home_score: 95,
			away_score: 78,
			time: 'Full Time'
		},
		{
			home_team: 'Geelong',
			away_team: 'Hawthorn',
			home_icon: '/team_icon/geelong.png',
			away_icon: '/team_icon/hawthorn.png',
			home_score: 0,
			away_score: 0,
			time: 'Sat 2.30PM'
		},
		{
			home_team: 'Geelong',
			away_team: 'Hawthorn',
			home_icon: '/team_icon/geelong.png',
			away_icon: '/team_icon/hawthorn.png',
			home_score: 0,
			away_score: 0,
			time: 'Sat 2.30PM'
		},
		{
			home_team: 'Geelong',
			away_team: 'Hawthorn',
			home_icon: '/team_icon/geelong.png',
			away_icon: '/team_icon/hawthorn.png',
			home_score: 0,
			away_score: 0,
			time: 'Sat 2.30PM'
		},
		{
			home_team: 'Geelong',
			away_team: 'Hawthorn',
			home_icon: '/team_icon/geelong.png',
			away_icon: '/team_icon/hawthorn.png',
			home_score: 0,
			away_score: 0,
			time: 'Sat 2.30PM'
		},
		{
			home_team: 'Geelong',
			away_team: 'Hawthorn',
			home_icon: '/team_icon/geelong.png',
			away_icon: '/team_icon/hawthorn.png',
			home_score: 0,
			away_score: 0,
			time: 'Sat 2.30PM'
		},
		{
			home_team: 'Geelong',
			away_team: 'Hawthorn',
			home_icon: '/team_icon/geelong.png',
			away_icon: '/team_icon/hawthorn.png',
			home_score: 0,
			away_score: 0,
			time: 'Sat 2.30PM'
		},
		{
			home_team: 'Geelong',
			away_team: 'Hawthorn',
			home_icon: '/team_icon/geelong.png',
			away_icon: '/team_icon/hawthorn.png',
			home_score: 0,
			away_score: 0,
			time: 'Sat 2.30PM'
		},
		{
			home_team: 'Geelong',
			away_team: 'Hawthorn',
			home_icon: '/team_icon/geelong.png',
			away_icon: '/team_icon/hawthorn.png',
			home_score: 0,
			away_score: 0,
			time: 'Sat 2.30PM'
		}
	];
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
					<div class="grid grid-cols-3 items-center gap-1">
						<div>
							<p class="text-sm font-medium leading-none">
								<img src="team_icons/geelong-small.png" class="inline" alt="icon" />
								{game.home_team}
							</p>
						</div>
						<div class="flex justify-center">
							<p class="content-center text-sm leading-none">
								{game.home_score}
							</p>
						</div>
						<div class="row-span-2 flex justify-end">
							<p class="text-sm text-muted-foreground">
								{game.time}
							</p>
						</div>
						<div>
							<p class="text-sm font-medium leading-none">
								<img src="team_icons/hawthorn-small.png" class="inline" alt="icon" />
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

<script lang="ts" context="module">
	function isIOS(): boolean {
		const userAgent = window.navigator.userAgent;
		return /iPad|iPhone|iPod/.test(userAgent) && !(window as any).MSStream;
	}

	export function isIOSWithoutNotifications(): boolean {
		return !('Notification' in window) && isIOS();
	}
</script>

<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Switch } from '$lib/components/ui/switch/index.js';
	import { createEventDispatcher } from 'svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import * as Select from '$lib/components/ui/select';
	import { PUBLIC_API_BASE_URL } from '$env/static/public';
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import Reload from 'svelte-radix/Reload.svelte';

	const dispatch = createEventDispatcher();

	const defaultTeam = { label: 'All', value: 'null' };

	let notificationsEndpoint: String | null = null;
	let closeGamesEnabled = true;
	let quarterScoresEnabled = false;
	let finalScoresEnabled = true;
	let selectedTeam = {
		label: 'All',
		value: 'null'
	};

	let iOSAlert = false;
	let notificationsAlert = false;
	let submittingSubscription = false;

	const options = [
		{ value: 'null', label: 'All' },
		{ value: '1', label: 'Adelaide' },
		{ value: '2', label: 'Brisbane' },
		{ value: '3', label: 'Carlton' },
		{ value: '4', label: 'Collingwood' },
		{ value: '5', label: 'Essendon' },
		{ value: '6', label: 'Fremantle' },
		{ value: '7', label: 'Geelong' },
		{ value: '8', label: 'Gold Coast' },
		{ value: '9', label: 'GWS' },
		{ value: '10', label: 'Hawthorn' },
		{ value: '11', label: 'Melbourne' },
		{ value: '12', label: 'North Melbourne' },
		{ value: '13', label: 'Port Adelaide' },
		{ value: '14', label: 'Richmond' },
		{ value: '15', label: 'St Kilda' },
		{ value: '16', label: 'Sydney' },
		{ value: '17', label: 'West Coast' },
		{ value: '18', label: 'Western Bulldogs' }
	];

	onMount(async () => {
		const reg = await navigator.serviceWorker.ready;
		let sub = await reg.pushManager.getSubscription();

		if (sub) {
			let encodedUrl = encodeURIComponent(sub.endpoint);
			const response = await fetch(`${PUBLIC_API_BASE_URL}/subscription?endpoint=${encodedUrl}`);
			if (response.ok) {
				const data = await response.json();
				closeGamesEnabled = data.close_games;
				quarterScoresEnabled = data.quarter_scores;
				finalScoresEnabled = data.final_scores;

				for (const option of options) {
					if (option.label === data.team) {
						// Adjust this condition based on your criteria
						selectedTeam = option;
						break; // Exit loop once a match is found
					}
				}
				notificationsEndpoint = sub.endpoint;
			}
		}
	});

	async function sendTestNotification() {
		let encodedUrl = encodeURIComponent(String(notificationsEndpoint));

		try {
			const response = await fetch(
				`${PUBLIC_API_BASE_URL}/test_notification?endpoint=${encodedUrl}`,
				{
					method: 'POST'
				}
			);

			if (response.ok) {
				toast.success('Notification sent!');
			} else {
				toast.error('API error sending test notification', {
					description: String(response.statusText)
				});
			}
		} catch (error) {
			toast.error('API error sending test notification', {
				description: String(error)
			});
		}
	}

	async function savePreferences() {
		if (!('Notification' in window)) {
			if (isIOS()) {
				iOSAlert = true;
			} else {
				toast.warning('Push notification error', {
					description: 'Push notifications are not available in your browser!'
				});
			}
		} else {
			notificationsAlert = true;
		}
	}

	async function acceptNotifications() {
		submittingSubscription = true;
		const permission = await Notification.requestPermission();

		if (permission != 'granted') {
			toast.warning('Push notification permission error', {
				description: 'Please enable push notifications for FootyAlerts.',
				action: {
					label: 'Learn more',
					onClick: () => window.open('https://support.google.com/chrome/answer/3220216', '_blank')
				}
			});
			submittingSubscription = false;
			return;
		}

		const reg = await navigator.serviceWorker.ready;
		let sub;
		sub = await reg.pushManager.getSubscription();
		console.log(sub);
		console.log('Accepted!');
		if (!sub) {
			// Fetch VAPID public key
			sub = await reg.pushManager.subscribe({
				userVisibleOnly: true,
				applicationServerKey:
					'BKZ7f_R7nwROpGQZQMD95KiySA27zUTMFAHIbwyGdhTj0QxK_bYtjJcpj-o5fETke8Gf6X7HpF89PumZ1D1Rdqw'
			});
		}

		let team: number | null = parseInt(selectedTeam.value);
		if (isNaN(team)) {
			team = null;
		}

		const data = {
			team: team, // Assuming defaultTeam is the currently selected team
			close_games: closeGamesEnabled, // Assuming you want this to always be true
			final_scores: finalScoresEnabled, // Assuming you want this to always be true
			quarter_scores: quarterScoresEnabled, // Assuming you want this to always be true
			web_push: sub
		};

		console.log(JSON.stringify(data));

		try {
			const response = await fetch(`${PUBLIC_API_BASE_URL}/subscription`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(data)
			});
			if (!response.ok) {
				toast.error('API error subscribing', {
					description: response.statusText
				});
			} else {
				toast.success('Successfully subscribed!');
				notificationsEndpoint = sub.endpoint;
			}
		} catch (error) {
			toast.error('API error subscribing', {
				description: String(error)
			});
		}

		submittingSubscription = false;
		return;
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Alert Settings</Card.Title>
		<Card.Description>Manage which push notifications you receive.</Card.Description>
	</Card.Header>
	<Card.Content class="grid gap-6">
		<div class="flex items-center justify-between space-x-2">
			<Label for="full_time" class="flex flex-col space-y-1">
				<span>Team</span>
				<span class="text-xs font-normal leading-snug text-muted-foreground">
					Notifications for teams
				</span>
			</Label>
			<Select.Root bind:selected={selectedTeam}>
				<Select.Trigger class="w-[180px]">
					<Select.Value placeholder="Team to notify for" />
				</Select.Trigger>
				<Select.Content>
					{#each options as option}
						<Select.Item value={option.value}>{option.label}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
		<div class="flex items-center justify-between space-x-2">
			<Label for="quarter" class="flex flex-col space-y-1">
				<span>Quarter time scores</span>
				<span class="text-xs font-normal leading-snug text-muted-foreground">
					Notifications on scores at the end of each quarter.
				</span>
			</Label>
			<Switch bind:checked={quarterScoresEnabled} id="quarter" aria-label="Quarter" />
		</div>
		<div class="flex items-center justify-between space-x-2">
			<Label for="full_time" class="flex flex-col space-y-1">
				<span>Full time scores</span>
				<span class="text-xs font-normal leading-snug text-muted-foreground">
					Notifications on scores at full time.
				</span>
			</Label>
			<Switch bind:checked={finalScoresEnabled} id="full_time" aria-label="Full" />
		</div>
		<div class="flex items-center justify-between space-x-2">
			<Label for="close" class="flex flex-col space-y-1">
				<span>Close game alerts</span>
				<span class="text-xs font-normal leading-snug text-muted-foreground">
					Notifications on the scores being close at the end of the game.
				</span>
			</Label>
			<Switch bind:checked={closeGamesEnabled} id="close_game" aria-label="Close" />
		</div>
	</Card.Content>
	<Card.Footer>
		<div class="w-full space-y-1">
			<Button variant="destructive" class="w-full" on:click={savePreferences}>
				{#if submittingSubscription}
					<Reload class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				Save preferences</Button
			>

			{#if notificationsEndpoint}
				<Button class="w-full" variant="outline" on:click={sendTestNotification}
					>Send test notification</Button
				>
			{/if}
		</div>
		<AlertDialog.Root bind:open={notificationsAlert}>
			<AlertDialog.Trigger />
			<AlertDialog.Content>
				<AlertDialog.Header>
					<AlertDialog.Title>Enable Notifications</AlertDialog.Title>
					<AlertDialog.Description>
						Clicking continue will prompt your browser for you to accept notifications.
					</AlertDialog.Description>
				</AlertDialog.Header>
				<AlertDialog.Footer>
					<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
					<AlertDialog.Action on:click={acceptNotifications}>Continue</AlertDialog.Action>
				</AlertDialog.Footer>
			</AlertDialog.Content>
		</AlertDialog.Root>
	</Card.Footer>
</Card.Root>

<div class="hidden">
	<AlertDialog.Root bind:open={iOSAlert}>
		<AlertDialog.Trigger />
		<AlertDialog.Content>
			<AlertDialog.Header>
				<AlertDialog.Title>Alerts on iOS</AlertDialog.Title>
				<AlertDialog.Description>
					For notifications to trigger on iOS, Footy Alerts needs to be added as a home screen
					application. To do so hit the "share" button the "add to home screen". Once done, open
					FootyAlerts and save alert preferences.
				</AlertDialog.Description>
			</AlertDialog.Header>
			<AlertDialog.Footer>
				<AlertDialog.Cancel>OK</AlertDialog.Cancel>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>
</div>

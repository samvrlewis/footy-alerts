<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>About</Card.Title>
		<Card.Description>
			FootyAlerts is an open source project developed by <a href="https://github.com/samvrlewis"
				>@samvrlewis</a
			>.
		</Card.Description>
	</Card.Header>
	<Card.Content class="grid gap-6">
		<span class="text-xs font-normal leading-snug text-muted-foreground">
			Footy Alerts uses the <a href="https://developer.mozilla.org/en-US/docs/Web/API/Push_API"
				>browser push API</a
			> to send AFL score notifications. This works across mobile devices and desktops. On mobile devices,
			notifications will be received even if the browser is not open.
		</span>
	</Card.Content>
</Card.Root>

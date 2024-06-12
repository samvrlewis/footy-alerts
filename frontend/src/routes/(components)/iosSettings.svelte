<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
</script>

<Card.Root class="bg-red-200 font-medium">
	<Card.Header>
		<Card.Title>Alerts on iOS</Card.Title>
		<Card.Description
			>Warning! Alerts on iOS don't work unless the site is on home screen.</Card.Description
		>
	</Card.Header>
	<Card.Content>
		<p>
			To get notifications on your iPhone, you need to add Footy Alerts to your home screen. Hit the
			<span class="inline-flex items-center">
				<img src="ios-share-icon.png" alt="Share icon" class="mx-1 h-5" />
			</span>
			and choose&nbsp;<b>Add to Home Screen</b>. If you need more help, check out
			<a href="https://support.apple.com/en-au/guide/iphone/iph42ab2f3a7/ios">Apple Support</a> for details.
		</p>
	</Card.Content>
</Card.Root>

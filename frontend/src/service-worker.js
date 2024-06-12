self.addEventListener('activate', async (event) => {
	console.log('HELLO FROM SW');
});

self.addEventListener('push', (event) => {
	console.log('Waiting for notification');
	const options = {
		body: event.data.text(),
		icon: '/apple-touch-icon.png',
		badge: '/android-chrome-192x192.png'
	};
	console.log('Notification');
	event.waitUntil(self.registration.showNotification('Footy Alerts', options));
});

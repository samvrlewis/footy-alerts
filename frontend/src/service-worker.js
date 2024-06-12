self.addEventListener('activate', async (event) => {
	console.log('HELLO FROM SW');
});

self.addEventListener('push', (event) => {
	console.log('Waiting for notification');
	const options = {
		body: event.data.text(),
		icon: '/apple-touch-icon.png',
		badge: '/notification-badge.png'
	};
	console.log('Notification');
	event.waitUntil(self.registration.showNotification('Footy Alerts', options));
});

self.addEventListener('notificationclick', function (event) {
	event.notification.close();
	event.waitUntil(clients.openWindow('https://footyalerts.fyi'));
});

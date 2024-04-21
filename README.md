# Footy Alerts
(Currently a WIP!)

An attempt to replicate the alerts & notifications that used to be sent by the FootyLive app (before it stopped working). 
Because, in today's fast paced world, it's important to get up to date football scores when and as they happen!

The intention, once complete, is to have a website where people can go and register for notifications for:

- End of quarter scores
- End of match scores
- Close game alerts
- Filtered by all games or only games with a certain team playing

## High Level Design
- A front end where people can go to register for notifications they're interested in
- A webserver that will store the notification settings for users
[webpush notifications](https://pqvst.com/2023/11/21/web-push-notifications/).
- A long running task that consumes events from the [squiggle API](https://api.squiggle.com.au/#section_event) for game
  events and then sends out notifications when necessary

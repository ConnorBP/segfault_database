## Segfault Database
This is the middleman between applications using my servers for say storing stats and whatnot or checking if users are subscribed to vip, and the backend databases actually storing said information, or the rest api for the donation platoform itself. Goals of this are to be efficient and async, secured with seperate auth tokens for each server contacting this service for info, and ideally this should also be stateless to allow for later use of something like kubernetes.

##TODO
- ~~Implement connection to database~~
- connect with paddle rest api
- create rest api for applications using my service
    - ~~stats access api~~
    - additional stats endpoints for index-based or discord
    - endpoint for getting top ranked players
    - vip check api
    - secure tokens identifying each server/app that can access my service for authentication
        - ~~admin accounts associated with specific server tokens~~
        - Secure and Lock down the api with either NGINX or inside actix
- discord bot functionality
    - ~start a bot~
    - connect bot to api
    - add functions for linking accounts
- [ ] create service config for running app on our server
- [ ] create nginx forwarding configuration

# Plugin Todo
- [ ] add top players leaderboard
- [ ] add scoreboard ranks display
- [ ] add scoreboard rws display
- [ ] add vip integration
- [ ] add core server manager plugin
    - [ ] Offload our Command Chat Alias Management
    - [ ] manage loaded plugins
    - [ ] keep track of who has vip
    - [ ] some commands for admins to contact me or use some tool on my web service
    - [ ] per-map configurations and gamemode configuring that stays past-map changes
        - [ ] save current chosen active game mode in text file
        - [ ] every server load use the active mode to choose which plugins are loaded and which config to launch
- [ ] tweak the giving of round points
    - [ ] add more events that players should be rewarded for
    - [ ] limit damage given rewards to the ammount of hp the victim actually had left before the attack
        - [ ] after it is limited we have access to a more useful ADR
- [ ] add a few more additional stats such as KDR, ADR, HSP, etc (just the core important ones, not too many)

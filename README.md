## Segfault Database
This is the middleman between applications using my servers for say storing stats and whatnot or checking if users are subscribed to vip, and the backend databases actually storing said information, or the rest api for the donation platoform itself. Goals of this are to be efficient and async, secured with seperate auth tokens for each server contacting this service for info, and ideally this should also be stateless to allow for later use of something like kubernetes.

##TODO
- Implement connection to database
- connect with paddle rest api
- create rest api for applications using my service
    - stats access api
    - vip check api
    - secure tokens identifying each server/app that can access my service for authentication
    - admin accounts associated with specific server tokens
- discord bot functionality
- faceit api integrations

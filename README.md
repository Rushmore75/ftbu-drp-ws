# ftbu-drp-ws
FTB Utilities Discord Rich Presence web server
___

# To Do List
- [ ] Discord command that links minecraft account to discord account
    - [ ] Then nick player in discord server as minecraft name (or other way, idk)
- [ ] Impl config.
- [ ] Serenity has a cache feature, use it.
- [ ] Create channels when a team is created


# setup
Something like this for setting up the database:
`sudo docker run --name friendly-fire -e POSTGRES_PASSWORD={password} -e POSTGRES_DB=ftbu -itd postgres`

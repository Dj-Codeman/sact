# SACT (superuser act) 
## A work in progress
## A fork of rhun 

SACT is a alternative to sudo / doas. It is written in rust, accepts no command
line flags and it's config data will be encrypted. The configs will have to be edited by uid 0.
Running `make install` will install it, Running `sact initialize` will run the config editor
for the first user.

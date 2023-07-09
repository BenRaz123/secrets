# `secrets` - A secure way of keeping secrets

## What is this?

`secrets` is a cli app which aids in the secure management of potentially embarassing secrets.

## How do I use this?

run `secrets {x}` where x is either

| Name | Action |
|------|--------|
| new  | create a new secret |
| list | list all secrets |
| remove | remove from your list of secrets |

## Status

- [x] Implement basic app
    - [x] Implement `secrets new`
    - [x] Implement `secrets list`
    - [x] Implement `secrets remove`
    - [x] Hardcode Password to `'hello'`
- [ ] Make app more secure
    - [ ] Stop hardcoding passwords, store user-supplied passwords in seperate file
    - [ ] Make passwords more secure with encryption
    - [ ] Use `sqLite` to strore passwords
- [ ] Make app more accessible
    - [x] Create README
    - [x] Upload to Github
    - [ ] Upload to crates.io

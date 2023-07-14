# ⚠️ CRITICAL BUG ⚠️ app not working right now

# seecrets 🤫 - A secure way of keeping secrets

## What is this?

`seecrets` is a cli app which aids in the secure management of potentially embarassing secrets.

## How do I use this?

run `seecrets {x}` where x is either

| Name | Action |
|------|--------|
| new  | create a new secret |
| list | list all seecrets |
| remove | remove from your list of seecrets |

## Status

- [x] Implement basic app
    - [x] Implement `seecrets new`
    - [x] Implement `seecrets list`
    - [x] Implement `seecrets remove`
    - [x] Hardcode Password to `'hello'`
- [ ] Make app more secure
    - [x] Stop hardcoding passwords, store user-supplied passwords in seperate file
    - [x] Make passwords more secure with encryption
    - [x] Use `sqLite` to strore passwords
- [ ] Make app more accessible
    - [x] Create README
    - [x] Upload to Github
    - [ ] Upload to crates.io
- [ ] Make app work on multi-user systems

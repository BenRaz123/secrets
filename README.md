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
- [x] Make app more secure
    - [x] Stop hardcoding passwords, store user-supplied passwords in seperate file
    - [x] Make passwords more secure with encryption
    - [x] Use `sqLite` to strore passwords
- [x] Make app more accessible
    - [x] Create README
    - [x] Upload to Github
    - [x] Upload to crates.io
- [ ] Make app work on multi-user systems

## Statements on version 1.x

I uploaded version 1.x without any proper testing. Because of this, the app did not work at all. I have yanked all versions under 1.x as none of them currently work. Over the past week, more than 30 people have downloaded `seecrets`. To all of those people, I would like to apologize.

Everyone who, can should upgrade to version 2.0.0

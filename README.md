### WIP

Using this to learn Rust and see what all the fuss is about.

# Git Rip

A command line utility which deletes merged git branches conveniently.

 - deletes all branches on your local machine merged into a branch you specify.
 - allows you to protect specific branches to never be deleted.
 - will not delete the current branch you are in.

# Todo

- [ ] Force users to add-protected-branches as we won't allow this to work until default branches are added.

### Why

I often work on multiple branches every day and rarely delete branches I work on, review and test. This results in a messy directory where I end up cleaning things manually.

### Installation

`cargo install gitrip`

### Usage

View help
```sh
gitrip --help
```

Git rip allows you to add protected branches which will 

Add a protected directory and confirm
```sh
gitrip --add-protected-branches 'master, staging'
cat .git/config
```

Show local branches which have been merged into staging
```sh
gitrip --show-merged staging
180-sidekiq-upgrade
```

Delete branches merged into staging
```sh
gitrip --branch staging
ca-comms % git branch
* 193-current-branch
  master
  staging
```

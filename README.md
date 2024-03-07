# adhd-helper-bot
![Build](https://github.com/Asm-Rosie/adhd-helper-bot/actions/workflows/rust.yml/badge.svg)

## How to run
first get ur discord bot token

once you have it create in ur project root dir a file called "Secrets.toml" you place this in there 
```toml
DISCORD_TOKEN="replace with ur token"
```
and then save the file, now u gotta install a specific version of cargo-shuttle on ur system with 
`cargo install cargo-shuttle@0.35.2`
once you have done that you can start the bot with 
`cargo shuttle run` 

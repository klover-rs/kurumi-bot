# Kurumi-bot
![Build](https://github.com/Asm-Rosie/adhd-helper-bot/actions/workflows/rust.yml/badge.svg)

## How to run
## First get ur discord bot token

once you have it create in ur project root dir a file called "Secrets.toml" you place this in there 
```toml
DISCORD_TOKEN="replace with ur token"
APP_ID="your app id"
BOT_ID = "bot user id here"
DB_NAME = "my_bot"
DB_PW = "your database password"
```


make sure you have [PostgreSQL](https://www.postgresql.org/download/) installed for your OS and set up.

once its setup, you should make some addinitional configurations. 

first of all you should create a dedicated Database for it you can use this by using and query tool for postgre, on windows you can use the query tool in PgAdmin4.

if you are using an Linux operating system, you can use the following command to enter the query tool `sudo -u postgres psql`

after you are in the query tool you can create a new database first for example
```SQL
CREATE DATABASE my_bot
```
you can do this in the query tool by using the following query
```SQL
ALTER USER postgres PASSWORD 'your_password';
```

after you have ensured that all these values are set start the discord bot with `cargo run`

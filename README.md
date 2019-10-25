# Zubotsu
Why? Because we live in a society

# Documentation
In order to run you need to have a postgres instance running that rust can talk to, 
this is best done by creating an .env folder and having an environment variable

```sh
DATABASE_URL=postgresql://127.0.0.1:5432/dbname?user=dbuser&password=dbpassword
RUST_LOG=debug
DISCORD_TOKEN=MYTOKEN
GUILD_ID=myGuildIDInteger
```

if it is your first time running the postgres side of the server you will need to run the following commands
```sh
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```

if you want to change the schema you will need to 
1. change the `Users` model in models.rs
2. run a command `diesel migration generate my_new_migration_name` 
3. fill the `up.sql` and `down.sql`
4. run the `disel migration run`

this should add some new things to the `schemas.rs` which will give you compile time errors with the associated model if you try to add the macros
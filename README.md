## Setup & Building
```bash
cargo install cargo-watch
cd app-service
cargo build
cd ..
cd auth-service
cargo build
cd ..
```

## Run servers locally (Manually)
#### App service
```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service
```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

## Run servers locally (Docker)
```bash
./docker.sh
```
```CMD for Windows
.\docker.bat
```

visit http://localhost:8000 and http://localhost:3000



## Database Migrations
```bash
#more info about sqlx-cli
https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#install

1. cargo install sqlx-cli --no-default-features --features native-tls,postgres


#NOTE: Make sure to execute this command in the auth-service directory!
# creates 2 new migration scripts named:
# "migrations/[TIMESTAMP]_create_users_table_down.sql" and "migrations/[TIMESTAMP]_create_users_table_up.sql"
2. sqlx migrate add -r create_users_table

#NOTE: Make sure to execute this command in the auth-service directory!
#Generates build.rs script, which is used to generate the database schema
3. sqlx migrate build-script

4.
a #SQLX_OFFLINE environment variable to avoid connecting to the database at runtime
#Add this line to Dockerfile 
# # Build application
# COPY . .
ENV SQLX_OFFLINE true
#RUN cargo build --release --bin auth-service

b # Adn to .env file
SQLX_OFFLINE=true

c#And execute this
cargo sqlx prepare

d #Restart rust-analyzer sever
```

## Docker Postgres
```bash
# Pull the latest Postgres image from Docker Hub
1. docker pull postgres:15.2-alpine

# Run the Postgres container
2. docker run --name ps-db -e POSTGRES_PASSWORD=[YOUR_POSTGRES_PASSWORD] -p 5432:5432 -d postgres:15.2-alpine
```

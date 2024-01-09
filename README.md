# Rust: Todo List API using axum and diesel

This project is a quick example of how to implement CRUD todo list APIs in Rust using
- [axum](https://github.com/tokio-rs/axum) for API server
- [diesel](https://github.com/diesel-rs/diesel) for database ORM

Some boilerplace code is taken from [Todos API example by axum](https://github.com/tokio-rs/axum/tree/main/examples/todos)
then integrate with `diesel` to perform actual database queries, which is Postgres in this example.

Disclaimer: I am very new to rust so there might be some beginner mistakes here :) but I hope this is beneficial for Rust beginners like me.

## Quick Start
```sh
# Spin up postgres db
docker-compose up -d

# Setup DATABASE_URL env to be used by diesel's migrations and our app
export DATABASE_URL=postgres://postgres:1234@localhost:5432/todos

# Make sure to create a `todos` database in postgres then
# create database tables
make migrate

# Build and serve APIs at port 3000
make dev
```

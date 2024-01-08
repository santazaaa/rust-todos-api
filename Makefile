dev:
	cargo build && cargo run

fix:
	cargo fix --allow-dirty

dbenv:
	export DATABASE_URL=postgres://postgres:1234@localhost:5432/todos

migrate:
	diesel migration run

re-migrate:
	diesel migration redo

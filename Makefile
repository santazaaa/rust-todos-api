dev:
	cargo build && cargo run

fix:
	cargo fix --allow-dirty

migrate:
	diesel migration run

re-migrate:
	diesel migration redo

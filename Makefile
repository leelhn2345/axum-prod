docker build:
	docker build --tag zero2prod --file Dockerfile .
prepare:
	cargo sqlx prepare
migrate:
	sqlx migrate run

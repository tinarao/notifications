run: redis-up
	@sleep 5
	cargo run

redis-up:
	docker compose up -d

redis-stop:
	docker compose stop

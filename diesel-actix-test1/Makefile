include .env
export

init: docker-up
# Initialize Diesel migrations
	diesel setup

migrate: init
# generate migration
	diesel migration generate create_users
# edit migration file
	cp ./up.sql ./migrations/*_create_users/up.sql

run_migrate:
# run migration
	diesel migration run

run:
	cargo run

watch:
	cargo watch -x 'run'

docker-up:
	docker-compose up -d

docker-down: 
	docker-compose down

list_dbs:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -c "\l"

list_tables:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -c "\dt"

# passed via cammad line
table_schema:
	@if [ -z "$(TBL)" ]; then echo "Usage: make table_schema TBL=table_name" && exit 1; fi
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -c "\d $(TBL)"

select_all:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -c "select * from $(TABLE_NAME)"


.PHONY: init migrate run docker-up
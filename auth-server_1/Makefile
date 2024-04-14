include .env
export

init:
# Initialize Diesel migrations
	diesel setup

migrate: init
# generate migration
	diesel migration generate create_user_invitations
# edit migration file
	cp ./up.sql ./migrations/*create_user_invitations/up.sql

# run migration
run_migrate:
	diesel migration run

#run on ur machine export SECRET_KEY=$(make generate_key)
generate_key:
	@openssl rand -base64 64

print:
	@echo $$SECRET_KEY

encrypt:
	@openssl enc -aes-256-cbc -pbkdf2 -k "$$SECRET_KEY" -in ".env" -out "encrypted.env"

decrypt:
	@echo "Decrypting .env file..."
	@./decrypt.sh

run:
	cargo run 

watch:
	cargo watch -x 'run'

# Docker commands

docker-up:
	docker-compose up -d

docker-down: 
	docker-compose down --remove-orphans

docker-inspect:
	@docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $(CONTAINER_NAME)

# Postgres commands

list_dbs:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -c "\l"

list_tables:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -c "\dt"

table_schema:
	@if [ -z "$(T)" ]; then echo "Usage: make table_schema T=table_name" && exit 1; fi
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -c "\d $(T)"

select_records:
	@if [ -z "$(T)" ]; then echo "Usage: make table_schema T=table_name" && exit 1; fi
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -c "select * from $(T)"

delete_records:
	@if [ -z "$(T)" ]; then echo "Usage: make delete_records T=table_name"; exit 1; fi
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) -c "DELETE FROM $(T);"
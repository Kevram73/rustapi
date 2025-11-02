.PHONY: help build up down logs restart clean

help: ## Affiche l'aide
	@echo "Commandes disponibles:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Construire les images Docker
	docker-compose build

up: ## Démarrer les services
	docker-compose up -d

down: ## Arrêter les services
	docker-compose down

logs: ## Voir les logs
	docker-compose logs -f api

restart: ## Redémarrer les services
	docker-compose restart

clean: ## Arrêter et supprimer les volumes
	docker-compose down -v

dev: ## Démarrer en mode développement (avec logs)
	docker-compose up

test: ## Exécuter les tests
	cargo test


# Guide de dépannage Docker

Ce guide aide à résoudre les problèmes courants lors du build et de l'exécution avec Docker.

## Problèmes courants

### 1. Cargo.lock non trouvé

**Erreur :**
```
failed to solve: "/Cargo.lock": not found
```

**Solution :**
Le Dockerfile a été corrigé pour gérer ce cas. Assurez-vous que :
- `Cargo.lock` n'est pas dans `.dockerignore`
- Ou générez-le avec `cargo build` avant le build Docker

```bash
cargo build
docker compose build
```

### 2. Permission denied pour Docker

**Erreur :**
```
permission denied while trying to connect to the Docker daemon socket
```

**Solutions :**

**Option A : Ajouter votre utilisateur au groupe docker**
```bash
sudo usermod -aG docker $USER
newgrp docker
```

**Option B : Utiliser sudo**
```bash
sudo docker compose build
sudo docker compose up
```

### 3. Version obsolète dans docker-compose.yml

**Avertissement :**
```
WARN: the attribute `version` is obsolete
```

**Solution :**
La version a été retirée du `docker-compose.yml`. Ce n'est qu'un avertissement, cela ne bloque pas l'exécution.

### 4. Port déjà utilisé

**Erreur :**
```
Error: bind: address already in use
```

**Solutions :**

**Option A : Changer le port dans docker-compose.yml**
```yaml
ports:
  - "3001:3000"  # Utilise le port 3001 au lieu de 3000
```

**Option B : Arrêter le service qui utilise le port**
```bash
# Trouver le processus
sudo lsof -i :3000

# Arrêter le processus (remplacez PID)
kill -9 <PID>
```

### 5. Base de données non accessible

**Erreur :**
```
error connecting to database
```

**Solutions :**

Vérifiez que PostgreSQL est démarré :
```bash
docker compose ps
```

Redémarrez les services :
```bash
docker compose down
docker compose up -d
```

Vérifiez les logs :
```bash
docker compose logs postgres
```

### 6. Build lent

**Solutions :**

Utiliser le cache Docker :
```bash
# Ne pas utiliser --no-cache sauf si nécessaire
docker compose build

# Utiliser BuildKit pour des builds plus rapides
DOCKER_BUILDKIT=1 docker compose build
```

Optimiser les layers Docker :
- Les dépendances système sont déjà installées en une seule couche
- Les dépendances Cargo sont mises en cache

### 7. Erreur de compilation Rust

**Vérifications :**

1. Version de Rust :
```bash
rustc --version  # Doit être 1.88+
```

2. Compiler localement d'abord :
```bash
cargo build --release
```

3. Vérifier les erreurs de compilation :
```bash
docker compose build 2>&1 | grep -A 10 error
```

### 8. Migrations non exécutées

**Vérifications :**

1. Les migrations sont dans le dossier `migrations/`
2. Le dossier `migrations/` est copié dans le Dockerfile
3. Les migrations sont exécutées au démarrage (dans `main.rs`)

Vérifier dans les logs :
```bash
docker compose logs api | grep -i migration
```

### 9. Variables d'environnement non chargées

**Solutions :**

Vérifier les variables dans le container :
```bash
docker compose exec api env | grep DATABASE_URL
```

Vérifier le fichier `.env` (s'il existe) :
```bash
cat .env
```

Les variables sont définies dans `docker-compose.yml` sous `environment:`.

### 10. Image trop volumineuse

**Optimisations :**

1. Utiliser un stage builder multi-stage (déjà fait)
2. Utiliser `.dockerignore` pour exclure les fichiers inutiles
3. Nettoyer les caches :
```bash
docker system prune -a
```

## Commandes utiles

### Nettoyer complètement

```bash
# Arrêter et supprimer les containers, volumes et images
docker compose down -v --rmi all

# Nettoyer le système Docker
docker system prune -a --volumes
```

### Voir les logs

```bash
# Logs de l'API
docker compose logs -f api

# Logs de PostgreSQL
docker compose logs -f postgres

# Logs des 50 dernières lignes
docker compose logs --tail=50
```

### Entrer dans un container

```bash
# Shell dans le container API
docker compose exec api sh

# Shell dans PostgreSQL
docker compose exec postgres psql -U rustapi -d rustapi
```

### Rebuild complet

```bash
# Rebuild sans cache
docker compose build --no-cache

# Rebuild et redémarrer
docker compose up --build -d
```

### Vérifier l'état

```bash
# Statut des services
docker compose ps

# Utilisation des ressources
docker stats

# Images créées
docker images | grep rustapi
```

## Debugging avancé

### Build avec logs détaillés

```bash
DOCKER_BUILDKIT=1 docker compose build --progress=plain
```

### Tester une étape spécifique du Dockerfile

```bash
# Build jusqu'à une étape spécifique
docker build --target builder -t rustapi:builder .
docker run -it rustapi:builder sh
```

### Vérifier les layers de l'image

```bash
docker history rustapi_rustapi:latest
```

## Ressources

- [Documentation Docker](https://docs.docker.com/)
- [Documentation Docker Compose](https://docs.docker.com/compose/)
- [Best practices Dockerfile](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/)


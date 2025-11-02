# Guide de Déploiement

Ce guide explique comment déployer l'API Rust en production.

## Prérequis

- Docker et Docker Compose installés
- Serveur avec au moins 2GB RAM
- Accès SSH au serveur
- Domaine configuré (optionnel, pour HTTPS)

## Options de déploiement

### Option 1 : Déploiement avec Docker Compose (Recommandé)

#### 1. Préparer le serveur

```bash
# Mettre à jour le système
sudo apt update && sudo apt upgrade -y

# Installer Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Installer Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

#### 2. Cloner et configurer

```bash
# Cloner le projet
git clone <repository-url> /opt/rustapi
cd /opt/rustapi

# Créer le fichier .env de production
cp .env.example .env.production
```

Éditez `.env.production` :
```env
DATABASE_URL=postgresql://rustapi:CHANGE_THIS_PASSWORD@postgres:5432/rustapi
JWT_SECRET=GENERATE_A_STRONG_SECRET_KEY_HERE
JWT_EXPIRATION=3600
API_VERSION=v1
LOG_LEVEL=info
PORT=3000
HOST=0.0.0.0
```

**Important** : Générez un secret JWT fort :
```bash
openssl rand -base64 32
```

#### 3. Configuration Docker Compose pour la production

Créez `docker-compose.prod.yml` :

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:16-alpine
    container_name: rustapi_postgres_prod
    environment:
      POSTGRES_USER: rustapi
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: rustapi
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - rustapi_network
    restart: always
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U rustapi"]
      interval: 10s
      timeout: 5s
      retries: 5

  api:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: rustapi_app_prod
    env_file:
      - .env.production
    ports:
      - "127.0.0.1:3000:3000"  # Écouter uniquement sur localhost
    depends_on:
      postgres:
        condition: service_healthy
    networks:
      - rustapi_network
    restart: always
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 1G
        reservations:
          cpus: '1'
          memory: 512M

volumes:
  postgres_data:

networks:
  rustapi_network:
    driver: bridge
```

#### 4. Déployer

```bash
# Construire et démarrer
docker-compose -f docker-compose.prod.yml --env-file .env.production up -d --build

# Vérifier les logs
docker-compose -f docker-compose.prod.yml logs -f

# Vérifier le statut
docker-compose -f docker-compose.prod.yml ps
```

#### 5. Configuration Nginx (Reverse Proxy)

Installez Nginx :
```bash
sudo apt install nginx -y
```

Configuration `/etc/nginx/sites-available/rustapi` :
```nginx
server {
    listen 80;
    server_name votre-domaine.com;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
```

Activer :
```bash
sudo ln -s /etc/nginx/sites-available/rustapi /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

#### 6. HTTPS avec Let's Encrypt

```bash
# Installer Certbot
sudo apt install certbot python3-certbot-nginx -y

# Obtenir un certificat
sudo certbot --nginx -d votre-domaine.com

# Renouvellement automatique
sudo certbot renew --dry-run
```

### Option 2 : Déploiement binaire natif

#### 1. Build pour la production

Sur votre machine de développement :
```bash
# Build release
cargo build --release

# Le binaire se trouve dans target/release/rustapi
```

#### 2. Transférer sur le serveur

```bash
scp target/release/rustapi user@server:/opt/rustapi/
scp -r migrations user@server:/opt/rustapi/
```

#### 3. Créer un service systemd

`/etc/systemd/system/rustapi.service` :
```ini
[Unit]
Description=Rust API Service
After=network.target postgresql.service

[Service]
Type=simple
User=rustapi
WorkingDirectory=/opt/rustapi
Environment="DATABASE_URL=postgresql://rustapi:password@localhost:5432/rustapi"
Environment="JWT_SECRET=your-secret-key"
Environment="LOG_LEVEL=info"
Environment="PORT=3000"
Environment="HOST=0.0.0.0"
ExecStart=/opt/rustapi/rustapi
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Activer et démarrer :
```bash
sudo systemctl daemon-reload
sudo systemctl enable rustapi
sudo systemctl start rustapi
sudo systemctl status rustapi
```

## Monitoring

### Logs

#### Avec Docker
```bash
# Logs en temps réel
docker-compose logs -f api

# Logs des 100 dernières lignes
docker-compose logs --tail=100 api

# Logs depuis une date
docker-compose logs --since="2024-01-01T00:00:00" api
```

#### Avec systemd
```bash
# Voir les logs
sudo journalctl -u rustapi -f

# Logs depuis aujourd'hui
sudo journalctl -u rustapi --since today
```

### Health Check

L'endpoint `/api/health` peut être utilisé pour le monitoring :

```bash
# Vérification manuelle
curl http://localhost:3000/api/health

# Monitoring avec cron
*/5 * * * * curl -f http://localhost:3000/api/health || echo "API down" | mail -s "Alert" admin@example.com
```

### Métriques

#### Utiliser Prometheus (optionnel)

Ajoutez une route `/metrics` pour exposer les métriques :
- Nombre de requêtes
- Latence
- Erreurs
- Connexions DB

## Sauvegarde

### Base de données

#### Backup automatique

Créez un script `/opt/backup-rustapi.sh` :
```bash
#!/bin/bash
BACKUP_DIR="/opt/backups"
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p $BACKUP_DIR

# Backup PostgreSQL
docker exec rustapi_postgres_prod pg_dump -U rustapi rustapi | gzip > $BACKUP_DIR/rustapi_$DATE.sql.gz

# Garder seulement les 7 derniers backups
find $BACKUP_DIR -name "rustapi_*.sql.gz" -mtime +7 -delete
```

Ajoutez au crontab :
```bash
# Backup quotidien à 2h du matin
0 2 * * * /opt/backup-rustapi.sh
```

#### Restauration

```bash
# Décompresser et restaurer
gunzip < backup.sql.gz | docker exec -i rustapi_postgres_prod psql -U rustapi rustapi
```

## Mise à jour

### Avec Docker

```bash
# Pull les dernières modifications
git pull

# Rebuild et redémarrer
docker-compose -f docker-compose.prod.yml up -d --build

# Vérifier les logs
docker-compose -f docker-compose.prod.yml logs -f api
```

### Avec binaire natif

```bash
# Arrêter le service
sudo systemctl stop rustapi

# Mettre à jour le binaire
scp target/release/rustapi user@server:/opt/rustapi/

# Redémarrer
sudo systemctl start rustapi
```

## Rollback

En cas de problème après déploiement :

```bash
# Avec Docker
git checkout <previous-commit>
docker-compose -f docker-compose.prod.yml up -d --build

# Avec binaire
sudo systemctl stop rustapi
# Restaurer l'ancien binaire
sudo systemctl start rustapi
```

## Sécurité en production

### Checklist

- [ ] JWT_SECRET fort et unique
- [ ] POSTGRES_PASSWORD fort
- [ ] Firewall configuré (seulement ports 80, 443, 22)
- [ ] HTTPS activé avec certificat valide
- [ ] Logs configurés (rotation automatique)
- [ ] Backup automatique de la DB
- [ ] Monitoring en place
- [ ] Mises à jour de sécurité régulières
- [ ] CORS configuré pour les domaines autorisés uniquement
- [ ] Rate limiting configuré (recommandé)

### Variables d'environnement sensibles

Ne jamais commiter :
- `.env`
- `.env.production`
- Secrets et mots de passe

Utilisez des outils comme :
- HashiCorp Vault
- AWS Secrets Manager
- Docker Secrets

## Scaling

### Horizontal Scaling

Pour plusieurs instances :

1. Utilisez un load balancer (Nginx, HAProxy)
2. Base de données partagée
3. Session stateless (déjà le cas avec JWT)

Exemple avec Nginx :
```nginx
upstream rustapi {
    least_conn;
    server 127.0.0.1:3000;
    server 127.0.0.1:3001;
    server 127.0.0.1:3002;
}

server {
    location / {
        proxy_pass http://rustapi;
    }
}
```

## Troubleshooting

### L'API ne démarre pas

1. Vérifier les logs :
   ```bash
   docker-compose logs api
   ```

2. Vérifier la connexion à la DB :
   ```bash
   docker-compose exec postgres psql -U rustapi -d rustapi
   ```

3. Vérifier les variables d'environnement :
   ```bash
   docker-compose exec api env | grep DATABASE_URL
   ```

### Base de données non accessible

1. Vérifier que PostgreSQL est démarré
2. Vérifier les credentials
3. Vérifier le réseau Docker

### Performance dégradée

1. Vérifier les logs pour les requêtes lentes
2. Analyser les requêtes SQL (EXPLAIN ANALYZE)
3. Vérifier les ressources serveur (CPU, RAM)
4. Augmenter le pool de connexions si nécessaire

## Support

En cas de problème :
1. Consulter les logs
2. Vérifier la documentation
3. Créer une issue sur le dépôt Git


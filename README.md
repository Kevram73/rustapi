# Rust API - Framework générique avec gestion de tâches

Une API REST générique et modulaire en Rust pour démarrer rapidement vos projets, avec une gestion de tâches simple intégrée.

## 📚 Documentation

Une documentation complète est disponible dans le dossier `docs/` :

- **[Architecture](docs/ARCHITECTURE.md)** - Architecture technique et structure du projet
- **[Guide de développement](docs/DEVELOPMENT.md)** - Guide pour développer et contribuer
- **[Guide de déploiement](docs/DEPLOYMENT.md)** - Guide de déploiement en production
- **[Documentation API](docs/API.md)** - Documentation complète de l'API REST
- **[Exemples API](API_EXAMPLES.md)** - Exemples de requêtes API
- **[Spécification OpenAPI](openapi.yaml)** - Spécification OpenAPI/Swagger complète

Pour générer la documentation Rust :
```bash
cargo doc --open
```

## 🚀 Fonctionnalités

- **Framework web**: Axum pour un serveur HTTP performant et asynchrone
- **Base de données**: PostgreSQL avec SQLx
- **Authentification**: JWT (JSON Web Tokens)
- **Middleware**: CORS, logging, request ID
- **Gestion d'erreurs**: Système d'erreurs typé et centralisé
- **Validation**: Validation des données avec `validator`
- **Logging**: Logging structuré avec `tracing`
- **Configuration**: Gestion de la configuration via variables d'environnement
- **Gestion de tâches**: CRUD complet pour les tâches
- **Dockerisation**: Dockerfile et docker-compose.yml inclus

## 📋 Prérequis

- **Rust 1.88+** (requis pour certaines dépendances)
  - Si vous avez une version antérieure, mettez à jour avec: `rustup update`
- PostgreSQL (ou MySQL/SQLite - modifiable dans `database.rs`)
- Docker et Docker Compose (pour la dockerisation)

## 🛠️ Installation

### Option 1: Avec Docker (Recommandé)

1. **Cloner ou copier le projet**

2. **Lancer avec Docker Compose**:
```bash
docker-compose up --build
```

L'API sera accessible sur `http://localhost:3000`

### Option 2: Installation locale

1. **Cloner ou copier le projet**

2. **Copier le fichier d'environnement**:
```bash
cp .env.example .env
```

3. **Modifier le fichier `.env`** avec vos paramètres:
```env
DATABASE_URL=postgresql://user:password@localhost/rustapi
JWT_SECRET=votre-clé-secrète-très-longue-et-aléatoire
```

4. **Installer les dépendances et compiler**:
```bash
cargo build --release
```

## 🏃 Utilisation

### Avec Docker

```bash
# Démarrer les services
docker-compose up -d

# Voir les logs
docker-compose logs -f api

# Arrêter les services
docker-compose down

# Arrêter et supprimer les volumes (données)
docker-compose down -v
```

### Localement

```bash
cargo run
```

Le serveur démarre sur `http://0.0.0.0:3000` par défaut.

## 📡 Endpoints disponibles

### Santé de l'API
- `GET /api/health` - Vérification de santé de l'API

### Gestion des tâches

- `GET /api/tasks` - Liste toutes les tâches (avec pagination)
  - Query params: `page`, `limit`
  - Exemple: `GET /api/tasks?page=1&limit=10`

- `GET /api/tasks/:id` - Récupère une tâche par ID
  - Exemple: `GET /api/tasks/123e4567-e89b-12d3-a456-426614174000`

- `POST /api/tasks` - Crée une nouvelle tâche
  - Body:
    ```json
    {
      "title": "Ma nouvelle tâche",
      "description": "Description optionnelle"
    }
    ```

- `PUT /api/tasks/:id` - Met à jour une tâche
  - Body (tous les champs sont optionnels):
    ```json
    {
      "title": "Titre modifié",
      "description": "Nouvelle description",
      "completed": true
    }
    ```

- `DELETE /api/tasks/:id` - Supprime une tâche

### Exemples de requêtes

```bash
# Créer une tâche
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Faire les courses", "description": "Acheter du lait et du pain"}'

# Lister les tâches
curl http://localhost:3000/api/tasks

# Récupérer une tâche
curl http://localhost:3000/api/tasks/{id}

# Mettre à jour une tâche
curl -X PUT http://localhost:3000/api/tasks/{id} \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'

# Supprimer une tâche
curl -X DELETE http://localhost:3000/api/tasks/{id}
```

## 📁 Structure du projet

```
rustapi/
├── src/
│   ├── main.rs          # Point d'entrée de l'application
│   ├── config.rs        # Configuration de l'application
│   ├── database.rs      # Connexion et gestion de la base de données
│   ├── errors.rs        # Gestion des erreurs
│   ├── models/          # Modèles de données
│   │   └── mod.rs       # Task, User, etc.
│   ├── routes/          # Définition des routes
│   │   └── mod.rs       # Routes principales et tâches
│   ├── api/             # Handlers et DTOs
│   │   ├── handlers.rs  # Handlers génériques
│   │   ├── task_handlers.rs  # Handlers pour les tâches
│   │   └── dto.rs       # DTOs et réponses API
│   ├── middleware/      # Middlewares (auth, CORS, logging)
│   └── utils/           # Utilitaires (hash, validation)
├── migrations/          # Migrations SQL
│   └── 20240101000001_create_tasks.sql
├── Dockerfile           # Configuration Docker
├── docker-compose.yml   # Configuration Docker Compose
├── Cargo.toml          # Dépendances du projet
└── README.md
```

## 🔧 Développement

### Ajouter une nouvelle route

1. Créer un handler dans `src/api/` ou un nouveau fichier
2. Ajouter la route dans `src/routes/mod.rs`

Exemple:
```rust
// Dans src/routes/mod.rs
Router::new()
    .route("/users", get(get_users).post(create_user))
```

### Ajouter un nouveau modèle

1. Créer la structure dans `src/models/mod.rs` ou un nouveau fichier
2. Ajouter les migrations SQL dans `migrations/`
3. Créer les handlers correspondants dans `src/api/`

### Utiliser l'authentification

Ajouter le middleware d'authentification sur vos routes:
```rust
Router::new()
    .route("/protected", get(protected_handler))
    .layer(axum::middleware::from_fn(auth::auth_middleware))
```

## 🗄️ Base de données

### Migrations

Les migrations sont exécutées automatiquement au démarrage de l'application.

### Structure de la table tasks

```sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 🐳 Docker

### Commandes Docker utiles

```bash
# Construire l'image
docker-compose build

# Démarrer en arrière-plan
docker-compose up -d

# Voir les logs
docker-compose logs -f

# Arrêter
docker-compose down

# Rebuild complet
docker-compose down -v
docker-compose up --build
```

### Variables d'environnement Docker

Les variables d'environnement sont définies dans `docker-compose.yml`. Pour les modifier, éditez le fichier ou créez un `.env` qui sera automatiquement chargé par Docker Compose.

## 🔐 Sécurité

- **JWT**: Tokens signés pour l'authentification
- **Hashing**: Mots de passe hashés avec bcrypt
- **Validation**: Validation des entrées utilisateur
- **CORS**: Configuration CORS pour les requêtes cross-origin

## 📝 Variables d'environnement

| Variable | Description | Par défaut |
|----------|-------------|------------|
| `DATABASE_URL` | URL de connexion à la base de données | `postgresql://...` |
| `JWT_SECRET` | Clé secrète pour signer les JWT | `your-secret-key...` |
| `JWT_EXPIRATION` | Durée de vie des tokens (secondes) | `3600` |
| `API_VERSION` | Version de l'API | `v1` |
| `LOG_LEVEL` | Niveau de logging | `info` |
| `PORT` | Port du serveur | `3000` |
| `HOST` | Adresse d'écoute | `0.0.0.0` |

## 🧪 Tests

```bash
cargo test
```

## 📦 Build pour production

### Localement

```bash
cargo build --release
```

Le binaire se trouvera dans `target/release/rustapi`.

### Avec Docker

```bash
docker-compose build --release
```

## 🤝 Contribution

Ce framework est conçu pour être personnalisé selon vos besoins. N'hésitez pas à:
- Ajouter vos propres modules
- Personnaliser les middlewares
- Ajouter des fonctionnalités spécifiques
- Adapter la structure à vos besoins

## 📄 License

MIT
# rustapi

# Architecture du Projet

Ce document décrit l'architecture technique de l'API Rust générique.

## Vue d'ensemble

L'application est construite avec les technologies suivantes :
- **Langage** : Rust (édition 2021)
- **Framework Web** : Axum 0.7
- **Base de données** : PostgreSQL avec SQLx
- **Logging** : Tracing
- **Authentification** : JWT (JSON Web Tokens)
- **Containerisation** : Docker & Docker Compose

## Structure des modules

```
src/
├── main.rs              # Point d'entrée, initialisation du serveur
├── config.rs            # Configuration centralisée
├── database.rs          # Gestion de la connexion DB
├── errors.rs            # Système de gestion d'erreurs
├── api/                 # Couche API
│   ├── handlers.rs      # Handlers génériques
│   ├── task_handlers.rs # Handlers spécifiques aux tâches
│   ├── dto.rs           # Data Transfer Objects
│   └── mod.rs           # Module API
├── models/              # Modèles de données
│   └── mod.rs           # Structures de données
├── routes/              # Définition des routes
│   └── mod.rs           # Configuration du routing
├── middleware/          # Middlewares HTTP
│   ├── auth.rs          # Authentification JWT
│   ├── cors.rs          # Gestion CORS
│   ├── logging.rs       # Logging des requêtes
│   ├── request_id.rs    # Ajout d'ID de requête
│   └── mod.rs
└── utils/               # Utilitaires
    ├── hash.rs          # Hash de mots de passe
    ├── validation.rs    # Validation des données
    └── mod.rs
```

## Flux de requête

```
Client HTTP
    ↓
Axum Router
    ↓
Middleware Stack
    ├── Request ID
    ├── CORS
    └── Logging
    ↓
Route Handler
    ↓
Validation (utils/validation)
    ↓
Business Logic (handlers)
    ↓
Database Layer (sqlx)
    ↓
Response Formatter (DTOs)
    ↓
Client HTTP
```

## Composants principaux

### 1. Configuration (`config.rs`)

Le module de configuration charge les variables d'environnement et fournit une structure typée `AppConfig` pour accéder à la configuration de l'application.

**Responsabilités** :
- Chargement des variables d'environnement
- Validation des valeurs de configuration
- Fourniture d'une configuration typée à toute l'application

### 2. Base de données (`database.rs`)

Gère la connexion à PostgreSQL via un pool de connexions SQLx.

**Responsabilités** :
- Création et gestion du pool de connexions
- Exécution des migrations SQL
- Fourniture de l'accès au pool via le trait Repository

**Pool de connexions** :
- Maximum : 10 connexions
- Timeout d'acquisition : 30 secondes

### 3. Gestion d'erreurs (`errors.rs`)

Système centralisé de gestion d'erreurs utilisant `thiserror` pour créer des types d'erreurs personnalisés.

**Types d'erreurs** :
- `Database` : Erreurs SQLx
- `Validation` : Erreurs de validation
- `Authentication` : Erreurs d'authentification
- `Authorization` : Erreurs d'autorisation
- `NotFound` : Ressources non trouvées
- `Internal` : Erreurs internes
- `BadRequest` : Requêtes invalides
- `Serialization` : Erreurs de sérialisation

**Trait IntoResponse** : Toutes les erreurs sont automatiquement converties en réponses HTTP appropriées.

### 4. Middleware

#### CORS (`middleware/cors.rs`)
Configure les en-têtes CORS pour permettre les requêtes cross-origin.

#### Logging (`middleware/logging.rs`)
Enregistre toutes les requêtes HTTP avec :
- Méthode HTTP
- URI
- Latence de réponse
- Erreurs éventuelles

#### Request ID (`middleware/request_id.rs`)
Ajoute un UUID unique à chaque requête dans l'en-tête `x-request-id` pour le traçage.

#### Auth (`middleware/auth.rs`)
Valide les tokens JWT et extrait les claims pour l'authentification.

### 5. Routes (`routes/mod.rs`)

Structure modulaire des routes :
- Routes génériques (health check)
- Routes spécifiques (tasks) organisées en modules

**Principe** : Chaque domaine métier a son propre module de routes.

### 6. Handlers (`api/`)

Les handlers sont organisés par domaine métier :
- `handlers.rs` : Handlers génériques
- `task_handlers.rs` : Handlers pour la gestion des tâches

**Pattern** :
1. Extraction des paramètres (Path, Query, Body)
2. Validation des données
3. Appel à la base de données
4. Formatage de la réponse
5. Retour de la réponse JSON

### 7. Modèles (`models/mod.rs`)

Les modèles représentent les structures de données :
- **Structures de base** : Correspondent aux tables de la base de données
- **Request DTOs** : Structures pour les requêtes (avec validation)
- **Response DTOs** : Structures pour les réponses (sans données sensibles)

**Exemple** : `Task` (DB) → `CreateTaskRequest` (Input) → `TaskResponse` (Output)

## Gestion de l'état

L'application utilise le système d'état d'Axum pour partager :
- `Database` : Pool de connexions à la base de données
- `AppConfig` : Configuration de l'application

Accès via `State<T>` dans les handlers.

## Sécurité

### Authentification JWT
- Tokens signés avec HS256
- Claims : `sub` (user ID), `exp` (expiration), `iat` (issued at)
- Validation automatique via middleware

### Hashing des mots de passe
- Algorithme : bcrypt
- Coût : DEFAULT_COST (10)

### Validation des entrées
- Utilisation de `validator` crate
- Validation au niveau des DTOs
- Rejet des données invalides avec codes d'erreur appropriés

### CORS
- Configuration configurable
- Par défaut : autorise toutes les origines (à restreindre en production)

## Base de données

### Migrations
Les migrations SQL sont stockées dans `migrations/` et exécutées automatiquement au démarrage.

**Convention de nommage** : `YYYYMMDDHHMMSS_description.sql`

### Pool de connexions
- Type : PostgreSQL via SQLx
- Gestion : Pool avec réutilisation des connexions
- Monitoring : Logs de connexion/erreur

## Logging

Système de logging structuré avec `tracing` :
- Niveaux : ERROR, WARN, INFO, DEBUG, TRACE
- Format : JSON (configurable)
- Filtrage : Par module (ex: `rustapi=debug`)

## Performance

### Optimisations
- Pool de connexions pour la réutilisation
- Async/await pour la concurrence
- Compression gzip pour les réponses HTTP
- Requêtes SQL optimisées avec index

### Monitoring
- Request IDs pour le traçage
- Logging de la latence
- Health check endpoint

## Extensibilité

### Ajouter un nouveau domaine métier

1. Créer le modèle dans `models/mod.rs`
2. Créer la migration SQL dans `migrations/`
3. Créer les handlers dans `api/`
4. Ajouter les routes dans `routes/mod.rs`

### Ajouter un middleware

1. Créer le fichier dans `middleware/`
2. Implémenter la fonction middleware
3. Ajouter le layer dans `main.rs`

## Limitations actuelles

- Pas de cache
- Pas de rate limiting
- Pas de pagination avancée (limite/offset basique)
- Pas de recherche/filtrage avancé

## Améliorations futures

- [ ] Cache Redis pour les requêtes fréquentes
- [ ] Rate limiting par IP
- [ ] Pagination avec curseurs
- [ ] Recherche full-text
- [ ] WebSockets pour les mises à jour en temps réel
- [ ] Métriques Prometheus
- [ ] Tests d'intégration automatisés


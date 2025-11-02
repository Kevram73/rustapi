# Guide de Développement

Ce guide vous aidera à contribuer et développer sur ce projet.

## Prérequis de développement

### Outils requis
- Rust 1.88+ (`rustup update`)
- Cargo (inclus avec Rust)
- Docker & Docker Compose
- PostgreSQL 16+ (optionnel, peut utiliser Docker)
- Git

### Outils recommandés
- **IDE** : VS Code avec extensions Rust
- **Linting** : `rust-analyzer` (inclus dans VS Code)
- **Formatage** : `rustfmt` (inclus avec Rust)

## Configuration de l'environnement

### 1. Cloner le projet

```bash
git clone <repository-url>
cd rustapi
```

### 2. Configuration locale

Copiez et configurez le fichier `.env` :

```bash
cp .env.example .env
```

Éditez `.env` avec vos valeurs :
```env
DATABASE_URL=postgresql://rustapi:rustapi_password@localhost:5432/rustapi
JWT_SECRET=your-secret-key-for-development
LOG_LEVEL=debug
```

### 3. Base de données locale

**Option A : Avec Docker**
```bash
docker-compose up -d postgres
```

**Option B : PostgreSQL local**
```bash
# Créer la base de données
createdb rustapi
psql rustapi < migrations/20240101000001_create_tasks.sql
```

## Workflow de développement

### Lancer l'application en développement

```bash
# Mode watch (recompile automatiquement)
cargo watch -x run

# Ou simplement
cargo run
```

### Tests

```bash
# Tous les tests
cargo test

# Tests avec output
cargo test -- --nocapture

# Tests spécifiques
cargo test task_handlers
```

### Formatage du code

```bash
# Formatter le code
cargo fmt

# Vérifier le formatage
cargo fmt -- --check
```

### Linting

```bash
# Analyser le code
cargo clippy

# Clippy avec suggestions
cargo clippy -- -W clippy::all
```

## Ajout de nouvelles fonctionnalités

### 1. Ajouter un nouveau modèle

**Étape 1** : Ajouter le modèle dans `src/models/mod.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MonModele {
    pub id: Uuid,
    pub nom: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMonModeleRequest {
    #[validate(length(min = 1, max = 100))]
    pub nom: String,
}
```

**Étape 2** : Créer la migration SQL

```bash
# Créer un fichier de migration
touch migrations/$(date +%Y%m%d%H%M%S)_create_mon_modele.sql
```

Contenu de la migration :
```sql
CREATE TABLE IF NOT EXISTS mon_modele (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nom VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Étape 3** : Créer les handlers

Créez `src/api/mon_modele_handlers.rs` avec les handlers CRUD.

### 2. Ajouter de nouvelles routes

Dans `src/routes/mod.rs` :

```rust
use crate::api::mon_modele_handlers;

pub fn create_router(db: Database) -> Router {
    Router::new()
        .nest("/mon-modele", mon_modele_routes())
        .with_state(db)
}

fn mon_modele_routes() -> Router<Database> {
    Router::new()
        .route("/", get(mon_modele_handlers::list))
        .route("/:id", get(mon_modele_handlers::get))
}
```

### 3. Ajouter un middleware personnalisé

**Étape 1** : Créer le fichier middleware

`src/middleware/mon_middleware.rs` :

```rust
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn mon_middleware(request: Request, next: Next) -> Response {
    // Votre logique ici
    next.run(request).await
}

pub fn create_mon_middleware_layer() -> impl tower::Layer<axum::Router> {
    axum::middleware::from_fn(mon_middleware)
}
```

**Étape 2** : Exporter dans `src/middleware/mod.rs`

```rust
pub mod mon_middleware;
```

**Étape 3** : Ajouter dans `src/main.rs`

```rust
.layer(mon_middleware::create_mon_middleware_layer())
```

## Bonnes pratiques

### Code Rust

1. **Clarté avant optimisation prématurée**
   - Écrivez du code lisible d'abord
   - Optimisez seulement si nécessaire

2. **Gestion d'erreurs**
   - Utilisez `Result<T, AppError>` pour les erreurs récupérables
   - Utilisez `unwrap()` seulement dans les tests ou les cas sûrs
   - Utilisez `?` pour propager les erreurs

3. **Noms de fonctions**
   - Préfixez avec le verbe : `get_`, `create_`, `update_`, `delete_`
   - Utilisez des noms explicites

4. **Documentation**
   - Documentez les fonctions publiques avec `///`
   - Ajoutez des exemples avec `/// # Example`

### Structure de code

1. **Séparation des responsabilités**
   - Handlers : logique HTTP
   - Models : structures de données
   - Database : accès aux données

2. **DRY (Don't Repeat Yourself)**
   - Extrayez la logique commune dans des fonctions
   - Réutilisez les utilitaires

3. **SOLID**
   - Single Responsibility
   - Open/Closed
   - Liskov Substitution
   - Interface Segregation
   - Dependency Inversion

### Base de données

1. **Requêtes SQL**
   - Utilisez des requêtes paramétrées (protection SQL injection)
   - Indexez les colonnes fréquemment interrogées
   - Évitez les requêtes N+1

2. **Migrations**
   - Testez les migrations avant de les déployer
   - Les migrations doivent être réversibles (idéalement)
   - Nommez les migrations de manière descriptive

### Tests

1. **Tests unitaires**
   - Testez chaque fonction individuellement
   - Mock les dépendances externes

2. **Tests d'intégration**
   - Testez les endpoints complets
   - Utilisez une base de données de test

3. **Exemple de test**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        // Arrange
        let request = CreateTaskRequest {
            title: "Test".to_string(),
            description: None,
        };

        // Act & Assert
        assert!(validate(&request).is_ok());
    }
}
```

## Debugging

### Logs

Configurez le niveau de log dans `.env` :
```env
RUST_LOG=rustapi=debug,tower_http=debug
```

### Logs dans le code

```rust
tracing::debug!("Message de debug");
tracing::info!("Information");
tracing::warn!("Avertissement");
tracing::error!("Erreur");
```

### Debugger avec VS Code

1. Installez l'extension "CodeLLDB"
2. Créez `.vscode/launch.json` :
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug Rust API",
            "cargo": {
                "args": ["build", "--bin", "rustapi"],
                "filter": {
                    "name": "rustapi",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

## Git Workflow

### Commits

Format des messages de commit :
```
type(scope): description courte

Description détaillée (optionnelle)
```

Types :
- `feat` : Nouvelle fonctionnalité
- `fix` : Correction de bug
- `docs` : Documentation
- `style` : Formatage
- `refactor` : Refactorisation
- `test` : Tests
- `chore` : Maintenance

Exemple :
```
feat(tasks): ajout de la pagination

Ajout de la pagination avec limite et offset pour la liste des tâches.
Les paramètres page et limit sont maintenant supportés.
```

### Branches

- `main` : Production
- `develop` : Développement
- `feature/nom` : Nouvelles fonctionnalités
- `fix/nom` : Corrections de bugs

## Performance

### Profiling

Utilisez `cargo flamegraph` pour analyser les performances :
```bash
cargo install flamegraph
cargo flamegraph --bin rustapi
```

### Optimisations courantes

1. **Pool de connexions** : Déjà configuré (10 connexions max)
2. **Compression** : Gzip activé dans tower-http
3. **Requêtes SQL** : Utilisez EXPLAIN ANALYZE pour optimiser

## Sécurité

### Checklist avant commit

- [ ] Pas de secrets dans le code
- [ ] Validation des entrées utilisateur
- [ ] Requêtes SQL paramétrées
- [ ] Gestion d'erreurs appropriée
- [ ] Logs ne contiennent pas d'informations sensibles

### Audit de sécurité

```bash
cargo audit
```

Installez avec :
```bash
cargo install cargo-audit
```

## Ressources

- [Documentation Rust](https://doc.rust-lang.org/)
- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)


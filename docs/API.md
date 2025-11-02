# Documentation API

Documentation complète de l'API REST.

## Base URL

- **Développement** : `http://localhost:3000/api`
- **Production** : `https://api.example.com/api`

## Format de réponse

Toutes les réponses suivent le format standard :

```json
{
  "success": true,
  "data": { ... },
  "message": "Message optionnel"
}
```

En cas d'erreur :
```json
{
  "error": "Message d'erreur",
  "status": 400
}
```

## Codes de statut HTTP

- `200` - Succès
- `400` - Requête invalide (validation échouée)
- `401` - Non authentifié
- `403` - Non autorisé
- `404` - Ressource non trouvée
- `500` - Erreur interne du serveur

## Endpoints

### Health Check

#### `GET /health`

Vérifie l'état de l'API.

**Réponse :**
```json
{
  "success": true,
  "data": {
    "status": "ok",
    "timestamp": "2024-01-01T12:00:00Z"
  }
}
```

---

### Tâches

#### `GET /tasks`

Récupère la liste de toutes les tâches.

**Paramètres de requête :**
- `page` (optionnel) : Numéro de page (défaut: 1)
- `limit` (optionnel) : Nombre d'éléments par page (défaut: 20, max: 100)

**Exemple :**
```bash
GET /api/tasks?page=1&limit=10
```

**Réponse :**
```json
{
  "success": true,
  "data": [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "title": "Faire les courses",
      "description": "Acheter du lait",
      "completed": false,
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z"
    }
  ]
}
```

---

#### `GET /tasks/{id}`

Récupère une tâche par son ID.

**Paramètres :**
- `id` (path) : UUID de la tâche

**Exemple :**
```bash
GET /api/tasks/123e4567-e89b-12d3-a456-426614174000
```

**Réponse :**
```json
{
  "success": true,
  "data": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "Faire les courses",
    "description": "Acheter du lait",
    "completed": false,
    "created_at": "2024-01-01T12:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z"
  }
}
```

**Erreurs :**
- `404` : Tâche non trouvée

---

#### `POST /tasks`

Crée une nouvelle tâche.

**Body :**
```json
{
  "title": "Nouvelle tâche",
  "description": "Description optionnelle"
}
```

**Validation :**
- `title` : Requis, 1-200 caractères
- `description` : Optionnel, max 1000 caractères

**Réponse :**
```json
{
  "success": true,
  "data": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "Nouvelle tâche",
    "description": "Description optionnelle",
    "completed": false,
    "created_at": "2024-01-01T12:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z"
  },
  "message": "Tâche créée avec succès"
}
```

**Erreurs :**
- `400` : Validation échouée

---

#### `PUT /tasks/{id}`

Met à jour une tâche existante. Tous les champs sont optionnels.

**Paramètres :**
- `id` (path) : UUID de la tâche

**Body :**
```json
{
  "title": "Titre modifié",
  "description": "Nouvelle description",
  "completed": true
}
```

**Validation :**
- `title` : Optionnel, 1-200 caractères
- `description` : Optionnel, max 1000 caractères
- `completed` : Optionnel, booléen

**Réponse :**
```json
{
  "success": true,
  "data": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "Titre modifié",
    "description": "Nouvelle description",
    "completed": true,
    "created_at": "2024-01-01T12:00:00Z",
    "updated_at": "2024-01-01T12:05:00Z"
  },
  "message": "Tâche mise à jour avec succès"
}
```

**Erreurs :**
- `400` : Validation échouée
- `404` : Tâche non trouvée

---

#### `DELETE /tasks/{id}`

Supprime une tâche.

**Paramètres :**
- `id` (path) : UUID de la tâche

**Exemple :**
```bash
DELETE /api/tasks/123e4567-e89b-12d3-a456-426614174000
```

**Réponse :**
```json
{
  "success": true,
  "data": null,
  "message": "Tâche supprimée avec succès"
}
```

**Erreurs :**
- `404` : Tâche non trouvée

---

## Modèles de données

### Task

```typescript
interface Task {
  id: string;              // UUID
  title: string;           // 1-200 caractères
  description?: string;    // Max 1000 caractères
  completed: boolean;      // Défaut: false
  created_at: string;      // ISO 8601 datetime
  updated_at: string;      // ISO 8601 datetime
}
```

### CreateTaskRequest

```typescript
interface CreateTaskRequest {
  title: string;           // Requis, 1-200 caractères
  description?: string;    // Optionnel, max 1000 caractères
}
```

### UpdateTaskRequest

```typescript
interface UpdateTaskRequest {
  title?: string;          // Optionnel, 1-200 caractères
  description?: string;    // Optionnel, max 1000 caractères
  completed?: boolean;     // Optionnel
}
```

## Authentification (Futur)

L'authentification JWT sera implémentée prochainement. Les tokens devront être inclus dans l'en-tête :

```
Authorization: Bearer <token>
```

## Rate Limiting (Futur)

Des limites de débit seront appliquées :
- 100 requêtes/minute par IP pour les endpoints publics
- 1000 requêtes/minute par IP pour les endpoints authentifiés

## Pagination

La pagination utilise les paramètres `page` et `limit` :

- `page` : Numéro de page (commence à 1)
- `limit` : Nombre d'éléments par page (max 100)

L'offset est calculé automatiquement : `offset = (page - 1) * limit`

## Filtrage et tri (Futur)

Les fonctionnalités suivantes sont prévues :
- Filtrage par statut (`completed`)
- Tri par date de création
- Recherche par titre

## Codes d'erreur détaillés

### 400 - Bad Request

```json
{
  "error": "Erreur de validation: title: length",
  "status": 400
}
```

### 404 - Not Found

```json
{
  "error": "Ressource non trouvée: Tâche avec l'id 123e4567-e89b-12d3-a456-426614174000 non trouvée",
  "status": 404
}
```

### 500 - Internal Server Error

```json
{
  "error": "Une erreur interne s'est produite",
  "status": 500
}
```

## Spécification OpenAPI

Une spécification OpenAPI complète est disponible dans le fichier `openapi.yaml`.

Vous pouvez visualiser la documentation interactive avec Swagger UI :

```bash
# Installer swagger-ui
docker run -p 8080:8080 -e SWAGGER_JSON=/openapi.yaml -v $(pwd)/openapi.yaml:/openapi.yaml swaggerapi/swagger-ui
```

Puis ouvrir http://localhost:8080 dans votre navigateur.


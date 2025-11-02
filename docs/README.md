# Documentation du Projet

Bienvenue dans la documentation du projet Rust API. Cette documentation couvre tous les aspects du projet.

## 📚 Guide de lecture

### Pour les nouveaux développeurs

1. Commencez par le [README principal](../README.md) pour une vue d'ensemble
2. Lisez [ARCHITECTURE.md](ARCHITECTURE.md) pour comprendre la structure
3. Consultez [DEVELOPMENT.md](DEVELOPMENT.md) pour commencer à développer

### Pour les développeurs expérimentés

1. [ARCHITECTURE.md](ARCHITECTURE.md) - Détails techniques approfondis
2. [API.md](API.md) - Documentation complète de l'API
3. [DEVELOPMENT.md](DEVELOPMENT.md) - Guide de contribution

### Pour les opérations / DevOps

1. [DEPLOYMENT.md](DEPLOYMENT.md) - Guide complet de déploiement
2. [ARCHITECTURE.md](ARCHITECTURE.md) - Section Monitoring et Performance

## 📖 Index de la documentation

### Technique

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Architecture du projet
  - Structure des modules
  - Flux de requête
  - Composants principaux
  - Gestion de l'état
  - Sécurité
  - Performance
  - Extensibilité

- **[DEVELOPMENT.md](DEVELOPMENT.md)** - Guide de développement
  - Configuration de l'environnement
  - Workflow de développement
  - Ajout de fonctionnalités
  - Bonnes pratiques
  - Tests
  - Debugging
  - Git workflow

- **[API.md](API.md)** - Documentation de l'API
  - Endpoints disponibles
  - Format de réponse
  - Modèles de données
  - Codes d'erreur
  - Authentification
  - Pagination

### Opérationnel

- **[DEPLOYMENT.md](DEPLOYMENT.md)** - Guide de déploiement
  - Options de déploiement
  - Configuration Docker
  - Configuration Nginx
  - HTTPS avec Let's Encrypt
  - Monitoring
  - Sauvegarde
  - Mise à jour
  - Troubleshooting

- **[DOCKER_TROUBLESHOOTING.md](DOCKER_TROUBLESHOOTING.md)** - Guide de dépannage Docker
  - Problèmes courants
  - Solutions et commandes utiles
  - Debugging avancé

### Référence

- **[../API_EXAMPLES.md](../API_EXAMPLES.md)** - Exemples de requêtes
  - Exemples curl
  - Exemples JavaScript/Fetch
  - Gestion des erreurs

- **[../openapi.yaml](../openapi.yaml)** - Spécification OpenAPI
  - Spécification complète au format OpenAPI 3.0
  - Compatible Swagger UI
  - Documentation interactive

## 🔍 Recherche rapide

### Comment faire X ?

- **Ajouter un endpoint** → [DEVELOPMENT.md](DEVELOPMENT.md#ajouter-une-nouvelle-route)
- **Déployer en production** → [DEPLOYMENT.md](DEPLOYMENT.md)
- **Comprendre l'architecture** → [ARCHITECTURE.md](ARCHITECTURE.md)
- **Tester l'API** → [API_EXAMPLES.md](../API_EXAMPLES.md)
- **Gérer les erreurs** → [ARCHITECTURE.md](ARCHITECTURE.md#3-gestion-derreurs-errorsrs)
- **Ajouter un middleware** → [DEVELOPMENT.md](DEVELOPMENT.md#3-ajouter-un-middleware-personnalisé)
- **Configurer la base de données** → [ARCHITECTURE.md](ARCHITECTURE.md#base-de-données)
- **Debugger** → [DEVELOPMENT.md](DEVELOPMENT.md#debugging)

## 📝 Documentation Rust

Pour générer la documentation Rust complète du code :

```bash
# Générer et ouvrir dans le navigateur
cargo doc --open

# Générer uniquement
cargo doc

# La documentation sera dans target/doc/
```

## 🤝 Contribution

Si vous souhaitez améliorer cette documentation :

1. Lisez [DEVELOPMENT.md](DEVELOPMENT.md) pour le workflow Git
2. Modifiez les fichiers Markdown dans `docs/`
3. Assurez-vous que les liens fonctionnent
4. Soumettez une pull request

## 📞 Support

- **Issues** : Créez une issue sur le dépôt Git
- **Questions** : Consultez d'abord cette documentation
- **Bugs** : Reportez-les avec les logs et étapes de reproduction

## 🔄 Mise à jour

Cette documentation est maintenue à jour avec le code. Si vous trouvez une incohérence, merci de créer une issue.

---

*Dernière mise à jour : 2024*


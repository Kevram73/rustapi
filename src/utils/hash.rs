use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    hash(password, DEFAULT_COST).map_err(|e| anyhow::anyhow!("Erreur lors du hachage: {}", e))
}

pub fn verify_password(password: &str, hash: &str) -> anyhow::Result<bool> {
    verify(password, hash).map_err(|e| anyhow::anyhow!("Erreur lors de la vérification: {}", e))
}


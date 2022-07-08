use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Secret {
    pub id: String,
    pub name: String,
    pub digest: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub secrets: Vec<Secret>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecretResponse {
    pub secret: Secret,
}

pub fn validate_name(name: &str) -> Result<(), String> {
    let regex = regex::Regex::new(r"^[a-zA-Z0-9_]{1,64}$").unwrap();

    if regex.is_match(name) {
        Ok(())
    } else {
        panic!("Invalid name. Secret names are limited to 64 characters in length, must be alphanumeric (with underscores) and are automatically uppercased.");
    }
}

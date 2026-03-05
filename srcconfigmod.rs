use std::env;
use dotenv::dotenv;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Environment variable {0} not found")]
    MissingEnvVar(String),
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(String, String),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub firestore_project_id: String,
    pub firestore_credentials_path: String,
    pub instance_id: String,
    pub max_memory_mb: u32,
    pub log_level: String,
    pub planning_depth_limit: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();
        
        let get_env = |key: &str| {
            env::var(key).map_err(|_| ConfigError::MissingEnvVar(key.to_string()))
        };
        
        Ok(Config {
            firestore_project_id: get_env("FIRESTORE_PROJECT_ID")?,
            firestore_credentials_path: get_env("FIRESTORE_CREDENTIALS_PATH")?,
            instance_id: get_env("INSTANCE_ID").unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()),
            max_memory_mb: get_env("MAX_MEMORY_MB")
                .unwrap_or_else(|_| "450".to_string())
                .parse()
                .map_err(|e| ConfigError::InvalidValue("MAX_MEMORY_MB".to_string(), e.to_string()))?,
            log_level: get_env("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            planning_depth_limit: get_env("PLANNING_DEPTH_LIMIT")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .map_err(|e| ConfigError::InvalidValue("PLANNING_DEPTH_LIMIT".to_string(), e.to_string()))?,
        })
    }
    
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_memory_mb > 500 {
            return Err(ConfigError::InvalidValue(
                "MAX_MEMORY_MB".to_string(),
                "Cannot exceed 500MB for micro-instance".to_string()
            ));
        }
        
        // Validate Firestore credentials exist
        if !std::path::Path::new(&self.firestore_credentials_path).exists() {
            return Err(ConfigError::InvalidValue(
                "FIRESTORE_CREDENTIALS_PATH".to_string(),
                "Credentials file does not exist".to_string()
            ));
        }
        
        Ok(())
    }
}
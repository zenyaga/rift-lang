use thiserror::Error;

#[derive(Error, Debug)]
pub enum RiftError {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Execution error in {language}: {message}")]
    ExecutionError { language: String, message: String },
    
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),
    
    #[error("Deployment error for {target}: {message}")]
    DeploymentError { target: String, message: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Language transformation error: {from} -> {to}: {message}")]
    TransformationError {
        from: String,
        to: String,
        message: String,
    },
    
    #[error("Variable not found: {0}")]
    VariableNotFound(String),
    
    #[error("Function not found: {0}")]
    FunctionNotFound(String),
    
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    
    #[error("Dependency installation failed: {language}: {dependency}")]
    DependencyError { language: String, dependency: String },
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Tree-sitter parsing error: {0}")]
    TreeSitterError(String),
}

pub type Result<T> = std::result::Result<T, RiftError>;

impl From<String> for RiftError {
    fn from(s: String) -> Self {
        RiftError::ParseError(s)
    }
}

impl From<&str> for RiftError {
    fn from(s: &str) -> Self {
        RiftError::ParseError(s.to_string())
    }
}
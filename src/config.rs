use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub orchestrator: Orchestrator,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Orchestrator {
    pub base_llm: String,
    pub db_connection: DbConnection,
    pub logging: Logging,
    pub health_checks: HealthChecks,
    pub error_recovery: ErrorRecovery,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct DbConnection {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Logging {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct HealthChecks {
    pub interval_seconds: u64,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ErrorRecovery {
    pub auto_restart: bool,
    pub retry_intervals: Vec<u32>,
}

impl Default for Orchestrator {
    fn default() -> Self {
        Orchestrator {
            base_llm: "model_gpt4".to_string(),
            db_connection: DbConnection::default(),
            logging: Logging::default(),
            health_checks: HealthChecks::default(),
            error_recovery: ErrorRecovery::default(),
        }
    }
}

impl Default for DbConnection {
    fn default() -> Self {
        DbConnection {
            host: "localhost".to_string(),
            port: 5432,
            user: "admin".to_string(),
            password: "securepassword".to_string(),
        }
    }
}

impl Default for Logging {
    fn default() -> Self {
        Logging {
            level: "INFO".to_string(),
            format: "standard".to_string(),
        }
    }
}

impl Default for HealthChecks {
    fn default() -> Self {
        HealthChecks {
            interval_seconds: 3600,
            enabled: true,
        }
    }
}

impl Default for ErrorRecovery {
    fn default() -> Self {
        ErrorRecovery {
            auto_restart: true,
            retry_intervals: vec![1, 5, 15],
        }
    }
}
use clap::ValueEnum;
use serde::Deserialize;
use std::fmt;
use std::time::Duration;
use tokio_process_tools::UnixGracefulPhase;

#[derive(Copy, Clone, PartialEq, Eq, Deserialize, ValueEnum)]
#[serde(rename_all = "UPPERCASE")]
pub enum UnixSignal {
    #[value(name = "SIGINT")]
    Sigint,
    #[value(name = "SIGTERM")]
    Sigterm,
}

impl fmt::Debug for UnixSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            UnixSignal::Sigint => "SIGINT",
            UnixSignal::Sigterm => "SIGTERM",
        })
    }
}

impl UnixSignal {
    pub fn parse_env(s: &str) -> Result<Self, String> {
        match s.trim() {
            "SIGINT" => Ok(UnixSignal::Sigint),
            "SIGTERM" => Ok(UnixSignal::Sigterm),
            other => Err(format!(
                "invalid LEPTOS_GRACEFUL_SHUTDOWN_UNIX_SIGNAL value `{other}` \
                 (expected `SIGINT` or `SIGTERM`)"
            )),
        }
    }

    pub fn to_graceful_shutdown_phase(self, timeout: Duration) -> UnixGracefulPhase {
        match self {
            UnixSignal::Sigint => UnixGracefulPhase::interrupt(timeout),
            UnixSignal::Sigterm => UnixGracefulPhase::terminate(timeout),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_env_accepts_sigint() {
        assert_eq!(UnixSignal::parse_env("SIGINT"), Ok(UnixSignal::Sigint));
    }

    #[test]
    fn parse_env_accepts_sigterm() {
        assert_eq!(UnixSignal::parse_env("SIGTERM"), Ok(UnixSignal::Sigterm));
    }

    #[test]
    fn parse_env_trims_whitespace() {
        assert_eq!(UnixSignal::parse_env("  SIGINT\n"), Ok(UnixSignal::Sigint));
    }

    #[test]
    fn parse_env_is_case_sensitive() {
        assert!(UnixSignal::parse_env("sigint").is_err());
        assert!(UnixSignal::parse_env("SigInt").is_err());
    }

    #[test]
    fn parse_env_rejects_unknown_signal() {
        let err = UnixSignal::parse_env("SIGKILL").unwrap_err();
        assert!(err.contains("SIGKILL"));
        assert!(err.contains("SIGINT"));
        assert!(err.contains("SIGTERM"));
    }

    #[test]
    fn deserializes_from_uppercase() {
        let s: UnixSignal = serde_json::from_str(r#""SIGINT""#).unwrap();
        assert_eq!(s, UnixSignal::Sigint);
        let s: UnixSignal = serde_json::from_str(r#""SIGTERM""#).unwrap();
        assert_eq!(s, UnixSignal::Sigterm);
    }

    #[test]
    fn deserialize_rejects_lowercase() {
        assert!(serde_json::from_str::<UnixSignal>(r#""sigint""#).is_err());
    }
}

use anyhow::anyhow;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::time::Duration;
use tokio_process_tools::UnixGracefulPhase;

/// Signal used to gracefully shut down the user application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixSignal {
    Sigint,
    Sigterm,
}

impl UnixSignal {
    pub fn to_graceful_shutdown_phase(self, timeout: Duration) -> UnixGracefulPhase {
        match self {
            UnixSignal::Sigint => UnixGracefulPhase::interrupt(timeout),
            UnixSignal::Sigterm => UnixGracefulPhase::terminate(timeout),
        }
    }
}

impl FromStr for UnixSignal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "SIGINT" => Ok(UnixSignal::Sigint),
            "SIGTERM" => Ok(UnixSignal::Sigterm),
            other => Err(anyhow!(
                "Invalid UnixSignal `{other}`. Expected one of: `SIGINT`, `SIGTERM`."
            )),
        }
    }
}

impl<'de> Deserialize<'de> for UnixSignal {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = <&str>::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_str {
        use super::*;

        #[test]
        fn accepts_uppercase() {
            assert_eq!(UnixSignal::from_str("SIGINT").unwrap(), UnixSignal::Sigint);
            assert_eq!(
                UnixSignal::from_str("SIGTERM").unwrap(),
                UnixSignal::Sigterm
            );
        }

        #[test]
        fn is_case_sensitive() {
            assert!(UnixSignal::from_str("sigint").is_err());
            assert!(UnixSignal::from_str("SigInt").is_err());
        }

        #[test]
        fn trims_whitespace() {
            assert_eq!(
                UnixSignal::from_str("  SIGINT\n").unwrap(),
                UnixSignal::Sigint
            );
        }

        #[test]
        fn rejects_unknown_signal() {
            assert_eq!(
                UnixSignal::from_str("SIGKILL").unwrap_err().to_string(),
                "Invalid UnixSignal `SIGKILL`. Expected one of: `SIGINT`, `SIGTERM`."
            );
        }
    }

    mod deserialize {
        use super::*;

        #[test]
        fn deserializes_from_uppercase() {
            let s: UnixSignal = serde_json::from_str(r#""SIGTERM""#).unwrap();
            assert_eq!(s, UnixSignal::Sigterm);
            assert!(serde_json::from_str::<UnixSignal>(r#""sigterm""#).is_err());
        }

        #[test]
        fn rejects_lowercase() {
            assert!(serde_json::from_str::<UnixSignal>(r#""sigterm""#).is_err());
        }
    }
}

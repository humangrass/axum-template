use std::path::PathBuf;
use clap::Parser;
use multitool_hg::logger::tracer_logger::LogLevel;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom path to config file
    #[arg(short, long, value_name = "CONFIG_FILE", default_value = "auth.config.yaml")]
    pub config: PathBuf,

    /// Sets a custom log level
    #[arg(
        short,
        long,
        value_name = "LOG_LEVEL",
        default_value = "info",
        value_enum
    )]
    pub log_level: LogLevel,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_arguments() {
        let args = Cli::try_parse_from(&["test-app"]).unwrap();
        assert_eq!(args.config, PathBuf::from("auth.config.yaml"));
        assert_eq!(args.log_level, LogLevel::Info);
    }

    #[test]
    fn test_custom_arguments() {
        let args = Cli::try_parse_from(&[
            "test-app",
            "--config", "custom_config.yaml",
            "--log-level", "debug"
        ]).unwrap();

        assert_eq!(args.config, PathBuf::from("custom_config.yaml"));
        assert_eq!(args.log_level, LogLevel::Debug);
    }
}

use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not identify environment {}", env))]
    #[snafu(visibility(pub))]
    Environment { env: String },

    #[snafu(display("Config Error: {} [{}]", msg, source))]
    #[snafu(visibility(pub))]
    ConfigError {
        msg: String,
        source: config::ConfigError,
    },

    #[snafu(display("Env Var Error: {}", msg))]
    #[snafu(visibility(pub))]
    EnvVarError {
        msg: String,
        source: std::env::VarError,
    },

    #[snafu(display("Miscellaneous Error: {}", msg))]
    #[snafu(visibility(pub))]
    MiscError { msg: String },

    #[snafu(display("Tokio IO Error: {}", msg))]
    #[snafu(visibility(pub))]
    TokioIOError {
        msg: String,
        source: tokio::io::Error,
    },

    #[snafu(display("Std IO Error: {}", msg))]
    #[snafu(visibility(pub))]
    IOError { msg: String, source: std::io::Error },

    #[snafu(display("JSON Error: {} - {}", msg, source))]
    #[snafu(visibility(pub))]
    JSONError {
        msg: String,
        source: serde_json::Error,
    },

    #[snafu(display("Reqwest Error: {} - {}", msg, source))]
    #[snafu(visibility(pub))]
    ReqwestError { msg: String, source: reqwest::Error },

    #[snafu(display("INotify Error: {}", source))]
    #[snafu(visibility(pub))]
    INotifyError {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("File Error '{}': {}", path.display(), source))]
    #[snafu(visibility(pub))]
    FileIOError {
        path: std::path::PathBuf,
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("File Error: {}", details))]
    #[snafu(visibility(pub))]
    FileError { details: String },

    #[snafu(display("UUID Error: {}", source))]
    #[snafu(visibility(pub))]
    UuidError { source: uuid::Error },

    #[snafu(display("YAML Error: {}", source))]
    #[snafu(visibility(pub))]
    YamlError { source: serde_yaml::Error },
}

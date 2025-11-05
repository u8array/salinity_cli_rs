use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error reading from stdin: {source}")]
    ReadStdin {
        #[source]
        source: std::io::Error,
    },

    #[error("Error reading file '{path}': {source}")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid JSON for --inputs-json: {source}")]
    ParseInputsJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("Invalid JSON for --assumptions-json: {source}")]
    ParseAssumptionsJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("Invalid JSON in input document: {source}")]
    ParseCmdInputJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("Could not serialize output to JSON: {source}")]
    SerializeOutput {
        #[source]
        source: serde_json::Error,
    },

    #[error("Unexpected error: {0}")]
    Other(String),

    #[error("Missing input data: provide --input or --inputs-json")]
    MissingInputData,

    #[error(
        "Missing assumptions: provide --assumptions-json or include 'assumptions' in the input document"
    )]
    MissingAssumptions,
}

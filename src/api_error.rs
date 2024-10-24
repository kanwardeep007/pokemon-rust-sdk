#[derive(thiserror::Error, Debug)]
pub enum PokemonSdkError {
    #[error("Unable to generate URL for the requested resource. Error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Http Error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Error occurred when running a middleware. Error: {0}")]
    MiddlewareError(#[from] reqwest_middleware::Error),
    #[error{"Non 2xx response. Response Status Code: {status_code:?}. Response Text {response_text:?}"}]
    FailureResponse {
        status_code: reqwest::StatusCode,
        response_text: String,
    },
    #[error("Unexpected error happened: {0}")]
    Other(#[from] anyhow::Error),
}

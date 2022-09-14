use actix_web::{
    error::ResponseError,
    http::StatusCode,
    Error, FromRequest, HttpResponse,
};
use awc::Client;
use actix_web_httpauth::{
    extractors::bearer::BearerAuth,
    headers::www_authenticate::bearer::Bearer,
};
use derive_more::Display;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::Deserialize;
use std::{collections::HashSet, future::Future, pin::Pin};

use crate::types::ErrorMessage;

const AUDIENCE_URL: &'static str = "https://alpinemd.com/";
const AUTHORITY_URL: &'static str = "https://dev-g8-zbmf7.us.auth0.com/";
const JWKS_URL: &'static str = "https://dev-g8-zbmf7.us.auth0.com/.well-known/jwks.json";

#[derive(Debug, Display)]
enum ClientError {
    #[display(fmt = "authentication")]
    Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),

    #[display(fmt = "decode")]
    Decode(jsonwebtoken::errors::Error),

    #[display(fmt = "not_found")]
    NotFound(String),

    #[display(fmt = "jwks_fetch_failure")]
    JWKSFetchFailure,

    #[display(fmt = "unsupported_algorithm")]
    UnsupportedAlgorithm(AlgorithmParameters),
}

impl ResponseError for ClientError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
            Self::Decode(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(
                    "Authorization header value must follow this format: Bearer access-token".to_string()
                ),
                message: "Bad credentials".to_string(),
            }),
            Self::NotFound(msg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(msg.to_string()),
                message: "Bad credentials".to_string(),
            }),
            Self::JWKSFetchFailure => HttpResponse::InternalServerError().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Failed to fetch jwks to validate access token".to_string(),
            }),
            Self::UnsupportedAlgorithm(alg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(format!(
                    "Unsupported encryption algorithm expected RSA got {:?}",
                    alg
                )),
                message: "Bad credentials".to_string(),
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    permissions: Option<HashSet<String>>,
}

impl Claims {
    pub fn validate_permissions(&self, required_permissions: &HashSet<String>) -> bool {
        self.permissions.as_ref().map_or(false, |permissions| permissions.is_superset(required_permissions))
    }
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extractor = BearerAuth::extract(req);
        Box::pin(async move {
            let credentials = extractor.await.map_err(ClientError::Authentication)?;
            let token = credentials.token();
            let header = decode_header(token).map_err(ClientError::Decode)?;
            let kid = header.kid.ok_or_else(|| {
                ClientError::NotFound("kid not found in token header".to_string())
            })?;
            let jwks: JwkSet = Client::new()
                .get(JWKS_URL)
                .send()
                .await
                .map_err(|_| ClientError::JWKSFetchFailure)?
                .json()
                .await
                .map_err(|_| ClientError::JWKSFetchFailure)?;
            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| ClientError::NotFound("No JWK found for kid".to_string()))?;

            match jwk.clone().algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let mut validation = Validation::new(Algorithm::RS256);
                    validation.set_audience(&[AUDIENCE_URL]);
                    validation.set_issuer(&[AUTHORITY_URL]);
                    let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .map_err(ClientError::Decode)?;
                    let token =
                        decode::<Claims>(token, &key, &validation).map_err(ClientError::Decode)?;
                    Ok(token.claims)
                }
                algorithm => Err(ClientError::UnsupportedAlgorithm(algorithm).into())
            }
        })
    }
}

use crate::errors::ServiceError;
use argon2::{self, Config};

lazy_static::lazy_static! {
    pub static ref IDENTITY_SECRET_KEY: String = std::env::var("IDENTITY_SECRET_KEY").expect("IDENTITIY_SECRET_KEY must be set.");
    pub static ref PWHASH_SECRET_KEY: String = std::env::var("PWHASH_SECRET_KEY").expect("PWHASH_SECRET_KEY must be set.");
}

const SALT: &'static [u8] = b"supersecuresalt";

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = Config {
        secret: PWHASH_SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), PWHASH_SECRET_KEY.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}
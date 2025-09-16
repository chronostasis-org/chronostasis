use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Serialize)]
pub struct UserGetDto {
  pub id: String,
  pub slug: String,
  pub email: String,
  pub username: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UserCreateDto {
  // Username rules:
  // - Allowed: a-z, A-Z, 0-9, hyphen (-)
  // - Cannot start or end with a hyphen
  // - No consecutive hyphens
  // - Max length: 39
  // - ASCII only
  // Compatible to Github username rules
  #[validate(custom(function = "validate_ascii"))]
  #[validate(length(min = 1, max = 39))]
  #[validate(regex(
    path = *USERNAME_RE,
    message = "username may contain only letters, digits, and single hyphens between characters; it cannot start/end with a hyphen"
  ))]
  pub username: String,

  // Email: ASCII only + valid email format
  #[validate(custom(function = "validate_ascii"))]
  #[validate(email)]
  pub email: String,

  // Password: ASCII only; allowed chars: A–Z, a–z, 0–9, !@#$%^&*; length 8–32
  #[validate(custom(function = "validate_ascii"))]
  #[validate(length(min = 8, max = 32))]
  #[validate(regex(
    path = *PASSWORD_RE,
    message = "password may contain only A-Z, a-z, 0-9, and !@#$%^&*"
  ))]
  pub password: String,
}

// Lazily compiled regexes (compiled once on first use)
// Username: segments of [A-Za-z0-9] separated by single '-' only.
// This ensures: no leading/trailing '-', no consecutive '-', only allowed chars.
static USERNAME_RE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*$").expect("valid USERNAME regex"));

// Password allows only A-Z, a-z, 0-9, and !@#$%^&*
static PASSWORD_RE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[A-Za-z0-9!@#$%^&*]{8,32}$").expect("valid PASSWORD regex"));

impl UserCreateDto {
  /// Safe normalization:
  /// - Lowercase + trim email for consistent uniqueness checks
  pub fn normalize(mut self) -> Self {
    self.email = self.email.trim().to_lowercase();
    self
  }
}

/// Custom validator ensuring the whole string is ASCII-only
fn validate_ascii(value: &str) -> Result<(), ValidationError> {
  if value.is_ascii() {
    Ok(())
  } else {
    Err(ValidationError::new("non_ascii"))
  }
}

/// Slugify username: just lowercase
pub fn slug_from_username(username: &str) -> String {
  username.to_ascii_lowercase()
}

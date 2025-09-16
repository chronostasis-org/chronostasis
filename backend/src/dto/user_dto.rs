use serde::{Deserialize, Serialize};
use validator::Validate;

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
  // Username: A–Z, a–z, 0–9, dot; length 3–16
  #[validate(length(min = 3, max = 16))]
  #[validate(regex(
    path = *USERNAME_RE,
    message = "username may contain only letters, digits, and dots"
  ))]
  pub username: String,

  #[validate(email)]
  pub email: String,

  // Password: printable ASCII excluding space; length 8–32
  #[validate(length(min = 8, max = 32))]
  #[validate(regex(
    path = *PASSWORD_RE,
    message = "password must be printable ASCII without spaces"
  ))]
  pub password: String,
}

// Lazily compiled regexes (compiled once on first use)
static USERNAME_RE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[A-Za-z0-9.]{3,16}$").expect("valid USERNAME regex"));

static PASSWORD_RE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[\x21-\x7E]{8,32}$").expect("valid PASSWORD regex"));

impl UserCreateDto {
  /// Safe normalization:
  /// - Lowercase + trim email for consistent uniqueness checks
  pub fn normalize(mut self) -> Self {
    // self.username = self.username.trim().to_string();
    self.email = self.email.trim().to_lowercase();
    self
  }
}

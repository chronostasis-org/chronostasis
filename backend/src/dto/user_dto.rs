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
  // Username: A–Z, a–z, 0–9, dot; length 3–16; must be ASCII; cannot start with dot
  #[validate(custom(function = "validate_ascii"))]
  #[validate(length(min = 3, max = 16))]
  #[validate(regex(
    path = *USERNAME_RE,
    message = "username may contain only letters, digits, and dots, and cannot start with a dot"
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
// Username cannot start with a dot; only letters, digits, and dots; length handled by length validator
static USERNAME_RE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^(?!\.)[A-Za-z0-9.]{3,16}$").expect("valid USERNAME regex"));

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

// Optional helper if you decide to separate slug from username later:
//
// pub fn slug_from_username(username: &str) -> String {
//   // Example slug policy: lowercase, replace '.' with '-', keep only [a-z0-9-], collapse repeats
//   let mut s = username.to_ascii_lowercase().replace('.', "-");
//   s.retain(|c| c.is_ascii_alphanumeric() || c == '-');
//   // Collapse multiple '-' to single
//   let mut collapsed = String::with_capacity(s.len());
//   let mut prev_dash = false;
//   for ch in s.chars() {
//     if ch == '-' {
//       if !prev_dash {
//         collapsed.push('-');
//         prev_dash = true;
//       }
//     } else {
//       collapsed.push(ch);
//       prev_dash = false;
//     }
//   }
//   collapsed.trim_matches('-').to_string()
// }

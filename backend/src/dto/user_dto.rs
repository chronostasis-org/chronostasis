use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserGetDto {
  pub id: String,
  pub slug: String,
  pub email: String,
  pub username: String,
  // Add other fields as needed
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserCreateDto {
  /// Unique handle
  /// Note: The service can generate `slug` from this.
  // #[serde(alias = "name")] // Optional: accept legacy "name" field if present
  pub username: String,

  /// Unique email address.
  pub email: String,

  /// Raw password; will be hashed server-side.
  pub password: String,
}

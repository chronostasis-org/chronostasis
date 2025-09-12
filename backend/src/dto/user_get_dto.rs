use serde::Serialize;

#[derive(Serialize)]
pub struct UserGetDto {
  pub id: String,
  pub slug: String,
  pub email: String,
  pub username: String,
  // Add other fields as needed
}

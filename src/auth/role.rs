use crate::error::AppError;
use super::extractor::AuthUser;

// pub fn require_admin(user: &AuthUser) -> Result<(), AppError> {
//     if user.role != "admin" {
//         Err(AppError::Forbidden("Admin access only".into()))
//     } else {
//         Ok(())
//     }
// }
pub fn require_admin(role: &str) -> Result<(), AppError> {
    if role != "admin" {
        return Err(AppError::Forbidden("Admin access only".into()));
    }
    Ok(())
}
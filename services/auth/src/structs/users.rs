use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use regex::Regex;
use models::user::User;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.username.len() < 3 || self.username.len() > 30 {
            return Err("Username must be between 3 and 30 characters.".into());
        }

        let email_re = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w{2,}$").unwrap();
        if !email_re.is_match(&self.email) {
            return Err("Invalid email format.".into());
        }

        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters long.".into());
        }
        let has_uppercase = self.password.chars().any(|c| c.is_uppercase());
        let has_lowercase = self.password.chars().any(|c| c.is_lowercase());
        let has_digit = self.password.chars().any(|c| c.is_digit(10));
        let has_special = self.password.chars().any(|c| !c.is_alphanumeric());

        if !(has_uppercase && has_lowercase && has_digit && has_special) {
            return Err("Password must contain uppercase, lowercase, digit, and special character.".into());
        }

        Ok(())
    }

    pub fn model(&self) -> User {
        User {
            id: 0,
            username: self.username.clone(),
            email: self.email.clone(),
            password_hash: Self::hash_password(&self.password),
            status: "active".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).expect("Failed to hash password")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_data() {
        let request = CreateUserRequest {
            username: "validuser".to_string(),
            email: "user@example.com".to_string(),
            password: "Valid@123".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_username() {
        let request = CreateUserRequest {
            username: "ab".to_string(),
            email: "user@example.com".to_string(),
            password: "Valid@123".to_string(),
        };
        assert_eq!(
            request.validate().unwrap_err(),
            "Username must be between 3 and 30 characters."
        );
    }

    #[test]
    fn test_validate_invalid_email() {
        let request = CreateUserRequest {
            username: "validuser".to_string(),
            email: "userexample.com".to_string(),
            password: "Valid@123".to_string(),
        };
        assert_eq!(request.validate().unwrap_err(), "Invalid email format.");
    }

    #[test]
    fn test_validate_short_password() {
        let request = CreateUserRequest {
            username: "validuser".to_string(),
            email: "user@example.com".to_string(),
            password: "Short1!".to_string(),
        };
        assert_eq!(
            request.validate().unwrap_err(),
            "Password must be at least 8 characters long."
        );
    }

    #[test]
    fn test_validate_password_missing_uppercase() {
        let request = CreateUserRequest {
            username: "validuser".to_string(),
            email: "user@example.com".to_string(),
            password: "lower@123".to_string(),
        };
        assert_eq!(
            request.validate().unwrap_err(),
            "Password must contain uppercase, lowercase, digit, and special character."
        );
    }

    #[test]
    fn test_validate_password_missing_lowercase() {
        let request = CreateUserRequest {
            username: "validuser".to_string(),
            email: "user@example.com".to_string(),
            password: "UPPER@123".to_string(),
        };
        assert_eq!(
            request.validate().unwrap_err(),
            "Password must contain uppercase, lowercase, digit, and special character."
        );
    }

    #[test]
    fn test_validate_password_missing_digit() {
        let request = CreateUserRequest {
            username: "validuser".to_string(),
            email: "user@example.com".to_string(),
            password: "NoDigits@".to_string(),
        };
        assert_eq!(
            request.validate().unwrap_err(),
            "Password must contain uppercase, lowercase, digit, and special character."
        );
    }

    #[test]
    fn test_validate_password_missing_special_character() {
        let request = CreateUserRequest {
            username: "validuser".to_string(),
            email: "user@example.com".to_string(),
            password: "NoSpecial123".to_string(),
        };
        assert_eq!(
            request.validate().unwrap_err(),
            "Password must contain uppercase, lowercase, digit, and special character."
        );
    }
}

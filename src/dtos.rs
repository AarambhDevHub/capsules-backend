use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;


#[derive(Debug, Clone)]
pub struct Capsule {
    pub id: Uuid,
    pub public_id: String,
    pub name: String,
    pub email: String,
    pub title: String,
    pub message: String,
    pub unlock_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub is_unlocked: Option<bool>,
    pub email_sent: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCapsuleRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    
    #[validate(email(message = "Invalid Email format"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    
    #[validate(length(min = 1, message = "Message is required"))]
    pub message: String,

    pub unlock_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateCapsuleResponse {
    pub public_id: String,
    pub unlock_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CapsuleDto {
    pub id: Uuid,
    pub public_id: String,
    pub name: String,
    pub title: String,
    pub message: String,
    pub unlock_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub is_unlocked: bool,
}

impl From<Capsule> for CapsuleDto {
    fn from(c: Capsule) -> Self {
        CapsuleDto { 
            id: c.id, 
            public_id: c.public_id, 
            name: c.name, 
            title: c.title, 
            message: c.message, 
            unlock_at: c.unlock_at.unwrap(), 
            created_at: c.created_at.unwrap(), 
            is_unlocked: c.is_unlocked.unwrap() 
        }
    }
}
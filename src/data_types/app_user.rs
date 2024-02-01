use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AppUserListElement {
    pub id: Uuid,
    pub username: String,
}

impl Default for AppUserListElement {
    fn default() -> Self {
        AppUserListElement {
            id: Uuid::nil(),
            username: String::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppUserShort {
    pub id: Uuid,
    pub username: String,
    pub role: Option<i16>,
    pub active: Option<bool>,
    pub deleted: Option<bool>,
}
impl Default for AppUserShort {
    fn default() -> Self {
        AppUserShort {
            id: Uuid::nil(),
            username: String::new(),
            role: None,
            active: None,
            deleted: None,
        }
    }
}

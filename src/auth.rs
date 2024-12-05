use crate::models::{User, UserRole};
use crate::errors::StoreError;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};

#[derive(Serialize, Deserialize)]
pub struct Auth {
    users: HashMap<String, User>,
    current_user: Option<User>,
}

impl Auth {
    pub fn new() -> Self {
        let auth = Auth {
            users: HashMap::new(),
            current_user: None,
        };

        // Try to load existing users
        match File::open("users.json") {
            Ok(file) => {
                match serde_json::from_reader(file) {
                    Ok(users) => {
                        println!("Loaded existing users");
                        return Auth {
                            users,
                            current_user: None,
                        };
                    },
                    Err(e) => {
                        println!("Error reading users.json, creating new file: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("No users.json found, creating new file: {}", e);
            }
        }

        // Create default admin user for new installation
        let mut new_auth = auth;
        if let Err(e) = new_auth.register(
            "admin".to_string(),
            "admin123".to_string(),
            UserRole::Manager,
        ) {
            println!("Warning: Could not create default admin user: {}", e);
        } else {
            println!("Created default admin user");
        }

        new_auth
    }

    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }

    pub fn user_exists(&self, username: &str) -> bool {
        self.users.contains_key(username)
    }

    pub fn register(&mut self, username: String, password: String, role: UserRole) -> Result<(), StoreError> {
        if self.user_exists(&username) {
            return Err(StoreError::InvalidInput(format!("Username '{}' already exists", username)));
        }

        let password_hash = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|_| StoreError::AuthError)?;

        let user = User {
            id: Uuid::new_v4(),
            username: username.clone(),
            password_hash,
            role,
        };

        self.users.insert(username, user);

        // Save users after registration
        self.save_users()?;
        Ok(())
    }

    fn save_users(&self) -> Result<(), StoreError> {
        println!("Saving users to file...");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("users.json")
            .map_err(|e| {
                println!("Error creating/opening users.json: {}", e);
                StoreError::DatabaseError(e.to_string())
            })?;

        serde_json::to_writer_pretty(file, &self.users)
            .map_err(|e| {
                println!("Error writing users to file: {}", e);
                StoreError::DatabaseError(e.to_string())
            })?;

        println!("Users saved successfully");
        Ok(())
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<(), StoreError> {
        println!("Attempting login for user: {}", username);
        println!("Available users: {:?}", self.users.keys().collect::<Vec<_>>());

        let user = self.users.get(username).ok_or_else(|| {
            println!("User not found: {}", username);
            StoreError::AuthError
        })?;

        if !verify(password.as_bytes(), &user.password_hash)
            .map_err(|e| {
                println!("Password verification failed: {}", e);
                StoreError::AuthError
            })? {
            println!("Invalid password");
            return Err(StoreError::AuthError);
        }

        self.current_user = Some(user.clone());
        Ok(())
    }

    pub fn is_manager(&self) -> bool {
        self.current_user
            .as_ref()
            .map(|user| matches!(user.role, UserRole::Manager))
            .unwrap_or(false)
    }

    pub fn get_current_user(&self) -> Option<&User> {
        self.current_user.as_ref()
    }

    pub fn logout(&mut self) {
        self.current_user = None;
    }
}

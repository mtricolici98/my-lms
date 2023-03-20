use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub pass_text: String,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationUser {
    pub username: String,
    pub email: String,
    pub pass_text: String, 
}



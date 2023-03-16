use std::{collections::BTreeMap, sync::Arc};

use crate::{prelude::{W, Error}, utils::{macros::map, hashing::hash_passwd}, error};

use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use surrealdb::{
    sql::{thing, Array, Object, Value},
    Datastore, Response, Session,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl From<User> for Value {
    fn from(val: User) -> Self {
        match val.id {
            Some(v) => map![
                    "id".into() => v.into(),
                    "username".into() => val.username.into(),
                    "email".into() => val.email.into(),
                    "password_hash".into() => val.password_hash.into(),
                    "created_at".into() => val.created_at.map(Into::into).unwrap_or_default(),
            ]
            .into(),
            None => map![
                "username".into() => val.username.into(),
                "email".into() => val.email.into(),
            ]
            .into(),
        }
    }
}

impl TryFrom<Value> for User {
    type Error  = error::Error;
    
    fn try_from(val: Value) -> Result<Self, error::Error>  {
        info!("Converting from {:?} to user", val);
        match val {
            Value::Object(el) => {
                let id = match el.get("id") {
                    Some(Value::Thing(el)) => Some(el.to_string()),
                    _ => None,
                };
                let username = match el.get("username") {
                    Some(Value::Strand(el)) => el.to_string(),
                    _ => String::new(),
                };
                let email = match el.get("email") {
                    Some(Value::Strand(el)) => el.to_string(),
                    _ => String::new(),
                };
                let password_hash = match el.get("password_hash") {
                    Some(Value::Strand(el)) => Some(el.to_string()),
                    _ => None,
                };
                Ok(
                    Self {
                    id,
                    username,
                    email,
                    password_hash,
                    created_at: None,
                }
            )
            }
            _ => Err(error::Error::XValueNotOfType("Could not convert UserValue to User"))
        }
    }

}

impl Creatable for User {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}

pub trait Creatable: Into<Value> {}

#[derive(Clone)]
pub struct UserDB {
    pub ds: Arc<Datastore>,
    pub sesh: Session,
}

impl UserDB {
    pub async fn execute(
        &self,
        query: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Vec<Response>, crate::error::Error> {
        let res = self.ds.execute(query, &self.sesh, vars, false).await?;
        Ok(res)
    }

    pub async fn add_user(&self, username: String, email: String, password: String) -> Result<User, crate::error::Error> {
        let passwrd_hash = hash_passwd(password);
        debug!("generated hash for user {}, {}", username, passwrd_hash);
        let sql = "CREATE users SET username = $username, email = $email, password_hash = $password_hash, created_at = time::now()";
        let vars: BTreeMap<String, Value> = map![
            "username".into() => Value::Strand(username.into()),
            "email".into() => Value::Strand(email.into()),
            "password_hash".into() => Value::Strand(passwrd_hash.into()),
            ];
        let res = self.execute(sql, Some(vars)).await?;
        debug!("{:?}", res);
        let first_res = res.into_iter().next().expect("Did not get a response");
        User::try_from(first_res.result?.first())
    }


    pub async fn get_user_by_id(&self, id: String) -> Result<User, crate::error::Error> {
        let sql = "SELECT * FROM $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let ress = self.execute(sql, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        User::try_from(first_res.result?.first())
    }


    pub async fn get_user_by_email(&self, email: String) -> Result<User, crate::error::Error> {
        let sql = "SELECT * FROM users WHERE email = $email";
        let vars: BTreeMap<String, Value> = map!["email".into() => Value::Strand(email.into())];
        let ress = self.execute(sql, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        User::try_from(first_res.result?.first())
    }


    pub async fn delete_user(&self, id: String) -> Result<AffectedRows, crate::error::Error> {
        let sql = "Delete $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let _ = self.execute(sql, Some(vars)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }
}

use std::{collections::BTreeMap, sync::Arc};

use crate::{prelude::W, utils::macros::map};

use chrono::{DateTime, Utc};
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
    pub created_at: Option<DateTime<Utc>>,
}

impl From<User> for Value {
    fn from(val: User) -> Self {
        match val.id {
            Some(v) => map![
                    "id".into() => v.into(),
                    "username".into() => val.username.into(),
                    "email".into() => val.email.into(),
            ]
            .into(),
            None => map![
                "username".into() => val.username.into(),
                "email".into() => val.email.into()
            ]
            .into(),
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

    pub async fn add_user(&self, username: String, email: String) -> Result<Object, crate::error::Error> {
        let sql = "CREATE users SET username = $username, email = $email, created_at = time::now()";
        let vars: BTreeMap<String, Value> = map![
            "username".into() => Value::Strand(username.into()),
            "email".into() => Value::Strand(email.into()),
            ];
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn get_user(&self, id: String) -> Result<Object, crate::error::Error> {
        let sql = "SELECT * FROM $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let ress = self.execute(sql, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Object>, crate::error::Error> {
        let sql = "SELECT * FROM users ORDER BY created_at ASC;";

        let res = self.execute(sql, None).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn delete_user(&self, id: String) -> Result<AffectedRows, crate::error::Error> {
        let sql = "Delete $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let _ = self.execute(sql, Some(vars)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }
}

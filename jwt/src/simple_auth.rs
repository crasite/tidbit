use std::{collections::HashMap, future::Future};

use crate::core::Authenticator;

pub struct BasicAuthenticator;
use anyhow::Result;

impl Authenticator for BasicAuthenticator {
    type Credential = (String, String);
    type Output = HashMap<&'static str, String>;

    fn authenticate(
        &self,
        credential: Self::Credential,
    ) -> impl Future<Output = Result<Self::Output>> {
        async move {
            let (username, password) = credential;
            if username == "admin" && password == "admin" {
                let mut map = HashMap::new();
                map.insert("role", "admin".to_string());
                map.insert("username", username);
                Ok(map)
            } else {
                Err(anyhow::anyhow!("Invalid username or password"))
            }
        }
    }
}

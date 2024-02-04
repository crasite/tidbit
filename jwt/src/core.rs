use std::{borrow::Borrow, future::Future, time::SystemTime};

use anyhow::Result;
use josekit::{
    jws::{JwsHeader, HS256},
    jwt::{self, JwtPayload},
};

pub trait Authenticator {
    type Credential;
    type Output;
    fn authenticate(
        &self,
        credential: Self::Credential,
    ) -> impl Future<Output = Result<Self::Output>>;
}

pub trait JwtManager {
    // fn create_jwt<K, V, M>(map: M)
    // where
    //     K: AsRef<str>,
    //     V: AsRef<str>,
    //     M: IntoIterator<Item = (K, V)>;

    fn create_jwt_with_key_and_time<K, V, M>(
        map: M,
        system_time: impl Borrow<SystemTime>,
        key: impl AsRef<[u8]>,
    ) -> Result<String>
    where
        K: AsRef<str>,
        V: AsRef<str>,
        M: IntoIterator<Item = (K, V)>,
    {
        let mut header = JwsHeader::new();
        header.set_token_type("JWT");
        let mut payload = JwtPayload::new();
        payload.set_issued_at(system_time.borrow());
        for (k, v) in map.into_iter() {
            payload.set_claim(k.as_ref(), Some(v.as_ref().into()))?;
        }

        // Signing JWT
        let signer = HS256.signer_from_bytes(key)?;
        Ok(jwt::encode_with_signer(&payload, &header, &signer)?)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_payload_creation() {
        struct TestJwtManager;
        impl JwtManager for TestJwtManager {}
        let mut m = HashMap::new();
        let key = b"0123456789ABCDEF0123456789ABCDEF";
        m.insert("hi", "me");
        let jwt = <TestJwtManager as JwtManager>::create_jwt_with_key_and_time(
            m,
            &SystemTime::now(),
            key,
        )
        .unwrap();
        assert_eq!(
            jwt,
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzb21la2V5Ijoic"
        )
    }
}

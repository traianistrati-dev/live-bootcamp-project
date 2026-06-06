use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version,
};

use crate::domain::data_stores::UserStoreError;
use color_eyre::eyre::{eyre, Result};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn parse_password_hash(hash: String) -> Result<HashedPassword, String> {
        let expected_password_hash =
            argon2::password_hash::PasswordHash::new(&hash).map_err(|op| op.to_string())?;
        Ok(HashedPassword(expected_password_hash.to_string()))
    }

    #[tracing::instrument(name = "Verify raw password", skip_all)]
    pub async fn verify_raw_password(&self, password_candidate: &str) -> Result<()>
// Result<(), Box<dyn Error + Send + Sync>>
    {
        let current_span: tracing::Span = tracing::Span::current();

        let password_hash = self.as_ref().to_owned();
        let password_candidate = password_candidate.to_owned();

        let result = tokio::task::spawn_blocking(move ||
               // -> Result<(), Box<dyn Error + Send + Sync>>
                {
                current_span.in_scope(|| {
                    let expected_password_hash =
                        argon2::password_hash::PasswordHash::new(&password_hash)?;

                    Argon2::default()
                        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                        .map_err(|e| e.into())
                })
            })
        .await;

        result?
    }

    #[tracing::instrument(name = "HashedPassword Parse", skip_all)]
    pub async fn parse(pass: String) -> Result<Self> {
        if is_valid_password(&pass) {
            // match compute_password_hash(&pass).await {
            //     Ok(hashed) => return Ok(HashedPassword(hashed)),
            //     Err(e) => return Err(UserStoreError::UnexpectedError(e.into())),
            // }

            let result = compute_password_hash(&pass)
                .await
                .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

            Ok(Self(result))
        } else {
            Err(eyre!("Failed to parse string to a HashedPassword type"))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: &str) -> Result<String>
//Result<String, Box<dyn Error + Send + Sync>>
{
    let current_span: tracing::Span = tracing::Span::current();

    let password = password.to_owned();

    let result = tokio::task::spawn_blocking(move ||
        //-> color_eyre::eyre::Result<String>
            //-> Result<String, Box<dyn Error + Send + Sync>>
            {
            current_span.in_scope(|| {
                let salt = SaltString::generate(&mut OsRng);
                let password_hash = Argon2::new(
                    Algorithm::Argon2id,
                    Version::V0x13,
                    Params::new(15000, 2, 1, None)?,
                )
                .hash_password(password.as_bytes(), &salt)?
                .to_string();

                Ok(password_hash)
                //Err(Box::new(std::io::Error::other("oh no!")) as Box<dyn Error + Send + Sync>)
                //Err(eyre!("oh no!"))
            })
        })
    .await;

    result?
}

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn is_valid_password(pass: &str) -> bool {
    pass.len() > 7
}

#[cfg(test)]
mod tests {
    use super::HashedPassword; // updated!
    use argon2::{
        // new
        password_hash::{rand_core::OsRng, SaltString},
        Algorithm,
        Argon2,
        Params,
        PasswordHasher,
        Version,
    };
    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;
    use quickcheck::Gen;
    use rand::SeedableRng;

    // updated!
    #[tokio::test]
    async fn empty_string_is_rejected() {
        let password = "".to_owned();

        // updated!
        assert!(HashedPassword::parse(password).await.is_err());
    }

    // updated!
    #[tokio::test]
    async fn string_less_than_8_characters_is_rejected() {
        let password = "1234567".to_owned();
        // updated!
        assert!(HashedPassword::parse(password).await.is_err());
    }

    // new
    #[test]
    fn can_parse_valid_argon2_hash() {
        // Arrange - Create a valid Argon2 hash
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );

        let hash_string = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Act
        let hash_password = HashedPassword::parse_password_hash(hash_string.clone()).unwrap();

        // Assert
        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));
    }

    // new
    #[tokio::test]
    async fn can_verify_raw_password() {
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );

        let hash_string = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let hash_password = HashedPassword::parse_password_hash(hash_string.clone()).unwrap();

        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));

        // TODO: Use verify_raw_password to verify the password match
        let result = HashedPassword::parse(raw_password.to_owned())
            .await
            .unwrap()
            .verify_raw_password(raw_password)
            .await;

        assert!(result.is_ok());
        // TODO: Assert the verification succeeds assert_eq!(result, ())
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = g.size() as u64;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let password = FakePassword(8..30).fake_with_rng(&mut rng);
            Self(password)
        }
    }

    // updated!
    #[tokio::test]
    #[quickcheck_macros::quickcheck]
    async fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        HashedPassword::parse(valid_password.0).await.is_ok() // updated!
    }
}

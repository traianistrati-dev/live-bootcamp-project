use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version,
};

use crate::domain::data_stores::UserStoreError;
// use color_eyre::eyre::{eyre, Result};
use color_eyre::eyre::{eyre, Result};

use secrecy::{ExposeSecret, SecretString};

#[derive(Clone, Debug)]
pub struct HashedPassword(SecretString);

impl PartialEq for HashedPassword {
    // New!
    fn eq(&self, other: &Self) -> bool {
        // We can use the expose_secret method to expose the SecretString
        // in a controlled manner when needed!
        self.0.expose_secret() == other.0.expose_secret() // Updated!
    }
}

impl HashedPassword {
    #[tracing::instrument(name = "HashedPassword Parse", skip_all)]
    pub async fn parse(pass: SecretString) -> Result<Self> {
        if validate_password(&pass) {
            let result = compute_password_hash(&pass)
                .await
                .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

            Ok(Self(result))
        } else {
            Err(eyre!("Failed to parse string to a HashedPassword type"))
        }
    }

    pub fn parse_password_hash(hash: SecretString) -> Result<HashedPassword> {
        if let Ok(hashed_string) = argon2::PasswordHash::new(hash.expose_secret().as_ref()) {
            Ok(Self(SecretString::new(
                hashed_string.to_string().into_boxed_str(),
            )))
        } else {
            Err(eyre!("Failed to parse string to a HashedPassword type"))
        }
    }

    #[tracing::instrument(name = "Verify raw password", skip_all)]
    pub async fn verify_raw_password(&self, password_candidate: &SecretString) -> Result<()>
// Result<(), Box<dyn Error + Send + Sync>>
    {
        let current_span: tracing::Span = tracing::Span::current();

        let password_hash = self.as_ref().expose_secret().to_owned();
        let password_candidate = password_candidate.expose_secret().to_owned();

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

    pub fn as_str(&self) -> &SecretString {
        &self.0
    }
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: &SecretString) -> Result<SecretString>
//Result<String, Box<dyn Error + Send + Sync>>
{
    let current_span: tracing::Span = tracing::Span::current();

    let password = password.expose_secret().to_owned();

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

                Ok(SecretString::new(password_hash.into_boxed_str()))
                //Err(Box::new(std::io::Error::other("oh no!")) as Box<dyn Error + Send + Sync>)
                //Err(eyre!("oh no!"))
            })
        })
    .await;

    result?
}

impl AsRef<SecretString> for HashedPassword {
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

fn validate_password(pass: &SecretString) -> bool {
    pass.expose_secret().len() > 7
}

#[cfg(test)]
mod tests {
    use super::HashedPassword;

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
    use secrecy::{ExposeSecret, SecretString};

    #[tokio::test]
    async fn empty_string_is_rejected() {
        let password = SecretString::new("".to_owned().into_boxed_str());
        assert!(HashedPassword::parse(password).await.is_err());
    }

    #[tokio::test]
    async fn string_less_than_8_characters_is_rejected() {
        let password = SecretString::new("1234567".to_owned().into_boxed_str());
        assert!(HashedPassword::parse(password).await.is_err());
    }

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

        let password_secret_str = SecretString::new(hash_string.into_boxed_str());
        // Act
        let hash_password =
            HashedPassword::parse_password_hash(password_secret_str.clone()).unwrap();

        // Assert
        assert_eq!(
            hash_password.as_ref().expose_secret(),
            password_secret_str.expose_secret()
        );
        assert!(hash_password
            .as_ref()
            .expose_secret()
            .starts_with("$argon2id$v=19$"));
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

        let password_secret_str = SecretString::new(hash_string.into_boxed_str());

        let hash_password =
            HashedPassword::parse_password_hash(password_secret_str.clone()).unwrap();

        assert_eq!(
            hash_password.as_ref().expose_secret(),
            password_secret_str.expose_secret()
        );
        assert!(hash_password
            .as_ref()
            .expose_secret()
            .starts_with("$argon2id$v=19$"));

        let raw_password = SecretString::new(raw_password.to_owned().into_boxed_str());

        let result = HashedPassword::parse(raw_password.to_owned())
            .await
            .unwrap()
            .verify_raw_password(&raw_password)
            .await;

        assert!(result.is_ok());
        // TODO: Assert the verification succeeds assert_eq!(result, ())
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub SecretString);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = g.size() as u64;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let password: String = FakePassword(8..30).fake_with_rng(&mut rng);
            Self(SecretString::new(password.into_boxed_str())) // Updated!
        }
    }

    #[tokio::test]
    #[quickcheck_macros::quickcheck]
    async fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        HashedPassword::parse(valid_password.0).await.is_ok()
    }
}

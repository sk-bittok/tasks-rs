use validator::Validate;

use crate::Error;

pub struct Validator<T>(pub T)
where
    T: Validate;

impl<T> Validator<T>
where
    T: Validate,
{
    pub fn new(t: T) -> Self {
        Self(t)
    }

    pub fn validate(&self) -> Result<&T, Error> {
        let result = self.0.validate();

        match result {
            Ok(()) => Ok(&self.0),
            Err(e) => {
                let mut errors = std::collections::BTreeMap::new();

                for (key, value) in e.field_errors() {
                    errors.insert(
                        key.to_string(),
                        value
                            .iter()
                            .map(|val_err| val_err.message.as_deref().unwrap_or("Field error"))
                            .collect::<Vec<&str>>()
                            .join(", "),
                    );
                }

                let val_error = serde_json::json!(errors).to_string();

                Err(Error::Validation(val_error))
            }
        }
    }
}

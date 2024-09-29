use flaggy::{Flaggy, FlaggyError};
use once_cell::sync::Lazy;

static FLAGGY: Lazy<Flaggy> = Lazy::new(|| {
    Flaggy::new("your-flaggy-api-key")
        .expect("Failed to initialize Flaggy")
});

pub fn is_feature_enabled(feature_name: &str, user_id: &str) -> Result<bool, FlaggyError> {
    FLAGGY.is_enabled(feature_name, user_id)
}
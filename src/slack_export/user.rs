use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString};
use url::Url;

#[derive(Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

#[derive(Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub deleted: bool,
    pub real_name: String,
    pub profile: UserProfile,
    pub is_bot: bool,
    pub is_admin: bool,
    pub is_owner: bool,
}

#[serde_as]
#[derive(Deserialize)]
pub struct UserProfile {
    #[serde_as(as = "NoneAsEmptyString")]
    pub display_name: Option<String>,
    pub image_512: Url,
}

impl User {
    pub fn display_name(&self) -> &str {
        self.profile
            .display_name
            .as_deref()
            .unwrap_or(&self.real_name)
    }
}

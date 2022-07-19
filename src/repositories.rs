use core::option::Option;
use core::option::Option::{None, Some};
use crate::domain_models::User;

pub enum UserRepository {
    StubSome,
    StubNone,
}

impl UserRepository {
    pub(crate) fn get_by_id(&self, _id: &str) -> Option<User> {
        match self {
            UserRepository::StubSome => {
                Some(User::new("Taro"))
            }
            UserRepository::StubNone => {
                None
            }
        }
    }
}

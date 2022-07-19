pub mod manage {
    pub mod user {
        use crate::{User, repositories::UserRepository};

        #[cfg(test)]
        mod tests {
            use crate::manage::user::UserManager;
            use crate::UserRepository;

            #[test]
            fn test_user_manager_get() {
                let manager = UserManager { repository: UserRepository::StubSome };
                assert_eq!(manager.get().unwrap().nickname(), "Taro");
            }
        }

        pub struct UserManager {
            pub(crate) repository: UserRepository,
        }

        impl<'a> UserManager {
            pub fn get(&self) -> Option<User> {
                self.repository.get_by_id("dummy")
            }
        }
    }
}

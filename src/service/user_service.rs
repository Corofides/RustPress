use crate::repository::user_repository::UserRepository;
use crate::User;
use async_std::task;

pub enum ServiceError {
    CreateUserError
}

pub struct UserService<T: UserRepository> {
    repository: T
}

impl<T: UserRepository> UserService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository
        }
    }
    pub fn get_users(&self) -> Vec<User> {
        task::block_on(async {
            self.repository
                .fetch_all()
                .await
        })
    }
    pub fn create_user(&self, user: &User) -> Result<(), ServiceError> {
        let result = task::block_on(async {
            self.repository
                .create_user(user)
                .await
        });

        match result {
            Ok(_) => return Ok(()),
            Err(_) => return Err(ServiceError::CreateUserError)
        }
    }
}

use crate::repository::{
    Repository,
    user_repository::UserFilters,
};
use crate::User;
use async_std::task;

pub enum ServiceError {
    CreateUserError
}

pub struct UserService<T: Repository<User, UserFilters>> {
    repository: T
}

impl<T: Repository<User, UserFilters>> UserService<T> {
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
    pub fn create_user(&self, user: User) -> Result<i64, ServiceError> {
        let result = task::block_on(async {
            self.repository
                .add(user)
                .await
        });

        match result {
            Ok(value) => return Ok(value),
            Err(_) => return Err(ServiceError::CreateUserError)
        }
    }
}

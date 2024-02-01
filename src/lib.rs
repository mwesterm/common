use serde::*;
use strum::{Display, EnumString, IntoStaticStr};

pub use uuid as common_uuid;

pub mod data_types;
pub use data_types::*;

//Definition of roles
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString, Display, Serialize, Deserialize)]
pub enum Role {
    ADMIN = 1,
    OFFICE = 2,
    USER = 3,
    SUPER = 4,
    GUEST = 5,
}

#[derive(EnumString, IntoStaticStr, Display)]
pub enum ApiUrls {
    #[strum(serialize = "/login")]
    Login,
    #[strum(serialize = "/logout")]
    Logout,
    #[strum(serialize = "/appUser/all_app_user")]
    AllAppUsers,
    #[strum(serialize = "/appUser/create")]
    CreateAppUser,
    #[strum(serialize = "/appUser/deleteAppUser/:id")]
    DeleteAppUser,
    #[strum(serialize = "/appUser/find_by_name/:name")]
    FindAppUserByName,
    #[strum(serialize = "/appUser/find_by_id/:id")]
    FindAppUserById,
    #[strum(serialize = "/appUser/update")]
    UpdateAppUser,
    #[strum(serialize = "/student/all_students")]
    AllStudents,
    #[strum(serialize = "/student/create")]
    CreateStudent,
    #[strum(serialize = "/student/find_by_id/:id")]
    FindStudentByID,
    #[strum(serialize = "/student/find_by_name/:name")]
    FindStudentByName,
    #[strum(serialize = "/student/update/:id")]
    UpdateStudent,
    #[strum(serialize = "/student/delete/:id")]
    DeleteStudent,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        assert_eq!(Role::ADMIN.to_string(), "Admin");
        assert_eq!(Role::ADMIN, Role::from_str("Admin").unwrap());
    }
}

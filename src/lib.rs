use strum::*;
//Definition of roles
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString, Display)]
pub enum Role {
    Admin,
    User,
    Super,
    Guest,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        assert_eq!(Role::Admin.to_string(), "Admin");
        assert_eq!(Role::Admin, Role::from_str("Admin").unwrap());
    }
}

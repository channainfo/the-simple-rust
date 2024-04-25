use crate::data_type::gender::Gender;

#[derive(Debug)]
pub struct User<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub dob: u32,
    pub gender: Gender,
}

impl<'a> User<'a> {
    #[allow(dead_code)]
    pub fn new(first_name: &'a str, last_name: &'a str, dob: u32, gender: Gender) -> User<'a> {
        User {
            first_name: first_name,
            last_name: last_name,
            dob: dob,
            gender: gender,
        }
    }

    #[allow(dead_code)]
    pub fn full_name(self: &User<'a>) -> String {
        let result = format!("{} {}", self.first_name, self.last_name);
        result
    }
}

#[cfg(test)]
mod user_test {
    // use super::*;
    use crate::data_type::gender::Gender;
    use crate::model::user::User;

    #[test]
    fn test_new() {
        let first_name = "joo";
        let last_name = "ann";
        let dob = 1267897738u32;
        let gender: Gender = Gender::Male;
        let user = User::new(first_name, last_name, dob, gender);

        assert_eq!(user.first_name, first_name);
        assert_eq!(user.last_name, last_name);
        assert_eq!(user.dob, dob);
        assert_eq!(user.gender, Gender::Male);
    }

    #[test]
    fn test_full_name() {
        let first_name = "joo";
        let last_name = "ann";
        let dob = 1267897738u32;
        let gender: Gender = Gender::Male;
        let user = User::new(first_name, last_name, dob, gender);

        assert_eq!(user.full_name(), "joo ann");
    }
}

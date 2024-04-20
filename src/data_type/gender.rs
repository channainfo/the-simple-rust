use std::fmt::{Debug, Display};
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Debug for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
            Self::Other => write!(f, "Other"),
        }
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Gender::Female => write!(f, "Female"),
            Gender::Male => write!(f, "Male"),
            _ => write!(f, "N/A"),
        }
    }
}

#[cfg(test)]
mod gender_test {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let male = Gender::Male;
        assert_eq!(male, Gender::Male)
    }
}

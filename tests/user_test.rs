use rollapp;
use rollapp::data_type::gender::Gender;
use rollapp::model::user::User;

mod common;

#[test]
fn test_user_full_name() {
    common::setup();

    let first_name = "joo";
    let last_name = "ann";
    let dob = 1267897738u32;
    let gender: Gender = Gender::Male;
    let user = User::new(first_name, last_name, dob, gender);

    assert_eq!(user.full_name(), "joo ann");

    common::tear_down();
}

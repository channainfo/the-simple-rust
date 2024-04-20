mod data_type;
mod model;

use crate::data_type::gender::Gender;
use crate::model::user::User;

fn main() {
    let first_name = "joo";
    let last_name = "ann";
    let dob = 1267897738u32;
    let gender: Gender = Gender::Female;
    let user = User::new(first_name, last_name, dob, gender);

    println!("User: {:?}", &user);
    println!("Hello {}!", user.full_name());
}

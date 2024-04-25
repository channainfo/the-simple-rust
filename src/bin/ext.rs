use rollapp::{self, data_type::gender::Gender, model::user::User};

fn main() {
    let first_name = "joo";
    let last_name = "ann";
    let dob = 1267897738u32;
    let gender: Gender = Gender::Female;
    let user = User::new(first_name, last_name, dob, gender);

    println!("Application name: ext");
    println!("=================================");
    println!("User: {:?}", &user);
    println!("Hello {}!", user.full_name());
}

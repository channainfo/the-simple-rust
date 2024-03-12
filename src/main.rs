use num::complex::Complex;

fn greeting() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = Complex::new(1.5, 5.0_f32);
    let b = Complex {
        re: 2.5,
        im: 7.3_f32,
    };
    let c = a + b;

    greeting();
    println!("{} + {} -> {}", a, b, c);
    let i: i32 = 10;
    let j = 40_i32;

    println!("The sum of {} + {} is = {}", i, j, add(i, j));

    let year_count = 35_u8;
    let month_count = 650_u16;

    let year: f32 = (month_count / 12) as f32;

    if year > year_count.try_into().unwrap() {
        println!("{} months is older than {}", month_count, year_count);
    } else {
        println!("{} months is younger than {}", month_count, year_count);
    }
}

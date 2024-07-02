mod service;
use crate::service::sum::Sum;

fn main() -> () {
    std::println!("Hello, world!");
    let a: i64 = 25;
    let b: i64 = 75;
    let _sum = Sum::new(a, b);
    let addition = _sum.add();

    match addition {
        Ok(value) => println!("sum of {} and {} is {}", a, b, value),
        Err(err) => println!("error: {}", err)
    }

    let a: i64 = -25;
    let b: i64 = 75;
    let _sum = Sum::new(a, b);
    let addition = _sum.add();

    match addition {
        Ok(value) => println!("sum of {} and {} is {}", a, b, value),
        Err(err) => println!("error: {}", err)
    }

}
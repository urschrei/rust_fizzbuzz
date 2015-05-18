// http://composition.al/blog/2013/03/02/fizzbuzz-revisited

enum Rem {
    Zero,
    Other(NonZeroRem)
}

enum NonZeroRem {
    One, Two, Three, Four
}

fn int_to_rem(num: i32) -> Rem {
    match num {
        0 => Rem::Zero,
        1 => Rem::Other(NonZeroRem::One),
        2 => Rem::Other(NonZeroRem::Two),
        3 => Rem::Other(NonZeroRem::Three),
        4 => Rem::Other(NonZeroRem::Four),
        _ => panic!("oops")
    }
}

fn main() {
    let nums = (1..101).collect::<Vec<i32>>();
    for num in nums.iter() {
        let m = match (int_to_rem(num % 3), int_to_rem(num % 5)) {
            (Rem::Zero, Rem::Zero) => "FizzBuzz".to_string(),
            (Rem::Zero, Rem::Other(_)) => "Fizz".to_string(),
            (Rem::Other(_), Rem::Zero) => "Buzz".to_string(),
            (Rem::Other(_), Rem::Other(_)) => num.to_string()
        };
        println!("{}", m)
    }
}

// Based on http://composition.al/blog/2013/03/02/fizzbuzz-revisited
// All the way down the rabbit hole: http://chrismorgan.info/blog/rust-fizzbuzz.html

/// Used to match against a zero or non-zero remainder
enum Rem {
    // No remainder
    Zero,
    // A remainder between 1 and 4
    Other(NonZeroRem),
}

/// Used to match a non-zero remainder
/// We aren't doing anything with these values, but we could
enum NonZeroRem {
    One,
    Two,
    Three,
    Four,
}

/// Easily allows us to exhaustively decide the return value
/// Input is the remainder of a modulo operation from a match expression
fn int_to_rem(num: u8) -> Rem {
    match num {
        0 => Rem::Zero,
        1 => Rem::Other(NonZeroRem::One),
        2 => Rem::Other(NonZeroRem::Two),
        3 => Rem::Other(NonZeroRem::Three),
        4 => Rem::Other(NonZeroRem::Four),
        _ => panic!("oops"),
    }
}

fn main() {
    for num in 1..101 {
        println!(
            "{}",
            match (int_to_rem(num % 3), int_to_rem(num % 5)) {
                (Rem::Zero, Rem::Zero) => "FizzBuzz".to_string(),
                (Rem::Zero, Rem::Other(_)) => "Fizz".to_string(),
                (Rem::Other(_), Rem::Zero) => "Buzz".to_string(),
                (Rem::Other(_), Rem::Other(_)) => num.to_string(),
            }
        )
    }
}

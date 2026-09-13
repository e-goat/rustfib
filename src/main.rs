use std::io;

fn main() {
    let mut count = 0u32;
    loop {
        if count == 0 {
            println!("Enter number:");
        } else {
            println!("Enter number again or quit (Ctrl + C):");
        }
        count += 1;
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read the entered number\n");

        let num: u32 = match num
            .trim()
            .parse() {
                Ok(i) => i,
                Err(err) => {
                    println!("{}",err);
                    continue;
                },
            };

        if num > 99 {
            println!("Whoops! Sorry, I've limited the maximum input integer to be 99.");
            continue;
        }

        let result = fibonacci(num);

        println!("Nth fibonacci number is:\n {result}");
    }
}

fn fibonacci(nth: u32) -> u32 {
    const ZERO_ONE: [u32; 2] = [0, 1];

    if ZERO_ONE.contains(&nth) {
        return nth;
    }

    // calculate constants
    const GOLDEN_RATIO: f64 = 5.0;
    let gr_sqr_root: f64 = GOLDEN_RATIO.sqrt();
    // golden ratio
    let mut phi: f64 = (1.0 + gr_sqr_root)/2.0;
    // conjugate
    let mut psi: f64 = (1.0 - gr_sqr_root)/2.0;

    // raise both phi and psi constants to the power of nth
    phi = phi.powf(nth.into());
    psi = psi.powf(nth.into());

    let result = (phi - psi)/gr_sqr_root;

    return result.round() as u32;
}

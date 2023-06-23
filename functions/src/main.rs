fn main() {
    println!(
        "{}",
        print_phase("{I am the argument for print_pharse, a &str}é")
    );

    let answer: u64 = gcd(20, 5);
    println!("{answer}");
    let mut x = 0;
    'loopA: loop {
        let mut y = 0;
        println!("loopA: {x}");
        loop {
            println!("{:<5}loop2", "");
            if x > 1 {
                println!("{:<5}Breaking from 'loopA", "");
                break 'loopA;
            }
            if y == 1 {
                break;
            }
            y += 1;
        }
        x += 1;
    }
}

fn print_phase(phrase: &str) -> usize {
    println!("Printing from print_phase function {}", phrase.len());
    return phrase.chars().count();
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }

    b
}

fn main() {
    println!(
        "{}",
        print_phase("{I am the argument for print_pharse, a &str}é")

    );


}

fn print_phase(phrase: &str) -> usize {
    println!("Printing from print_phase function {}", phrase.len());
    return phrase.chars().count();
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    
    while a!= 0{
        if a<b{
            let c = a;
            a=b
            b=c
        }
        a = a%b;
    }
    
    b
}

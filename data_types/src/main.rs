use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}
impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    // let x: i8 = 10;
    // println!("{}", x);
    // let y: u8 = 10; //e
    // println!("{}", y);
    // let decimal = 02_55;
    // let hex = 0xff;
    // let octal = 0o377;
    // let binary = 0b1111_1111;
    // println!("{}", decimal);
    // println!("{}", hex);
    // println!("{}", octal);
    // println!("{}", binary);

    // let byte = b'A';
    // println!("{}", byte);

    let t = true;
    let f: bool = false;
    println!("t= {}\nf={}", t, f);

    let a = 10;
    let b: i8 = 4;

    let remainder = a % b;
    println!("\n{}\n", remainder);

    let tup = (1, "ok", true);
    let (x, y, z) = tup;
    println!("x={} y={} z={} tup.1={}", x, y, z, tup.1);
    let array = [1, 2, 3];
    println!("{}", array[0]);
    let mut array2: [i8; 3] = [1, 2, 3];
    array2[0] = 10;
    println!("{}\n{}\n{}", array2[0], array2[1], array2[2]);
    println!("{:?}", array2);
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("{:?}", nums);
    nums.pop();
    println!("{:?}", nums);
    let mut vec: Vec<&str> = Vec::new(); // vec!
    let mut vec1: Vec<i128> = Vec::new();
    vec.push("test");
    let mut d = vec![];
    d.push("value");
    println!("{:?}\n{:?}", vec, d);
    let x: i128 = 123456780000000009123456780000000009;
    vec1.push(123456780000000009);
    vec1.push(x);
    vec1.reverse();
    println!("{:?}", vec1);
    println!("{number:*<5}", number = 1);
    let x = City {
        name: "abc",
        lat: 53.31564,
        lon: -6.25487,
    };
    let y = City {
        name: "def",
        lat: 102.8964,
        lon: 12.48787,
    };
    let city = vec![x, y];
    for c in city {
        println!("{}", c)
    }

    let a = Color {
        red: 128,
        green: 255,
        blue: 90,
    };
    let b: Color = Color {
        red: (0),
        green: (3),
        blue: (254),
    };
    for color in [a, b] {
        println!("{:?}", color);
    }

    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(123);
    let vec2 = vec![1, 2, 3];
    println!("{:?}", vec2);
}

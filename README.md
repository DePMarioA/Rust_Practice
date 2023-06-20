# Rust_Pratice (using windows)

### To start and create a directory

#### In terminal type: cargo new directory_name

#### Example:

        cargo new hello_world

1.  ## hello_world
    - ### First attempt to run code and keep version control
      - Should be simple but ran into and error.
        - error: linker `link.exe` not found
        - note: program not found
      - ### Fix: I didn't fully finish installing VS studio Build tools.
        - In the VS installer finish installing build tools total should 8gb, 2022 ver. 17.6.3
    - ## To run code:
      - cargo run in the directory that contains toml in this case
        hello_world> `cargo run`
        - cargo run automatically does cargo build
2.  ## variables

    - let and const are immutable.
    - `let mut x` , mut allows let be mutable.
    - `let tup = ("okay",1, true);` Tuples allow multiple types
      - to access: `tup.0 tup.1 tup.2`
      - to unpack: `let (x,y,z) = tup`
    - `let array = [1,2,3]` rust-analyzer will adjust to it
      - formal way let array2:[i32;3] =[1,2,3]
        - set type and length of array seperate by `;` `[i32;3]` will give an array of len 3 of i32 type
    - vectors `let mut vec = vec![1,2,3,4];` is quick way
      - formal way instanciate `let mut vec:Vec<&str> = Vec::new()` and then push `vec.push("msg1")` `vec.push("msg2")`
      - look into vector methods, tons you can do. ex: `vec.reverse()`
    - _Looked into formatting for structs:_

      - quickest way is to use `#[derive(Debug)]` right before the struct then you can use println!({:?},variable)
      - long way:
      - import:
      - `use std::fmt::{self, Display, Formatter};`
      - create struct
      -         impl Display for City { fn fmt(&self, f: &mut Formatter<'\_>) -> fmt::Result {
                    let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' }; let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
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
        - `in impl Display for City {}` with rust-analyzer type `fn fmt` and it will automatically write the variables needed

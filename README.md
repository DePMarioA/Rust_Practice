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
2.  ## data_types

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
      - creating new vectors with known capcity, it will automatically extend (find a new region in memory that supports the space)
        - `let mut vect = Vec::<i32>::with_capacity(2);`
      - quick vector creation:
      -       let v: Vec<i32> = (0..5).collect();
        - `(0..5).collect();` creates vector 0-4
      - output:
        -       v  = [0, 1, 2, 3, 4]
    - slicing: 'points to a range of consective values ' `(non owning)` vs reference: points to a single value `(non owning)`

      - creating reference to vector and storing it to a variable
        -       let sliced_v: &[i32] = &v;
      - creating a sliced reference:
        -       let sliced_v: &[i32] = &v[2..4];
      - where `[2..4]` slices the vector v by using a fat pointer or starts the reference to the slice in this case v[2] and ending (inclusive) v[3] v length = 5
      - output:
        -       v = [0, 1, 2, 3, 4]
                sliced_v = [0, 1, 2, 3, 4]
                sliced_v = [2, 3]

    - Strings:
      - ways to create strings:
      -         let name = String::from("Tommy");
                let language = "Rust".to_string();
                let new_name = name.replace("Tommy", "timmy");
    - &str = "string slice" or 'stir'. this is a string slice or &str doesnt allocate memory on the heap
      -       let str1 = "hello";
    - String literals:
      -       let rust = "\x52\x75\x73\x74";
    - ### Differences between the two:

    1. #### String Slice (&str):
       - A string slice is a borrowed reference to a portion of a string or a string literal.
       - String slices have a fixed size known at compile time and are stored on the stack.
       - They are immutable and cannot be modified directly.
       - String slices do not own the underlying data and are therefore lightweight and cheap to copy.
       - They are useful for working with string data without needing ownership or in scenarios where you want to pass string data without transferring ownership.
    2. #### Dynamically Allocated String (String):

       - A dynamically allocated string (String) is mutable and can grow or shrink as needed.
       - String data owned by a String type is allocated on the heap, which means the memory is dynamically managed.
       - The size of a String can vary at runtime, and it has a flexible capacity to accommodate changes.
       - Strings are used when you need ownership of the string data and the ability to modify or manipulate it.
       - String data can be shared across different parts of the program by passing ownership or using references (&String or &str).

    - #### Choosing between a string slice and a dynamically allocated string depends on the specific requirements of your code. Some considerations include:

      - Ownership: If you need to own and modify the string data, use String. If you only need to borrow or reference existing string data, use a string slice (&str).
      - Flexibility: String slices have a fixed size, while String can dynamically grow or shrink as needed. Choose accordingly based on the requirements of your application.
      - Performance: String slices are lightweight and copied by value, while String involves memory allocations and deallocations. In performance-critical scenarios, string slices can be more efficient.

    - ### STACK AND HEAP SUMMARY:
      - the stack is used for managing function calls and local variables with automatic memory management, while the heap is used for dynamic memory allocation with manual or automated memory management. The stack provides fast and deterministic memory access, while the heap offers flexibility and larger memory capacity but with additional overhead and responsibility for memory management.
      - in depth:
        1. #### Stack:
           - The stack is a region of memory that is organized in a Last-In-First-Out (LIFO) manner.
           - It is typically used for managing function calls, local variables, and function parameters.
           - Memory allocation and deallocation on the stack are managed automatically by the compiler.
           - Stack memory is fast to allocate and deallocate, as it involves simple pointer manipulation.
           - The stack has a fixed size determined at compile-time, and the size of each stack frame is known in advance.
           - Stack memory is limited, and if the stack size exceeds its capacity, it results in a stack overflow error.
        2. #### Heap:
           - The heap is a region of memory used for dynamic memory allocation.
           - It is not organized in a specific order and is more flexible than the stack.
           - Memory allocation and deallocation on the heap are managed explicitly by the programmer or with the help of memory management systems like garbage collectors or smart pointers.
           - Heap memory is slower to allocate and deallocate compared to the stack due to more complex bookkeeping and fragmentation concerns.
           - The heap has a larger size compared to the stack and can grow dynamically as needed (within system limits).
           - Heap memory is not automatically reclaimed when no longer needed, and it can result in memory leaks or dangling pointers if not managed correctly.
    - ### Why use string literals over String or &str

      1. #### Binary Data or Non-Textual Data:

         - Sometimes, you may be dealing with binary data that is not intended to represent text. In such cases, the data may contain byte sequences that are not valid UTF-8. For example, when working with image files, compressed data, or cryptographic data, the content may include arbitrary byte values that do not conform to UTF-8 encoding rules.

      2. #### Parsing or Processing Raw Data:

         - When parsing or processing data from external sources, such as network protocols or file formats, you may encounter malformed or non-UTF-8 sequences. It could be useful to handle and analyze these sequences for error detection, data extraction, or custom processing.

      3. #### Legacy Systems or Interoperability:

         - In certain scenarios, you may need to interface with legacy systems or external libraries that do not strictly adhere to UTF-8 encoding rules. This could involve handling non-UTF-8 sequences for compatibility or interoperability purposes.

      4. #### Performance Optimization:
         - In some performance-sensitive applications, there might be situations where manipulating or processing byte-level data directly can provide performance advantages over UTF-8 decoding and encoding operations. However, such optimizations should be carefully considered and applied only when necessary.

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
      - Ways to add padding or spaces to print statments:
        -       println!("{number:*<5}", number = 1);
                println!("{number:*<5}", number = 11);
          - `{number:*<5}` where : seems like it opens up format fn and allows less than 5 `*` to be placed to right of varible number
        - output:
          -       1****
                  11***

    3.  ## Functions

        - Conventions:
          - snake casing `fn snake_case_when_typing_a_fxn(){}`
        - to create a function use `fn`
          - typical function functions:
            - can have arguments
            - can return values
              - returns special wat of writing them
        - example:
          -       fn length_of_str( mut param:&str) -> usize {
                  let str_length = param.chars().count()
                  println!("{}", str_length)
                  str_length
                  } //param.len() returns byte size of &str so you must use param.chars().count()
                  // return can be done by leaving str_length without semicolon at the end
        - Loops:

          - loops can be named by using `'`

            - ` 'name_of_loop loop {}`
            - nested loops can now be broken as shown in the project folder
              -       break 'name_of_loop

    4.  ## guessing_game
        - Topics:
          1.  using `use` to import lib or 'crates'
              - to access subset or function rust uses `::`, kind of like `.` in functions or `from` `import` in python.
                - std::io
                - std::cmp::Ordering
                - rand::Rng
          2.  Matching
              - Depending on outcome of choices. Normal pattern matching. check example below.
              - if value matches a certain value
          3.  Result
              - Some functions return Result which contains:
                - Ok(\_)
                - Err(\_)
              - Must error handle, either by using .expect or return matching
                -        let number:u32 = match guess.trim().parse() { //<--- return match of resultant Ok or Err
                                            Ok(num) => num, // if Ok return the resultant, in this case, u32 value called num;
                                            Err(k) => {
                                                println!("Error:{k}.");
                                                return 0;//always must return a u32 value or error handle a differnt way
                                            }
                                        }
                -       io::stdin()
                        .read_line(&mut guess)
                        .expect("Please type in a number");//<----- .expect

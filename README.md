# Rust_Pratice (using windows)

### To start and create a directory

    - In terminal type: cargo new directory_name
        - Example: cargo new hello_world

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

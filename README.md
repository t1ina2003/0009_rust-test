# RUST easy start

### ref course
[Rust 编程语言入门教程 [2021]](https://www.youtube.com/watch?v=e3-2uxHhbzs&list=PL3azK8C0kje1DUJbaOqce19j3R_-tIc4_&ab_channel=DaveYoung)

### update rust 
```
$rustup update
```
### uninstall rust
```
$rustup self uninstall
```
### check rust version
```
$rustc --version
rustc 1.71.1 (eb26296b5 2023-08-03)
```
version, hash, date

### build RS
```
rustc main.rs
```
nice report mechanism
```bash
  |
2 |     println("Hello World!")
  |     ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
2 |     println!("Hello World!")
  |            +
```
### run program
```
.\main.exe
```
### 1-3 Hello World
- no TAB, 4 spaces
- .pdb for debug
- ahead of time, self quick build and others run it. 
- rustc only for small program.
### 1-4 Cargo
- cargo --version
- cargo new hello_cargo
  - create new folder "hello-cargo" with .git
  - toml configure
    - dependencies called "crate" 
- cargo build
  - .cargo-lock
- cargo run
  - 3 steps: build, finish, running
- cargo check
  - `fast! try!`
- cargo build --release
  - `run fast, but build long`
### 2-1 guess number
- let
- mut
- read_line
- expect
### 2-2 rand
- crates.io
- cargo update: "major.minor.build", it only update latest build under specific minor 
  - also update .lock
- rand::Rng
### 2-3 
- shawdow: reassign mutable variable.
- match, arm Guess too big or too small or you win.
### 2-4
- add Game loop, and Gameover break
- match: process non-numeric situation.
### 3-1
- let, const, shadow
### 3-2
- i32, u32...
- arch, isize, usize
- only Byte(u8 only) = b'A'
- overflow "panic" only seen in Debug, not release
- char: 4 bytes
### 3-3 
- tuple (a,b,c,d), many types
- Rust check out of bounds.
### 3-4
- fn
- statement, expression
- early return 
# Don't @ me

Welcome to my mind! Here we try to memorize whatever `cargo` is. And before reading if I remember correctly cargo for Rust is like NPM for Node.js (package manager). *Back to reading*

Answer: Package manager AND Rust's build system.

# Using Cargo

Build a project using Cargo.

```
$ cargo new hello_cargo
$ cd hello_cargo
```

Note 1: `toml` file is similar to `package.json` file from the Node.js world. Nothing special, just a file to include every package so they can be easily included if they are missing. There is probably command like `npm install` to install every dependency for the project.

Note 2: Apparently Cargo should create a new git repository for the project, but for some reason it didn't. Probably because I have git repository out sided of this directory. 
*Why do I even write this at the same time as I read.* 
Can be overwritten with `--vcs=git` flag while creating a project.

Note 3: Converting project to use Cargo can be done manually by creating the correct structure (src directory and Cargo.toml file).

# Building & Running with Cargo

Build can be created with `cargo build`.

```
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Note 4: Building command creates new directory `target` which everything necessary. There are a lot of directories and files so it might seem like a mess at the start.

Running a project can be done with `cargo run`.

*This command also rebuilds the project if it detects that the contents have changed.*

```
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_cargo`
Hello, cargo!
```

# Check without running

Some times you may want to build without running which can be done with `cargo check`.

*Checking is faster because it doesn't produce new build. Great way to check if the code is compilable without compiling it.*

```
cargo check
    Checking hello_cargo v0.1.0 (/home/petterim/Development/Personal/Studies/Rust/Getting Started/1.3. Hello, Cargo/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
```

# Ready for releasing

Building releasable build can be compiled with `cargo build --release`. This creates new directory inside of `target` called `release`. Release build is just optimized version of previous development build.


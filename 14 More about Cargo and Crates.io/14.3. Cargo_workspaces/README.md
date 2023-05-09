# Cargo Workspaces

In continuous development it's good practice to split the project into necessary pieces, so the project stays at maintainable state. One tool that Rust offers is feature called workspaces that 
**can help manage multiple related packages that are developed in tandem**.

## Creating a Workspace

Because there is many ways of doing almost if not everything when it comes to software development the book is just introducing one common way of maintaining a project that consist of multiple libraries. With the instructions the project should basically contain another project. The root toml file should be edited to include `workspace` section which in turn contains the members of the workspace and the project should contain its own toml file and src directory will contain the `rs` files.

```
// Visualized directory structure
├── Cargo.lock
├── Cargo.toml
├── library_name
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

The book has great explanation about the target directory and its purpose being in the root of the project.

**The workspace has one target directory at the top level that the compiled artifacts will be placed into; the adder package doesn’t have its own target directory. Even if we were to run cargo build from inside the adder directory, the compiled artifacts would still end up in add/target rather than add/adder/target. Cargo structures the target directory in a workspace like this because the crates in a workspace are meant to depend on each other. If each crate had its own target directory, each crate would have to recompile each of the other crates in the workspace to place the artifacts in its own target directory. By sharing one target directory, the crates can avoid unnecessary rebuilding.**

## Creating the Second Package in the Workspace

Creating another package inside of a workspace works pretty much the same way as creating a new project. The main difference is that the workspace needs to be declared in toml file and it should include the members of the workspace. `Cargo build` builds the proejct based on the settings in toml file so basically it works as previously. If I wanted to run a specific part of the project I could pass the package name with `-p` flag, like so `cargo run -p adder`.

```
// Terminal output after running adder library
Finished dev [unoptimized + debuginfo] target(s) in 0.39s
Running `target\debug\adder.exe`
10 plus one is 11
```

## Depending on an External Package in a Workspace

Because the project knows there is multiple libraries within it, it will only contain the root level `Cargo.lock` file, which will make sure that every part of the project uses the same version of dependencies. It mainly helps to maintain dependencies. *Note that libraries should include in their toml files what dependencies they use even if they are used in another library. Workspaces cannot break existing rules and that's why they need to include their dependencies.*

## Adding a Test to a Workspace

Running tests on root directory (or top level) of the project will trigger all tests, as expected. With `-p` flag we can limit which tests are triggered, like so `cargo test -p add_one`.

*Note that every library in a workspace should be published separately, if they are published in crates.io.*
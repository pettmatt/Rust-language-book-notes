Commands to compile and run the code.

```
$ rustc main.rs
$ ./main
Hello, world!
```

*Pretty amazing. If I were say so myself.*

Note 1: In windows the command to run code would be written with backwards slash including the file extension "exe". Like this -> `.\main.exe`

Note 2: It seems like Rust doesn't necessary need semicolons, but let's see how far I get without using them.

Note 3: Seems like note 2 is obsolete if I would use the code formatter which is used to unify the code. (Great tool when there is more than one developer).

Note 4: The `!` after the `println` calls a Rust macro. Without the `!` Rust would call a function which would result to an error in this case.

Note 5: If we want to be precise the file that is compiled is called `a binary executable`.

*For cleaner code check this url https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html*
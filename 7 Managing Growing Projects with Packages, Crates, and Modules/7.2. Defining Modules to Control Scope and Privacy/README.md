# Defining Modules to Control Scope and Privacy

Scope control and privacy, probably the most scary combination in programming, but necessary to write clean and safe programs.

## Modules Cheat Sheet

**New keywords:**
- ``mod``, declare a module `mod name;`
- ``use``, within a scope creates shortcuts to items to reduce repetition of long paths
- ``pub``, change privacy of a module
- ``as``, 

`mod`, The compiler will look for the module’s code in these places:
1) Inline, within curly brackets that replace the semicolon following mod name
2) In the file src/garden.rs
3) In the file src/garden/mod.rs

Using the same logic, inside of a module we can include submodules:
1) Inline, directly following mod vegetables, within curly brackets instead of the semicolon
2) In the file src/garden/vegetables.rs
3) In the file src/garden/vegetables/mod.rs

`Paths` are part of how modules can be included and used inside of any other file other than the crate root. If the file is part of the crate it should be able to include modules from for example the root file. That is if the privacy settings allow it. **For example, an Asparagus type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.**

`Privacy`, by default modules are private from their parent modules. To change this behavior the modules need to be overwritten to be public by declaring it with `pub mod` instead of only `mod`. **To make items within a public module public as well, use `pub` before their declarations.**

`Use`, In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.
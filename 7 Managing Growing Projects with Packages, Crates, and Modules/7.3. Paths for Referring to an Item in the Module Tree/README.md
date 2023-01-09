# Paths for Referring to an Item in the Module Tree

Different languages handle paths a little differently. In Rust there are *absolute paths* and *relative paths*. A absolute path is full path, from the crate root to the desired module. A Relative path starts from current module which allows Rust to use `self` or `super` to identify the current module. 
**Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).**

*Locating a module uses different syntax than directory route, which is why visualizing the route with `/` could make it easier to understand. **Check Rust file `lib`.***

**Book: Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.**

It's important to understand and note why there is functions/modules that are kept private. 

**Book: Rust chose to have the module system function this way so that hiding inner implementation details is the default. That way, you know which parts of the inner code you can change without breaking outer code. However, Rust does give you the option to expose inner parts of child modules’ code to outer ancestor modules by using the pub keyword to make an item public.**

Personally I changed `hosting` module and the child function to public right away without really thinking why the child doesn't adopt the same status as the parent module, which to be honest isn't really a good way to study a new topic. So let's go through this with my own example why things work the way they do. 
*Just because person has access to specific area of a building doesn't mean they have access to certain cabinet. Maybe they don't have the key or maybe they just don't know that it is there (because it's not public knowledge).*

**Book: The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. Because modules are containers, there’s not much we can do by only making the module public; we need to go further and choose to make one or more of the items within the module public as well.**

To be honest, it feels little bit dirty to put `pub` in front of the parent and the child. Maybe it's just a me thing and I should get used to telling (in code) what functions/variables/values are (I'm mainly a JS-user, which is why I try to keep a distance to TS).

**... lets us use these paths in add_to_waitlist with respect to the privacy rules...** Yes. Let's do so. And nothing note worthy about this topic.

# Starting Relative Paths with super

So what does `super` keyword do? It "moves" the source from current module to parent, think of it as `..` operator.

That's kinda it.

# Making Structs and Enums Public

Making structs or enums public have few buts. Because those types can include fields that are private they need to be turned public case by case basis. **Check `lib`-file.**

*Question is how the field can be turned to public. Without changing the status of the field everywhere.*
**Answer: Don't know.** 

**Book: Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.**
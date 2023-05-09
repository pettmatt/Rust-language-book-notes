## Customizing Builds with Release Profiles

Profiles function pretty much the same way as they do in Javascript, but the name might be different. In a nutshell profiles contain different values and option that are used on run time (just like in Node.js `node run dev` `node run prod`)

A new profile can be added to cargo through the toml file by adding `[profile.PROFILE_NAME]` in the file and under it whatever values are needed.

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

Good point from the book is that if there is too many "env variables" the compiling time will be increased.
*`opt-level` is probably shorted from optimization level, which controls the -C opt-level flag which controls the level of optimization. 0 = none, 1 = basic, 2 = some, 3 = full optimization. There are also "s" (optimize for binary size) and "z" (same as s, but also turn off loop vectorization) options*

**Applying more optimizations extends compiling time, so if you’re in development and compiling your code often, you’ll want fewer optimizations to compile faster even if the resulting code runs slower. The default opt-level for dev is therefore 0. When you’re ready to release your code, it’s best to spend more time compiling. You’ll only compile in release mode once, but you’ll run the compiled program many times, so release mode trades longer compile time for code that runs faster. That is why the default opt-level for the release profile is 3.**


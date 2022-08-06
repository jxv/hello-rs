# hello-rs
🦀 How to stub test in Rust

README taken from [https://github.com/SeanShubin/hello#readme](https://github.com/SeanShubin/hello#readme)

# Hello Sample
- Sample project demonstrating how to test drive your non deterministic code
- Non-determinism highlighted:
    - console output
    - file system
    - system clock

# Steps to run

* Install rust with `rustup`
* Install `cargo-watch`

```shell
cargo watch -- cargo test
```

# Specification
- input
    - command line argument
    - specifies the name of a file
- behavior
    - load the target from the file in utf-8
    - compose a greeting message that says hello to the target
    - display the greeting message to the console
    - display the time taken in milliseconds to the console

# Tests should be both deterministic and fast
- deterministic
    - we only access what the code can directly control
    - we do not access
        - network
        - file system
        - system clock
        - environment variables
        - database
- fast
    - we are only testing one thing, and perform only just enough setup to do that one thing
    - we do not
        - read a configuration file
        - perform dynamic wiring
        - start a framework to invoke us
        - start a web container
        - create massive amounts of test data

# Approach in Rust

Testing can be defined in the same file of the source.
Define a conttract by using trait objects to [implement an object-oriented design pattern](https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html).
It's the conventional, non-magical approach: depedency injection of underlying systems, wiring objects at the top-level, and mock testing through stubbed objects.
In methods setting the first argument as `&mut self` shifts the control to the encapsulated object to determine its statefulness.

Remember to restrict the trait as object-safe:
* Don't return `Self`
* Don't use generics in the trait



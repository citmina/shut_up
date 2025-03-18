# Shut up! Trust me!
Use some keywords replace `unsafe` in Rust. Compiler shut up! Readers shut up too! My code has no bugs, trust me!

add the following line to your Cargo.toml:
```toml
shut_up = "0.1.1"
```
You can use the following keywords replace `unsafe`.

```rust
use shut_up::{trust_me, shut_up, cpp_start, sudo, i_am_your_father, freestyle,
              i_hereby_declare_that_i_fully_understand_that_any_consequence_is_my_own_liability,
              i_can_do_it, get_approval_from_a_senior_or_you_will_get_fired, showtime};

fn main() {
    trust_me! {
        println!("Trust me! This is safe!");
    }

    shut_up! {
        println!("Shut up! This is safe, tooooo!");
    }

    cpp_start! {
        println!("C++, Start!");
    }

    sudo! {
        println!("I am the superuser! Let me do it!");
    }

    i_am_your_father! {
        println!("I am your father! This is safe, too!");
    }

    i_hereby_declare_that_i_fully_understand_that_any_consequence_is_my_own_liability! {
        println!("I hereby declare that I fully understand that any consequence is my own liability.")
    }

    i_can_do_it! {
        println!("I can do it!")
    }

    get_approval_from_a_senior_or_you_will_get_fired! {
        println!("Get approval from a senior, or you will get fired!")
    }

    freestyle! {
        println!("Are you freestyle?")
    }

    showtime! {
        println!("show time!")
    }
}
```

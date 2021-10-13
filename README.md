# The best macros fo RUST
## About this crate
This crate was created when I had a macros for creating pub structs where all fileds are public.  
## Last changes
* Add public_struct macros.
## How to use
### Public struct
Previously, to implement a public structure with public fields, you wrote
```rust
pub struct Ticket {
    pub id: String,
    pub museum_id: String,
    pub description: String,
    pub date: String,
    pub time: String,
    pub quantity_groups: Vec<QuantityGroup>,
}

``` 
Now you can write like this
```rust
use best_macros::public_struct;

...

#[public_struct]
struct Ticket {
    id: String,
    museum_id: String,
    description: String,
    date: String,
    time: String,
    quantity_groups: Vec<QuantityGroup>,
}

```
And this struct and all her fields become a public.
## PS
Write your ideas in issue and I will try to implement them
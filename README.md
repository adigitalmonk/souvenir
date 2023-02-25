# Souvenir

A simple memoization tool.

## Installation

```toml
[dependencies]
souvenir = { git = "https://github.com/adigitalmonk/souvenir", tag = "1.0.0" }
```

## Usage

Using Souvenir, provides two simple memoization abstractions.
- `Memory`, a memoization tool that can remember lots of things.
- `Recall`, a memoization tool that can only remember one thing.

`Memory` is meant to work as a simple cache layer, while `Recall` works like a lazy evaluated function.

```rust
use souvenir::Memory;
use souvenir::Recall;

fn main() {
    let mut doubler = Memory::new(|key: &u32| key * 2);
    debug_assert_eq!(doubler.resolve(&2), 4);
    debug_assert_eq!(doubler.resolve(&2), 4); // Didn't recalculate
    debug_assert_eq!(doubler.resolve(&4), 8);

    let base_value = 2;
    let mut recaller = Recall::new(|| base_value * 2);
    debug_assert_eq!(recaller.value(), 4);
    debug_assert_eq!(recaller.value(), 4); // Didn't recalculate
}
```

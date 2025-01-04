# fsrsrs

A Rust implementation of the [FSRS](https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Algorithm) scheduler.

```rust
use fsrsrs::{Grade, review, now};

let card_1 = review(None, now(), Grade::Hard);
let card_2 = review(Some(card_1), now(), Grade::Good);
```

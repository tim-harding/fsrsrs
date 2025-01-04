# fsrsrs

A Rust implementation of the [FSRS](https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Algorithm) scheduler.

```rust
use chrono::Utc;
use fsrsrs::{Grade, review};

let card_1 = review(None, Utc::now(), Grade::Hard);
let card_2 = review(Some(card_1), Utc::now(), Grade::Good);
```

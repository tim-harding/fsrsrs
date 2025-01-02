# fsrsrs

A Rust implementation of the [FSRS](https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Algorithm) scheduler.

```rust
use chrono::Utc;
use fsrsrs::{Fsrs, Parameters, Grade};

let fsrs = Fsrs::new(Parameters::default());
let review_1 = fsrs.next_card(None, Utc::now(), Grade::Hard);
let review_2 = fsrs.next_card(Some(review_1), Utc::now(), Grade::Good);
```

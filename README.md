# time-humanize

A rust crate that displays duration in a human readable format.

This project is a port of [chrono-humanize-rs](https://github.com/imp/chrono-humanize-rs) and
now has 0 dependencies.


# Usage

```rust
use std::time::Duration;
use time_humanize::HumanTime;


let duration = Duration::from_secs(60);
let human_time = HumanTime::from(duration);
println!("{}", human_time);
```

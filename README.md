# dumbo-config  
dumbo-config is a config loader.  


## Features  
- Load project configurations  


## Quick Start  
```rust
use dumbo_config::load_config;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct TestConfig {
    name: String,
    value: i32,
}

...

let config: Option<TestConfig> = load_config();
```

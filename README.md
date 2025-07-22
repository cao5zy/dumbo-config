# dumbo-config  
dumbo-config is a config loader.  


## Features  
- Load project configurations  


## Quick Start  
Your configuration file.
```yaml
name: "test config"
value: 32
```

### configuration file name
The following file names is qualified.
- config.yml
- config.yaml
- config.{ENV}.yml
- config.{ENV}.yaml
Where `ENV` is the value of the environment variable "ENV". If "ENV" is not set, it defaults to searching `config.yml` and `config.yaml`.

You can also use `load_named_config` with specified config file.

Rust file for loading TestConfig
```rust
use dumbo_config::{load_config, load_named_config};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct TestConfig {
    name: String,
    value: i32,
}

...

let config: Option<TestConfig> = load_config();

let config_path: Path = ...;

let config: Option<TestConfig> = load_named_config(&config_path);
```

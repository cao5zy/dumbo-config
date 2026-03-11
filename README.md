# dumbo-config  
dumbo-config 是一个灵活配置加载器，提供三种不同层次的 API 来满足各种配置加载需求。

## 核心 API

dumbo-config 提供三个主要入口函数，按使用场景从简单到复杂排列：

### 1. `load_config` - 自动搜索配置文件

最简单的使用方式，自动按顺序搜索以下配置文件：
1. `config.{ENV}.yml`
2. `config.{ENV}.yaml`
3. `config.yml`
4. `config.yaml`

其中 `ENV` 是环境变量 "ENV" 的值。如果未设置 "ENV"，则默认搜索 `config.yml` 和 `config.yaml`。

```rust
use dumbo_config::load_config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    name: String,
    value: i32,
}

// 自动搜索并加载配置
let config: Option<AppConfig> = load_config();
```

**适用场景**：标准项目结构，配置文件位于项目根目录，使用默认命名约定。

---

### 2. `load_named_config` - 指定配置文件路径

当配置文件不在默认位置或使用非标准名称时，使用此函数指定具体路径。

```rust
use dumbo_config::load_named_config;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AppConfig {
    name: String,
    value: i32,
}

// 从指定路径加载配置
let config_path = Path::new("configs/production.yaml");
let config: Option<AppConfig> = load_named_config(config_path);
```

**适用场景**：配置文件位于非标准位置，或需要根据运行时条件动态选择配置文件。

---

### 3. `load_config_with_param` - 高级配置加载

最强大的加载方式，支持从文件和环境变量组合加载配置，提供完善的错误处理。

#### LoadingParam 结构

`load_config_with_param` 函数接收一个 `LoadingParam` 参数，它定义配置加载的来源：

```rust
pub struct LoadingParam {
    /// 配置文件路径，如果为 None 则不从文件加载
    pub file: Option<&'static Path>,
    /// 环境变量前缀配置，如果为 None 则不从环境变量加载
    pub env_prefix: Option<EnvConfig>,
}
```

**字段说明**：
- `file`: 配置文件路径，设置为 `Some(path)` 时从文件加载
- `env_prefix`: 环境变量前缀配置，设置为 `Some(EnvConfig)` 时从环境变量加载

#### 使用示例

```rust
use dumbo_config::{LoadingParam, EnvConfig, load_config_with_param};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AppConfig {
    database_url: String,
    port: u16,
    debug: bool,
}

// 仅从文件加载
let param = LoadingParam {
    file: Some(Path::new("config.yaml")),
    env_prefix: None,  // 不从环境变量加载
};
let config: AppConfig = load_config_with_param(&param)?;

// 仅从环境变量加载
let param = LoadingParam {
    file: None,  // 不从文件加载
    env_prefix: Some(EnvConfig::new("MY_APP".to_string(), None)),
};
let config: AppConfig = load_config_with_param(&param)?;

// 从文件和环境变量加载（环境变量优先）
let param = LoadingParam {
    file: Some(Path::new("config.yaml")),
    env_prefix: Some(EnvConfig::new("MY_APP".to_string(), None)),
};
let config: AppConfig = load_config_with_param(&param)?;
```

**适用场景**：
- 需要从环境变量覆盖配置文件
- 需要详细的错误信息
- 生产环境需要更严格的配置验证

**返回值**：`Result<T, ConfigError>`，提供全面的错误处理。

#### 最佳实践：文件 + 环境变量组合配置

推荐同时配置文件和环境变量，**环境变量会自动覆盖配置文件中的值**。这样可以在不同环境下使用同一套代码，只需通过环境变量覆盖需要变化的配置项。

**优势**：
- ✅ 代码简洁，无需环境判断逻辑
- ✅ config.yml 作为默认配置模板，包含所有配置项的默认值
- ✅ 生产环境只需设置需要覆盖的环境变量（如敏感信息）
- ✅ 灵活，可以部分覆盖配置

**配置对比示例**：

假设您的应用配置结构如下：
```rust
#[derive(Debug, Deserialize)]
struct AppConfig {
    database_url: String,
    port: u16,
    debug: bool,
}
```

**config.yml（默认配置）**：
```yaml
database_url: "postgres://localhost:5432/dev_db"
port: 3000
debug: true
```

**.env（生产环境覆盖）**：
```bash
# 只需覆盖需要变化的配置项
MY_APP__DATABASE_URL=postgres://prod-server:5432/prod_db
MY_APP__DEBUG=false
# port 未设置，将使用 config.yml 中的默认值 3000
```

**统一的加载代码**：
```rust
use dumbo_config::{LoadingParam, EnvConfig, load_config_with_param};
use std::path::Path;

fn load_app_config() -> Result<AppConfig, ConfigError> {
    // 同时配置文件和环境变量
    // 开发环境：只使用 config.yml
    // 生产环境：config.yml + 环境变量覆盖
    let param = LoadingParam {
        file: Some(Path::new("config.yml")),
        env_prefix: Some(EnvConfig::new("MY_APP".to_string(), None)),
    };
    
    load_config_with_param(&param)
}
```

**工作流程**：
1. 首先加载 `config.yml` 中的所有配置项作为默认值
2. 然后加载以 `MY_APP__` 为前缀的环境变量
3. 环境变量会覆盖 config.yml 中同名的配置项
4. 未设置环境变量的配置项保持 config.yml 中的默认值

---

## 环境变量配置

使用 `load_config_with_param` 时，可以通过 `EnvConfig` 配置环境变量加载。

### EnvConfig 结构

```rust
pub struct EnvConfig {
    /// 环境变量前缀，例如 "MY_APP"
    pub name: String,
    /// 环境变量分隔符，默认为 "__"
    pub separator: Option<String>,
}
```

**创建方式**：
```rust
// 使用默认分隔符 "__"
let env_config = EnvConfig::new("MY_APP".to_string(), None);

// 使用自定义分隔符
let env_config = EnvConfig::new("MY_APP".to_string(), Some("_".to_string()));
```

### 环境变量命名规则

给定以下配置结构：
```rust
struct DatabaseConfig {
    host: String,
    port: u16,
    credentials: Credentials,
}

struct Credentials {
    username: String,
    password: String,
}
```

使用 `EnvConfig::new("MY_APP".to_string(), None)`（默认分隔符 "__"），对应的环境变量为：
```bash
# 顶层字段
export MY_APP__HOST="localhost"
export MY_APP__PORT="5432"

# 嵌套字段（使用双下划线作为分隔符）
export MY_APP__CREDENTIALS__USERNAME="myuser"
export MY_APP__CREDENTIALS__PASSWORD="mypass"
```

**注意**：环境变量前缀不应包含分隔符。例如，如果前缀是 "RESUME_AGENT" 而分隔符是 "_"，这将导致配置加载错误。

---

## 日志记录和调试

该库在 INFO 级别提供详细的日志记录：

- 始终记录正在使用的配置源
- 当设置了 `env_prefix` 并且 `SHOW_SETTINGS` 环境变量设置为 "true"（不区分大小写）时，记录配置加载成功的信息
- 如果未找到指定前缀的环境变量，则记录警告并继续加载配置，不使用环境变量

**注意**：`SHOW_SETTINGS` 环境变量仅在配置了 `env_prefix` 时才会被检查。这是因为如果没有设置 `env_prefix`，意味着用户直接从文件加载配置，他们可以直接检查配置文件来查看设置。当使用环境变量进行配置（通过 `env_prefix`）时，实际值可能不容易看到，因此 `SHOW_SETTINGS` 提供了一种记录加载的配置以进行调试的方法。

要启用配置调试，如果 `env_prefix` 是 'MY_APP'，请设置 `SHOW_SETTINGS` 环境变量：

```bash
export MY_APP__SHOW_SETTINGS=true
./your-application
```

`SHOW_SETTINGS` 支持的值（不区分大小写）："true"、"1"、"yes"、"on"

---

## 错误处理

`load_config_with_param` 提供全面的错误处理，所有错误都包装在 `ConfigError` 枚举中：

- **InvalidLoadingParam**: `LoadingParam` 的 `file` 和 `env_prefix` 字段都为 `None` - 告诉运维人员需要配置至少一个来源
- **InvalidEnvConfig**: 环境前缀包含分隔符
- **FileNotFound**: 指定的配置文件不存在
- **ShowSettingsParseError**: SHOW_SETTINGS 环境变量无法解析为布尔值

`load_config` 和 `load_named_config` 返回 `Option<T>`，加载失败时返回 `None`，错误信息通过日志记录。

---

## 功能特性总结

- ✅ 三种加载 API，适应不同使用场景
- ✅ 支持环境变量，可配置前缀和自定义分隔符
- ✅ 增强的日志记录，包含加载源信息
- ✅ SHOW_SETTINGS 环境变量用于调试配置
- ✅ 全面的错误处理，提供运维友好的错误信息

## 联系方式
电子邮件: zongying_cao@163.com

![涵树的微信公众号](./assets/wechat.jpg)

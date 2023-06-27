<div align="center">
<h1>洛书客户端</h1>
</div>

### 安装洛书

```shell
cargo install luoshu
```

### 运行洛书
不开放管理web接口
```shell
luoshu
```
开放管理web接口
```shell
luoshu --web
```


### 洛书客户端使用

引入洛书客户端
```toml
[workspace.dependencies]
# ...
luoshu_rust_client = "0.1.0"
# ...
```
订阅配置信息，并注册服务到洛书并编写业务代码
```rust
use std::thread::sleep;
use luoshu_rust_client::LuoshuClient;
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    test1: String,
    test2: Vec<usize>,
}
#[tokio::test]
async fn it_works() -> LuoshuClientResult<()> {
    let mut client = LuoshuClient::new(15666, "test_rust_server".to_string(), None).await;
    client
        .subscribe_config(
            "test_config2".to_string(),
            |config: Config| println!("config changed:{:#?}", config),
            None,
        )
        .await?;
    tokio::spawn(async move {
        client.registry().await.expect("TODO: panic message");
    });
    
    // 此处使用无限循环模拟服务的持续运行
    loop {
        println!("sleep");
        sleep(Duration::from_secs(10))
    }
}
```
<div align="center">
<h1>洛书</h1>
<p>
<a href="https://github.com/hubertshelley/luoshu/actions">
    <img alt="build status" src="https://github.com/hubertshelley/luoshu/workflows/build/badge.svg?branch=master&event=push" />
</a>
<br/>
<a href="https://crates.io/crates/luoshu"><img alt="crates.io" src="https://img.shields.io/crates/v/luoshu" /></a>
<a href="https://docs.rs/luoshu"><img alt="Documentation" src="https://docs.rs/luoshu/badge.svg" /></a>
<a href="https://github.com/rust-secure-code/safety-dance/"><img alt="unsafe forbidden" src="https://img.shields.io/badge/unsafe-forbidden-success.svg" /></a>
<a href="https://blog.rust-lang.org/2022/09/22/Rust-1.64.0.html"><img alt="Rust Version" src="https://img.shields.io/badge/rust-1.64%2B-blue" /></a>
<br/>

[//]: # (<a href="https://luoshu.rs">)

[//]: # (    <img alt="Website" src="https://img.shields.io/badge/https-luoshu.rs-%23f00" />)

[//]: # (</a>)
<a href="https://codecov.io/gh/hubertshelley/luoshu">
<img alt="codecov" src="https://codecov.io/gh/axnsan12/hubertshelley/luoshu/master/graph/badge.svg" />
</a>
<a href="https://crates.io/crates/luoshu"><img alt="Download" src="https://img.shields.io/crates/d/luoshu.svg" /></a>
<img alt="License" src="https://img.shields.io/crates/l/luoshu.svg" />
</p>
</div>

### 运行说明
#### 编译前端
```shell
cd luoshu-frontend
npm install
npm run build

```
#### 运行后端
```shell
cargo run --release --bin luoshu -- --web
```
打开浏览器访问 http://localhost:19999
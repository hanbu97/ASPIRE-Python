poem 和 axum 的 Route/IntoResponse/Json 等等 API 设计几乎是一模一样的

以 IntoResponse 为例子实现之后 handler 函数的返回值就可以用定制类型

但 poem 的 IntoResponse 还比 axum 多要求一个 Send 导致很难用

```rust
#[derive(Serialize)]
pub struct Resp<T: Serialize> {
    pub data: T,
    pub code: u16,
    pub message: String,
}

impl<T: Serialize> poem::IntoResponse for Resp<T> {
    fn into_response(self) -> poem::Response {
        poem::web::Json(self).into_response()
    }
}
```

而且 poem 那个宏太多了，还是拥抱零宏的的 axum

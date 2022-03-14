## rbatis 不足:
- 不支持 postgres enum 类型
- 数据传输多了一层 json 层抽象以便抹平不同数据库格式的差异导致性能变慢
- 不支持根据表结构自动生成 rust 模型代码
- ORM 的 filter 语句传入的是字段名称字符串无法做到编译时检查

## diesel 不足:
- cli 工具表结构生成 Rust 代码是宏代码不如 sea-orm 直接生成 Rust 结构体可读性好

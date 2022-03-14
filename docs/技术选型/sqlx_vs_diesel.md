## sqlx 的不足
- 不能像 diesel 那样 ORM 写代码的时候数据库表字段补全，不能在写代码时检查使用了数据库中不存在的字段
- sqlx 可读性也不如 ORM
- 几乎不能换数据库(例如 pg 换 sqlite)

## diesel 的不足
- 不支持异步读写
- 不支持 order by field 等较少用的 SQL 语句
- 复杂多个 join 的 SQL 语句几乎没法写
- 事务处理 API 不好用

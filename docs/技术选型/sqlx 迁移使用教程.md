## 前提需要

cargo install sqlx

## 新增迁移文件

**记得加上 -r 参数** 否则不会加上 down.sql 无法做迁移 revert (回滚)

sqlx migrate add -r create_orders

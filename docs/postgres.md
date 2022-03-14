
## 加载项目根目录的 ddl.sql

```
drop schema public cascade; create schema public; \i ddl.sql; \i insert_test_data.sql

sea-orm-cli generate entity -o crates/db_schema/src/codegen --database-url postgres://w:w@localhost/w
```

## 测试用的数据

请看 insert_test_data.sql

## 常用元信息查询

|查询语句|作用|
|---|---|
|select current_database();|当前连接的库名(看命令行 prompt 也可得知库名)|
|\l|列出所有数据库|
|\c $db_name|切换到 $db_name 库|
|\du|列出所有数据库用户(role)|
|\dt|列出当前库的所有表|
|\d $table_name|列出某个表的所有字段|


## 常用配置/参数查询

### 允许 localhost 访问

psql 默认连本地数据库走的 Unix domain socket 跟走 tcp 的鉴权方式不一样

```
[w@fedora WXWork]$ psql -U w -h localhost
psql: error: FATAL:  Ident authentication failed for user "w"
```

获取 pg_hba.conf 配置文件路径

```
[w@fedora WXWork]$ sudo -u postgres psql
[sudo] password for w: 
psql (13.4)
Type "help" for help.

postgres=# show hba_file;
            hba_file             
---------------------------------
 /var/lib/pgsql/data/pg_hba.conf
```

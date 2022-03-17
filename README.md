
## 数据库修改
- 直接在db_schema::data下修改对应的.rs文件，每个文件对应一个数据库
- 数据库中的enum统一在sea_orm_active_enums文件中进行管理
- 为数据库中的类型实现arbitary以便后续例子的生成，实现的方法可参考现有的实现
- 数据库发生修改后建议重新建立数据库，在项目根目录中运行下面语句，可以删除所有的表，根据migration的规则，依次建表
``` 
sea-orm-cli migrate fresh
```
- 建立数据表后，使用下列语句可生成测试数据
```
cargo run --bin fill_test_db
```

## 项目启动
- 在根目录下.env文件中配置项目运行的ip，端口，数据库连接
- 配置完成后，用下列语句运行
```
cargo run --bin idppay
```


## 接口详情
- 参阅apifox接口说明以及输入输出样例









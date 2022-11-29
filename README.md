# xbatis2sql

`xbatis2sql`，用来抽取散落在各个 `xml` 文件中的 `sql`，供集中进行后续处理。

`xbatis2sql`, collect sql statements from iBATIS sqlmap files/MyBatis mapper files.

## 用法 Usage

```verilog
# xbatis2sql -h
Usage: xbatis2sql [-i|-m] -t [Oracle/MySQL] -s ... -o ...

Options:
    -i, --ibatis        try to parse iBATIS sqlmap files
    -m, --mybatis       try to parse MyBatis mapper files
    -t, --type DB       db type
    -s, --src SRC       source directory
    -o, --output OUTPUT output directory
    -h, --help          print this help menu
```

### iBATIS

```shell
xbatis2sql -i -t Oracle -s /java/use_ibatis_proj/src -o /tmp
```

或 or

```shell
xbatis2sql --ibatis --type Oracle --src /java/use_ibatis_proj/src --output /tmp
```

执行后可获得文件： `/tmp/result.sql`

After executing, the result will be exist in `/tmp/result.sql`.

### MyBatis

```shell
xbatis2sql -m -t Oracle -s /java/use_mybatis_proj/src -o /tmp
```

或 or

```shell
xbatis2sql --mybatis --type Oracle --src /java/use_mybatis_proj/src --output /tmp
```

执行后可获得文件： `/tmp/result.sql`

After executing, the result will be exist in `/tmp/result.sql`.

## 感谢 Thanks

感谢 [mybatis-mapper-2-sql](https://github.com/actiontech/mybatis-mapper-2-sql) / [sqle](https://github.com/actiontech/sqle)

Thanks to [mybatis-mapper-2-sql](https://github.com/actiontech/mybatis-mapper-2-sql) / [sqle](https://github.com/actiontech/sqle)

## 样例 Sample

### MyBatis

[mapper-demo.xml](./test_data/mapper-demo.xml) 将转化为 `result.sql`。
[mapper-demo.xml](./test_data/mapper-demo.xml) will be converted to `result.sql`.

**`result.sql`**

```sql
-- ./test_data/mapper-demo.xml
--- insert
INSERT INTO TAB1(A,B,C,D) VALUES (:?,:?,:?,:?);
--- insert.selectKey
SELECT 1 FROM DUAL;
--- select
SELECT * FROM TAB1 WHERE COLUMN1 IN ( :?);
--- insert2
INSERT INTO TAB2 ( ID)VALUES ( :?);
--- select2
SELECT COLUMN1, COLUMN2 , (SELECT 1 FROM DUAL) FROM TAB3 WHERE COLUMN1 = :? ORDER BY COLUMN2 DESC;
--- update
UPDATE TAB1 SET COLUMN1 = :? WHERE COLUMN1 = :?;
--- delete
DELETE FROM TAB1 WHERE COLUMN1 = :? AND COLUMN2 = :?;
```

### iBATIS

[sqlmap-demo.xml](./test_data/sqlmap-demo.xml) 将转化为 `result.sql`。
[sqlmap-demo.xml](./test_data/sqlmap-demo.xml) will be converted to `result.sql`.

**`result.sql`**

```sql
-- ./test_data/sqlmap-demo.xml
--- select
SELECT COUNT(1) , (SELECT 1 FROM DUAL) FROM __REPLACE_SCHEMA__.TAB1 WHERE COLUMN1 = 'BALABALA' AND COLUMN2 = :?;
--- update
UPDATE __REPLACE_SCHEMA__.TAB2 SET COLUMN2 = :? WHERE COLUMN1 = :?;
--- delete
DELETE FROM __REPLACE_SCHEMA__.TAB1 WHERE COLUMN1 = :?;
--- insert
INSERT INTO __REPLACE_SCHEMA__.TAB1 (COLUMN1, COLUMN2, COLUMN3, COLUMN4, COLUMN5) VALUES (:?, :?, :?, :?, :?);
```

> 如果是 `MySQL` 模式，`:?` 改为 `@1`。
> If under `MySQL` mode, `:?` will be replaced with `@1`.

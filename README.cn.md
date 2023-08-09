# xbatis2sql

[英文版本](README.md)

`xbatis2sql`，用来抽取散落在各个 `xml` 文件中的 `sql`，供集中进行后续处理。

## 安装方法

```shell
cargo install xbatis2sql
```

## 用法

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

或

```shell
xbatis2sql --ibatis --type Oracle --src /java/use_ibatis_proj/src --output /tmp
```

执行后可获得文件： `/tmp/result.sql`。

### MyBatis

```shell
xbatis2sql -m -t Oracle -s /java/use_mybatis_proj/src -o /tmp
```

或

```shell
xbatis2sql --mybatis --type Oracle --src /java/use_mybatis_proj/src --output /tmp
```

执行后可获得文件： `/tmp/result.sql`。

## 样例

### MyBatis

[mapper-demo.xml](./test_data/mapper-demo.xml) 将转化为 `result.sql`。

**`result.sql`**

```sql
SELECT "XML -FILE: ./test_data/mapper-demo.xml" AS XML_FILE FROM DUAL;
SELECT "STAT -ID: insert" AS STAT_ID FROM DUAL;
INSERT INTO TAB1(A,B,C,D) VALUES (:?,:?,:?,:?);
SELECT "STAT -ID: insert.selectKey" AS STAT_ID FROM DUAL;
SELECT 1 FROM DUAL;
SELECT "STAT -ID: select" AS STAT_ID FROM DUAL;
SELECT * FROM TAB1 WHERE COLUMN1 IN ( :?);
SELECT "STAT -ID: insert2" AS STAT_ID FROM DUAL;
INSERT INTO TAB2 ( ID)VALUES ( :?);
SELECT "STAT -ID: select2" AS STAT_ID FROM DUAL;
SELECT COLUMN1, COLUMN2 , (SELECT 1 FROM DUAL) FROM TAB3 WHERE COLUMN1 = :? ORDER BY COLUMN2 DESC;
SELECT "STAT -ID: update" AS STAT_ID FROM DUAL;
UPDATE TAB1 SET COLUMN1 = :? WHERE COLUMN1 = :?;
SELECT "STAT -ID: delete" AS STAT_ID FROM DUAL;
DELETE FROM TAB1 WHERE COLUMN1 = :? AND COLUMN2 = :?;
```

### iBATIS

[sqlmap-demo.xml](./test_data/sqlmap-demo.xml) 将转化为 `result.sql`。

**`result.sql`**

```sql
SELECT "XML -FILE: ./test_data/sqlmap-demo.xml" AS XML_FILE FROM DUAL;
SELECT "STAT -ID: select" AS STAT_ID FROM DUAL;
SELECT COUNT(1) , (SELECT 1 FROM DUAL) FROM __REPLACE_SCHEMA__.TAB1 WHERE COLUMN1 = 'BALABALA' AND COLUMN2 = :?;
SELECT "STAT -ID: update" AS STAT_ID FROM DUAL;
UPDATE __REPLACE_SCHEMA__.TAB2 SET COLUMN2 = :? WHERE COLUMN1 = :?;
SELECT "STAT -ID: delete" AS STAT_ID FROM DUAL;
DELETE FROM __REPLACE_SCHEMA__.TAB1 WHERE COLUMN1 = :?;
SELECT "STAT -ID: insert" AS STAT_ID FROM DUAL;
INSERT INTO __REPLACE_SCHEMA__.TAB1 (COLUMN1, COLUMN2, COLUMN3, COLUMN4, COLUMN5) VALUES (:?, :?, :?, :?, :?);
```

> 如果是 `MySQL` 模式，`:?` 改为 `@1`。

## 更新记录

见 [ChangeLog](ChangeLog.md)

## 感谢

感谢 [mybatis-mapper-2-sql](https://github.com/actiontech/mybatis-mapper-2-sql) / [sqle](https://github.com/actiontech/sqle)

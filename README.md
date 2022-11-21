# xbatis2sql

`xbatis2sql`，用来抽取散落在各个 `xml` 文件中的 `sql`，供集中进行后续处理。

`xbatis2sql`, collect sql statements from iBATIS sqlmap files/MyBatis mapper files.

## 用法 Usage

### iBATIS

```shell
xbatis2sql -i -s /java/use_ibatis_proj/src -o /tmp
# or
xbatis2sql --ibatis --src /java/use_ibatis_proj/src --output /tmp
```

执行后可获得文件： `/tmp/result.sql`

After executing, the result will be exisiting in `/tmp/result.sql`.

### MyBatis

```shell
xbatis2sql -m -s /java/use_mybatis_proj/src -o /tmp
# or
xbatis2sql --mybatis --src /java/use_mybatis_proj/src --output /tmp
```

执行后可获得文件： `/tmp/result.sql`

After executing, the result will be exisiting in `/tmp/result.sql`.

## 感谢 Thanks

感谢 [mybatis-mapper-2-sql](https://github.com/actiontech/mybatis-mapper-2-sql) / [sqle](https://github.com/actiontech/sqle)

Thanks to [mybatis-mapper-2-sql](https://github.com/actiontech/mybatis-mapper-2-sql) / [sqle](https://github.com/actiontech/sqle)

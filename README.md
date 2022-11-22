# xbatis2sql

`xbatis2sql`，用来抽取散落在各个 `xml` 文件中的 `sql`，供集中进行后续处理。

`xbatis2sql`, collect sql statements from iBATIS sqlmap files/MyBatis mapper files.

## 用法 Usage

```verilog
# xbatis2sql -h
Usage: xbatis2sql [-i|-m] -s ... -o ...

Options:
    -i, --ibatis        try to parse iBATIS sqlmap files
    -m, --mybatis       try to parse MyBatis mapper files
    -s, --src SRC       source directory
    -o, --output OUTPUT output directory
    -h, --help          print this help menu    
```

### iBATIS

```shell
xbatis2sql -i -s /java/use_ibatis_proj/src -o /tmp
```

或 or

```shell
xbatis2sql --ibatis --src /java/use_ibatis_proj/src --output /tmp
```

执行后可获得文件： `/tmp/result.sql`

After executing, the result will be exist in `/tmp/result.sql`.

### MyBatis

```shell
xbatis2sql -m -s /java/use_mybatis_proj/src -o /tmp
```

或 or

```shell
xbatis2sql --mybatis --src /java/use_mybatis_proj/src --output /tmp
```

执行后可获得文件： `/tmp/result.sql`

After executing, the result will be exist in `/tmp/result.sql`.

## 感谢 Thanks

感谢 [mybatis-mapper-2-sql](https://github.com/actiontech/mybatis-mapper-2-sql) / [sqle](https://github.com/actiontech/sqle)

Thanks to [mybatis-mapper-2-sql](https://github.com/actiontech/mybatis-mapper-2-sql) / [sqle](https://github.com/actiontech/sqle)

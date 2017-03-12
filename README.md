# chi-lang

chi语言

# 变量
`var a = 123`

# 函数
`fn f = (a, b) -> a + b`

# 类
```
class C {
    // 属性
    var a = 123;
    var b = 'abc';
    // 方法
    fn c = (x, y) -> x + y;
}
```

## 内部类
chi语言不支持内部类。

# 条件
## if
```
var min = if (x < y) {
    x
} else {
    y
}
```
if语句的最后一行是返回值，如果分支不完整（没有else），则当没有匹配结果时返回undefined

## case


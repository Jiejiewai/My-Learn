### 1. @babel/parser

基于 acorn 实现,提供了
1. parse
2. parseExpression
两个API,这两个API的功能都是把语言转成AST，但是
parse返回整个文件
parseExpression返回Expression

```javascript
parse(input: string, options?: ParserOptions)
parseExpression(input: string, options?: ParserOptions): Expression)
```

options实例:
```javascript
    require("@babel/parser").parse("code", {
    sourceType: "module",
    plugins: [
        "jsx",
        "typescript"
    ]
    });
```
原文中有一句
//  TAT commonjs和es等的区别还是不太熟，需要加强
（因为 @babel/parser 等包都是通过 es module 导出的，所以通过 commonjs 的方式引入有的时候要取 default 属性。）

### 2. @babel/traverse

#### 2.1 traverse

```javascript
function traverse(parent, opts)
```
parent: 需要遍历的AST节点
opts: 对应的visitor函数

```javascript
// 进入 FunctionDeclaration 节点时调用
traverse(ast, {
  FunctionDeclaration: {
      enter(path, state) {}
  }
})

// 默认是进入节点时调用，和上面等价
traverse(ast, {
  FunctionDeclaration(path, state) {}
})

// 通过别名指定离开各种 Declaration 节点时调用
traverse(ast, {
  Declaration: {
      exit(path, state) {}
  }
})
```

如果FunctionDeclaration默认是函数，则代表enter函数
如果FunctionDeclaration是对象，则可以配置enter和exit。

#### 2.2 FunctionDeclaration中的path

path 是遍历过程中的路径，会保留上下文信息，有很多属性和方法。

有啥方法在原文中有，不想写了。

第二个参数 state 则是遍历过程中在不同节点之间传递数据的机制，插件会通过 state 传递 options 和 file 信息，我们也可以通过 state 存储一些遍历过程中的共享数据。

### 3. @babel/types

遍历 AST 的过程中需要创建一些 AST 和判断 AST 的类型，这时候就需要 @babel/types 包。

### 4. @babel/template

通过 @babel/types 创建 AST 还是比较麻烦的，要一个个的创建然后组装，如果 AST 节点比较多的话需要写很多代码，这时候就可以使用 @babel/template 包来批量创建。

### 5. @babel/generator
把AST重新变成高级语言。

```javascript
  const { code, map } = generate(ast, { sourceMaps: true })
```

### 6. @babel/code-frame
当有错误信息要打印的时候，需要打印错误位置的代码，可以使用@babel/code-frame。

```javascript
  const result = codeFrameColumns(rawLines, location, {
    /* options */
  });
```


options 可以设置 highlighted （是否高亮）、message（展示啥错误信息）。
```javascript
const { codeFrameColumns } = require("@babel/code-frame");

try {
 throw new Error("xxx 错误");
} catch (err) {
  console.error(codeFrameColumns(`const name = guang`, {
      start: { line: 1, column: 14 }
  }, {
    highlightCode: true,
    message: err.message
  }));
}
```
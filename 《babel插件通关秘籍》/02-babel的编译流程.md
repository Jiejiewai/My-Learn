### 1. 编译器和转译器
 - 编译器(Compiler)：高级语言 => 低级语言
 - 转译器(Transpiler)： 高级语言 => 高级语言

### 2. babel的编译流程
 - parse: 把 ts/js/es6 等程序员写的语言转译成 计算机能理解的AST树
 - transform: 处理AST🌲，删去 { } ; 之类符号
 - generate: 再把AST🌲转回👨🏻‍💻能看懂的语言

#### 2.1 parse
    中文翻译成词法分析或者语法分析。

    分词，并进行递归地组装。
#### 2.2 transform
    遍历AST树，发现不同（？这个不同是指的什么不同）的AST节点，会调用相应的注册的visitor 函数里对 AST 节点进行增删改.

    visitor函数会返回新的AST🌲。

#### 2.3 generate
    转回js语言，并生成 ***sourcemap***.

    从AST的根节点，递归打印生成js语言。

    不同的AST节点，对应了不同结构的字符串。

    ***sourcemap 记录了源码到目标代码的转换关系，通过它我们可以找到目标代码中每一个节点对应的源码位置。***
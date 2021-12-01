### AST节点的类型
1. Literal: 字面量

2. Identifier: 变量

3. Statement: 语句
	break;
    continue;
    return;
    debugger;
    throw Error();
    {}
    try {} catch(e) {} finally{}
    for (let key in obj) {}
    for (let i = 0;i < 10;i ++) {}
    while (true) {}
    do {} while (true)
    switch (v){case 1: break;default:;}
    label: console.log();
    with (a){}

	
    如上的每行都是一个statement

4. Declaration: 声明语句是一种特殊的语句，它执行的逻辑是在作用域内声明一个变量、函数、class、import、export 等。
	const a = 1;
    function b(){}
    class C {}

    import d from 'e';

    export default e = 1;
    export {e};
    export * from 'e';
            每行都是一个declaration

 5. Expression: expression 是表达式，特点是执行完以后有返回值，这是和语句 (statement) 的区别。
       
            [1,2,3]
    a = 1
    1 + 2;
    -1;
    function(){};
    () => {};
    class{};
    a;
    this;
    super;
    a::b;

6. Class 

7. Modules


8. Program & Directive
    program 是代表整个程序的节点，它有 body 属性代表程序体，存放 statement 数组，就是具体执行的语句的集合。还有 directives 属性，存放Directive 节点，比如"use strict" 这种指令会使用 Directive 节点表示。

9. File & Comment
    babel 的 AST 最外层节点是 File，它有 program、comments、tokens 等属性，分别存放 Program 程序体、注释、token 等，是最外层节点。

    注释分为块注释和行内注释，对应 CommentBlock 和 CommentLine 节点。   

### AST 的公共属性

每种 AST 都有自己的属性，但是它们也有一些公共属性：

- type: 标识节点的类型
- start、end、loc：start 和 end 代表该节点对应的源码字符串的开始和结束下标，不区分行列。

而 loc 属性是一个对象，有 line 和 column 属性分别记录开始和结束行列号。

- leadingComments、innerComments、trailingComments： 表示开始的注释、中间的注释、结尾的注释，因为每个 AST 节点中都可能存在注释，而且可能在开始、中间、结束这三种位置，通过这三个属性来记录和 Comment 的关联。

- extra：记录一些额外的信息，用于处理一些特殊情况。比如 StringLiteral 修改 value 只是值的修改，而修改 extra.raw 则可以连同单双引号一起修改。

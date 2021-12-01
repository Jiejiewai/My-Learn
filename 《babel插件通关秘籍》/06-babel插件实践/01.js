/**
 * 功能：我们经常会打印一些日志来辅助调试，但是有的时候会不知道日志是在哪个地方打印的。希望通过 babel 能够自动在 console.log 等 api 中插入文件名和行列号的参数，方便定位到代码。
 */

 const parser = require('@babel/parser');
 /**
  * （因为 @babel/parser 等包都是通过 es module 导出的，
  * 所以通过 commonjs 的方式引入有的时候要取 default 属性。）
  */
 const traverse = require('@babel/traverse').default;
 const generate = require('@babel/generator').default;
 const types = require('@babel/types');
 
 const sourceCode = `
     console.log(1);
 
     function func() {
         console.info(2);
     }
 
     export default class Clazz {
         say() {
             console.debug(3);
         }
         render() {
             return <div>{console.error(4)}</div>
         }
     }
 `;
 
 const ast = parser.parse(sourceCode,{
     sourceType: 'unambiguous',
     plugins: ['jsx']
 })
 
 // traverse是直接对ast进行的修改，没有返回。
 traverse(ast,{
     CallExpression(path, state){
         if(types.isMemberExpression(path.node.callee)// 如果是成员表达式
         && path.node.callee.object.name === 'console'// 并且name为console
         && ['log','info','error','debug'].includes(path.node.callee.property.name)// 并且属性名中含有这些
         ){
             // 从path.node.loc中获取行列号
             const { line, column } = path.node.loc.start;
             // 原来存在arguments里
             path.node.arguments.unshift(types.stringLiteral(`filename:(${line},${column})`))
         }
     }
 });
 
 const { code, map } = generate(ast);
 console.log(code);
 
 
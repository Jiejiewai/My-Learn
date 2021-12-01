/**
 * 后来我们觉得在同一行打印会影响原本的参数的展示，所以想改为在 console.xx 节点之前打印的方式
 */

/**
 * 有两个需要注意的点
 * 1. JSX 中的 console 代码不能简单的在前面插入一个节点，而要把整体替换成一个数组表达式，因为 JSX 中只支持写单个表达式。
 * 2. 用新的节点替换了旧的节点之后，babel traverse 会继续遍历新节点，这是没必要的，所以要跳过新生成的节点的处理。
 */
 const parser = require('@babel/parser');
 const traverse = require('@babel/traverse').default;
 const generate = require('@babel/generator').default;
 const types = require('@babel/types');
const template = require('@babel/template');

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

 const targetCalleeName = ['log','info','error','debug'].map(item => `console.${item}`);
 traverse(ast,{
     CallExpression(path,state){
        if(path.node.isNew){
            return;
        }
        const calleeName = generate(path.node.callee).code;
        if(targetCalleeName.includes(calleeName)){
            const { line, column } = path.node.loc.start;
            const newNode = template.expression(`console.log("filename:(${line},${column})")`)();
            newNode.isNew = true;

            /**
             * 判断是 insertBefore 还是 replaceWith 要看当前节点是否在 JSXElement 之下，
             * 所以要用path.findParent 方法顺着 path 查找是否有 JSXElement 节点。
             * replace 的新节点要调用 path.skip 跳过后续遍历。
             */
            if(path.findParent(path => path.isJSXElement())){
                // 替换整体节点用path.replaceWith
                path.replaceWith(types.arrayExpression([newNode, path.node]));
                path.skip();
            }else{
                // 插入 AST
                path.insertBefore(newNode);
            }
        }
     }
 })

 const { code, map } = generate(ast);
 console.log(code);
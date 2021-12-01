// 使用generator对traverse函数进行优化

const targetCalleeName = ['log','info','error','debug'].map(item => `console.${item}`);
traverse(ast,{
    CallExpression(path,state){
        const calleeName = generate(path.node.callee).code;
        if(targetCalleeName.includes(calleeName)){
            const { line, column } = path.node.loc.start;
            path.node.arguments.unshift(types.stringLiteral(`filename: (${line}, ${column})`))
        }
    }
})
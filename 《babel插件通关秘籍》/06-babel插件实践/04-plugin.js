/**
 * 将前面开发的东西生成插件 
 * */

 const { transformFileSync } = require('@babel/core');
 const path = require('path');
 const insertParametersPlugin = require('./parameters-insert-plugin.js');
 
 
 const code = transformFileSync(path.join(__dirname,'./sourceCode.js'),{
     plugins:[insertParametersPlugin],
     parserOpts: {
         sourceType: 'unambiguous',
         plugins: ['jsx']
     }
 })
 
 console.log(code);
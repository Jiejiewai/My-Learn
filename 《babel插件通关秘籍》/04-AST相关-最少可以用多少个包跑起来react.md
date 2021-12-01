```javascript
// react 包
1.react
2.react-dom

// babel

/* 
*    babel的主要作用是将高版本es编译成低版本es，解析jsx和es6
*    babel-core的作用是把js代码分析成ast，方便各个插件分析语法进行相应的处理。有些新语法在低版本js中不存在，如箭头函数，
*    这种语言层面的不兼容只能通过将代码转为ast，分析其语法后再转为低版本js
*/
3.babel-core


/* 
*    此 package 允许你使用 Babel 和 webpack 转译 JavaScript 文件。
*    babel-loader必须和babel-core结合使用，babel-core封装了babel-loader需要用到的api
*    babel-loader和babel-core的版本需要对应(core需要比loader高一个版本才能用)
*/
4.babel-loader
/* 
*   对babel-loader进行相关的配置。 使用 .babelrc
*/
5.babel-preset-env

/* 
*   此预设（preset）始终包含以下插件：
*   @babel/plugin-syntax-jsx
*   @babel/plugin-transform-react-jsx
*   @babel/plugin-transform-react-display-name
*   如果开启了 development 参数，还将包含以下插件：
*   Classic runtime adds:
*   @babel/plugin-transform-react-jsx-self
*   @babel/plugin-transform-react-jsx-source
*/
6.babel-preset-react

// webpack 打包
7.html-webpack-plugin
8. webpack
9. webpack-cli

// webpack-dev-server 是一个小型的node express服务器，它使用webpack-dev-middle中间件来为通过webpack打包生成的资源文件提供web服务
10. webpack-dev-server

```

事实证明真的需要至少10个包才能跑起来最简单的react

@babel/polyfill 用于实现es6的一些新特性，例如promise async/await 等
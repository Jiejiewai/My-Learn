> 一个开发服务器，基于原生的ES模块，提供了超快的HMR。
将代码通过Rollup打包静态资源用于生产环境（因为Rollup好用就是因为它提出的TreeShacking）
使用esbuild 作为开发时的打包器，因为它是用GO语言写的所以速度特别快(直接将代码编译为native),比基于 JavaScript 的打包器快 10-100 倍的预打包依赖项
为什么不适用esbuild作为生产环境的打包器呢？因为esbuild对代码分割和css处理还有待加强。
简单说一下vite为什么那么快，利用http标头，对依赖项和源代码模块做了不同的缓存处理。还有一点是Native ESM based dev server 和 Bundle based dev server 的区别，vite先启动开发服务器，再根据浏览器请求按需加载对应模块，而Bundle based dev server是一旦有source改动，就会重新打包所有代码，显然效率很低。



作者：速冻鱼
链接：https://juejin.cn/post/7035849836029018119
来源：稀土掘金
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
# package.json

## package-lock.json 文件的结构


package-lock.json 文件中的 name、version 与 package.json 中的 name、version 一样，描述了当前包的名字和版本，dependencies 是一个对象，该对象和 node_modules 中的包结构一一对应，对象的 key 为包的名称，值为包的一些描述信息， 根据 package-lock-json官方文档 ，主要的结构如下：



version ：包版本，即这个包当前安装在 node_modules 中的版本

resolved ：包具体的安装来源

integrity ：包 hash 值，验证已安装的软件包是否被改动过、是否已失效

requires ：对应子依赖的依赖，与子依赖的 package.json 中 dependencies 的依赖项相同

dependencies ：结构和外层的 dependencies 结构相同，存储安装在子依赖 node_modules 中的依赖包



**需要注意的是，并不是所有的子依赖都有 dependencies 属性，只有子依赖的依赖和当前已安装在根目录的 node_modules 中的依赖冲突之后，才会有这个属性。**
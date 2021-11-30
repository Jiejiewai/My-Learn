css的继承

```
.base{
	color: 'red';
}
.text{
	composes: base;
    background: 'blackpink';
}

// 还支持引入别的文件中的css module

// index.css
.base{
	color: 'red';
}

// another.css
.text{
	composes: base from './index.css';
    background: 'blackpink';
}
```
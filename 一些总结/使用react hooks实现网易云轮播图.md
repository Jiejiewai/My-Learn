参考了这篇博客:
css部分也大部分使用的这篇博文里的样式
[JS轮播图（网易云轮播图）](https://blog.csdn.net/m0_50855872/article/details/113739931)

最终效果：
![在这里插入图片描述](https://img-blog.csdnimg.cn/ba1b244f1b9e4d088c3dafea1fab6a4f.png?x-oss-process=image/watermark,type_ZmFuZ3poZW5naGVpdGk,shadow_10,text_aHR0cHM6Ly9ibG9nLmNzZG4ubmV0L3dlaXhpbl80Mzk2ODM5Mg==,size_16,color_FFFFFF,t_70)

只写了自动轮播，没有写点击左右移动

使用的方法是用数组来循环改变轮播图的css属性

JSX部分
```javascript
import React, { useEffect,useState,useRef } from 'react';
import './index.less'
import banner1 from '@assets/static/banner1.jpg';
import banner2 from '@assets/static/banner2.jpg';
import banner3 from '@assets/static/banner3.jpg';
import banner4 from '@assets/static/banner4.jpg';


const EmoCarousel = () => {
  const clsRef = useRef(['one','two','three','four'])
  const dotsRef = useRef(['change','','',''])

  const [dots,setDots]  = useState([''])
  const [cls,setCls]= useState([''])

  useEffect(()=> {
    setCls([...clsRef.current])
    setDots([...dotsRef.current])
    const time = setInterval(()=>{
      const clsTmp = [...clsRef.current]
      const dotsTmp = [...dotsRef.current]
      let tmp = String(clsTmp.pop())
      clsTmp.unshift(tmp)
      let dotTmp = String(dotsTmp.pop())
      dotsTmp.unshift(dotTmp)
      setCls(clsTmp)
      setDots(dotsTmp)
      clsRef.current = clsTmp
      dotsRef.current = dotsTmp
    },3000)
    return () => clearInterval(time)
  },[])

  return (
    <div className="box">
      <ul className='imgs'>
       <li className={cls[0]}>
        <img src={banner1} />
       </li>
        <li className={cls[1]}>
          <img src={banner2} />
        </li>
        <li className={cls[2]}>
          <img src={banner3} />
        </li>
        <li className={cls[3]}>
          <img src={banner4} />
        </li>
      </ul>

      <ul className="list">
        <li className={dots[0]}></li>
        <li className={dots[1]}></li>
        <li className={dots[2]}></li> 
        <li className={dots[3]}></li>
      </ul> 
    </div>
  )
}

export default EmoCarousel;
```
CSS部分

```css

li {
   list-style-type: none;
 }
 img {
   width: 100%;
  //  border-radius: 20px;
   box-shadow: 5px 5px 5px rgba(0,0,0,0.2);
 }
 .box {
   position: relative;
   width: 958px;
   height: 284px;
   /* 居中 */
   top: -30px;
   left: 50%;
   transform: translate(-50%,10%);
 } 
 .imgs {
   position: absolute;
   width: 375px;
   height: 284px;
   top: 0;
   left: 50%;
   transform: translate(-50%,0%);
 }
 .imgs li {
   position: absolute;
   width: 375px;
   transition: 0.5s;
 }
 .imgs .one {
   transform: translateX(-300px) scale(0.9);
   z-index: 1;
   opacity: .8;
 }
 .imgs .two {
   width: 375px;
   z-index: 2;
 }
 .imgs .three {
   transform: translateX(300px) scale(0.9);
   z-index: 1;
   opacity: .8;
 }
 .imgs .four{
  z-index: 0;
  opacity: .8;
  transform: scale(0.9);
 }
 .list {
   position: absolute;
   top:210px;
   bottom: -25px;
   left: 45%;
   z-index: 777;
 }
 .list li {
   float: left;
   width: 7px;
   height: 7px;
   background-color: rgb(230, 230, 230);
   border-radius: 50%;
   margin: 0 6px;
   cursor: pointer; 
 }
 /* 小圆圈改变后的样式 */
 .list .change {
   background-color: rgb(236, 65, 65);
 }

```

==尝试过使用antd 的react-slick，失败了
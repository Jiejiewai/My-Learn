 fn main() {
    let str = String::from("hello world");
    // slice_func();
    let word_index = first_word(&str);
    let word_index_rewrite = first_word_rewrite(&str);
    // str.clear();
    //cannot borrow `str` as mutable because it is also borrowed as immutable
    println!("{}---{}---{}", word_index, str,word_index_rewrite);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // 转换为字节数组
    // println!("{:?}",bytes);
    // enumerate 对元素进行包装，返回(index, item),index为元素位置，item为元素本身
    for item in bytes.iter().enumerate(){
        // println!("{:?}", item);
        let &item_temp = item.1;
        if item_temp == b'w' {
            return item.0;
        }
    }
    s.len()
}

fn slice_func() {
    let s = String::from("hello rust");
    let hello = &s[0..5];
    let rust = &s[7..11];
    let _hello = &s[0..5];
    let _rust = &s[7..11];
    let all = &s[..];
    println!("hello:{}--rust:{}",hello, rust);
    println!("_hello:{}--_rust:{}",_hello, _rust);
    println!("all->{}",all);
}
/*
 * 注意：
 * 1. 字符串切片的范围索引必须发生在有效的UTF-8字符边界内
 * 2. 如果尝试从一个多字节的字符中创建字符串切片，程序会报错并退出
*/

// &str 表示返回字符串切片
fn first_word_rewrite(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

/* 有经验的Rust开发者会采用&str作为参数类型，因为这样就可以同时接收String和&str类型的参数了
*  定义函数时西永字符串切片来代替字符串引用会使我们的API更加通用，且不会损失任何功能
*/

fn _main(){
    let my_string = String::from("hello world");
    let word_index = _first_word(&my_string[..]);

    let my_string_literal = "hello world";
    let word_index_literal = _first_word(my_string_literal);

    println!("{},{}",word_index,word_index_literal);
}

fn _first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn other_demo(){
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
}
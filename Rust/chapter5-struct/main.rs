// 定义struct
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,// 即使是最后一组字段，后面也要有逗号,为了方便删减元素
}

fn main() {
    let mut user1 = User {
        email: String::from("abc@163.com"),
        username: String::from("Wang Clever"),
        active: true,
        sign_in_count: 556,
    };
    // 不可以缺少字段
    // 要给里面的字段赋新的值,user1 得可变，使用mut
    user1.email = String::from("fakeEmail@163.com");
    // 一旦struct的实例是可变的，那么实例中所有的字段都是可变的
}

// struct可以作为函数的返回值
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 556,
    }
}
// 可以简写
fn _build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 556,
    }
}
// struct更新语法
fn anotherStruct () {
    let mut user1 = User {
        email: String::from("abc@163.com"),
        username: String::from("Wang Clever"),
        active: true,
        sign_in_count: 556,
    };
    let user2 = User{
        email: String::from("anotherFake@163.com"),
        username: String::from("Wang_Clever"),
        ..user1
    };
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32,i32,i32);

fn tuple_struct(){
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    // black 和 origin是不同的类型，是不同的tuple struct的实例
}
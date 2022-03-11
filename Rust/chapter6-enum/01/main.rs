enum IpAddrKind{
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self){
        println!("nihao");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    route(IpAddrKind::V6);

    let q = Message::Quit;
    let m = Message::Move{x:12,y:24};
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0,255,255);
}

fn route(ip_kind: IpAddrKind){

}

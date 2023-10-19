///
/// 使用枚举类来存储不同类型数据(结构体)以及处理,使得数组/函数可以接受不同类型的数据作为参数
/// 

#[derive(Debug)]
enum Message {
    MsgEmpty,
    MsgStruct {x: u8, y: u8 },
    MsgStr (String),
    MsgTuple (u8,String),
}

fn analyse_message(msg: &Message) {
    match msg {
        Message::MsgStr(content) => {
            println!("{}",content)
        }
        Message::MsgStruct { x, y } => {
            println!("{},{}",x,y)
        }
        Message::MsgEmpty => {
            println!("message is emapty")
        }
        _ =>{}
    }
}

fn main() {
    let mut vec_msg: Vec<Message> = Vec::new();

    vec_msg.push(Message::MsgStr("a message".to_string()));
    vec_msg.push(Message::MsgStruct{x:1,y:2});
    vec_msg.push(Message::MsgEmpty);

    for msg in vec_msg.iter(){
        analyse_message(&msg);
    }
}
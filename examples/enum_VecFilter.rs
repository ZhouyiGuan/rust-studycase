///
/// 使用过滤器获取特定条件的枚举类数组
/// 

#[derive(Debug,PartialEq)]
enum Message {
    MsgEmpty,
    MsgStruct {x: u8, y: u8 },
    MsgStr (String),
    MsgTuple (u8,String),
}

fn main() {
    let mut vec_msg: Vec<Message> = Vec::new();

    vec_msg.push(Message::MsgStr("message 1".to_string()));
    vec_msg.push(Message::MsgStruct{x:1,y:2});
    vec_msg.push(Message::MsgEmpty);
    vec_msg.push(Message::MsgStr("message 2".to_string()));

    // 将会得到一个迭代器,会指向包含所有符合条件的元素的集合(迭代器类型引用)
    let iter_msg = vec_msg.iter().filter(|x| matches!(x, Message::MsgStr(_)));
    // assert_eq!(iter_msg.next(), Some(&Message::MsgStr("a message".to_string())));
    // assert_eq!(iter_msg.next(),None);

    for iter in iter_msg{
        if let Message::MsgStr(x) = iter {
            println!("{}",x);
        }
    }
}
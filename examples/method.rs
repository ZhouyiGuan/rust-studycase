///
/// 规范:结构体成员在外部不可直接访问 
/// 由于方法支持成员和成员函数同名,可以通过method来访问私有成员
/// 
use rust_studycase::mod_method::Rectangle;


fn main() {

    
    let mut rec: Rectangle = Rectangle::new("rec1".to_string(), 10, 20);
    println!("{}",rec.name());

    // 以下两种方式都会报错:
    let mut rec1: Rectangle = Rectangle{name: "rec1".to_string(),width: 10,height: 20};
    println!("{}",rec.name);

}
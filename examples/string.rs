use mio::unix::pipe::new;

fn main(){
    let s: &str = "a中";
    println!("{:?}",&s[1..4]);
}
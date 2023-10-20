///
/// 模式匹配中不仅对对象进行条件筛选同时还获取参数
/// 说实话没什么用 为什么不直接访问对象呢
/// 

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let mut vec_points: Vec<Point> = Vec::new();
    vec_points.push(Point{x:10,y:20});
    vec_points.push(Point{x:20,y:30});

    // 对point进行模式匹配的同时获取到其中的参数
    for point in &vec_points{
        if let p @ Point { x:10, y } = point {
            println!("this point has x = 10, with y = {}, p = {:?}",y,p);
        } 
    }
}
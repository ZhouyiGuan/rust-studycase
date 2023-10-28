mod global_static;
mod connect_operations;
pub mod load_config;


#[tokio::main]
pub async fn serialport_main() -> Result<(), Box<dyn std::error::Error>> {
    
    let future_async_fn = async_fn_1();

    println!("from main");

    future_async_fn.await;

    Ok(())
}

async fn async_fn_1() {
    println!("from async_fn_1");
}
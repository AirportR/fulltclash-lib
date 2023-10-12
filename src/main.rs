use std::io;
use std::io::{Read, Write};
use myspeed::speed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    speed_test().await.expect("TODO: panic message");
    Ok(())
}
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "按任意键退出...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
fn get_proxy_port() -> u32{
    println!("请输入socks5代理端口号以进行代理测速: ");
    let mut port = String::new();
    io::stdin().read_line(&mut port).expect("读取输入失败");
    let port: u32 = port.trim().parse().expect("转换整数失败");
    port
}

async fn speed_test() -> Result<(), Box<dyn std::error::Error>>{
    println!("测速即将开始");
    let port = get_proxy_port();
    println!("开始连接{}端口测速，占用10s带宽，请耐心等待...", port);
    speed(String::from("127.0.0.1"), port, vec![String::from("https://dl.google.com/dl/android/studio/install/3.4.1.0/android-studio-ide-183.5522156-windows.exe")], 4).await;
    println!("测速已结束");
    pause();
    Ok(())
}
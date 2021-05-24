// 引用需要的依赖包
use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn main() {
    // 定义一个地址
    let address = "127.0.0.1:7878";
    // 创建一个 tcp listener，绑定本机回环网卡端口7878
    let temp_listener = TcpListener::bind(address);

    // 这里temp_listener接受到的是一个Result的枚举类型，使用match模式匹配处理错误。
    match temp_listener {
        // 正确的分支
        Ok(listener) => {
            // 打印一下
            println!("start tcp server at {}",address);
            // 使用for 循环接受tcp中的stream(字节流)
            for stream in listener.incoming() {
                // 因为这里stream也是Result类型 使用unwarp()方法粗糙处理一下，有错误会panic,
                // 更详细点可以使用match模式匹配或者 if let 语法处理err
                let stream = stream.unwrap();
                // 处理stream的过程
                handle_stream(stream);
            }
        }
        // 错误的分支，打印错误
        Err(err) => {
            println!("创建tcp server 失败！detail: {}", err)
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    // 申请一块512个字节缓冲区，这里因为要吧stream中的内容读入buffer，使用mut关键字
    let mut buffer = [0; 512];
    // 将stream中的内容读入buffer,
    // 同样使用unwarp(),快速处理一下，有错误会panic，更详细点可以使用match模式匹配或者 if let 语法处理err
    stream.read(&mut buffer).unwrap();
    // 打印buffer内容
    println!("Request:\n {}", String::from_utf8_lossy(&buffer[..]))
}

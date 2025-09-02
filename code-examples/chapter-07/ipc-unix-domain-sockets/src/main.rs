#[cfg(unix)]
fn main() -> std::io::Result<()> {
    use std::io::{self, prelude::*, BufReader};
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::thread;
    use std::fs; // For fs::remove_file

    println!("--- IPC Unix Domain Sockets Example ---");

    // ソケットのパスを定義する。
    let socket_path = "/tmp/rust-uds-example.sock";

    // 古いソケットファイルがあればクリーンアップする。
    let _ = fs::remove_file(socket_path);

    // サーバーのスレッドを開始する。
    let server_handle = thread::spawn(move || -> io::Result<()> {
        let listener = UnixListener::bind(socket_path)?;
        println!("サーバーが {} でリッスン中", socket_path);

        // 単一の接続を受け入れる。
        let (mut stream, addr) = listener.accept()?;
        println!("サーバーが接続を受け入れました: {:?}", addr);

        let mut buffer = String::new();
        let mut reader = BufReader::new(stream.try_clone()?);
        reader.read_line(&mut buffer)?; // 改行まで読み込む
        println!("サーバーが受信: {}", buffer.trim());

        // エコーバック
        stream.write_all(b"Hello from server!\n")?;
        Ok(())
    });

    // サーバーが起動するまで少し待つ。
    thread::sleep(std::time::Duration::from_millis(100));

    // クライアントを開始する。
    let mut stream = UnixStream::connect(socket_path)?;
    println!("クライアントが {} に接続しました", socket_path);

    stream.write_all(b"Hello from client!\n")?;
    println!("クライアントがメッセージを送信しました。");

    let mut buffer = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut buffer)?;
    println!("クライアントが受信: {}", buffer.trim());

    // サーバーのスレッドが終了するのを待つ。
    server_handle.join().unwrap()?;

    // ソケットファイルをクリーンアップする。
    let _ = fs::remove_file(socket_path);

    println!("\n例が終了しました。");
    Ok(())
}

#[cfg(not(unix))]
fn main() {
    println!("この例はUnix系システムでのみ動作します。");
    println!("Unixドメインソケットの例をスキップします。");
}

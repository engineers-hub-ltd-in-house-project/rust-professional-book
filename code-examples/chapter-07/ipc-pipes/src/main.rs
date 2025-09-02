use std::io::{self, Write, Read};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    println!("--- IPC パイプの例 ---");

    // 1. 子プロセスを作成し、その標準入力を標準出力にエコーバックさせる。
    // 標準入力と標準出力をパイプで接続する。
    let mut child = Command::new("cat") // `cat`は単に入力をエコーバックする
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // 子プロセスの標準入力と標準出力のハンドルを取得する。
    // `unwrap()`は`Stdio::piped()`を設定しているため安全。
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    // 2. 子プロセスの標準入力にデータを書き込む。
    let message = b"Parent processからのメッセージ (パイプ経由)！\n";
    println!("親プロセスが子プロセスに書き込み中: {:?}", String::from_utf8_lossy(message));
    stdin.write_all(message)?;
    
    // `cat`が終了し、標準出力をフラッシュするように、stdinをドロップしてEOFを子プロセスに通知することが重要。
    drop(stdin); 

    // 3. 子プロセスの標準出力からデータを読み取る。
    let mut buffer = String::new();
    stdout.read_to_string(&mut buffer)?;
    println!("子プロセスがエコーバック: {:?}", buffer.trim());

    // 4. 子プロセスが終了するのを待つ。
    let status = child.wait()?;
    println!("子プロセスがステータス: {} で終了しました", status);

    Ok(())
}

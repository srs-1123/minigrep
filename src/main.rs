extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // collect: 引数をベクタに変換
    let args: Vec<String> = env::args().collect();

    // プログラム名がベクタの最初の値、args[0]を占めている
    // unwrap_or_else: Errの中身をerr引数のクロージャに渡している
    // env::args(): イテレータを返す
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // 引数解析に問題
        eprintln!("Problem parsing arguments: {}", err); // 標準エラー出力に出力
        process::exit(1);
    });

    // {}を探しています
    println!("Searching for {}", config.query);
    // {}というファイルの中
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
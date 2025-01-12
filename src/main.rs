use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // collect: 引数をベクタに変換
    let args: Vec<String> = env::args().collect();

    // プログラム名がベクタの最初の値、args[0]を占めている
    let query = &args[1];
    let filename = &args[2];

    // {}を探しています
    println!("Searching for {}", query);
    // {}というファイルの中
    println!("In file {}", filename);

    // ファイルを開く
    // expect: 例外処理
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");
    
    // テキストは/n{}です
    println!("with text:/n{}", contents);
}
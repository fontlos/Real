use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str(){
        "windows"|"macos" => desktop(),
        "android"|"ios" => mobile(),
        "wasm" => wasm(),
        _=>()
    }
}

fn desktop(){

}

fn mobile(){

}

fn wasm(){

}
use aten_engine::engine::Engine;

fn main() {

    let res = Engine::run();
    if res.is_err() {
        println!("Error: {:?}", res.err().unwrap());
    }
    println!("Application exiting");

}
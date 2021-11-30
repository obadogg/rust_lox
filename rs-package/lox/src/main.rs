fn main() {
    let now = std::time::Instant::now();

    let data = std::fs::read_to_string("test.lox").unwrap();
    lox_compiler::interpret(&data);

    println!("耗时：{:?}", now.elapsed());
}

fn gen_it(name: &str, rawabi: &str, filename: &str) {
    //let counter = include_str!("./counter.json");
    let abigen = revmup_gen::Abigen::new(name, rawabi).unwrap();
    let gen = abigen.generate().unwrap();
    //println!("{:}", gen);
    gen.write_to_file(format!("./examples/basics/src/{}.rs", filename))
        .expect("write to file");
}

fn main() {
    let counter = include_str!("../../../examples/counter.json");
    gen_it("Counter", counter, "counter");

    let erc20 = include_str!("../../../examples/erc20.json");
    gen_it("MockErc20", erc20, "erc20");
}

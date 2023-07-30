fn gen_it(name: &str, rawabi: &str, filename: &str) {
    let abigen = revmup_gen::Abigen::new(name, rawabi).unwrap();
    let gen = abigen.generate().unwrap();
    gen.write_to_file(format!("./examples/basics/src/{}.rs", filename))
        .expect("write to file");
}

fn main() {
    let erc20 = include_str!("../../../examples/erc20.json");
    gen_it("MockErc20", erc20, "erc20");
}

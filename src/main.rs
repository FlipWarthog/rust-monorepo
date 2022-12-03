use rust_monorepo::get_vins;

fn main() {
    for v in get_vins() {
        println!("Vin is {}", v);
    }
}

use nih_plug::util;

fn main() {
    let min = util::db_to_gain(-36.0);
    let max = util::db_to_gain(0.0);

    println!("{} | {}", min, max);
}

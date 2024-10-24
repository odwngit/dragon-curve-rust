mod dragonhelper;

fn main() {
    let iterations: i32 = 5;

    let mut turns: Vec<bool> = vec![true]; // 1 is a right turn, 0 is a left turn
    for _ in 0..iterations {
        dragonhelper::iterate_curve(&mut turns);
    }

    for turn in turns {
        println!("{turn}");
    }
}



fn iterate_curve(turns: &mut Vec<u8>) {
    let mut appendage: Vec<u8> = vec![1];
    turns.append(&mut appendage);
}

fn main() {
    let mut turns: Vec<u8> = vec![1]; // 1 is a right turn, 0 is a left turn
    iterate_curve(&mut turns);

    for turn in turns {
        print!("{turn}");
    }
}

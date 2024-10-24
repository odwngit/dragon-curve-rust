fn invert_turns(turns: &mut Vec<bool>) {
    for turn in turns {
        match turn { // Match cases and panic if turn isn't 1 or 0
            false => *turn = true, // * dereferences, changing the original
            true => *turn = false,
        }
    }
}

pub fn iterate_curve(turns: &mut Vec<bool>) {
    let mut appendage: Vec<bool> = turns.clone();
    appendage.reverse();
    invert_turns(&mut appendage);

    turns.push(true);
    turns.append(&mut appendage);
}
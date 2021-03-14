use rand::prelude::*;

pub fn shuffle<I>(to_shuffle: &mut Vec<I>) {
    let mut rng = rand::thread_rng();
    let length = to_shuffle.len();
    let mut current_pos = 0;
    

    while current_pos < to_shuffle.len() {
        let mut temp: I;
        let mut current: I;
        let temp_pos = rng.gen_range(0..length-1);

        current = to_shuffle.remove(current_pos);
        temp = to_shuffle.remove(temp_pos);
        
        to_shuffle.insert(temp_pos, current);
        to_shuffle.insert(current_pos, temp);

        current_pos += 2;
    }
}
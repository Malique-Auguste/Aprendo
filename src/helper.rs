use rand::prelude::*;

pub fn shuffle<I>(to_shuffle: &mut Vec<I>) {
    let mut rng = rand::thread_rng();
    let length = to_shuffle.len();
    let mut current_pos = 0;

    while current_pos < to_shuffle.len() {
        let temp_pos = rng.gen_range(0..length - 1);

        let current = to_shuffle.remove(current_pos);
        let temp = to_shuffle.remove(temp_pos);

        to_shuffle.insert(temp_pos, current);
        to_shuffle.insert(current_pos, temp);

        current_pos += 2;
    }
}

pub fn get_rand_unique_indices(collection_len: usize, number: usize, except: Option<usize>) -> Result<Vec<usize>, String> {
    if number > collection_len {
        return Err("The number of indicies must not be breater than the collection length.".into())
    }

    

    let mut indices: Vec<usize> = (0..collection_len).collect();

    shuffle(&mut indices);

    if let Some(u) = except {
        indices.remove(u);
        indices = indices.drain(0..number - 1).collect();
        indices.insert(rand::thread_rng().gen_range(0..indices.len()), u);
        return Ok(indices)
    }
    else {
        return Ok(indices.drain(0..number).collect())
    }
}

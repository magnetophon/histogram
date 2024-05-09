fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

// Function to find the bin for a given linear audio value
fn find_bin(value: f32) -> u8 {
    const DB_MIN: f32 = -96.0;
    const DB_MAX: f32 = 24.0;
    const NR_EDGES: usize = 255;
    // calculate the linear edges from DB_MIN to DB_MAX, evenly spaced in the db domain
    let edges: [f32; NR_EDGES] = (0..=NR_EDGES-1)
        .map(|x| db_to_linear(DB_MIN+ x as f32 *((DB_MAX-DB_MIN)/(NR_EDGES as f32 -1.0)))).collect::<Vec<_>>().try_into().unwrap();


    // Check if the value is smaller than the first edge
    if value < edges[0] {
        return 0;
    }

    // Check if the value is larger than the last edge
    if value > *edges.last().unwrap() {
        return edges.len() as u8;
    }
    // Binary search to find the bin for the given value
    let mut left = 0;
    let mut right = edges.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if value >= edges[mid] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    // Return the bin index
    left as u8 
}

fn main() {
    let nr_tests = 512;
    let db_min = -100.0;
    let db_max = 25.0;

    // Example usage
    for x in 0..nr_tests {
        let db_value = db_min+ x as f32 *((db_max-db_min)/(nr_tests as f32 -1.0));
        println!("Value {} falls into bin {}", db_value, find_bin(db_to_linear(db_value)));
    }
}

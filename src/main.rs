const DB_MIN: f32 = -96.0;
const DB_MAX: f32 = 24.0;
const NR_BINS: usize = 255;

fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

// Function to find the bin for a given linear audio value
fn find_bin(value: f32) -> usize {
    // calculate the linear edges from DB_MIN to DB_MAX, evenly spaced in the db domain
    const NR_EDGES: usize = NR_BINS - 1;
    let edges: [f32; NR_EDGES] = (0..NR_EDGES)
        .map(|x| db_to_linear(DB_MIN + x as f32 * ((DB_MAX - DB_MIN) / (NR_EDGES as f32 - 1.0))))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    // Check if the value is smaller than the first edge
    if value < edges[0] {
        return 0;
    }
    // Check if the value is larger than the last edge
    if value > *edges.last().unwrap() {
        return edges.len() as usize;
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
    left as usize
}


struct XorShiftRng {
    state: u64,
}
// Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
//
impl XorShiftRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> f32 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        (self.state & 0x7fffffff as u64) as f32 % 33.33 / 33.33
    }
}
fn main() {
    let nr_tests = 4*48000; // sec * SR
        let db_min = DB_MIN - 1.0;
    let db_max = DB_MAX + 1.0;

    let mut bins: [f32; NR_BINS] = [0.0; NR_BINS];

    let mut rng = XorShiftRng::new(1); // Initialize with a seed

    for _ in 0..nr_tests {
        let db_value = db_min + rng.next() * (db_max - db_min);
        let bin_index = find_bin(db_to_linear(db_value));
        bins[bin_index] += 1.0; // Increment the count for the bin
    }

    // Normalize the counts to get the relative frequencies
    let total_count = bins.iter().sum::<f32>();
    for i in 0..NR_BINS {
        bins[i] /= total_count;
    }

    for i in 0..NR_BINS {
        println!("index: {},  value: {}", i, bins[i]);
    }
    println!("total count: {}", total_count)

}

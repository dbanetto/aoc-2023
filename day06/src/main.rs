use std::ops::Range;

fn main() {
    let records: Vec<(u64,u64)> = vec![
        // time, distance
        // (52, 426),
        // (94, 1374),
        // (75, 1279),
        // (94, 1216),
        (52947594,  426137412791216),
    ];

    let mut sums: u64 = 1;

    for (t_max, distance) in records {
        let mut count = 0;

        for min in 0..t_max {
            let d_steady = min * (t_max - min);

            let d = d_steady;

            if d >= distance {
                count += 1;
            }
        }

        sums *= count;
    }

    println!("Sums {}", sums);
}

// t * (n)
// t=0 n=0 d=0;
//
// Accerlation: 1/2 * (t * t); s=t
// Flat: s * (t_max - s)
// (s * s)/2 + (s * (t_max - s)) = d

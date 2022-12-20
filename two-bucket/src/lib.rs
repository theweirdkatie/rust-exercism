use std::cmp;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let (capacities, mut buckets) = ([capacity_1, capacity_2], [0,0]);
    let mut moves = 1;
    let from_bucket = *start_bucket as usize;
    buckets[from_bucket] = capacities[from_bucket];

    while buckets[0] != goal && buckets[1] != goal {
        let to_bucket = from_bucket ^ 1;

        if buckets[from_bucket] == 0 {
            buckets[from_bucket] = capacities[from_bucket];
        } else if buckets[to_bucket] == capacities[to_bucket] {
            buckets[to_bucket] = 0;
        } else if buckets[to_bucket] == 0 && capacities[to_bucket] == goal {
            buckets[to_bucket] = capacities[to_bucket];
        } else {
            let temp = cmp::min(
                buckets[from_bucket],
                capacities[to_bucket] - buckets[to_bucket],
            );
            buckets[to_bucket] += temp;
            buckets[from_bucket] -= temp;
        }
        moves += 1;

        if moves >= 255 {
            return None;
        }
    }
    let (goal_bucket, other_bucket) = if buckets[0] == goal {
        (Bucket::One, buckets[1])
    } else {
        (Bucket::Two, buckets[0])
    };

    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    })
}

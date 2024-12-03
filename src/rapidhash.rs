pub const RAPID_SEED: u64 = 0xbdd8_9aa9_8270_4029;
pub const RAPID_SECRET: [u64; 3] = [
    0x2d35_8dcc_aa6c_78a5,
    0x8bb8_4b93_962e_acc9,
    0x4b33_a62e_d433_d4a3,
];

#[inline]
fn rapid_mum(a: &mut u64, b: &mut u64) {
    let r = (*a as u128).wrapping_mul(*b as u128);
    *a = r as u64;
    *b = (r >> 64) as u64;
}

#[inline]
fn rapid_mix(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    rapid_mum(&mut a, &mut b);
    a ^ b
}

#[inline]
fn rapid_read64(p: &[u8]) -> u64 {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&p[..8]);
    u64::from_le_bytes(buf)
}

#[inline]
fn rapid_read32(p: &[u8]) -> u32 {
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&p[..4]);
    u32::from_le_bytes(buf)
}

#[inline]
fn rapid_read_small(p: &[u8], k: usize) -> u64 {
    ((p[0] as u64) << 56) | ((p[k >> 1] as u64) << 32) | (p[k - 1] as u64)
}

pub fn rapidhash(key: &[u8], seed: u64, secret: &[u64; 3]) -> u64 {
    let len = key.len();
    let mut seed = seed ^ rapid_mix(seed ^ secret[0], secret[1]) ^ len as u64;
    let mut a;
    let mut b;

    if len <= 16 {
        if len >= 4 {
            let delta = ((len & 24) >> (len >> 3)) as usize;
            let p0 = rapid_read32(&key[0..]);
            let p1 = rapid_read32(&key[len - 4..]);
            a = ((p0 as u64) << 32) | (p1 as u64);
            let p2 = rapid_read32(&key[delta..]);
            let p3 = rapid_read32(&key[len - 4 - delta..]);
            b = ((p2 as u64) << 32) | (p3 as u64);
        } else if len > 0 {
            a = rapid_read_small(key, len);
            b = 0;
        } else {
            a = 0;
            b = 0;
        }
    } else {
        let mut p = key;
        let mut i = len;
        if i > 48 {
            let mut see1 = seed;
            let mut see2 = seed;
            while i >= 48 {
                seed = rapid_mix(
                    rapid_read64(&p[0..]) ^ secret[0],
                    rapid_read64(&p[8..]) ^ seed,
                );
                see1 = rapid_mix(
                    rapid_read64(&p[16..]) ^ secret[1],
                    rapid_read64(&p[24..]) ^ see1,
                );
                see2 = rapid_mix(
                    rapid_read64(&p[32..]) ^ secret[2],
                    rapid_read64(&p[40..]) ^ see2,
                );
                p = &p[48..];
                i -= 48;
            }
            seed ^= see1 ^ see2;
        }
        if i > 16 {
            seed = rapid_mix(
                rapid_read64(&p[0..]) ^ secret[2],
                rapid_read64(&p[8..]) ^ seed ^ secret[1],
            );
            if i > 32 {
                seed = rapid_mix(
                    rapid_read64(&p[16..]) ^ secret[2],
                    rapid_read64(&p[24..]) ^ seed,
                );
            }
        }
        a = rapid_read64(&key[len - 16..]);
        b = rapid_read64(&key[len - 8..]);
    }

    a ^= secret[1];
    b ^= seed;
    rapid_mum(&mut a, &mut b);
    rapid_mix(a ^ secret[0] ^ len as u64, b ^ secret[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;
    use statrs::distribution::{ChiSquared, ContinuousCDF};

    struct TestCase {
        key: String,
        seed: Option<u64>,
        gt: u64,
    }

    #[test]
    fn test_correct_cases() {
        let test_cases = [
            TestCase {
                key: "hello world".to_string(),
                seed: None,
                gt: 17498481775468162579,
            },
            TestCase {
                key: "hello world".to_string(),
                seed: Some(0),
                gt: 6388527444622164108,
            },
            TestCase {
                key: "hello,".to_string(),
                seed: Some(0),
                gt: 17861179120578160190,
            },
            TestCase {
                key: "hello, world!".to_string(),
                seed: Some(1),
                gt: 4668653575921246457,
            },
            TestCase {
                key: "Hello, world!".to_string(),
                seed: Some(1),
                gt: 7739271034020981250,
            },
            TestCase {
                key: "hello world! ".to_string(),
                seed: Some(2),
                gt: 10327466050248778708,
            },
        ];

        for test_case in test_cases.iter() {
            let key = test_case.key.as_bytes();
            let seed = test_case.seed.unwrap_or(RAPID_SEED);
            let hash = rapidhash(key, seed, &RAPID_SECRET);
            assert_eq!(hash, test_case.gt);
        }
    }

    #[test]
    fn test_long_text() {
        let test_cases = [
            TestCase {
                key: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
                eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, \
                quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
                Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat \
                nulla pariatur. Excepteur sint occaecat cupidatat non proident, \
                sunt in culpa qui officia deserunt mollit anim id est laborum."
                    .to_string(),
                seed: None,
                gt: 1221157664313218070,
            },
            TestCase {
                key: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
                eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, \
                quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
                Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat \
                nulla pariatur. Excepteur sint occaecat cupidatat non proident, \
                sunt in culpa qui officia deserunt mollit anim id est laborum."
                    .to_string(),
                seed: Some(1),
                gt: 805527774265126985,
            },
            TestCase {
                key: "Sed ut perspiciatis, unde omnis iste natus error sit voluptatem accusantium \
                doloremque laudantium, totam rem aperiam eaque ipsa, quae ab illo inventore \
                veritatis et quasi architecto beatae vitae dicta sunt, explicabo. Nemo enim ipsam \
                voluptatem, quia voluptas sit, aspernatur aut odit aut fugit, sed quia consequuntur \
                magni dolores eos, qui ratione voluptatem sequi nesciunt, neque porro quisquam est, \
                qui dolorem ipsum, quia dolor sit, amet, consectetur, adipisci velit, sed quia non \
                numquam eius modi tempora incidunt, ut labore et dolore magnam aliquam quaerat \
                voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis \
                suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum \
                iure reprehenderit, qui in ea voluptate velit esse, quam nihil molestiae \
                consequatur, vel illum, qui dolorem eum fugiat, quo voluptas nulla pariatur? At \
                vero eos et accusamus et iusto odio dignissimos ducimus, qui blanditiis praesentium \
                voluptatum deleniti atque corrupti, quos dolores et quas molestias excepturi sint, \
                obcaecati cupiditate non provident, similique sunt in culpa, qui officia deserunt \
                mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est \
                et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio, \
                cumque nihil impedit, quo minus id, quod maxime placeat, facere possimus, omnis \
                voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut \
                officiis debitis aut rerum necessitatibus saepe eveniet, ut et voluptates \
                repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a \
                sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut \
                perferendis doloribus asperiores repellat.".to_string(),
                seed: None,
                gt: 16702286806359783625,
            }
        ];
        for test_case in test_cases.iter() {
            let key = test_case.key.as_bytes();
            let seed = test_case.seed.unwrap_or(RAPID_SEED);
            let hash = rapidhash(key, seed, &RAPID_SECRET);
            assert_eq!(hash, test_case.gt);
        }
    }

    fn chi_square_test(sample_size: usize, num_buckets: usize) -> f64 {
        let mut observed = vec![0usize; num_buckets];

        for i in 0..sample_size {
            let key = i.to_le_bytes();
            let hash = rapidhash(&key, RAPID_SEED, &RAPID_SECRET);
            let bucket = (hash % num_buckets as u64) as usize;
            observed[bucket] += 1;
        }

        let expected = sample_size as f64 / num_buckets as f64;
        let mut chi_square = 0.0;

        for &obs in &observed {
            let diff = obs as f64 - expected;
            chi_square += diff * diff / expected;
        }

        chi_square
    }

    fn chi_square_critical_value(confidence_level: f64, degrees_of_freedom: usize) -> f64 {
        let chi_squared = ChiSquared::new(degrees_of_freedom as f64).unwrap();
        chi_squared.inverse_cdf(confidence_level)
    }

    #[test]
    fn test_distribution() {
        let sample_size = 10000;
        let num_buckets = 256;
        let chi_square = chi_square_test(sample_size, num_buckets);
        let degrees_of_freedom = num_buckets - 1;
        // NOTE: This confidence level is confidence level of null hypothesis. Lower is better.
        //      Lower Confidence Level means higher probability to **NOT** reject null hypothesis,
        //      and less likely to have false positive.
        let confidence_level = 0.90;
        let critical_value = chi_square_critical_value(confidence_level, degrees_of_freedom);

        println!("Chi-square: {}", chi_square);
        println!("Critical value: {}", critical_value);

        assert!(
            chi_square < critical_value,
            "Chi-square test failed: chi_square = {}, critical_value = {}",
            chi_square,
            critical_value
        );
    }

    #[test]
    fn test_avalanche_effect() {
        const NUM_TESTS: usize = 1000;
        const INPUT_SIZE: usize = 16;
        const TOLERANCE: f64 = 0.1;

        let mut rng = rand::thread_rng();
        let mut total_diff_ratio = 0.0;

        for _ in 0..NUM_TESTS {
            let mut input = vec![0u8; INPUT_SIZE];
            rng.fill_bytes(&mut input);

            let original_hash = rapidhash(&input, RAPID_SEED, &RAPID_SECRET);

            for byte_idx in 0..INPUT_SIZE {
                for bit_idx in 0..8 {
                    let mut modified = input.clone();
                    modified[byte_idx] ^= 1 << bit_idx;

                    let modified_hash = rapidhash(&modified, RAPID_SEED, &RAPID_SECRET);

                    let diff_bits = (original_hash ^ modified_hash).count_ones();
                    let diff_ratio = diff_bits as f64 / 64.0;
                    total_diff_ratio += diff_ratio;
                }
            }
        }

        let total_bits = NUM_TESTS * INPUT_SIZE * 8;
        let avg_diff_ratio = total_diff_ratio / (total_bits as f64);

        println!(
            "Average bit difference ratio: {:.2}%",
            avg_diff_ratio * 100.0
        );

        assert!(
            (avg_diff_ratio - 0.5).abs() < TOLERANCE,
            "Avalanche effect test failed: average bit difference ratio = {:.2}%, expected close to 50%",
            avg_diff_ratio * 100.0
        );
    }

    #[test]
    fn test_collisions() {
        use std::collections::HashMap;

        const NUM_SAMPLES: usize = 10_000_000;
        const INPUT_SIZE: usize = 16;
        const MAX_ACCEPTABLE_COLLISION_RATE: f64 = 0.0001;

        let mut rng = rand::thread_rng();
        let mut hash_counts: HashMap<u64, u32> = HashMap::new();
        let mut collisions = 0;

        for _ in 0..NUM_SAMPLES {
            let mut input = vec![0u8; INPUT_SIZE];
            rng.fill_bytes(&mut input);

            let hash = rapidhash(&input, RAPID_SEED, &RAPID_SECRET);

            if let Some(count) = hash_counts.get_mut(&hash) {
                *count += 1;
                collisions += 1;
            } else {
                hash_counts.insert(hash, 1);
            }
        }

        let collision_rate = collisions as f64 / NUM_SAMPLES as f64;

        println!("Total samples: {}", NUM_SAMPLES);
        println!("Total collisions: {}", collisions);
        println!("Collision rate: {:.4}%", collision_rate * 100.0);

        assert!(
            collision_rate < MAX_ACCEPTABLE_COLLISION_RATE,
            "Collision rate too high: {:.4}% > {:.4}%",
            collision_rate * 100.0,
            MAX_ACCEPTABLE_COLLISION_RATE * 100.0
        );
    }

    #[test]
    fn benchmark() {
        use std::time::Instant;

        const NUM_SAMPLES: usize = 1_000_000;
        const INPUT_SIZE: usize = 4096;

        let mut rng = rand::thread_rng();
        let mut input = vec![0u8; INPUT_SIZE];
        rng.fill_bytes(&mut input);

        let start = Instant::now();
        for _ in 0..NUM_SAMPLES {
            let _ = rapidhash(&input, RAPID_SEED, &RAPID_SECRET);
        }
        let elapsed = start.elapsed();
        let elapsed_secs = elapsed.as_secs() as f64 + f64::from(elapsed.subsec_nanos()) * 1e-9;
        let hash_per_sec = NUM_SAMPLES as f64 / elapsed_secs;
        println!("Elapsed time: {:.3} seconds", elapsed_secs);
        println!("Hashes per second: {:.3}", hash_per_sec);
    }
}

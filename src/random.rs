#![deny(missing_docs)]

//! Generates random numbers.
//!
//! The random-number generator is based on [PCG, A Family of Better Random Number Generators](https://www.pcg-random.org).
//! The [minimal C implementation](https://www.pcg-random.org/download.html#id1) under [Apache License Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) is modified and used here.

/// Stores the state of the random number generator
pub struct Random {
    state: u64,
    inc: u64,
}

impl Random {
    /// Constructor.  
    ///   
    /// * `seed` - Random seed used as an input of the rng.
    pub fn new(seed: u64) -> Random {
        let mut random = Random {
            state: seed,
            inc: 0,
        };
        // get rid of the first 0
        random.gen_range(0., 0.);
        random
    }
    /// Returns a random number which is larger than `min` and smaller than `max`.
    ///   
    /// * `min` - Small limit.
    /// * `max` - Large limit.
    ///
    /// The original implementation is given here as well.
    ///
    /// ```c
    /// // *Really* minimal PCG32 code / (c) 2014 M.E. O'Neill / pcg-random.org
    /// // Licensed under Apache License 2.0 (NO WARRANTY, etc. see website)
    ///
    /// typedef struct { uint64_t state;  uint64_t inc; } pcg32_random_t;
    ///
    /// uint32_t pcg32_random_r(pcg32_random_t* rng)
    /// {
    ///     uint64_t oldstate = rng->state;
    ///     // Advance internal state
    ///     rng->state = oldstate * 6364136223846793005ULL + (rng->inc|1);
    ///     // Calculate output function (XSH RR), uses old state for max ILP
    ///     uint32_t xorshifted = ((oldstate >> 18u) ^ oldstate) >> 27u;
    ///     uint32_t rot = oldstate >> 59u;
    ///     return (xorshifted >> rot) | (xorshifted << ((-rot) & 31));
    /// }
    /// ```
    pub fn gen_range(&mut self, min: f64, max: f64) -> f64 {
        // PCG algorithm
        let old_state: u64 = self.state;
        self.state = old_state
            .wrapping_mul(6364136223846793005u64)
            .wrapping_add(self.inc | 1);
        let xor_shifted: u32 = (old_state.wrapping_shr(18) ^ old_state).wrapping_shr(27) as u32;
        let rot: u32 = old_state.wrapping_shr(59) as u32;
        let val: u32 =
            xor_shifted.wrapping_shr(rot) | xor_shifted.wrapping_shl((u32::MAX - rot) & 31);
        // cast it into f64
        min + (max - min) * (val as f64 / u32::MAX as f64)
    }
}

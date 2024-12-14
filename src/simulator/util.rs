use crate::simulator::NDIMS;

pub fn vec_to_array<T>(vector: Vec<T>) -> [T; NDIMS] {
    vector.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!(
            "invalid vector size: {}, which is expected to be identical to NDIMS: {}",
            v.len(),
            NDIMS
        )
    })
}

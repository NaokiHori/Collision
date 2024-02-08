use crate::simulator::NDIMS;

#[derive(Clone, Copy)]
pub struct MyVec {
    vec: [f64; NDIMS],
}

impl MyVec {
    pub fn new(vec: [f64; NDIMS]) -> MyVec {
        MyVec { vec }
    }
}

impl core::ops::Add for MyVec {
    type Output = MyVec;
    fn add(self, other: MyVec) -> MyVec {
        let mut result = MyVec { vec: [0.; NDIMS] };
        for dim in 0..NDIMS {
            result.vec[dim] = self.vec[dim] + other.vec[dim];
        }
        result
    }
}

impl core::ops::Sub for MyVec {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut result = MyVec { vec: [0.; NDIMS] };
        for dim in 0..NDIMS {
            result.vec[dim] = self.vec[dim] - other.vec[dim];
        }
        result
    }
}

impl core::ops::Mul<MyVec> for MyVec {
    type Output = f64;
    fn mul(self, other: MyVec) -> f64 {
        let mut result: f64 = 0.;
        for dim in 0..NDIMS {
            result += self.vec[dim] * other.vec[dim];
        }
        result
    }
}

impl core::ops::Mul<f64> for MyVec {
    type Output = MyVec;
    fn mul(self, factor: f64) -> MyVec {
        let mut result = MyVec { vec: [0.; NDIMS] };
        for dim in 0..NDIMS {
            result.vec[dim] = self.vec[dim] * factor;
        }
        result
    }
}

impl core::ops::Mul<MyVec> for f64 {
    type Output = MyVec;
    fn mul(self, vec: MyVec) -> MyVec {
        let mut result = MyVec { vec: [0.; NDIMS] };
        for dim in 0..NDIMS {
            result.vec[dim] = self * vec.vec[dim];
        }
        result
    }
}

impl core::ops::Div<f64> for MyVec {
    type Output = MyVec;
    fn div(self, factor: f64) -> MyVec {
        let mut result = MyVec { vec: [0.; NDIMS] };
        for dim in 0..NDIMS {
            result.vec[dim] = self.vec[dim] / factor;
        }
        result
    }
}

impl core::ops::Index<usize> for MyVec {
    type Output = f64;
    fn index(&self, n: usize) -> &Self::Output {
        &self.vec[n]
    }
}

impl core::ops::IndexMut<usize> for MyVec {
    fn index_mut(&mut self, n: usize) -> &mut Self::Output {
        &mut self.vec[n]
    }
}

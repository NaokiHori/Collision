use crate::simulator::cell::CellPosition;

fn get_index_of_absmin(data: &[f64]) -> usize {
    data.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.abs().total_cmp(&b.abs()))
        .map(|(index, _)| index)
        .unwrap()
}

pub fn correct_periodicity(dpos: f64, length: f64, cell_position: &CellPosition) -> f64 {
    match *cell_position {
        CellPosition::Centre => dpos,
        _ => {
            let cands: [f64; 3] = [dpos - length, dpos, dpos + length];
            let index: usize = get_index_of_absmin(&cands);
            cands[index]
        }
    }
}

#[cfg(test)]
mod test_get_index_of_absmin {
    use super::get_index_of_absmin as func;
    #[test]
    fn case1() {
        let data: [f64; 2] = [0., 1.];
        assert_eq!(func(&data), 0);
    }
    #[test]
    fn case2() {
        let data: [f64; 2] = [0., -1.];
        assert_eq!(func(&data), 0);
    }
    #[test]
    fn case3() {
        let data: [f64; 3] = [2., 0., -1.];
        assert_eq!(func(&data), 1);
    }
}

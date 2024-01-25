use std::f64::consts::PI;

pub fn rad_to_deg(rad: f64) -> isize {
    (rad * 180.0 / PI).round() as isize
}

/// discret search
pub fn search_near_zeroes((x_datas, y_datas): (&[f64], &[f64])) -> Vec<(f64, f64)> {
    assert_eq!(x_datas.len(), y_datas.len());
    let mut zeroes = vec![];

    let mut curr_zero = (x_datas[0], y_datas[0]);
    let mut decrease = false;
    for (&x, &y) in x_datas.iter().zip(y_datas) {
        if y.abs() < curr_zero.1.abs() {
            curr_zero = (x, y);
        }
        if y.abs() - curr_zero.1.abs() > 0.0 {
            if decrease {
                zeroes.push(curr_zero);
            }
            decrease = false;
            curr_zero = (x, y);
        } else {
            decrease = true;
        }
    }
    zeroes
}

pub fn round(n: f64, digit: u32) -> f64 {
    let fact = 10_f64.powi(digit as i32);
    (n * fact).round() / fact
}

pub fn search_period((x_datas, y_datas): (&[f64], &[f64])) -> f64 {
    let zeroes = search_near_zeroes((&x_datas, &y_datas));
    assert!(zeroes.len() >= 3);
    round(zeroes[2].0 - zeroes[0].0, 3)
}

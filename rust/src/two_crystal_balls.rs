use std::cmp::min;

pub fn two_crystal_balls(breaks: Vec<bool>) -> i32 {
    if breaks.len() == 0 {
        return -1;
    }

    let step_size = (breaks.len() as f64).sqrt() as usize;
    let mut step = 0;

    while step * step_size < breaks.len() {
        if breaks[step * step_size] {
            // step += 1;
            break;
        } else {
            step += 1;
        }
    }

    if step * step_size >= breaks.len() {
        step -= 1;
    }

    for i in step * step_size..min((step + 1) * step_size, breaks.len()) {
        if breaks[i] {
            return i as i32;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use crate::two_crystal_balls::*;

    #[test]
    fn test_two_crystal_balls() {
        assert_eq!(
            two_crystal_balls(vec![false, false, false, false, false, false, false, true]),
            7
        );
        assert_eq!(
            two_crystal_balls(vec![
                false, false, false, false, false, false, false, false, false, true
            ]),
            9
        );
        assert_eq!(
            two_crystal_balls(vec![
                false, false, false, false, false, false, true, true, true, true
            ]),
            6
        );
        assert_eq!(two_crystal_balls(vec![false, false, true]), 2);
        assert_eq!(two_crystal_balls(vec![true]), 0);
        assert_eq!(two_crystal_balls(vec![false, false]), -1);
        assert_eq!(two_crystal_balls(vec![]), -1);
    }
}

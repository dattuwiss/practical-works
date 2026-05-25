pub fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    if a.is_empty() || b.is_empty() {
        return 0;
    }
    let min_possible = *a.iter().max().unwrap();
    let max_possible = *b.iter().min().unwrap();
    let mut valid_count = 0;
    for x in min_possible..=max_possible {
        let is_a_factor = a.iter().all(|&element| x % element == 0);
        let is_x_factor = b.iter().all(|&element| element % x == 0);
        if is_a_factor && is_x_factor {
            valid_count += 1;
        }
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_total_x() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq(getTotalX(&a, &b), 2);
    }
}

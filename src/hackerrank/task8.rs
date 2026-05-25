pub fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut counts = vec![0; 6];
    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }
    let mut max_count = 0;
    let mut result_bird = 1;
    for bird_type in 1..=5 {
        if counts[bird_type] > max_count {
            max_count = counts[bird_type];
            result_bird = bird_type as i32;
        }
    }
    result_bird
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_migratory_birds() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq(migratoryBirds(&arr), 4);
    }
}

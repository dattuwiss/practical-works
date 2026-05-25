use std::collections::HashMap;

pub fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    if n <= 0 || ar.is_empty() {
        return 0;
    }
    let mut counts = HashMap::new();
    for &sock in ar {
        *counts.entry(sock).or_insert(0) += 1;
    }
    let mut pairs = 0;
    for &count in counts.values() {
        pairs += count / 2;
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sock_merchant() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq(sockMerchant(9, &ar), 3);
    }
}

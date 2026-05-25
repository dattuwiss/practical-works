pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut final_grades = Vec::new();
    for &grade in grades {
        if grade < 38 {
            final_grades.push(grade);
        } else {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                final_grades.push(next_multiple_of_5);
            } else {
                final_grades.push(grade);
            }
        }
    }
    final_grades
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grading_students() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq(grading_students(&input), expected);
    }
}

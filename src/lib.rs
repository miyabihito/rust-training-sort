pub fn run(input: &mut Vec<i32>) {
    sort(input);
}

fn sort(input: &mut Vec<i32>) {
    input.sort();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_asc() {
        let mut input = vec![3, 2, 1];
        sort(&mut input);

        assert_eq!(vec![1, 2, 3], input);
    }
}

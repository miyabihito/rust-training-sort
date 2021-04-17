pub fn run(input: Input) -> Vec<i32> {
    let mut output = input.target.clone();
    sort(&mut output);
    if !input.asc {
        output.reverse();
    }
    output
}

fn sort(input: &mut Vec<i32>) {
    input.sort();
}

pub struct Input {
    pub target: Vec<i32>,
    pub asc: bool,
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

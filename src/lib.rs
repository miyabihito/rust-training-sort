pub fn run(input: Input) -> Vec<i32> {
    let mut output = input.target.clone();
    sort(&mut output);
    if !input.asc {
        output.reverse();
    }
    output
}

fn sort(target: &mut Vec<i32>) {
    target.sort();
}

pub struct Input {
    pub target: Vec<i32>,
    pub asc: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
        let mut target = vec![3, 2, 1];
        sort(&mut target);

        assert_eq!(vec![1, 2, 3], target);

    }
}

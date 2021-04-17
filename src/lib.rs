pub fn run(input: Input) {
    let mut output = input.target.clone();
    sort(&mut output);
    if !input.asc {
        output.reverse();
    }

    let output = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
    println!("{}", output);
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

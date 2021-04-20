pub fn run(input: Input) -> Vec<i32> {
    let mut output = input.target.clone();

    match input.algorithm.as_str() {
        "default" => sort(&mut output),
        "bubble" => bubble_sort(&mut output),
        _ => (),
    }

    if !input.asc {
        output.reverse();
    }

    output
}

fn sort(target: &mut Vec<i32>) {
    target.sort();
}

fn bubble_sort(target: &mut Vec<i32>) {
    for i in (0..(target.len() - 1)).rev() {
        for j in 0..=i {
            if target[j] > target[j + 1] {
                target.swap(j, j + 1)
            }
        }
    }
}

pub struct Input {
    pub target: Vec<i32>,
    pub asc: bool,
    pub algorithm: String,
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

    #[test]
    fn test_bubble_sort() {
        let mut target = vec![3, 2, 1];
        bubble_sort(&mut target);

        assert_eq!(vec![1, 2, 3], target);
    }

    #[test]
    fn test_run_asc() {
        let input = Input {
            target: vec![1, 4, 3, 2],
            asc: true,
            algorithm: String::from("default"),
        };

        assert_eq!(vec![1, 2, 3, 4], run(input));
    }

    #[test]
    fn test_run_desc() {
        let input = Input {
            target: vec![1, 4, 3, 2],
            asc: false,
            algorithm: String::from("bubble"),
        };

        assert_eq!(vec![4, 3, 2, 1], run(input));
    }
}

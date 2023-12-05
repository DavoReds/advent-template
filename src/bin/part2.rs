fn process_input(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("../input.txt");
    let result = process_input(input);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = "";

        let result = process_input(input);

        assert_eq!(0, result);
    }
}


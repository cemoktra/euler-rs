pub fn solve(names: &mut [&str]) -> usize {
    names.sort();
    names.iter().enumerate().fold(0, |acc, (i, name)| {
        let name_score: usize = name.bytes().map(|b| b - 64).sum::<u8>().into();
        acc + ((i + 1) * name_score)
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let file_content = std::fs::read_to_string("./data/e00022.txt").unwrap();
        let mut names = file_content
            .split(',')
            .map(|name| &name[1..name.len() - 1])
            .collect::<Vec<_>>();

        assert_eq!(871198282, super::solve(names.as_mut_slice()));
    }
}

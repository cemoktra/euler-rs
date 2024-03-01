pub const DATA: &[&[usize]] = &[
    &[75],
    &[95, 64],
    &[17, 47, 82],
    &[18, 35, 87, 10],
    &[20, 4, 82, 47, 65],
    &[19, 1, 23, 75, 3, 34],
    &[88, 2, 77, 73, 7, 63, 67],
    &[99, 65, 4, 28, 6, 16, 70, 92],
    &[41, 41, 26, 56, 83, 40, 80, 70, 33],
    &[41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
    &[53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
    &[70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
    &[91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
    &[63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
    &[4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
];

pub fn solve(triangle: &[&[usize]]) -> usize {
    triangle
        .iter()
        .rev()
        .fold(None, |acc, r| {
            if let Some(acc) = acc {
                if r.len() > 1 {
                    let r = r.iter().zip(acc).map(|(a, b)| a + b).collect::<Vec<_>>();
                    Some(
                        r.windows(2)
                            .map(|w| w.iter().max().cloned().unwrap_or_default())
                            .collect::<Vec<_>>(),
                    )
                } else {
                    Some(vec![acc[0] + r[0]])
                }
            } else {
                Some(
                    r.windows(2)
                        .map(|w| w.iter().max().cloned().unwrap_or_default())
                        .collect::<Vec<_>>(),
                )
            }
        })
        .unwrap_or_default()[0]
}

#[cfg(test)]
mod test {
    const TEST_DATA: &[&[usize]] = &[&[3], &[7, 4], &[2, 4, 6], &[8, 5, 9, 3]];

    #[test]
    fn test() {
        assert_eq!(23, super::solve(TEST_DATA));
        assert_eq!(1074, super::solve(super::DATA));
    }
}

use std::collections::{HashMap, HashSet};

pub fn solve() -> usize {
    let mut triangles = HashMap::<usize, HashSet<(usize, usize, usize)>>::new();
    for m in 2..1225 {
        let m2 = m * m;
        for n in 1..m {
            let n2 = n * n;

            let mut k = 1;
            loop {
                let mut a = k * (m2 - n2);
                let mut b = k * 2 * m * n;
                let c = k * (m2 + n2);

                let perimeter = a + b + c;

                if perimeter > 1000 {
                    break;
                }
                if a > b {
                    std::mem::swap(&mut a, &mut b);
                }
                triangles.entry(perimeter).or_default().insert((a, b, c));

                k += 1;
            }
        }
    }

    let max = triangles
        .iter()
        .max_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

    max.map(|(k, _)| *k).unwrap_or_default()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(840, super::solve());
    }
}

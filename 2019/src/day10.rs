use crate::{
    util::{gcd, Point2, Point2i},
    Error, Solution,
};

pub fn solve(input: String) -> Result<Solution<usize, i32>, Error> {
    let asteroids: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let (p1, station) = solve_part1(&asteroids);
    let p2 = solve_part2(&asteroids, station, 200)?;
    Ok(Solution::new(p1, p2))
}

fn solve_part1(asteroids: &[Vec<bool>]) -> (usize, Point2<usize>) {
    let w = asteroids[0].len();
    let h = asteroids.len();
    let mut station = Point2::default();
    let mut result = 0;
    for y in 0..h {
        for x in 0..w {
            if asteroids[y][x] {
                let mut sights = 0;
                for cy in 0..h {
                    for cx in 0..w {
                        let delta = Point2::new(cx as i32 - x as i32, cy as i32 - y as i32);
                        if gcd(delta.x.abs(), delta.y.abs()) == 1 {
                            let mut curr = Point2::new(cx as i32, cy as i32);
                            while curr.in_bounds(0, 0, w as i32, h as i32) {
                                if asteroids[curr.y as usize][curr.x as usize] {
                                    sights += 1;
                                    break;
                                }
                                curr += delta;
                            }
                        }
                    }
                }
                if sights > result {
                    result = sights;
                    station = Point2::new(x, y);
                }
            }
        }
    }
    (result, station)
} // 127.23ms

fn solve_part2(
    asteroids: &[Vec<bool>],
    station: Point2<usize>,
    destroy_num: usize,
) -> Result<i32, Error> {
    let w = asteroids[0].len();
    let h = asteroids.len();
    if w != h {
        bail!("w ({}) must be equal to h ({}) for part2 to work", w, h);
    }
    // fractions (coordinates) from 0 to 1 with max denumerator max(station.x, station.y)
    let mut coords = farey(station.x.max(station.y) as i32);
    // extend by reciprocals (bottom right corner)
    coords.extend(
        coords
            .clone()
            .into_iter()
            .map(|p| Point2i::new(p.y, p.x))
            .rev()
            .skip(1),
    );
    // extent by negated denumerators (bottom left corner)
    coords.extend(
        coords
            .clone()
            .into_iter()
            .map(|p| Point2i::new(p.x, -p.y))
            .rev()
            .skip(1),
    );
    // extent by negated numerators (top left corner)
    coords.extend(
        coords
            .clone()
            .into_iter()
            .map(|p| Point2i::new(-p.x, p.y))
            .rev()
            .skip(1),
    );
    coords.pop();
    // matrix-coordinates have flipped y values
    coords = coords
        .into_iter()
        .map(|p| Point2i::new(p.x, -p.y))
        .collect();
    let station = Point2i::new(station.x as i32, station.y as i32);
    let mut destroy_order: Vec<Vec<Point2<i32>>> = Vec::with_capacity(w.max(h));
    for &delta in &coords {
        let mut curr = station + delta;
        let mut rotation = 0;
        while curr.in_bounds(0, 0, w as i32, h as i32) {
            if asteroids[curr.y as usize][curr.x as usize] {
                if destroy_order.len() <= rotation {
                    destroy_order.push(Vec::new());
                }
                destroy_order[rotation].push(curr);
                rotation += 1;
            }
            curr += delta;
        }
    }
    let result = *destroy_order
        .iter()
        .flatten()
        .take(destroy_num)
        .last()
        .ok_or_else(|| error!("Found no destroyed asteroids"))?;
    Ok(result.x * 100 + result.y)
}

// generate fractions from 0 to 1 with max denumerator n
fn farey(n: i32) -> Vec<Point2i> {
    let mut ab = Point2i::new(0, 1);
    let mut cd = Point2i::new(1, n);
    let mut sequence = vec![ab];
    while cd.x < n {
        let k = (n + ab.y) / cd.y;
        let old_cd = cd;
        cd = Point2i::new(k * cd.x - ab.x, k * cd.y - ab.y);
        ab = old_cd;
        sequence.push(ab);
    }
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test10() {
        let input = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let asteroids: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        assert_eq!(solve_part1(&asteroids), (33, Point2::new(5, 8)));
        let input = ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let asteroids: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        assert_eq!(solve_part1(&asteroids), (41, Point2::new(6, 3)));
        let input = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        assert_eq!(solve(input.to_owned()).unwrap(), Solution::new(210, 802));
        crate::util::tests::test_full_problem(10, solve, 344, 2732);
    }
}

pub mod point {
    use itertools::Itertools;
    use std::cmp::Ordering;

    #[derive(Debug, Clone, Copy)]
    pub struct Point(pub isize, pub isize, pub isize, pub isize);

    impl Point {
        pub fn neighbors(&self) -> Vec<Point> {
            let x: Vec<isize> = vec![-1, 0, 1];
            let y: Vec<isize> = vec![-1, 0, 1];
            let z: Vec<isize> = vec![-1, 0, 1];
            let w: Vec<isize> = vec![-1, 0, 1];
            let xy: Vec<(isize, isize)> = x.into_iter().cartesian_product(y.into_iter()).collect();
            let xyz: Vec<(isize, isize, isize)> = xy
                .into_iter()
                .cartesian_product(z.into_iter())
                .map(|((a, b), c)| (a, b, c))
                .collect();
            let xyzw: Vec<(isize, isize, isize, isize)> = xyz
                .into_iter()
                .cartesian_product(w.into_iter())
                .map(|((a, b, c), d)| (a, b, c, d))
                .filter(|&(a, b, c, d)| (a, b, c, d) != (0, 0, 0, 0))
                .collect();

            xyzw.iter()
                .map(|(a, b, c, d)| {
                    let Point(x, y, z, w) = *self;
                    Point(x + a, y + b, z + c, w + d)
                })
                .collect()
        }

        pub fn new(x: isize, y: isize, z: isize, w: isize) -> Point {
            Point(x, y, z, w)
        }
    }

    impl Ord for Point {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.0.cmp(&other.0) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => match self.1.cmp(&other.1) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => match self.2.cmp(&other.2) {
                        Ordering::Less => Ordering::Less,
                        Ordering::Greater => Ordering::Greater,
                        Ordering::Equal => match self.3.cmp(&other.3) {
                            Ordering::Less => Ordering::Less,
                            Ordering::Greater => Ordering::Greater,
                            Ordering::Equal => Ordering::Equal,
                        },
                    },
                },
            }
        }
    }

    impl PartialOrd for Point {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            let Point(a, b, c, d) = *self;
            let Point(x, y, z, w) = *other;
            (a, b, c, d) == (x, y, z, w)
        }
    }

    impl Eq for Point {}
}

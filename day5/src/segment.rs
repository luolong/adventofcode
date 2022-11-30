use std::{
    cmp::{max, min},
    error,
    fmt::Display,
    ops::{AddAssign, RangeInclusive},
    str::FromStr,
};

use crate::point::{self, Dim, Point};

use micromath::vector::{F32x2, Vector};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct Segment([Dim; 4]);

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [x1, y1, x2, y2] = self.0;
        write!(f, "{},{} -> {},{}", x1, y1, x2, y2)
    }
}

impl FromStr for Segment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" -> ")
            .ok_or_else(|| Error::SegmentFormat(s.to_string()))
            .and_then(|(a, b)| {
                let pt_a: Point = a.parse()?;
                let pt_b: Point = b.parse()?;
                let (x1, y1) = pt_a.into();
                let (x2, y2) = pt_b.into();
                Ok(Segment([x1, y1, x2, y2]))
            })
    }
}

impl From<(Point, Point)> for Segment {
    fn from(t: (Point, Point)) -> Self {
        let (x1, y1) = t.0.into();
        let (x2, y2) = t.1.into();
        Segment([x1, y1, x2, y2])
    }
}

impl From<((Dim, Dim), (Dim, Dim))> for Segment {
    fn from(t: ((Dim, Dim), (Dim, Dim))) -> Self {
        let ((x1, y1), (x2, y2)) = t;
        Segment([x1, y1, x2, y2])
    }
}

impl Segment {
    pub(crate) fn is_horizontal(&self) -> bool {
        let [x1, _, x2, _] = self.0;
        x1 == x2
    }

    pub(crate) fn is_vertical(&self) -> bool {
        let [_, y1, _, y2] = self.0;
        y1 == y2
    }
}

macro_rules! cmp {
    ($a:tt <= $b:tt <= $c:tt) => {
        ($a <= $b && $b <= $c)
    };
}

impl Segment {
    fn norm(&self) -> Segment {
        let [x1, y1, x2, y2] = self.0;

        if x1 > x2 || (x1 == x2 && y1 > y2) {
            Segment([x2, y2, x1, y1])
        } else {
            Segment([x1, y1, x2, y2])
        }
    }

    /// If the two segments intersect, this returns segment of the intersecting parts
    pub(crate) fn intersection(&self, other: &Segment) -> Option<Segment> {
        print!("Intersection of {self} and {other}");
        let l1 = self.norm();
        let l2 = other.norm();

        let [ax1, ay1, ax2, ay2] = l1.0;
        let [bx1, by1, bx2, by2] = l2.0;

        if is_overlap(ax1, ax2, bx1, bx2) && is_overlap(ay1, ay2, by1, by2) {
            let [sxa, sxb] = [slope(ax1, ax2), slope(bx1, bx2)];
            let [sya, syb] = [slope(ay1, ay2), slope(by1, by2)];

            {
                #[rustfmt::skip]
                println!(
                    " {}{}",
                    if sxa == 0 { '|' } else if sya > 0 { '\\' } else if sya > 0 { '/' } else { '-' },
                    if sxb == 0 { '|' } else if syb > 0 { '\\' } else if syb > 0 { '/' } else { '-' },
                );

                // implementation based on https://stackoverflow.com/a/565282/1712

                let p = [ax1, ay1];
                let r = [ax2 - ax1, ay2 - ay1];

                let q = [bx1, by1];
                let s = [bx2 - bx1, by2 - by1];

                println!("   p + t x r == q + u x s");
                println!("   {p:?} + t x {r:?} == {q:?} + u x {s:?}");

                let p = F32x2::from_slice(&p.map(|i| i as f32));
                let r = F32x2::from_slice(&r.map(|i| i as f32));
                let q = F32x2::from_slice(&q.map(|i| i as f32));
                let s = F32x2::from_slice(&s.map(|i| i as f32));

                let rxs = r.cross(s);
                let q_p = q - p;
                let q_pxr = q_p.cross(r);

                println!("   r⨯s={rxs:?}, (q-p)⨯r = {q_pxr:?}");
                if rxs == 0.0 && q_pxr == 0.0 {
                    let t1 = q_p.dot(r) / r.dot(s);
                    let t2 = t1 + s.dot(r) / r.dot(r);
                    println!("   t1={t1:?}, t2={t2:?}");
                    if cmp!(0.0 <= t1 <= 1.0) || cmp!(0.0 <= t2 <= 1.0) {
                        let t1 = t1.clamp(0.0, 1.0);
                        let t2 = t2.clamp(0.0, 1.0);
                        println!("   t1={t1:?}, t2={t2:?}");
                        let p1 = (p + r * t1).to_array();
                        let p2 = (p + r * t2).to_array();
                        let [x1, y1] = p1.map(|f| f.ceil() as Dim);
                        let [x2, y2] = p2.map(|f| f.ceil() as Dim);
                        println!(" is ({x1},{y1} -> {x2},{y2})");
                        return Some(Segment([x1, y1, x2, y2]));
                    }
                } else if !(rxs == 0.0 && q_pxr != 0.0) {
                    let qps = q_p.cross(s);
                    let t = qps / rxs;
                    let u = q_pxr / rxs;

                    if rxs != 0.0 && cmp!(0.0 <= t <= 1.0) && cmp!(0.0 <= u <= 1.0) {
                        let [x, y] = (p + r * t).to_array().map(|f| f.ceil() as Dim);
                        println!(" is ({x},{y})");
                        return Some(Segment([x, y, x, y]));
                    }
                }
            }
        }

        println!(" is ⦰");
        None
    }
}

impl IntoIterator for Segment {
    type Item = Point;

    type IntoIter = IntoPointIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoPointIter { s: self, n: 0 }
    }
}

impl FromIterator<Dim> for Segment {
    fn from_iter<T: IntoIterator<Item = Dim>>(iter: T) -> Self {
        let mut points: [Dim; 4] = Default::default();
        iter.into_iter()
            .enumerate()
            .take(4)
            .for_each(|(i, val)| points[i] = val);

        Segment(points)
    }
}

pub(crate) struct IntoPointIter {
    s: Segment,
    n: usize,
}

impl Iterator for IntoPointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let IntoPointIter { s, n } = self;
        let [x1, y1, x2, y2] = s.0;

        let n0 = *n as i32;
        let nx = n0 * slope(x1, x2);
        let ny = n0 * slope(y1, y2);
        if n0 > 0 && nx == 0 && ny == 0 {
            // the only reason this is possible is if the segment is a single point
            return None;
        }

        let (x, y) = (x1 + nx, y1 + ny);
        let (xmin, xmax) = minmax(x1, x2);
        let (ymin, ymax) = minmax(y1, y2);

        if cmp!(xmin <= x <= xmax) && cmp!(ymin <= y <= ymax) {
            n.add_assign(1);
            Some(Point::from((x, y)))
        } else {
            None
        }
    }
}

trait CrossProduct {
    type Output;
    fn cross(&self, rhs: Self) -> Self::Output;
}

impl CrossProduct for F32x2 {
    type Output = f32;
    fn cross(&self, rhs: Self) -> Self::Output {
        let [vx, vy] = self.to_array();
        let [wx, wy] = rhs.to_array();
        vx * wy - vy * wx
    }
}

#[rustfmt::skip]
fn minmax<T: PartialEq + PartialOrd + Copy>(a: T, b: T) -> (T, T) {
    if a > b { (b, a) } else { (a, b ) }
}

#[rustfmt::skip]
fn range_from<T: PartialEq + PartialOrd + Copy>(a: T, b: T) -> RangeInclusive<T> {
    if a > b { b..=a } else { a..=b }    
}

#[rustfmt::skip]
fn slope(a: Dim, b: Dim) -> i32 {
    if a < b { 1 } else if a > b { -1 } else { 0 }
}

fn is_overlap(a1: Dim, a2: Dim, b1: Dim, b2: Dim) -> bool {
    let (a1, a2) = minmax(a1, a2);
    let (b1, b2) = minmax(b1, b2);
    a1 <= b2 && a2 >= b1
}

fn overlap(a1: Dim, a2: Dim, b1: Dim, b2: Dim) -> Option<(Dim, Dim)> {
    let (a1, a2) = minmax(a1, a2);
    let (b1, b2) = minmax(b1, b2);
    if a1 <= b2 && a2 >= b1 {
        Some((max(a1, b1), min(a2, b2)))
    } else {
        None
    }
}

fn in_range(a: Dim, r1: Dim, r2: Dim) -> bool {
    range_from(r1, r2).contains(&a)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SegmentFormat(String),
    PointFormat(point::Error),
}

impl error::Error for Error {}

impl From<point::Error> for Error {
    fn from(cause: point::Error) -> Self {
        Error::PointFormat(cause)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SegmentFormat(s) => write!(f, "Invalid segment: {}", s),
            Error::PointFormat(source) => write!(f, "{}", source),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

    macro_rules! segment {
        ($x1:literal,$y1:literal -> $x2:literal, $y2:literal) => {
            Segment([$x1, $y1, $x2, $y2])
        };
    }

    macro_rules! assert_overlap {
        (none, $a:expr, $b:expr) => {
            assert_eq!(None, $a.intersection(&$b));
            assert_eq!(None, $b.intersection(&$a));
        };

        (expect $s:expr, $a:expr, $b:expr) => {
            assert_eq!(Some($s), $a.intersection(&$b));
            assert_eq!(Some($s), $b.intersection(&$a));
        };
    }

    #[test]
    fn it_normalizes_segment() {
        assert_eq!(segment!(0,9 -> 5,9), segment!(0,9 -> 5,9).norm());
        assert_eq!(segment!(7,0 -> 7,4), segment!(7,0 -> 7,4).norm());
        assert_eq!(segment!(1,4 -> 3,4), segment!(3,4 -> 1,4).norm());
        assert_eq!(segment!(2,1 -> 2,2), segment!(2,2 -> 2,1).norm());
        assert_eq!(segment!(0,8 -> 8,0), segment!(8,0 -> 0,8).norm());
        assert_eq!(segment!(0,0 -> 8,8), segment!(0,0 -> 8,8).norm());
    }

    #[test]
    fn it_can_parse_valid_segment() {
        assert_eq!(
            Ok(segment!(445,187 -> 912, 654)),
            parse("445,187 -> 912,654")
        );
    }

    #[test]
    fn it_fails_to_parse_invalid_segment() {
        assert_eq!(segment_fmt_err(""), parse(""));
        assert_eq!(segment_fmt_err("445,187"), parse("445,187"));
        assert_eq!(point_fmt_err(""), parse(" -> "));
        assert_eq!(point_fmt_err(""), parse("445,187 -> "));
        assert_eq!(point_fmt_err(""), parse(" -> 445,187"));
        assert_eq!(point_fmt_err("912,"), parse("445,187 -> 912,"));
        assert_eq!(point_fmt_err("foo"), parse("foo -> 912,"));
    }

    #[test]
    fn it_recognizes_horizontal_segments() {
        assert!(segment!(80,467 -> 80,48).is_horizontal());
        assert!(!segment!(820,46 -> 25,841).is_horizontal());
        assert!(!segment!(908,132 -> 714,132).is_horizontal());
    }

    #[test]
    fn it_recognizes_vertical_segments() {
        assert!(!segment!(80,467 -> 80,48).is_vertical());
        assert!(!segment!(820,46 -> 25,841).is_vertical());
        assert!(segment!(908,132 -> 714,132).is_vertical());
    }

    #[test]
    fn it_calculates_parallel_intersections() {
        assert_overlap!(none, segment!(2,2 -> 2,1), segment!(7,0 -> 7,4));

        assert_overlap!(
            expect segment!(0,9 -> 2,9),
            segment!(0,9 -> 5,9), segment!(0,9 -> 2,9)
        );

        assert_overlap!(
            expect segment!(2,9 -> 2,9),
            segment!(0,9 -> 2,9), segment!(2,9 -> 5,9)
        );

        assert_overlap!(none, segment!(0,9 -> 2,9), segment!(5,9 -> 3,9));

        assert_overlap!(
            expect segment!(7,1 -> 7,4),
            segment!(7,0 -> 7,4), segment!(7,8 -> 7,1)
        );

        assert_overlap!(
            expect segment!(7,4 -> 7,4),
            segment!(7,0 -> 7,4), segment!(7,8 -> 7,4)
        );

        assert_overlap!(none, segment!(7,0 -> 7,4), segment!(7,8 -> 7,5));
    }

    #[test]
    fn it_calculates_intersection_with_vertical_line() {
        //  |/
        //  /
        // /|
        assert_overlap!(
            expect segment!(7,4 -> 7,4),
            segment!(7,0 -> 7,5), segment!(8,3 -> 5,6)
        );

        // \|
        //  \
        //  |\
        assert_overlap!(
            expect segment!(7,4 -> 7,4),
            segment!(7,0 -> 7,5), segment!(6,3 -> 9,6)
        );

        //  \
        //   \
        //  | \
        assert_overlap!(none, segment!(1,1 -> 3,3), segment!(1,2 -> 1,3));

        //  | /
        //   /
        //  /
        assert_overlap!(none, segment!(3,1 -> 1,3), segment!(1,1 -> 1,2));
    }

    #[test]
    fn it_calculates_intersection_with_horizontal_line() {
        // --/--
        assert_overlap!(
            expect segment!(4,7 -> 4,7),
            segment!(2,7 -> 5,7), segment!(2,9 -> 5,6)
        );

        // --\--
        assert_overlap!(
            expect segment!(7,4 -> 7,4),
            segment!(7,0 -> 7,5), segment!(6,3 -> 9,6)
        );

        assert_overlap!(none, segment!(0,0 -> 8,8), segment!(1,4 -> 3,4));
    }

    #[test]
    fn it_calculates_cross_intersections() {
        assert_overlap!(none, segment!(2,2 -> 2,1), segment!(0,9 -> 2,9));

        assert_overlap!(
            expect segment!(7,4 -> 7,4),
            segment!(7,0 -> 7,4), segment!(9,4 -> 3,4)
        );
    }

    //fn it_calculates_

    #[test]
    fn it_calculates_cross_intersection_of_diagonal_lines() {
        assert_overlap!(
            expect segment!(4,4 -> 4,4),
            segment!(0,0 -> 8,8), segment!(8,0 -> 0,8)
        );

        assert_overlap!(none, segment!(0,0 -> 8,8), segment!(6,4 -> 2,0));
    }

    #[test]
    fn it_iterates_over_all_points_on_horizontal_line() {
        let mut it = segment!(9,7 -> 7,7).into_iter();
        assert_eq!(Some(Point::from((9, 7))), it.next());
        assert_eq!(Some(Point::from((8, 7))), it.next());
        assert_eq!(Some(Point::from((7, 7))), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn it_iterates_over_all_points_on_vertical_line() {
        let mut it = segment!(1,1 -> 1,3).into_iter();
        assert_eq!(Some(Point::from((1, 1))), it.next());
        assert_eq!(Some(Point::from((1, 2))), it.next());
        assert_eq!(Some(Point::from((1, 3))), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn it_iterates_over_all_points_on_diagonal_line() {
        let mut it = segment!(1,1 -> 3,3).into_iter();
        assert_eq!(Some(Point::from((1, 1))), it.next());
        assert_eq!(Some(Point::from((2, 2))), it.next());
        assert_eq!(Some(Point::from((3, 3))), it.next());
        assert_eq!(None, it.next());

        let mut it = segment!(9,7 -> 7,9).into_iter();
        assert_eq!(Some(Point::from((9, 7))), it.next());
        assert_eq!(Some(Point::from((8, 8))), it.next());
        assert_eq!(Some(Point::from((7, 9))), it.next());
        assert_eq!(None, it.next());
    }

    fn parse(s: &str) -> Result<Segment, Error> {
        s.parse()
    }

    fn segment_fmt_err(m: &str) -> Result<Segment, Error> {
        Err(Error::SegmentFormat(m.to_string()))
    }

    fn point_fmt_err(m: &str) -> Result<Segment, Error> {
        Err(point::Error::PointFormat(m.to_string()).into())
    }
}

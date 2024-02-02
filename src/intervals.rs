use std::cmp::min;
use std::fmt;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, BitAnd, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Interval<T> {
    pub start: T,
    pub stop: T,
}

impl<T> Display for Interval<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.stop)
    }
}

#[derive(Debug)]
pub struct IntervalCollection<T> {
    pub elts: Vec<Interval<T>>,
}
pub struct IC<T>(Vec<Interval<T>>);
impl<T> IC<T> {
    pub fn elts(&self) -> &Vec<Interval<T>> {
        &self.0
    }
}
pub type Toto<T> = Vec<Interval<T>>;

impl<T> Display for IntervalCollection<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, elt) in self.elts.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", elt)?;
        }
        write!(f, "]")
    }
}

impl<T> Add for Interval<T>
where
    T: Ord + Copy,
{
    type Output = IntervalCollection<T>;
    fn add(self, other: Interval<T>) -> IntervalCollection<T> {
        IntervalCollection { elts: vec![self] } + IntervalCollection { elts: vec![other] }
    }
}

/* Implement intersection between two Intervals */
impl<T> BitAnd for Interval<T>
where
    T: Clone + PartialEq + PartialOrd,
{
    type Output = Option<Interval<T>>;
    fn bitand(self, other: Interval<T>) -> Option<Interval<T>> {
        match self.overlap(&other) {
            true => Some(Interval {
                start: match self.start > other.start {
                    true => self.start,
                    false => other.start,
                },
                stop: match self.stop < other.stop {
                    true => self.stop,
                    false => other.stop,
                },
            }),
            false => None,
        }
    }
}

impl<T> Add<IntervalCollection<T>> for Interval<T>
where
    T: Ord + Copy,
{
    type Output = IntervalCollection<T>;
    fn add(self, other: IntervalCollection<T>) -> IntervalCollection<T> {
        IntervalCollection { elts: vec![self] } + other
    }
}

impl<T> Add<Interval<T>> for IntervalCollection<T>
where
    T: Ord + Copy,
{
    type Output = IntervalCollection<T>;
    fn add(self, other: Interval<T>) -> IntervalCollection<T> {
        self + IntervalCollection { elts: vec![other] }
    }
}

impl<T> Add for IntervalCollection<T>
where
    T: Ord + Copy,
{
    type Output = IntervalCollection<T>;
    fn add(self, other: IntervalCollection<T>) -> IntervalCollection<T> {
        let mut elts = Vec::new();
        let mut start = min(&self.elts[0], &other.elts[0]);

        loop {
            let swiping_line = *(&start.start);
            let mut horizon = *(&start.stop);

            horizon = self
                .elts
                .iter()
                .chain(other.elts.iter())
                .filter(|elt| &swiping_line <= &elt.start && &elt.start <= &horizon)
                .map(|elt| elt.stop)
                .max()
                .expect("Unexpected error");

            loop {
                match self
                    .elts
                    .iter()
                    .chain(other.elts.iter())
                    .filter(|elt| &elt.start <= &horizon && &horizon < &elt.stop)
                    .map(|elt| elt.stop)
                    .max()
                {
                    None => break,
                    Some(x) => horizon = x,
                }
            }
            elts.push(Interval {
                start: swiping_line,
                stop: horizon,
            });
            match self
                .elts
                .iter()
                .chain(other.elts.iter())
                .filter(|elt| &elt.start > &horizon)
                .min()
            {
                None => break,
                Some(x) => start = x,
            }
        }

        IntervalCollection { elts: elts }
    }
}

impl<T, Delta> Sub<Interval<T>> for IntervalCollection<T>
where
    T: Sub<T, Output = Delta> + Add<Delta, Output = T> + Copy + PartialOrd,
    Delta: Copy,
{
    type Output = IntervalCollection<T>;
    fn sub(self, other: Interval<T>) -> IntervalCollection<T> {
        let mut elts = Vec::new();
        for elt in self.elts {
            if elt.overlap(&other) {
                if other.start > elt.start {
                    elts.push(Interval {
                        start: elt.start,
                        stop: other.start,
                    })
                }
                if other.stop < elt.stop {
                    elts.push(Interval {
                        start: other.stop,
                        stop: elt.stop,
                    })
                }
            } else {
                elts.push(elt)
            }
        }
        IntervalCollection { elts: elts }
    }
}

impl<T, Delta> Sub for IntervalCollection<T>
where
    T: Sub<T, Output = Delta> + Add<Delta, Output = T> + Copy + PartialOrd,
    Delta: Copy,
{
    type Output = IntervalCollection<T>;
    fn sub(self, other: IntervalCollection<T>) -> IntervalCollection<T> {
        let mut res = self;
        for elt in other.elts {
            res = res - elt;
        }
        return res;
    }
}

impl<T, Delta> Interval<T>
where
    T: Sub<T, Output = Delta> + Add<Delta, Output = T> + Copy,
    Delta: Copy,
{
    pub fn duration(self) -> Delta {
        self.stop - self.start
    }
    pub fn shift(&self, delta: Delta) -> Interval<T> {
        Interval {
            start: self.start + delta,
            stop: self.stop + delta,
        }
    }
}

impl<T> Interval<T>
where
    T: PartialOrd,
{
    pub fn overlap(&self, other: &Interval<T>) -> bool {
        self.start <= other.stop && self.stop >= other.start
    }
}

impl<T, Delta> IntervalCollection<T>
where
    T: Sub<T, Output = Delta> + Add<Delta, Output = T> + Copy + PartialOrd,
    Delta: Copy + Sum,
{
    pub fn total_duration(&self) -> Delta {
        self.elts.iter().map(|elt| elt.duration()).sum()
    }
}

#[cfg(test)]
mod tests {

    use super::Interval;
    use chrono::{DateTime, Duration, Utc};

    static I1: Interval<i32> = Interval { start: 0, stop: 1 };
    static I2: Interval<i32> = Interval { start: 1, stop: 2 };
    static I3: Interval<i32> = Interval { start: 2, stop: 3 };
    static I4: Interval<i32> = Interval { start: 3, stop: 4 };
    static I5: Interval<i32> = Interval { start: 4, stop: 5 };

    #[test]
    fn interval_i32() {
        assert_eq!(I1.duration(), 1);
        let shifted = I1.shift(1);
        assert_eq!(shifted.duration(), 1);
        assert_ne!(shifted, I1);
        assert_eq!(shifted, I2);
        assert_eq!(format!("{:?}", shifted), "Interval { start: 1, stop: 2 }");
        assert_eq!(format!("{:}", shifted), "[1, 2]");
    }

    #[test]
    fn interval_dt() {
        let i_dt = Interval {
            start: "2024-01-20T12:00:00Z"
                .parse::<DateTime<Utc>>()
                .expect("error date"),
            stop: "2024-01-20T13:00:00Z"
                .parse::<DateTime<Utc>>()
                .expect("error date"),
        };
        assert_eq!(i_dt.duration(), Duration::hours(1));
        assert_eq!(i_dt.shift(Duration::days(1)).duration(), Duration::hours(1));
    }
    #[test]
    fn intervals_consistent() {
        assert_eq!(
            format!("{:?}", I1 + I2),
            "IntervalCollection { elts: [Interval { start: 0, stop: 2 }] }"
        );
        assert_eq!(format!("{:}", I1 + I2), "[[0, 2]]");
        assert_eq!(format!("{:}", I1 + I3), "[[0, 1], [2, 3]]");
        assert_eq!(format!("{:}", I2 + I4), "[[1, 2], [3, 4]]");
        let s1 = (I1 + I3) + (I2 + I4);
        assert_eq!(format!("{:}", s1), "[[0, 4]]");
        let s2 = (I1 + I3) + (I4 + I5);
        assert_eq!(format!("{:}", s2), "[[0, 1], [2, 5]]");
        let s3 = I1 + I3 + I4 + I5;
        assert_eq!(format!("{:}", s3), "[[0, 1], [2, 5]]");

        let i1 = Interval {
            start: "2024-01-20T12:00:00Z"
                .parse::<DateTime<Utc>>()
                .expect("error date"),
            stop: "2024-01-20T13:00:00Z"
                .parse::<DateTime<Utc>>()
                .expect("error date"),
        };
        let i2 = Interval {
            start: "2024-01-20T13:00:00Z"
                .parse::<DateTime<Utc>>()
                .expect("error date"),
            stop: "2024-01-20T14:00:00Z"
                .parse::<DateTime<Utc>>()
                .expect("error date"),
        };
        assert_eq!(
            format!("{:}", i1 + i2),
            "[[2024-01-20 12:00:00 UTC, 2024-01-20 14:00:00 UTC]]"
        );
        assert_eq!((i1 + i2).total_duration(), Duration::hours(2));
    }

    #[test]
    fn intervals_sub() {
        assert_eq!(format!("{:}", (I1 + I2 + I3) - I2), "[[0, 1], [2, 3]]");
        assert_eq!(format!("{:}", (I1 + I2) - (I3 + I2)), "[[0, 1]]");
    }
}

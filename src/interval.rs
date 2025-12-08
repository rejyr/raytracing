use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Interval::EMPTY
    }
}

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}

impl Interval {
    pub const EMPTY: Self = Interval::new(f64::INFINITY, -f64::INFINITY);
    pub const UNIVERSE: Self = Interval::new(-f64::INFINITY, f64::INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Create the interval tightly enclosing the two input intervals
    pub const fn from_intervals(a: &Interval, b: &Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Self::new(min, max)
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }

    pub const fn expand(self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max - padding)
    }
}

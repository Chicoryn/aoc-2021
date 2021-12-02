pub trait Submarine<S> {
    fn forward(&mut self, amount: isize) -> &mut S;
    fn up(&mut self, amount: isize) -> &mut S;
    fn down(&mut self, amount: isize) -> &mut S;
}

#[derive(Debug, PartialEq)]
pub struct SubmarineV1 {
    horizontal: isize,
    depth: isize,
}

impl Default for SubmarineV1 {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Submarine<Self> for SubmarineV1 {
    fn forward(&mut self, amount: isize) -> &mut Self {
        self.horizontal += amount;
        self
    }

    fn up(&mut self, amount: isize) -> &mut Self {
        self.depth -= amount;
        self
    }

    fn down(&mut self, amount: isize) -> &mut Self {
        self.depth += amount;
        self
    }
}

impl SubmarineV1 {
    pub fn new(horizontal: isize, depth: isize) -> Self {
        Self { horizontal, depth }
    }

    pub fn product(&self) -> isize {
        self.horizontal * self.depth
    }
}

#[derive(Debug, PartialEq)]
pub struct SubmarineV2 {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Default for SubmarineV2 {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Submarine<SubmarineV2> for SubmarineV2 {
    fn forward(&mut self, amount: isize) -> &mut Self {
        self.horizontal += amount;
        self.depth += self.aim * amount;
        self
    }

    fn up(&mut self, amount: isize) -> &mut Self {
        self.aim -= amount;
        self
    }

    fn down(&mut self, amount: isize) -> &mut Self {
        self.aim += amount;
        self
    }
}

impl SubmarineV2 {
    pub fn new(horizontal: isize, depth: isize, aim: isize) -> Self {
        Self {
            horizontal,
            depth,
            aim,
        }
    }

    pub fn product(&self) -> isize {
        self.horizontal * self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1() {
        let mut submarine = SubmarineV1::default();
        assert_eq!(submarine, SubmarineV1::new(0, 0));
        assert_eq!(*submarine.forward(5), SubmarineV1::new(5, 0));
        assert_eq!(*submarine.down(5), SubmarineV1::new(5, 5));
        assert_eq!(*submarine.forward(8), SubmarineV1::new(13, 5));
        assert_eq!(*submarine.up(3), SubmarineV1::new(13, 2));
        assert_eq!(*submarine.down(8), SubmarineV1::new(13, 10));
        assert_eq!(*submarine.forward(2), SubmarineV1::new(15, 10));
    }

    #[test]
    fn v2() {
        let mut submarine = SubmarineV2::default();
        assert_eq!(submarine, SubmarineV2::new(0, 0, 0));
        assert_eq!(*submarine.forward(5), SubmarineV2::new(5, 0, 0));
        assert_eq!(*submarine.down(5), SubmarineV2::new(5, 0, 5));
        assert_eq!(*submarine.forward(8), SubmarineV2::new(13, 40, 5));
        assert_eq!(*submarine.up(3), SubmarineV2::new(13, 40, 2));
        assert_eq!(*submarine.down(8), SubmarineV2::new(13, 40, 10));
        assert_eq!(*submarine.forward(2), SubmarineV2::new(15, 60, 10));
    }
}

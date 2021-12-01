pub struct Windowed<'a, T> {
    buf: &'a [T],
    index: usize,
    window_size: usize,
}

impl<'a, T> Windowed<'a, T> {
    pub fn new(buf: &'a [T], window_size: usize) -> Self {
        Self {
            buf,
            index: 0,
            window_size,
        }
    }
}

impl<'a, T> Iterator for Windowed<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let upper = self.index + self.window_size;
        let start = self.index;

        if upper <= self.buf.len() {
            self.index += 1;
            Some(&self.buf[start..upper])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _01_example() {
        let sonar_sweep: [isize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let windowed_sum = Windowed::new(&sonar_sweep, 3)
            .map(|w| w.iter().cloned().sum::<isize>())
            .collect::<Vec<_>>();

        assert_eq!(windowed_sum, vec![607, 618, 618, 617, 647, 716, 769, 792]);
    }
}

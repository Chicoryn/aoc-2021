use std::cmp::Ordering;

pub struct OrderingIter<T: PartialOrd + Clone, I: Iterator<Item = T>> {
    previous_measurement: Option<T>,
    measurements: I,
}

impl<T: PartialOrd + Clone, I: Iterator<Item = T>> OrderingIter<T, I> {
    pub fn new(mut measurements: I) -> Self {
        let previous_measurement = measurements.next();

        OrderingIter {
            previous_measurement: previous_measurement,
            measurements,
        }
    }
}

impl<T: PartialOrd + Clone, I: Iterator<Item = T>> Iterator for OrderingIter<T, I> {
    type Item = Ordering;

    fn next(&mut self) -> Option<Self::Item> {
        self.measurements.next().and_then(|curr| {
            if let Some(previous_measurement) = self.previous_measurement.clone() {
                self.previous_measurement = Some(curr.clone());

                curr.partial_cmp(&previous_measurement)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_is_empty() {
        let ordering = OrderingIter::new(Vec::<isize>::new().iter()).collect::<Vec<_>>();
        assert_eq!(ordering, vec![]);
    }

    #[test]
    fn iter_has_one_element() {
        let ordering = OrderingIter::new([0].iter()).collect::<Vec<_>>();
        assert_eq!(ordering, vec![]);
    }

    #[test]
    fn _01_example() {
        let sonar_sweep: [isize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let ordering = OrderingIter::new(sonar_sweep.iter()).collect::<Vec<_>>();

        assert_eq!(
            ordering,
            vec![
                Ordering::Greater,
                Ordering::Greater,
                Ordering::Greater,
                Ordering::Less,
                Ordering::Greater,
                Ordering::Greater,
                Ordering::Greater,
                Ordering::Less,
                Ordering::Greater
            ]
        );
    }
}

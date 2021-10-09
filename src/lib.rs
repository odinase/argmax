pub trait Argmax {
    type Maximum;

    fn argmax(self) -> Option<(usize, Self::Maximum)>;
}

impl<I> Argmax for I
where
    I: Iterator,
    I::Item: std::cmp::PartialOrd,
{
    type Maximum = I::Item;

    fn argmax(mut self) -> Option<(usize, Self::Maximum)> {
        let v0 = self.next()?;

        Some(
            self.enumerate()
                .fold((0, v0), |(i_max, val_max), (i, val)| {
                    if val > val_max {
                        (i + 1, val) // Add 1 as index is one off due to next() above
                    } else {
                        (i_max, val_max)
                    }
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_in_array() {
        assert_eq!(Some((3, &4.)), [1., 2., 3., 4., 3., 2., 1.].iter().argmax());
    }

    #[test]
    fn test_max_start_of_array() {
        assert_eq!(
            Some((0, &100.)),
            [100., 2., 3., 4., 3., 2., 1.].iter().argmax()
        );
    }

    #[test]
    fn empty_array() {
        let v: [u32; 0] = [];
        assert_eq!(
            None,
            v.iter().argmax()
        );
    }

    #[test]
    fn finds_first_maximum_array() {
        let v = [1, 1, 1, 1, 1];
        assert_eq!(
            Some((0, &1)),
            v.iter().argmax()
        );
    }
}

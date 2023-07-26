use std::iter::Peekable;
use std::ops::Div;

pub(crate) struct Averager<
    'a,
    T: Iterator<Item = &'a (u8, f32)>,
    R: Iterator<Item = &'a (u8, f32)>,
    const WEIGHT_A: usize,
    const WEIGHT_B: usize,
>(pub Peekable<T>, pub Peekable<R>);

impl<
        'a,
        T: Iterator<Item = &'a (u8, f32)>,
        R: Iterator<Item = &'a (u8, f32)>,
        const WEIGHT_A: usize,
        const WEIGHT_B: usize,
    > Iterator for Averager<'a, T, R, WEIGHT_A, WEIGHT_B>
{
    type Item = (u8, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let Some((a, _ap)) = self.0.peek() else {
            return self.1.next().cloned();
        };
        let Some((b, _bp)) = self.1.peek() else {
            return self.0.next().cloned();
        };
        match a.cmp(b) {
            std::cmp::Ordering::Less => self.0.next().cloned(),
            std::cmp::Ordering::Equal => {
                let (a, ap) = self.0.next().unwrap();
                let (_b, bp) = self.1.next().unwrap();
                Some((
                    *a,
                    (*ap * (WEIGHT_A as f32) + *bp * (WEIGHT_B as f32))
                        .div(WEIGHT_A as f32 + WEIGHT_B as f32),
                ))
            }
            std::cmp::Ordering::Greater => self.1.next().cloned(),
        }
    }
}

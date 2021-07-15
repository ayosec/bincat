pub struct Cycle<'a, T> {
    index: usize,
    items: &'a [T],
}

impl<'a, T> Cycle<'a, T> {
    pub fn new<R: AsRef<[T]>>(items: &'a R) -> Self {
        Self {
            index: 0,
            items: items.as_ref(),
        }
    }

    pub fn next(&mut self) -> &T {
        let item = &self.items[self.index];
        self.index = (self.index + 1..self.items.len()).next().unwrap_or(0);
        item
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}

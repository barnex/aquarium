use crate::prelude::*;

impl G {
    pub fn random<T>(&self) -> T
    where
        rand::distributions::Standard: rand::prelude::Distribution<T>,
    {
        self._rng.borrow_mut().r#gen()
    }

    pub fn random_range<T>(&self, range: Range<T>) -> T
    where
        rand::distributions::Standard: rand::prelude::Distribution<T>,
        T: rand::distributions::uniform::SampleUniform,
        T: PartialOrd,
    {
        self._rng.borrow_mut().gen_range(range)
    }

    pub fn random_range_incl<T>(&self, range: RangeInclusive<T>) -> T
    where
        rand::distributions::Standard: rand::prelude::Distribution<T>,
        T: rand::distributions::uniform::SampleUniform,
        T: PartialOrd,
    {
        self._rng.borrow_mut().gen_range(range)
    }

    pub fn random_vec<T>(&self, range: Range<T>) -> vec2<T>
    where
        rand::distributions::Standard: rand::prelude::Distribution<T>,
        T: rand::distributions::uniform::SampleUniform,
        T: PartialOrd,
        T: Clone,
    {
        vec2(self.random_range(range.clone()), self.random_range(range))
    }

    pub fn random_vec_incl<T>(&self, range: RangeInclusive<T>) -> vec2<T>
    where
        rand::distributions::Standard: rand::prelude::Distribution<T>,
        T: rand::distributions::uniform::SampleUniform,
        T: PartialOrd,
        T: Clone,
    {
        vec2(self.random_range_incl(range.clone()), self.random_range_incl(range))
    }

    pub fn pick_random<T: Copy, const N: usize>(&self, options: [T; N]) -> T {
        let mut rng = self._rng.borrow_mut();
        let i = rng.gen_range(0..options.len());
        options[i]
    }
}

use std::{
    ops::{Deref, DerefMut},
    slice::SliceIndex,
};

// Owned scratch/tables with explicit unchecked access; growth and ordinary indexing stay safe.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct UncheckedVec<T>(Vec<T>);

impl<T> UncheckedVec<T> {
    // SAFETY: callers must prove the index/range is within the initialized length.
    #[inline]
    pub(crate) unsafe fn get_unchecked<I: SliceIndex<[T]>>(&self, index: I) -> &I::Output {
        #[cfg(debug_assertions)]
        {
            self.0.get(index).expect("UncheckedVec index out of bounds")
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            self.0.get_unchecked(index)
        }
    }

    // SAFETY: callers must prove the index/range is within the initialized length.
    #[inline]
    pub(crate) unsafe fn get_unchecked_mut<I: SliceIndex<[T]>>(
        &mut self,
        index: I,
    ) -> &mut I::Output {
        #[cfg(debug_assertions)]
        {
            self.0
                .get_mut(index)
                .expect("UncheckedVec index out of bounds")
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            self.0.get_unchecked_mut(index)
        }
    }
}

impl<T> Default for UncheckedVec<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> From<Vec<T>> for UncheckedVec<T> {
    fn from(values: Vec<T>) -> Self {
        Self(values)
    }
}

impl<T> Deref for UncheckedVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for UncheckedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::UncheckedVec;
    use std::{cell::Cell, rc::Rc};

    #[test]
    fn indexed_access_tracks_growth_clear_and_ranges() {
        let mut values = UncheckedVec::from(vec![1, 2]);
        for value in 3..100 {
            values.push(value);
        }
        unsafe {
            assert_eq!(*values.get_unchecked(98), 99);
            values.get_unchecked_mut(1..3).copy_from_slice(&[20, 30]);
            assert_eq!(values.get_unchecked(..3), &[1, 20, 30]);
            assert!(values.get_unchecked(99..99).is_empty());
        }
        values.clear();
        values.push(7);
        assert_eq!(unsafe { *values.get_unchecked(0) }, 7);
        assert_eq!(values.get(1), None);
    }

    #[test]
    fn replacement_and_truncation_drop_owned_elements_once() {
        struct CountDrop(Rc<Cell<usize>>);
        impl Drop for CountDrop {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        let drops = Rc::new(Cell::new(0));
        let mut values =
            UncheckedVec::from(vec![CountDrop(drops.clone()), CountDrop(drops.clone())]);
        unsafe {
            *values.get_unchecked_mut(0) = CountDrop(drops.clone());
        }
        assert_eq!(drops.get(), 1);
        values.truncate(1);
        assert_eq!(drops.get(), 2);
        drop(values);
        assert_eq!(drops.get(), 3);
    }
}

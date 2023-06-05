pub(crate) trait VecExt {
    unsafe fn remove_last_unchecked(&mut self);
}

impl<T> VecExt for Vec<T> {
    unsafe fn remove_last_unchecked(&mut self) {
        self.set_len(self.len() - 1);
    }
}

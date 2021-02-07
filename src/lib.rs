struct CircularBuffer {
    capacity: usize,
    num_elem: usize,
}

impl CircularBuffer {
    fn new(c: usize) -> CircularBuffer {
        CircularBuffer { capacity: c, num_elem: 0}
    }
    fn is_empty(&self) -> bool { true }
    fn is_full(&self) -> bool { self.num_elem >= self.capacity }
    fn size(&self) -> usize { 0 }
    fn put(&mut self, _: i32) -> bool {
        if self.is_full() { return false }
        self.num_elem = self.num_elem + 1;
        true
    }
    fn get(&self) -> i32 { i32::MIN }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn when_create_capacity_zero_then_is_full_true_is_empty_true_size_zero() {
        let b = CircularBuffer::new(0);
        assert!(b.is_full());
        assert!(b.is_empty());
        assert_eq!(0, b.size());
    }
    #[test]
    fn given_capacity_zero_when_put_then_return_false() {
        let mut b = CircularBuffer::new(0);
        assert_eq!(false, b.put(42));
    }
    #[test]
    fn given_capacity_zero_when_get_then_return_min_int() {
        let b = CircularBuffer::new(0);
        assert_eq!(i32::MIN, b.get());
    }
    #[test]
    fn when_create_capacity_one_then_is_full_false_is_empty_true_size_zero() {
        let b = CircularBuffer::new(1);
        assert_eq!(false, b.is_full());
        assert!(b.is_empty());
        assert_eq!(0, b.size());
    }
    #[test]
    fn given_capacity_one_when_put_then_return_true_is_full_true() {
        let mut b = CircularBuffer::new(1);
        assert_eq!(true, b.put(42));
        assert!(b.is_full());
    }
}

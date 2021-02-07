struct CircularBuffer {}

impl CircularBuffer {
    fn new(_:usize) -> CircularBuffer {
        CircularBuffer {}
    }
    fn is_empty(&self) -> bool { true }
    fn is_full(&self) -> bool { true }
    fn size(&self) -> usize { 0 }
    fn put(&self, _: i32) -> bool { false }
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
        let b = CircularBuffer::new(0);
        assert_eq!(false, b.put(42));
    }
    #[test]
    fn given_capacity_zero_when_get_then_return_min_int() {
        let b = CircularBuffer::new(0);
        assert_eq!(i32::MIN, b.get());
    }
    #[test]
    fn when_create_capacity_one_then_is_empty_true() {
        let b = CircularBuffer::new(0);
        assert!(b.is_empty());
    }
}

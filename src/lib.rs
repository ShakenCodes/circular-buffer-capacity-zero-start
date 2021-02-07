struct CircularBuffer {}

impl CircularBuffer {
    fn new(_:usize) -> CircularBuffer {
        CircularBuffer {}
    }
    fn is_empty(&self) -> bool { true }
    fn is_full(&self) -> bool { true }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn when_create_capacity_zero_then_is_full_true_is_empty_true() {
        let b = CircularBuffer::new(0);
        assert!(b.is_full());
        assert!(b.is_empty())
    }
}

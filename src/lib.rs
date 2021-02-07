struct CircularBuffer {}

impl CircularBuffer {
    fn new(_:usize) -> CircularBuffer {
        CircularBuffer {}
    }
    fn is_full(&self) -> bool { true }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn when_create_zero_size_then_is_full_true() {
        let b = CircularBuffer::new(0);
        assert!(b.is_full());
    }
}

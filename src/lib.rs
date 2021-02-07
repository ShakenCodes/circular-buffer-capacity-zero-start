struct CircularBuffer {
    capacity: usize,
    num_elem: usize,
    at_in: usize,
    at_out: usize,
    elems: Vec<i32>,
}

impl CircularBuffer {
    fn new(c: usize) -> CircularBuffer {
        CircularBuffer {
            capacity: c, num_elem: 0,
            at_in: 0, at_out: 0,
            elems: vec![i32::MIN, c as i32] }
    }
    fn is_empty(&self) -> bool { self.num_elem == 0 }
    fn is_full(&self) -> bool { self.num_elem >= self.capacity }
    fn size(&self) -> usize { 0 }
    fn put(&mut self, v: i32) -> bool {
        if self.is_full() { return false }
        self.elems[self.at_in] = v;
        self.num_elem = self.num_elem + 1;
        self.at_in = self.at_in + 1;
        if self.at_in >= self.capacity { self.at_in = 0; }
        true
    }
    fn get(&mut self) -> i32 {
        if self.is_empty() { return i32::MIN }
        let v = self.elems[self.at_out];
        self.num_elem = self.num_elem - 1;
        self.at_out = self.at_out + 1;
        if self.at_out >= self.capacity { self.at_out = 0; }
        v
    }
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
        let mut b = CircularBuffer::new(0);
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
    fn given_capacity_one_when_put_then_return_true_is_full_true_is_empty_false() {
        let mut b = CircularBuffer::new(1);
        assert_eq!(true, b.put(42));
        assert!(b.is_full());
        assert_eq!(false, b.is_empty());
    }
    #[test]
    fn given_capacity_one_and_one_put_when_get_then_return_value_in_put() {
        let v = 99;
        let mut b = CircularBuffer::new(1);
        assert!(b.put(v));
        assert_eq!(v, b.get());
    }
    #[test]
    fn given_capacity_one_and_one_put_when_get_then_is_full_false_is_empty_true_size_zero() {
        let mut b = CircularBuffer::new(1);
        assert!(b.put(43));
        b.get();
        assert_eq!(false, b.is_full());
        assert!(b.is_empty());
        assert_eq!(0, b.size());
    }
    #[test]
    fn given_capacity_one_and_one_put_when_get_twice_then_return_is_min_int() {
        let mut b = CircularBuffer::new(1);
        assert!(b.put(43));
        b.get();
        assert_eq!(i32::MIN, b.get());
    }
    #[test]
    fn given_capacity_two_when_put_then_return_true_is_full_false_is_empty_false() {
        let mut b = CircularBuffer::new(2);
        assert_eq!(true, b.put(42));
        assert_eq!(false, b.is_full());
        assert_eq!(false, b.is_empty());
    }
    #[test]
    fn given_capacity_two_when_put_twice_then_return_true_is_full_true_is_empty_false() {
        let mut b = CircularBuffer::new(2);
        assert!(b.put(-1));
        assert!(b.put(42));
        assert_eq!(true, b.is_full());
        assert_eq!(false, b.is_empty());
    }
    #[test]
    fn given_capacity_two_and_put_twice_when_get_twice_then_value_returned_matches_put() {
        let v1 = 11;
        let v2 = -99;
        let mut b = CircularBuffer::new(2);
        assert!(b.put(v1));
        assert!(b.put(v2));
        assert_eq!(v1, b.get());
        assert_eq!(v2, b.get());
    }
    #[test]
    fn given_capacity_two_and_put_twice_get_put_again_when_get_twice_then_value_returned_matches_later_puts() {
        let v1 = 11;
        let v2 = -99;
        let v3 = 2020;
        let mut b = CircularBuffer::new(2);
        assert!(b.put(v1));
        assert!(b.put(v2));
        b.get();
        assert!(b.put(v3));
        assert_eq!(v2, b.get());
        assert_eq!(v3, b.get());
    }
}

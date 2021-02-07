struct CircularBuffer {
    capacity: usize,
    num_elem: usize,
    at_in: usize,
    at_out: usize,
    elems: Vec<i32>,
}

impl CircularBuffer {
    pub fn new(c: usize) -> CircularBuffer {
        CircularBuffer {
            capacity: c,
            num_elem: 0,
            at_in: 0,
            at_out: 0,
            elems: vec![i32::MIN; c + 1]
        }
    }
    pub fn is_empty(&self) -> bool { self.num_elem == 0 }
    pub fn is_full(&self) -> bool { self.num_elem >= self.capacity }
    pub fn put(&mut self, v: i32) -> bool {
        if self.is_full() { return false }
        self.elems[self.at_in] = v;
        self.num_elem = self.num_elem + 1;
        self.at_in = CircularBuffer::increment_and_clip(self.at_in, self.capacity);
        true
    }
    pub fn get(&mut self) -> i32 {
        if self.is_empty() { return i32::MIN }
        let v = self.elems[self.at_out];
        self.num_elem = self.num_elem - 1;
        self.at_out = CircularBuffer::increment_and_clip(self.at_out, self.capacity);
        v
    }
    fn increment_and_clip(n: usize, c: usize) -> usize {
        if n >= c { return 0 }
        n + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn when_create_capacity_zero_then_is_full_true_is_empty_true() {
        let b = CircularBuffer::new(0);
        assert!(b.is_full());
        assert!(b.is_empty());
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
    fn when_create_capacity_one_then_is_full_false_is_empty_true() {
        let b = CircularBuffer::new(1);
        assert_eq!(false, b.is_full());
        assert!(b.is_empty());
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
        let mut b = CircularBuffer::new(1);
        let v = 99;
        assert!(b.put(v));
        assert_eq!(v, b.get());
    }
    #[test]
    fn given_capacity_one_and_one_put_when_get_then_is_full_false_is_empty_true() {
        let mut b = create_full_buffer(1);
        b.get();
        assert_eq!(false, b.is_full());
        assert!(b.is_empty());
    }
    #[test]
    fn given_capacity_one_and_one_put_when_get_twice_then_return_is_min_int() {
        let mut b = create_full_buffer(1);
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
        let b = create_full_buffer(2);
        assert_eq!(true, b.is_full());
        assert_eq!(false, b.is_empty());
    }
    #[test]
    fn given_capacity_two_and_put_twice_when_get_twice_then_value_returned_matches_put() {
        let mut b = CircularBuffer::new(2);
        let v1 = 11;
        let v2 = -99;
        assert!(b.put(v1));
        assert!(b.put(v2));
        assert_eq!(v1, b.get());
        assert_eq!(v2, b.get());
    }
    #[test]
    fn given_capacity_two_and_put_twice_get_put_again_when_get_twice_then_value_returned_matches_later_puts() {
        let mut b = CircularBuffer::new(2);
        assert!(b.put(44));
        let v2 = -99;
        let v3 = 2020;
        assert!(b.put(v2));
        b.get();
        assert!(b.put(v3));

        assert_eq!(v2, b.get());
        assert_eq!(v3, b.get());
    }
    #[test]
    fn given_capacity_two_and_put_twice_get_twice_put_again_when_get_twice_then_value_returned_matches_last_put() {
        let mut b = create_full_buffer(2);
        get_n_times(&mut b, 2);
        let v = 11;
        assert!(b.put(v));
        assert_eq!(v, b.get());
    }
    #[test]
    fn given_capacity_two_and_put_twice_get_twice_put_twice_again_when_get_twice_then_value_returned_matches_last_put() {
        let mut b = create_full_buffer(2);
        get_n_times(&mut b, 2);
        let v = 11;
        assert!(b.put(v));
        assert_eq!(v, b.get());
    }
    #[test]
    fn given_capacity_two_and_put_twice_when_get_twice_then_is_full_false_is_empty_true() {
        let mut b = create_full_buffer(2);
        get_n_times(&mut b, 2);
        assert_eq!(false, b.is_full());
        assert_eq!(true, b.is_empty());
    }
    #[test]
    fn given_capacity_two_and_put_twice_get_twice_when_put_twice_then_is_full_true_is_empty_false() {
        let mut b = create_full_buffer(2);
        get_n_times(&mut b, 2);
        put_n_times(&mut b, 2);
        assert_eq!(true, b.is_full());
        assert_eq!(false, b.is_empty());
    }
    #[test]
    fn given_capacity_and_filled_and_emptied_twice_when_put_and_get_then_last_value_matches() {
        let mut b = create_full_buffer(10);
        get_n_times(&mut b, 10);
        put_n_times(&mut b, 10);
        get_n_times(&mut b, 10);

        let v = 99;
        assert!(b.put(v));
        assert_eq!(v, b.get());
        assert_eq!(true, b.is_empty());
        assert_eq!(false, b.is_full());
    }
    #[test]
    fn given_capacity_and_filled_and_emptied_twice_then_is_empty_true_is_full_false() {
        let mut b = create_full_buffer(10);
        get_n_times(&mut b, 10);
        put_n_times(&mut b, 10);
        get_n_times(&mut b, 10);
        assert_eq!(true, b.is_empty());
        assert_eq!(false, b.is_full());
    }
    #[test]
    fn given_capacity_and_filled_and_emptied_twice_when_filled_again_then_is_empty_false_is_full_true() {
        let mut b = create_full_buffer(10);
        get_n_times(&mut b, 10);
        put_n_times(&mut b, 10);
        get_n_times(&mut b, 10);
        put_n_times(&mut b, 10);
        assert_eq!(false, b.is_empty());
        assert_eq!(true, b.is_full());
    }

    fn create_full_buffer(c: usize) -> CircularBuffer {
        let mut b = CircularBuffer::new(c);
        put_n_times(&mut b, c);
        b
    }
    fn get_n_times(b: &mut CircularBuffer, n: usize) {
        for _ in 0..n {
            b.get();
        }
    }
    fn put_n_times(b: &mut CircularBuffer, n: usize) {
        for i in 42..(42 + n) {
            b.put(i as i32);
        }
    }
}

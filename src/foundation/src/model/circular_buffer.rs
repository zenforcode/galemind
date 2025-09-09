/* A generic circular buffer (ring buffer) implementation.

The `CircularBuffer<T>` stores up to a fixed number of elements (`capacity`).
New items are appended until the buffer reaches its capacity.
When capacity is full, new items overwrite the oldest ones in a circular way.

Key details:
- `push` inserts a new element, overwriting the oldest when full.
- `items` returns a slice of the current buffer contents in their stored order.
- `capacity` returns available capacity
- `len` returns current length
- `is_empty` checks if buffer is empty
- `is_full` checks if buffer is full
*/

#[derive(Debug, Default)]
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
    index: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
            index: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() < self.capacity {
            self.buffer.push(item);
        } else {
            self.buffer[self.index] = item;
        }
        self.index = (self.index + 1) % self.capacity;
    }

    pub fn items(&self) -> &[T] {
        &self.buffer
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.buffer.len() == self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_buffer() {
        let buf: CircularBuffer<i32> = CircularBuffer::new(3);
        assert_eq!(buf.len(), 0);
        assert!(buf.is_empty());
        assert!(!buf.is_full());
    }

    #[test]
    fn test_full_buffer() {
        let mut buf: CircularBuffer<i32> = CircularBuffer::new(3);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        assert!(!buf.is_empty());
        assert!(buf.is_full());
    }

    #[test]
    fn test_len_matches_items() {
        let mut buf = CircularBuffer::new(3);
        assert_eq!(buf.len(), buf.items().len());

        buf.push(1);
        buf.push(2);
        assert_eq!(buf.len(), buf.items().len());

        buf.push(3);
        buf.push(4); // overwrite
        assert_eq!(buf.len(), buf.items().len());
    }

    #[test]
    fn test_push_within_capacity() {
        let mut buf = CircularBuffer::new(3);
        buf.push(1);
        buf.push(2);
        assert_eq!(buf.items(), &[1, 2]);
        assert_eq!(buf.len(), 2);
    }

    #[test]
    fn test_push_overwrites_first_when_full() {
        let mut buf = CircularBuffer::new(3);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.push(4); // overwrites 1
        assert_eq!(buf.items(), &[4, 2, 3]);
    }

    #[test]
    fn test_push_wraps_around() {
        let mut buf = CircularBuffer::new(2);
        buf.push(10);
        buf.push(20);
        buf.push(30); // overwrites 10
        buf.push(40); // overwrites 20
        assert_eq!(buf.items(), &[30, 40]);
    }

    #[test]
    fn test_push_when_capacity_one() {
        let mut buf = CircularBuffer::new(1);
        buf.push(5);
        buf.push(6);
        assert_eq!(buf.items(), &[6]); // only the last survives
    }
}

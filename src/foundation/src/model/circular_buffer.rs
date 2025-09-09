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

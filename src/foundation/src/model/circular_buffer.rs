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

// Circular/ring buffers work by writing to head and reading from tail
pub struct CircularBuffer<T> {
    head: usize,
    tail: usize,
    buffer: Vec<Option<T>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            head: 0,
            tail: 0,
            buffer: (0..capacity).map(|_| None).collect::<Vec<Option<T>>>(),
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if !self.buffer.iter().any(|a| a.is_none()) {
            Err(Error::FullBuffer)
        } else {
            // Write to head, then increase head index
            // If head is pointing to last element, circle back to beginning
            self.buffer[self.head] = Some(_element);
            self.head = if self.head == self.buffer.len()-1 {0} else {self.head+1};
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer[self.tail].is_none() {
            return Err(Error::EmptyBuffer);
        } else {
            // Read from current index
            // If tail is pointing to last element, circle back to beginning
            let result = self.buffer[self.tail].take().unwrap();
            self.tail = if self.tail == self.buffer.len()-1 {0} else {self.tail+1};
            Ok(result)
        }
    }

    pub fn clear(&mut self) {
        // Re-write buffer with empty buffer
        // Re-initialize head and tail
        self.buffer = (0..self.buffer.len()).map(|_| None).collect::<Vec<Option<T>>>();
        self.head = 0;
        self.tail = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        // Head points to data, overwrite with new data
        // If head is pointing to last element, circle back to beginning
        // The buffer is still full so tail should point to same element as head
        self.buffer[self.head] = Some(_element);
        self.head = if self.head == self.buffer.len()-1 {0} else {self.head+1};
        self.tail = self.head;
    }

}

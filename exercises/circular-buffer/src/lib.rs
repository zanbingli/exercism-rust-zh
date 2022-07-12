pub struct CircularBuffer<T> {
    field: Vec<T>,
    read_ptr: usize,
    write_ptr: usize,
    caps: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
    where T: Clone + Default {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            field: vec![T::default(); capacity + 1],
            read_ptr: 0,
            write_ptr: 0,
            caps: capacity + 1,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.field.is_empty() || self.write_ptr == self.read_ptr {
            return Err(Error::EmptyBuffer);
        }
        let value = self.field[self.read_ptr].clone();
        self.read_ptr = (self.read_ptr + 1) % self.caps;
        Ok(value)
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.field[self.write_ptr] = _element;
        self.write_ptr = (self.write_ptr + 1) % self.caps;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.read_ptr = 0;
        self.write_ptr = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.is_full()  {
            self.read_ptr = (self.read_ptr + 1) % self.caps;
        }
        self.field[self.write_ptr] = _element;
        self.write_ptr = (self.write_ptr + 1) % self.caps;
    }

    pub fn is_full(&self) -> bool {
        // (self.write_ptr + 1) % self.caps == self.read_ptr
        (self.read_ptr as i32 - self.write_ptr as i32 + self.caps as i32) % self.caps as i32 == 1
    }
}

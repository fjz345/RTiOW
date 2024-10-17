use std::{
    mem::{self, ManuallyDrop, MaybeUninit},
    ops::Index,
    vec::IntoIter,
};

#[derive(Debug)]
pub struct RingBuffer<T, const N: usize> {
    data: MaybeUninit<[T; N]>,
    // data: [T; N],
    write_loc: usize,
    read_loc: usize,
    max_entries: usize,
}

impl<T, const N: usize> Iterator for RingBuffer<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        let maybe_uninit_data: MaybeUninit<[T; N]> = MaybeUninit::uninit();
        let write_loc = 0;
        let read_loc = 0;
        let max_entries = N;

        Self {
            // data: unsafe { maybe_uninit.assume_init() },
            data: maybe_uninit_data,
            write_loc,
            read_loc,
            max_entries,
        }
    }

    pub fn len(&self) -> usize {
        self.write_loc - self.read_loc
    }

    pub fn empty(&mut self) {
        while self.len() > 1 {
            self.pop();
        }
    }

    pub fn push(&mut self, entry: T) {
        assert_eq!(self.len() < self.max_entries, true, "RingBuffer Overflow");

        unsafe { self.data.assume_init_mut()[self.write_loc % self.max_entries] = entry };
        self.write_loc += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() <= 0 {
            return None;
        }

        let data = unsafe {
            mem::transmute_copy(&self.data.assume_init_mut()[self.read_loc % self.max_entries])
        };

        self.read_loc += 1;
        Some(data)
    }
}

impl<T, const N: usize> Drop for RingBuffer<T, N> {
    fn drop(&mut self) {
        while self.len() > 1 {
            drop(self.pop());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::PixelFuture;

    use super::*;

    #[test]
    fn test_ringbuffer_usize_unused() {
        println!("BEFORE");
        let mut _ringbuffer: RingBuffer<usize, 160> = RingBuffer::new();
        println!("AFTER");
    }

    #[test]
    fn test_ringbuffer_pixelfuture_unused() {
        println!("BEFORE");
        let mut _ringbuffer: RingBuffer<PixelFuture, 160> = RingBuffer::new();
        println!("AFTER");
    }

    #[test]
    fn test_ringbuffer_usize_equal_push_pop() {
        const RINGBUFFER_SIZE: usize = 10;
        let mut buffer: RingBuffer<usize, RINGBUFFER_SIZE> = RingBuffer::new();

        const PUSH_NUM: usize = 10;
        let mut push_counter: usize = 0;
        while push_counter < PUSH_NUM {
            let to_push = push_counter;
            println!("Push: {}", to_push);
            buffer.push(to_push);
            push_counter += 1;
            assert_eq!(buffer.len(), push_counter);
        }

        let mut pop_counter = push_counter;
        while buffer.len() >= 1 {
            let popped = buffer.pop().unwrap();
            println!("Pop: {}", popped);
            pop_counter -= 1;
            assert_eq!(buffer.len(), pop_counter);
        }

        assert_eq!(buffer.len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_ringbuffer_usize_overflow() {
        const RINGBUFFER_SIZE: usize = 10;
        let mut buffer: RingBuffer<usize, RINGBUFFER_SIZE> = RingBuffer::new();

        const PUSH_NUM: usize = 11;
        let mut push_counter: usize = 0;
        while push_counter < PUSH_NUM {
            let to_push = push_counter;
            println!("Push: {}", to_push);
            buffer.push(to_push);
            push_counter += 1;
            assert_eq!(buffer.len(), push_counter);
        }

        let mut pop_counter = push_counter;
        while buffer.len() >= 1 {
            let popped = buffer.pop().unwrap();
            println!("Pop: {}", popped);
            pop_counter -= 1;
            assert_eq!(buffer.len(), pop_counter);
        }

        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_ringbuffer_usize_test3() {
        const RINGBUFFER_SIZE: usize = 10;
        let mut buffer: RingBuffer<usize, RINGBUFFER_SIZE> = RingBuffer::new();

        const PUSH_NUM: usize = 9;
        let mut push_counter = 0;
        while push_counter < PUSH_NUM {
            let to_push = push_counter;
            println!("Push: {}", to_push);
            buffer.push(to_push);
            push_counter += 1;
            assert_eq!(buffer.len(), push_counter);
        }

        let mut pop_counter = push_counter;
        while buffer.len() >= 1 {
            let popped = buffer.pop().unwrap();
            println!("Pop: {}", popped);
            pop_counter -= 1;
            assert_eq!(buffer.len(), pop_counter);
        }

        assert_eq!(buffer.len(), 0);

        buffer.push(111);
        buffer.push(222);
        buffer.push(333);
        assert_eq!(buffer.len(), 3);

        buffer.pop();
        assert_eq!(buffer.len(), 2);
        buffer.push(444);
        assert_eq!(buffer.len(), 3);
        buffer.push(555);
        assert_eq!(buffer.len(), 4);
        buffer.pop();
        buffer.pop();
        buffer.pop();
        assert_eq!(buffer.len(), 1);

        buffer.push(666);
        buffer.push(777);
        buffer.push(888);
        buffer.push(999);
        assert_eq!(buffer.len(), 5);
    }

    #[test]
    fn test_ringbuffer_usize_test4() {
        const RINGBUFFER_SIZE: usize = 10;
        let mut buffer: RingBuffer<usize, RINGBUFFER_SIZE> = RingBuffer::new();

        const PUSH_NUM: usize = 9;
        let mut push_counter = 0;
        while push_counter < PUSH_NUM {
            let to_push = push_counter;
            println!("Push: {}", to_push);
            buffer.push(to_push);
            push_counter += 1;
            assert_eq!(buffer.len(), push_counter);
        }

        let mut pop_counter = push_counter;
        while buffer.len() >= 1 {
            let popped = buffer.pop().unwrap();
            println!("Pop: {}", popped);
            pop_counter -= 1;
            assert_eq!(buffer.len(), pop_counter);
        }

        assert_eq!(buffer.len(), 0);

        buffer.push(111);
        buffer.push(222);
        buffer.push(333);
        assert_eq!(buffer.len(), 3);

        buffer.pop();
        assert_eq!(buffer.len(), 2);
        buffer.push(444);
        assert_eq!(buffer.len(), 3);
        buffer.push(555);
        assert_eq!(buffer.len(), 4);
        buffer.pop();
        buffer.pop();
        buffer.pop();
        assert_eq!(buffer.len(), 1);

        buffer.push(666);
        buffer.push(777);
        buffer.push(888);
        buffer.push(999);
        assert_eq!(buffer.len(), 5);

        const PUSH_NUM_2: usize = 5;
        push_counter = 0;
        while push_counter < PUSH_NUM_2 {
            let to_push = push_counter;
            println!("Push: {}", to_push);
            buffer.push(to_push);
            push_counter += 1;
        }

        pop_counter = push_counter;
        while pop_counter >= 1 {
            let popped = buffer.pop().unwrap();
            println!("Pop: {}", popped);
            pop_counter -= 1;
        }
        assert_eq!(buffer.len(), 5);
    }

    #[test]
    #[should_panic]
    fn test_ringbuffer_usize_pop_before_push() {
        const RINGBUFFER_SIZE: usize = 10;
        let mut buffer: RingBuffer<usize, RINGBUFFER_SIZE> = RingBuffer::new();
        buffer.pop().unwrap();
    }
}

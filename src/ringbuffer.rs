use std::mem::{self, MaybeUninit};

#[derive(Debug)]
pub struct RingBuffer<T, const N: usize> {
    data: MaybeUninit<[T; N]>,
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
        while self.len() >= 1 {
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
    use rand::RngCore;

    use crate::renderer::PixelFuture;

    use super::*;

    #[test]
    fn test_ringbuffer_unused() {
        println!("usize BEFORE");
        let mut _ringbuffer: RingBuffer<usize, 160> = RingBuffer::new();
        println!("usize AFTER");

        println!("PixelFuture BEFORE");
        let mut _ringbuffer: RingBuffer<PixelFuture, 160> = RingBuffer::new();
        println!("PixelFuture AFTER");
    }

    #[test]
    fn test_ringbuffer_usize() {
        const RINGBUFFER_SIZE: usize = 10;
        println!("Buffersize: {RINGBUFFER_SIZE}, 0 spot left");
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

        const RINGBUFFER_SIZE_2: usize = 10;
        println!("Buffersize: {RINGBUFFER_SIZE_2}, 1 spot left");
        let mut buffer_2: RingBuffer<usize, RINGBUFFER_SIZE_2> = RingBuffer::new();

        const PUSH_NUM_2: usize = 9;
        let mut push_counter = 0;
        while push_counter < PUSH_NUM_2 {
            let to_push = push_counter;
            println!("Push: {}", to_push);
            buffer_2.push(to_push);
            push_counter += 1;
            assert_eq!(buffer_2.len(), push_counter);
        }

        let mut pop_counter = push_counter;
        while buffer_2.len() >= 1 {
            let popped = buffer_2.pop().unwrap();
            println!("Pop: {}", popped);
            pop_counter -= 1;
            assert_eq!(buffer_2.len(), pop_counter);
        }

        assert_eq!(buffer_2.len(), 0);

        const RINGBUFFER_SIZE_3: usize = 1572;
        println!("Buffersize: {RINGBUFFER_SIZE_3}");
        let mut buffer_3: RingBuffer<usize, RINGBUFFER_SIZE_3> = RingBuffer::new();

        const NUM_RANDOM_PUSH_POPS: usize = 898;
        let mut random_bits = [0u8; NUM_RANDOM_PUSH_POPS];
        rand::thread_rng().fill_bytes(&mut random_bits);
        let random_elements: [usize; NUM_RANDOM_PUSH_POPS] = [0; NUM_RANDOM_PUSH_POPS];
        rand::thread_rng().fill_bytes(&mut random_bits);

        let mut counter = 0;
        for ele in random_elements {
            let random_bit: bool = random_bits[counter] != 0;
            if random_bit && buffer_3.len() >= 1 {
                buffer_3.pop();
            } else {
                buffer_3.push(ele);
            }

            counter += 1;
        }

        buffer_3.empty();
        assert_eq!(buffer_3.len(), 0);
        const PUSH_NUM_3: usize = 5;
        push_counter = 0;
        while push_counter < PUSH_NUM_3 {
            let to_push = push_counter;
            println!("Push: {}", to_push);
            buffer_3.push(to_push);
            push_counter += 1;
        }

        pop_counter = push_counter;
        while pop_counter >= 1 {
            let popped = buffer_3.pop().unwrap();
            println!("Pop: {}", popped);
            pop_counter -= 1;
        }
        assert_eq!(buffer_3.len(), 0);
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
    #[should_panic]
    fn test_ringbuffer_usize_pop_before_push() {
        const RINGBUFFER_SIZE: usize = 10;
        let mut buffer: RingBuffer<usize, RINGBUFFER_SIZE> = RingBuffer::new();
        buffer.pop().unwrap();
    }

    #[test]
    fn test_ringbuffer_f32_iterator() {
        const RINGBUFFER_SIZE: usize = 10;
        println!("Buffer size {RINGBUFFER_SIZE}: loop enumerate");
        let mut buffer: RingBuffer<f32, RINGBUFFER_SIZE> = RingBuffer::new();

        buffer.push(32.0);
        buffer.push(1100.0);
        buffer.push(13320.0);
        for (i, f) in buffer.enumerate() {
            println!("[{i}]: {f}");
        }

        const RINGBUFFER_SIZE_2: usize = 10;
        println!("Buffer size {RINGBUFFER_SIZE_2}: into_iter()");
        let mut buffer: RingBuffer<f32, RINGBUFFER_SIZE_2> = RingBuffer::new();

        buffer.push(32.0);
        buffer.push(1100.0);
        buffer.push(13320.0);
        buffer.push(0.0);
        let mut iter = buffer.into_iter();
        iter.next().unwrap();
        iter.next().unwrap();

        iter.push(0.0);
        assert_eq!(iter.len(), 3);

        iter.empty();
        assert_eq!(iter.len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_ringbuffer_f32_underflow() {
        const RINGBUFFER_SIZE: usize = 10;
        let mut buffer: RingBuffer<f32, RINGBUFFER_SIZE> = RingBuffer::new();

        buffer.push(32.0);
        buffer.push(1100.0);
        buffer.push(13320.0);
        buffer.push(0.0);
        let mut iter = buffer.into_iter();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        iter.push(0.0);
        iter.pop().unwrap();
        iter.pop().unwrap();
        iter.pop().unwrap();
        iter.pop().unwrap();
    }
}

#![allow(dead_code)]

use std::fmt::{Display, Formatter};

const ENTITY_INDEX_BITS: u64 = 36;
const ENTITY_INDEX_MASK: u64 = (1 << ENTITY_INDEX_BITS) - 1;

const ENTITY_GENERATION_BITS: u32 = 28;
const ENTITY_GENERATION_MASK: u32 = (1 << ENTITY_GENERATION_BITS) - 1;

const CONTROL_BLOCK: usize = 0usize;
const DEFAULT_CAPACITY: usize = 16usize;

#[derive(Debug)]
pub struct Pool<T> {
    data: Vec<Slot<T>>,
}

impl<T> Pool<T> {
    #[inline]
    pub fn new() -> Self {
        let mut pool = Pool {
            data: Vec::<Slot<T>>::with_capacity(DEFAULT_CAPACITY),
        };
        Pool::init_pool(&mut pool);
        pool
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = {
            if capacity <= DEFAULT_CAPACITY {
                DEFAULT_CAPACITY
            } else if capacity > ENTITY_INDEX_BITS as usize + 1usize {
                panic!("capacity overflow: {}", capacity)
            } else {
                capacity
            }
        };
        let mut pool = Pool {
            data: Vec::<Slot<T>>::with_capacity(capacity),
        };
        Pool::init_pool(&mut pool);
        pool
    }

    pub fn add(&mut self, value: T) -> Index {
        // to do: check that we have sufficient capacity!!
        // for now, let's assume we have ...
        let (control, rest) = self.data.split_at_mut(CONTROL_BLOCK + 1usize);
        if let [Slot::Empty { next_empty, .. }] = control {
            let next_free_in_control = next_empty;
            let free_slot_index_in_slice = *next_free_in_control - 1usize;
            let slot = &mut rest[free_slot_index_in_slice];
            match slot {
                Slot::Empty { next_empty, gen } => {
                    let new_next_empty = *next_empty;
                    // to do: check 'next_gen' that we haven't reached usize::MAX which is invalid!
                    let next_gen = *gen;
                    let index = Index::new(*next_free_in_control as u64, next_gen);
                    *next_free_in_control = new_next_empty;
                    *slot = Slot::Occupied {
                        val: value,
                        gen: next_gen,
                    };
                    return index;
                }
                Slot::Occupied { .. } => {
                    panic!("index {} is already occupied!", *next_free_in_control);
                }
            }
        }
        panic!("control block is not empty!");
    }

    #[inline]
    fn init_pool(pool: &mut Pool<T>) {
        let capacity = pool.data.capacity();
        for i in 0..capacity {
            pool.data.push(Slot::initial_empty(i + 1usize));
        }
        // fix the 'next_empty' pointer in the last slot
        if let Slot::Empty { next_empty, .. } = &mut pool.data[capacity - 1usize] {
            *next_empty = usize::MAX;
        }
    }
}

impl<T> Default for Pool<T> {
    fn default() -> Self {
        Pool::new()
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Slot<T> {
    Empty { next_empty: usize, gen: u32 },
    Occupied { val: T, gen: u32 },
}

impl<T> Slot<T> {
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, Slot::Empty { .. })
    }

    #[inline]
    pub(crate) fn is_occupied(&self) -> bool {
        matches!(self, Slot::Occupied { .. })
    }

    #[inline]
    pub(crate) fn get(&self) -> Option<&T> {
        match self {
            Slot::Empty { .. } => None,
            Slot::Occupied { val, .. } => Some(val),
        }
    }

    #[inline]
    pub(crate) fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            Slot::Empty { .. } => None,
            Slot::Occupied { val, .. } => Some(val),
        }
    }

    #[inline]
    pub(crate) fn new_occupied(val: T, gen: u32) -> Self {
        Slot::Occupied { val, gen }
    }

    #[inline]
    pub(crate) fn new_empty(next_free: usize, gen: u32) -> Self {
        Slot::Empty {
            next_empty: next_free,
            gen,
        }
    }

    #[inline]
    pub(crate) fn initial_empty(next_free: usize) -> Self {
        Slot::new_empty(next_free, 0u32)
    }
}

// 36 bits for the index, 28 bits for the generation
#[derive(Clone, Copy, Debug)]
pub struct Index {
    id: u64,
}

impl Index {
    #[inline]
    pub fn index(&self) -> u64 {
        self.id & ENTITY_INDEX_MASK
    }

    #[inline]
    pub fn generation(&self) -> u32 {
        (self.id >> ENTITY_INDEX_BITS) as u32 & ENTITY_GENERATION_MASK
    }

    #[inline]
    pub fn new(idx: u64, gen: u32) -> Self {
        let id = (idx & ENTITY_INDEX_MASK)
            | (((gen & ENTITY_GENERATION_MASK) as u64) << (64 - ENTITY_GENERATION_BITS));
        Index { id }
    }
}

impl Display for Index {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "idx: {}, gen: {}", self.index(), self.generation())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn print_sizes() {
        let size1 = size_of::<Slot<()>>();
        println!("size: {}", size1);
        println!(
            "ENTITY_INDEX_MASK: {}",
            format!("{:#16x}", ENTITY_INDEX_MASK)
        );
        println!(
            "ENTITY_GENERATION_MASK: {}",
            format!("{:#8x}", ENTITY_GENERATION_MASK)
        );
    }

    #[test]
    fn print_index() {
        let idx = Index::new(1, 268_435_455);
        println!("Index Display: {}", idx);
        println!("Index Debug  : {:?}", idx);
    }

    #[test]
    fn test_create_pool_and_add() {
        let mut pool = Pool::<i32>::new();
        println!("pool: {:?}\n", pool);
        let elem = 42i32;
        let index = pool.add(elem);
        println!("new Index: {}", index);
        assert_eq!(1, index.index());
        println!("pool: {:?}\n", pool);
        let elem = 49i32;
        let index = pool.add(elem);
        println!("new Index: {}", index);
        assert_eq!(2, index.index());
        println!("pool: {:?}\n", pool);
    }
}

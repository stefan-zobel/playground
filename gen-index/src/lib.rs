#![allow(dead_code)]

use std::fmt::{Display, Formatter};

const SLOT_INDEX_BITS: u64 = 36;
const SLOT_INDEX_MASK: u64 = (1 << SLOT_INDEX_BITS) - 1;

const SLOT_GENERATION_BITS: u32 = 28;
const SLOT_GENERATION_MASK: u32 = (1 << SLOT_GENERATION_BITS) - 1;

const CTRL_BLOCK_IDX: usize = 0usize;
const DEFAULT_CAPACITY: usize = 16usize;

#[derive(Debug)]
pub struct Pool<T> {
    data: Vec<Slot<T>>,
}

impl<T> Pool<T> {
    #[inline]
    pub fn new() -> Self {
        Pool::with_capacity(DEFAULT_CAPACITY)
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = {
            if capacity <= DEFAULT_CAPACITY {
                DEFAULT_CAPACITY
            } else if capacity > SLOT_INDEX_BITS as usize + 1usize {
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

    #[inline]
    pub fn add(&mut self, value: T) -> Index {
        let (control, rest) = self.data.split_at_mut(CTRL_BLOCK_IDX + 1usize);
        if let [Slot::Empty { an_empty, .. }] = control {
            let next_free_in_control = an_empty;
            let free_slot_index_in_slice = *next_free_in_control - 1usize;
            let slot = &mut rest[free_slot_index_in_slice];
            match slot {
                Slot::Empty { an_empty, gen } => {
                    let new_next_empty = *an_empty;
                    let next_gen = *gen;
                    let index = Index::new(*next_free_in_control as u64, next_gen);
                    // this may write usize::MAX into the control block
                    *next_free_in_control = new_next_empty;
                    *slot = Slot::new_taken(value, next_gen);
                    // check 'new_next_empty' that we haven't reached usize::MAX which is invalid
                    if new_next_empty == usize::MAX {
                        self.grow();
                    }
                    return index;
                }
                Slot::Taken { .. } => {
                    panic!("index {} is already occupied!", *next_free_in_control);
                }
            }
        }
        panic!("control block is not empty!");
    }

    #[inline]
    pub(crate) fn remove_by_pos(&mut self, pos: usize) {
        if pos > 0 && pos < self.data.len() {
            let (control, rest) = self.data.split_at_mut(CTRL_BLOCK_IDX + 1usize);
            if let [Slot::Empty { an_empty, .. }] = control {
                let next_free_in_control = an_empty;
                let occupied_slot_index_in_slice = pos - 1usize;
                let slot = &mut rest[occupied_slot_index_in_slice];
                match slot {
                    Slot::Taken { gen, .. } => {
                        *slot = Slot::new_empty(*next_free_in_control, *gen + 1u32);
                        *next_free_in_control = pos;
                    }
                    Slot::Empty { .. } => {
                        panic!("index {} is already empty!", pos);
                    }
                }
            } else {
                panic!("control block is not empty!");
            }
        }
    }

    #[inline]
    pub fn remove(&mut self, index: Index) -> Option<T> {
        let pos = index.index() as usize;
        if pos > 0 && pos < self.data.len() {
            let (control, rest) = self.data.split_at_mut(CTRL_BLOCK_IDX + 1usize);
            if let [Slot::Empty { an_empty, .. }] = control {
                let next_free_in_control = an_empty;
                let occupied_slot_index_in_slice = pos - 1usize;
                let slot = &mut rest[occupied_slot_index_in_slice];
                if let Slot::Taken { gen, .. } = slot {
                    if *gen == index.generation() {
                        let next_gen = *gen + 1u32;
                        let old = std::mem::replace(
                            slot,
                            Slot::new_empty(*next_free_in_control, next_gen),
                        );
                        *next_free_in_control = pos;
                        if let Slot::Taken { val, .. } = old {
                            return Some(val);
                        }
                    }
                }
            } else {
                panic!("control block is not empty!");
            }
        }
        None
    }

    #[inline]
    pub fn get(&self, index: Index) -> Option<&T> {
        let pos = index.index() as usize;
        let version = index.generation();
        if pos > 0 && pos < self.data.len() {
            if let Slot::Taken { val, gen } = &self.data[pos] {
                if *gen == version {
                    return Some(val);
                }
            }
        }
        None
    }

    #[inline]
    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        let pos = index.index() as usize;
        let version = index.generation();
        if pos > 0 && pos < self.data.len() {
            if let Slot::Taken { val, gen } = self.data.get_mut(pos)? {
                if *gen == version {
                    return Some(val);
                }
            }
        }
        None
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.data.len() as u64
    }

    #[inline]
    pub fn clear(&mut self) {
        let length = self.data.len();
        // fix the control block
        self.fix_slot_pointer(CTRL_BLOCK_IDX, 1usize, true);
        // reset the slots
        for i in 1usize..length {
            let slot = &mut self.data[i];
            match slot {
                Slot::Empty { an_empty, .. } => {
                    *an_empty = i + 1usize;
                }
                Slot::Taken { gen, .. } => {
                    *slot = Slot::Empty {
                        an_empty: i + 1usize,
                        gen: *gen + 1u32,
                    };
                }
            }
        }
        // fix the 'an_empty' pointer in the last slot
        if let Slot::Empty { an_empty, .. } = &mut self.data[length - 1usize] {
            *an_empty = usize::MAX;
        }
    }

    #[inline]
    fn init_pool(pool: &mut Pool<T>) {
        let capacity = pool.data.capacity();
        for i in 0..capacity {
            pool.data.push(Slot::initial_empty(i + 1usize));
        }
        // fix the 'an_empty' pointer in the last slot
        pool.fix_slot_pointer(capacity - 1usize, usize::MAX, true);
    }

    #[inline]
    fn grow(&mut self) {
        let old_capacity = self.data.capacity();
        let new_capacity = old_capacity + (old_capacity >> 1);
        if new_capacity > SLOT_INDEX_BITS as usize + 1usize {
            panic!("capacity overflow: {}", new_capacity);
        }
        self.data.reserve_exact(new_capacity - old_capacity);
        let new_capacity = self.data.capacity();
        if new_capacity > old_capacity {
            // fix the control block if necessary
            self.fix_slot_pointer(CTRL_BLOCK_IDX, old_capacity, false);
            // fix the old 'an_empty' pointer if necessary
            self.fix_slot_pointer(old_capacity - 1usize, old_capacity, false);
            // initialize the additional slots
            for i in old_capacity..new_capacity {
                self.data.push(Slot::initial_empty(i + 1usize));
            }
            // fix the 'an_empty' pointer in the last slot
            self.fix_slot_pointer(new_capacity - 1usize, usize::MAX, true);
        }
    }

    #[inline]
    fn fix_slot_pointer(&mut self, position: usize, new_empty: usize, unconditionally: bool) {
        if let Some(Slot::Empty { an_empty, .. }) = self.data.get_mut(position) {
            if unconditionally {
                *an_empty = new_empty;
            } else if *an_empty == usize::MAX {
                *an_empty = new_empty;
            }
        }
    }
}

impl<T> Default for Pool<T> {
    #[inline]
    fn default() -> Self {
        Pool::new()
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Slot<T> {
    Empty { an_empty: usize, gen: u32 },
    Taken { val: T, gen: u32 },
}

impl<T> Slot<T> {
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, Slot::Empty { .. })
    }

    #[inline]
    pub(crate) fn is_occupied(&self) -> bool {
        matches!(self, Slot::Taken { .. })
    }

    #[inline]
    pub(crate) fn generation(&self) -> u32 {
        match self {
            Slot::Empty { gen, .. } => *gen,
            Slot::Taken { gen, .. } => *gen,
        }
    }

    #[inline]
    pub(crate) fn get(&self) -> Option<&T> {
        match self {
            Slot::Empty { .. } => None,
            Slot::Taken { val, .. } => Some(val),
        }
    }

    #[inline]
    pub(crate) fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            Slot::Empty { .. } => None,
            Slot::Taken { val, .. } => Some(val),
        }
    }

    #[inline]
    pub(crate) fn new_taken(val: T, gen: u32) -> Self {
        Slot::Taken { val, gen }
    }

    #[inline]
    pub(crate) fn new_empty(next_free: usize, gen: u32) -> Self {
        Slot::Empty {
            an_empty: next_free,
            gen,
        }
    }

    #[inline]
    pub(crate) fn initial_empty(next_free: usize) -> Self {
        Slot::new_empty(next_free, 0u32)
    }
}

/// An `Index` reserves 36 bits for the position, 28 bits for the generation.
#[derive(Clone, Copy, Debug)]
pub struct Index {
    id: u64,
}

impl Index {
    #[inline]
    pub fn index(&self) -> u64 {
        self.id & SLOT_INDEX_MASK
    }

    #[inline]
    pub fn generation(&self) -> u32 {
        (self.id >> SLOT_INDEX_BITS) as u32 & SLOT_GENERATION_MASK
    }

    #[inline]
    pub fn new(idx: u64, gen: u32) -> Self {
        let id = (idx & SLOT_INDEX_MASK)
            | (((gen & SLOT_GENERATION_MASK) as u64) << (64 - SLOT_GENERATION_BITS));
        Index { id }
    }

    #[inline]
    pub fn parts(&self) -> (u64, u32) {
        (self.index(), self.generation())
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
        println!("ENTITY_INDEX_MASK: {}", format!("{:#16x}", SLOT_INDEX_MASK));
        println!(
            "ENTITY_GENERATION_MASK: {}",
            format!("{:#8x}", SLOT_GENERATION_MASK)
        );
    }

    #[test]
    fn print_index() {
        let idx = Index::new(1, 268_435_455);
        println!("Index Display: {}", idx);
        println!("Index Debug  : {:?}", idx);
    }

    #[test]
    fn test_create_pool_then_add_and_remove_by_pos() {
        let mut pool = Pool::<i32>::new();
        println!("pool: {:?}\n", pool);
        // add 42, gen=0 at 1
        let elem = 42i32;
        let index = pool.add(elem);
        println!("new Index: {}", index);
        assert_eq!(1, index.index());
        println!("pool: {:?}\n", pool);
        // add 49, gen=0 at 2
        let elem = 49i32;
        let index = pool.add(elem);
        println!("new Index: {}", index);
        assert_eq!(2, index.index());
        println!("pool: {:?}\n", pool);
        pool.remove_by_pos(0);
        pool.remove_by_pos(16);
        // remove 42, gen=0 from 1
        pool.remove_by_pos(1);
        println!("pool: {:?}\n", pool);
        // add 51, gen=1 at 1
        let elem = 51i32;
        let index = pool.add(elem);
        println!("pool: {:?}\n", pool);
        assert_eq!(1, index.index());
        println!("new Index: {}", index);
        // remove 49, gen=0 from 2
        pool.remove_by_pos(2);
        println!("pool: {:?}\n", pool);
        // add 53, gen=1 at 2
        let elem = 53i32;
        let index = pool.add(elem);
        assert_eq!(2, index.index());
        println!("new Index: {}", index);
        println!("pool: {:?}\n", pool);
        // remove 51, gen=1 from 1
        pool.remove_by_pos(1);
        println!("pool: {:?}\n", pool);
        // remove 53, gen=1 from 2
        pool.remove_by_pos(2);
        println!("pool: {:?}\n", pool);
        // add 66, gen=2 at 2
        let elem = 66i32;
        let index = pool.add(elem);
        assert_eq!(2, index.index());
        println!("new Index: {}", index);
        println!("pool: {:?}\n", pool);
        // add 77, gen=2 at 1
        let elem = 77i32;
        let index = pool.add(elem);
        assert_eq!(1, index.index());
        println!("new Index: {}", index);
        println!("pool: {:?}\n", pool);
        pool.grow();
        println!("pool: {:?}\n", pool);
        pool.clear();
        println!("Pool after clear():\n {:?}\n", pool);
    }

    #[test]
    fn test_resize() {
        let mut pool = Pool::<i32>::new();
        // add entries from 1 to 14, last slot remains empty
        for i in 1..15 {
            pool.add(i);
        }
        println!("Pool after first insert round:\n {:?}\n", pool);
        // use up the last slot (store 15 in index 15)
        pool.add(15);
        println!("Pool after additional insert:\n {:?}\n", pool);
        // add 8 more from 16 to 23
        for i in 16..24 {
            pool.add(i);
        }
        println!("Pool after second insert round:\n {:?}\n", pool);
        assert_eq!(36, pool.data.capacity());
        assert_eq!(36, pool.data.len());
        // add 24 to slot 24
        pool.add(24);
        println!("Pool after additional insert:\n {:?}\n", pool);
    }

    #[test]
    fn test_remove() {
        let mut pool = Pool::<i32>::new();
        let idx = pool.add(15);
        let val = pool.remove(idx);
        println!("got back : {:?}", val);
        println!("Pool after removal:\n {:?}\n", pool);
    }
}

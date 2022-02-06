#![allow(dead_code)]

use std::fmt::{Display, Formatter};

const SLOT_INDEX_BITS: u64 = 36;
const SLOT_INDEX_MASK: u64 = (1 << SLOT_INDEX_BITS) - 1;

const SLOT_GENERATION_BITS: u32 = 28;
const SLOT_GENERATION_MASK: u32 = (1 << SLOT_GENERATION_BITS) - 1;

const CTRL_BLOCK_IDX: usize = 0usize;
const ONE: usize = 1usize;
const DEFAULT_CAPACITY: usize = 16usize;

const ERR_CAP_MSG: &str = "capacity overflow:";
const ERR_CTRL_MSG: &str = "control block is not empty!";

#[derive(Debug)]
pub struct Pool<T> {
    data: Vec<Slot<T>>,
    num_taken: usize,
    max_taken_pos: usize,
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
                DEFAULT_CAPACITY + ONE
            } else if capacity > SLOT_INDEX_BITS as usize + ONE {
                panic!("{} {}", ERR_CAP_MSG, capacity)
            } else {
                capacity + ONE
            }
        };
        let mut pool = Pool {
            data: Vec::<Slot<T>>::with_capacity(capacity),
            num_taken: 0usize,
            max_taken_pos: 0usize,
        };
        Pool::init_pool(&mut pool);
        pool
    }

    #[inline]
    pub fn add(&mut self, value: T) -> Index {
        self.num_taken += ONE;
        let max_taken = self.max_taken_pos;
        let (control, rest) = self.data.split_at_mut(CTRL_BLOCK_IDX + ONE);
        if let [Slot::Empty { an_empty, .. }] = control {
            let next_free_in_control = an_empty;
            if *next_free_in_control > max_taken {
                self.max_taken_pos = *next_free_in_control;
            }
            let free_slot_index_in_slice = *next_free_in_control - ONE;
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
                    panic!("index {} is already taken!", *next_free_in_control);
                }
            }
        }
        panic!("{}", ERR_CTRL_MSG);
    }

    #[inline]
    pub(crate) fn remove_by_pos(&mut self, pos: usize) {
        if pos > 0 && pos < self.data.len() {
            self.num_taken -= ONE;
            let max_taken = self.max_taken_pos;
            let (control, rest) = self.data.split_at_mut(CTRL_BLOCK_IDX + ONE);
            if let [Slot::Empty { an_empty, .. }] = control {
                let next_free_in_control = an_empty;
                let taken_slot_index_in_slice = pos - ONE;
                let slot = &mut rest[taken_slot_index_in_slice];
                match slot {
                    Slot::Taken { gen, .. } => {
                        *slot = Slot::new_empty(*next_free_in_control, *gen + 1u32);
                        *next_free_in_control = pos;
                        if pos == max_taken {
                            for i in (CTRL_BLOCK_IDX..taken_slot_index_in_slice).rev() {
                                if rest[i].is_taken() {
                                    self.max_taken_pos = i + ONE;
                                    break;
                                }
                            }
                            // if all slots to the left were empty
                            // or 'taken_slot_index_in_slice' was 0
                            if self.max_taken_pos == max_taken {
                                self.max_taken_pos = CTRL_BLOCK_IDX;
                            }
                        }
                    }
                    Slot::Empty { .. } => {
                        panic!("index {} is already empty!", pos);
                    }
                }
            } else {
                panic!("{}", ERR_CTRL_MSG);
            }
        }
    }

    //noinspection ALL
    #[inline]
    pub fn remove(&mut self, index: Index) -> Option<T> {
        let pos = index.index() as usize;
        if pos > 0 && pos < self.data.len() {
            let (control, rest) = self.data.split_at_mut(CTRL_BLOCK_IDX + ONE);
            if let [Slot::Empty { an_empty, .. }] = control {
                let next_free_in_control = an_empty;
                let taken_slot_index_in_slice = pos - ONE;
                let slot = &mut rest[taken_slot_index_in_slice];
                if let Slot::Taken { gen, .. } = slot {
                    if *gen == index.generation() {
                        let next_gen = *gen + 1u32;
                        let old = std::mem::replace(
                            slot,
                            Slot::new_empty(*next_free_in_control, next_gen),
                        );
                        *next_free_in_control = pos;
                        self.num_taken -= ONE;
                        let max_taken = self.max_taken_pos;
                        if pos == max_taken {
                            for i in (CTRL_BLOCK_IDX..taken_slot_index_in_slice).rev() {
                                if rest[i].is_taken() {
                                    self.max_taken_pos = i + ONE;
                                    break;
                                }
                            }
                            // if all slots to the left were empty
                            // or 'taken_slot_index_in_slice' was 0
                            if self.max_taken_pos == max_taken {
                                self.max_taken_pos = CTRL_BLOCK_IDX;
                            }
                        }
                        if let Slot::Taken { val, .. } = old {
                            return Some(val);
                        }
                    }
                }
            } else {
                panic!("{}", ERR_CTRL_MSG);
            }
        }
        None
    }

    #[inline]
    pub fn get(&self, index: Index) -> Option<&T> {
        let (pos, version) = index.parts();
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
        let (pos, version) = index.parts();
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
        (self.data.len() - ONE) as u64
    }

    #[inline]
    pub fn taken(&self) -> u64 {
        self.num_taken as u64
    }

    #[inline]
    pub fn clear(&mut self) {
        let length = self.data.len();
        // fix the control block
        self.fix_slot_pointer(CTRL_BLOCK_IDX, ONE, true);
        self.num_taken = 0usize;
        self.max_taken_pos = CTRL_BLOCK_IDX;
        // reset the slots
        for i in ONE..length {
            let slot = &mut self.data[i];
            match slot {
                Slot::Empty { an_empty, .. } => {
                    *an_empty = i + ONE;
                }
                Slot::Taken { gen, .. } => {
                    *slot = Slot::new_empty(i + ONE, *gen + 1u32);
                }
            }
        }
        // fix the 'an_empty' pointer in the last slot
        self.fix_slot_pointer(length - ONE, usize::MAX, true);
    }

    #[inline]
    fn init_pool(pool: &mut Pool<T>) {
        let capacity = pool.data.capacity();
        for i in 0..capacity {
            pool.data.push(Slot::initial_empty(i + ONE));
        }
        // fix the 'an_empty' pointer in the last slot
        pool.fix_slot_pointer(capacity - ONE, usize::MAX, true);
    }

    #[inline]
    fn grow(&mut self) {
        let old_capacity = self.data.capacity();
        let new_capacity = old_capacity + (old_capacity >> 1);
        if new_capacity > SLOT_INDEX_BITS as usize + ONE {
            panic!("{} {}", ERR_CAP_MSG, new_capacity);
        }
        self.data.reserve_exact(new_capacity - old_capacity);
        let new_capacity = self.data.capacity();
        if new_capacity > old_capacity {
            // fix the control block if necessary
            self.fix_slot_pointer(CTRL_BLOCK_IDX, old_capacity, false);
            // fix the old 'an_empty' pointer if necessary
            self.fix_slot_pointer(old_capacity - ONE, old_capacity, false);
            // initialize the additional slots
            for i in old_capacity..new_capacity {
                self.data.push(Slot::initial_empty(i + ONE));
            }
            // fix the 'an_empty' pointer in the last slot
            self.fix_slot_pointer(new_capacity - ONE, usize::MAX, true);
        }
    }

    #[inline]
    fn fix_slot_pointer(&mut self, position: usize, new_empty: usize, unconditionally: bool) {
        if let Some(Slot::Empty { an_empty, .. }) = self.data.get_mut(position) {
            if unconditionally || *an_empty == usize::MAX {
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
    pub(crate) fn is_taken(&self) -> bool {
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
    pub(crate) fn new(idx: u64, gen: u32) -> Self {
        let id = (idx & SLOT_INDEX_MASK)
            | (((gen & SLOT_GENERATION_MASK) as u64) << (64 - SLOT_GENERATION_BITS));
        Index { id }
    }

    #[inline]
    pub fn parts(&self) -> (usize, u32) {
        (self.index() as usize, self.generation())
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
        println!("SLOT_INDEX_MASK: {}", format!("{:#16x}", SLOT_INDEX_MASK));
        println!(
            "SLOT_GENERATION_MASK: {}",
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
        pool.remove_by_pos(17);
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
        // add entries from 1 to 15, last slot remains empty
        for i in 1..16 {
            pool.add(i);
        }
        println!("Pool after first insert round:\n {:?}\n", pool);
        println!("pool size: {}\n", pool.size());
        // use up the last slot (store 16 in index 16)
        pool.add(16);
        println!("Pool after additional insert:\n {:?}\n", pool);
        println!("pool size: {}\n", pool.size());
        // add 8 more from 17 to 24
        for i in 17..25 {
            pool.add(i);
        }
        println!("Pool after second insert round:\n {:?}\n", pool);
        println!("pool size: {}\n", pool.size());
        assert_eq!(37, pool.data.capacity());
        assert_eq!(37, pool.data.len());
        assert_eq!(36, pool.size());
        // add 25 to slot 25
        pool.add(25);
        println!("Pool after additional insert:\n {:?}\n", pool);
        println!("pool size: {}\n", pool.size());
    }

    #[test]
    fn test_remove() {
        let mut pool = Pool::<i32>::new();
        let idx = pool.add(15);
        let val = pool.remove(idx);
        println!("got back : {:?}", val);
        println!("Pool after removal:\n {:?}\n", pool);
    }

    #[test]
    fn test_max_taken_remove_by_pos() {
        println!("MAX_TAKEN_REMOVE_TEST");
        let mut pool = Pool::<i32>::new();
        println!("max_taken_pos after init: {}", pool.max_taken_pos);
        assert_eq!(0, pool.max_taken_pos);
        let _idx = pool.add(1);
        println!("max_taken_pos: {}", pool.max_taken_pos);
        assert_eq!(1, pool.max_taken_pos);
        pool.remove_by_pos(1);
        println!("max_taken_pos after remove: {}", pool.max_taken_pos);
        assert_eq!(0, pool.max_taken_pos);
        let _idx = pool.add(1);
        assert_eq!(1, pool.max_taken_pos);
        let _idx = pool.add(2);
        assert_eq!(2, pool.max_taken_pos);
        let _idx = pool.add(3);
        assert_eq!(3, pool.max_taken_pos);
        pool.remove_by_pos(2);
        assert_eq!(3, pool.max_taken_pos);
        pool.remove_by_pos(3);
        assert_eq!(1, pool.max_taken_pos);
        pool.remove_by_pos(1);
        assert_eq!(0, pool.max_taken_pos);
        println!("Pool:\n {:?}\n", pool);
        let _idx = pool.add(1);
        assert_eq!(1, pool.max_taken_pos);
        println!("Pool:\n {:?}\n", pool);
        let _idx = pool.add(2);
        assert_eq!(3, pool.max_taken_pos);
        println!("Pool:\n {:?}\n", pool);
        let _idx = pool.add(3);
        assert_eq!(3, pool.max_taken_pos);
        println!("Pool:\n {:?}\n", pool);
        let _idx = pool.add(4);
        assert_eq!(4, pool.max_taken_pos);
        println!("Pool:\n {:?}\n", pool);
        pool.remove_by_pos(4);
        println!("Pool:\n {:?}\n", pool);
        assert_eq!(3, pool.max_taken_pos);
        pool.remove_by_pos(3);
        assert_eq!(2, pool.max_taken_pos);
        pool.remove_by_pos(2);
        assert_eq!(1, pool.max_taken_pos);
        println!("Pool:\n {:?}\n", pool);
        pool.remove_by_pos(1);
        assert_eq!(0, pool.max_taken_pos);
        println!("Pool:\n {:?}\n", pool);
    }
}

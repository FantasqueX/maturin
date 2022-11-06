#![feature(btree_drain_filter)]

mod range_action_map;
mod resource;
use resource::Frame;

use range_action_map::*;

#[derive(Debug)]
pub struct Seg {
    pub start: usize,
    pub end: usize,
    pub flags: PTEFlags,
    frames: Vec<Frame>,
}

impl Seg {
    pub fn new(start: usize, end: usize, flags: PTEFlags) -> Self {
        let mut frames: Vec<Frame> = Vec::new();
        for _ in start..end {
            frames.push(Frame::alloc());
        }
        Self {
            start,
            end,
            flags,
            frames,
        }
    }
}

impl Segment for Seg {
    fn remove(&mut self, args: ArgsType) {
        self.frames.clear();
    }
    fn split(&mut self, pos: usize, args: ArgsType) -> Self {
        let right_frames = self.frames.drain(pos - self.start..).collect();
        let old_end = self.end;
        self.end = pos;
        Self {
            start: pos,
            end: old_end,
            flags: self.flags,
            frames: right_frames,
        }
    }
    fn modify(&mut self, new_flag: IdentType, args: ArgsType) {
        self.flags = new_flag
    }
}

pub fn test_find(ram: &mut RangeActionMap<Seg>, pos: usize) -> bool {
    println!("try find seg include {pos}");
    if let Some(seg) = ram.find(pos) {
        println!("find seg {} {}", seg.start, seg.end);
        true
    } else {
        println!("seg not found");
        false
    }
}

pub fn test_mmap_fixed(ram: &mut RangeActionMap<Seg>, start: usize, end: usize, flag: PTEFlags) {
    ram.mmap_fixed(start, end, Seg::new(start, end, flag), |seg, _| {
        println!("mapped to {} {}", seg.start, seg.end);
    });
}

pub fn test_mmap_anywhere(ram: &mut RangeActionMap<Seg>, hint: usize, len: usize, flag: PTEFlags) {
    ram.mmap_anywhere(hint, len, Seg::new(0, len, flag), |seg, start| {
        seg.start = start;
        seg.end = start + len;
        println!("mapped to {} {}", seg.start, seg.end);
    });
}

pub fn test_get_flag_at(ram: &mut RangeActionMap<Seg>, pos: usize) -> PTEFlags {
    ram.find(pos).unwrap().flags
}

fn main() {}

#[test]
fn test_ram() {
    let mut ram = RangeActionMap::<Seg>::new(ArgsType::default());
    test_mmap_fixed(&mut ram, 0x3000, 0x7000, PTE_RU());
    assert_eq!(test_find(&mut ram, 0x2111), false);
    assert_eq!(test_find(&mut ram, 0x5678), true);
    assert_eq!(test_find(&mut ram, 0x7000), false);
    test_mmap_fixed(&mut ram, 0x5000, 0x6000, PTE_RWU());
    assert_eq!(test_get_flag_at(&mut ram, 0x4fff), PTE_RU());
    assert_eq!(test_get_flag_at(&mut ram, 0x5000), PTE_RWU());
    assert_eq!(test_get_flag_at(&mut ram, 0x5fff), PTE_RWU());
    assert_eq!(test_get_flag_at(&mut ram, 0x6000), PTE_RU());
    ram.unmap(0x5050, 0x6060);
    assert_eq!(test_get_flag_at(&mut ram, 0x504f), PTE_RWU());
    assert_eq!(test_find(&mut ram, 0x5050), false);
    assert_eq!(test_find(&mut ram, 0x605f), false);
    assert_eq!(test_get_flag_at(&mut ram, 0x6060), PTE_RU());
    test_mmap_anywhere(&mut ram, 0x5000, 0x1000, PTE_NORMAL());
    assert_eq!(test_get_flag_at(&mut ram, 0x5000), PTE_RWU());
    assert_eq!(test_get_flag_at(&mut ram, 0x5050), PTE_NORMAL());
    assert_eq!(test_get_flag_at(&mut ram, 0x6049), PTE_NORMAL());
    assert_eq!(test_find(&mut ram, 0x6050), false);
    ram.unmap(0x5050, 0x6060);
    test_mmap_anywhere(&mut ram, 0x5061, 0x1000, PTE_NORMAL());
    assert_eq!(test_find(&mut ram, 0x5050), false);
    assert_eq!(test_find(&mut ram, 0x605f), false);
    assert_eq!(test_find(&mut ram, 0x605f), false);
    assert_eq!(test_get_flag_at(&mut ram, 0x7000), PTE_NORMAL());
    assert_eq!(test_get_flag_at(&mut ram, 0x7fff), PTE_NORMAL());
    assert_eq!(test_find(&mut ram, 0x8000), false);
}

#[test]
fn test_seg() {
    let mut seg = Seg::new(5, 10, PTE_RU());
    seg.shrink_to_left(8, ArgsType::default());
    assert_eq!(seg.start, 5);
    assert_eq!(seg.end, 8);
    //println!("{:#?}", seg);
    seg.shrink_to_right(7, ArgsType::default());
    //println!("{:#?}", seg);
    assert_eq!(seg.start, 7);
    assert_eq!(seg.end, 8);
    let mut seg = Seg::new(1, 100, PTE_RU());
    let mut rseg = seg.split_and_remove_middle(6, 13, ArgsType::default());
    assert_eq!(seg.start, 1);
    assert_eq!(seg.end, 6);
    assert_eq!(rseg.start, 13);
    assert_eq!(rseg.end, 100);
    rseg.modify(PTE_RWU(), ArgsType::default());
    assert_eq!(rseg.start, 13);
    assert_eq!(rseg.end, 100);
    assert_eq!(rseg.flags, PTE_RWU());
    let rrseg = rseg.modify_left(77, PTE_RXU(), ArgsType::default());
    assert_eq!(rseg.flags, PTE_RXU());
    assert_eq!(rseg.end, 77);
    assert_eq!(rrseg.flags, PTE_RWU());
    let rrseg = rseg.modify_right(72, PTE_U(), ArgsType::default());
    assert_eq!(rseg.flags, PTE_RXU());
    assert_eq!(rseg.end, 72);
    assert_eq!(rrseg.flags, PTE_U());
    assert_eq!(rrseg.start, 72);
    assert_eq!(rrseg.end, 77);
    let (mrseg, rrseg) = rseg.modify_middle(33, 55, PTE_NORMAL(), ArgsType::default());
    assert_eq!(rseg.flags, PTE_RXU());
    assert_eq!(rseg.end, 33);
    assert_eq!(mrseg.flags, PTE_NORMAL());
    assert_eq!(mrseg.start, 33);
    assert_eq!(mrseg.end, 55);
    assert_eq!(rrseg.flags, PTE_RXU());
    assert_eq!(rrseg.start, 55);
}

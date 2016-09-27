#![no_std]
#![feature(collections, alloc)]


extern crate alloc;
extern crate collections;

extern crate transform2d_component;
extern crate transform3d_component;
extern crate scene_graph;


mod rect;
mod rect_manager;

pub use rect::Rect;
pub use rect_manager::RectManager;

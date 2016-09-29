#![no_std]
#![feature(collections)]


extern crate collections;

extern crate shared;
extern crate transform_components;
extern crate scene_graph;


mod rect;
mod rect_manager;

pub use rect::Rect;
pub use rect_manager::RectManager;

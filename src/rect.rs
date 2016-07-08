use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;

use transform2d_component::Transform2D;
use transform3d_component::Transform3D;
use scene_graph::{Entity, Component, ComponentManager, Id};
use rect_manager::RectManager;


struct RectData {

    entity: Option<Entity>,
    rect_manager: Option<RectManager>,

    width: f32,
    height: f32,
}


#[derive(Clone)]
pub struct Rect {
    data: Arc<RefCell<RectData>>,
}

impl Rect {
    pub fn new() -> Self {
        Rect {
            data: Arc::new(RefCell::new(RectData {

                entity: None,
                rect_manager: None,

                width: 1f32,
                height: 1f32,
            }))
        }
    }

    pub fn rect_manager(&self) -> Option<RectManager> {
        self.data.borrow().rect_manager.clone()
    }
    pub fn set_rect_manager(&self, rect_manager: Option<RectManager>) {
        self.data.borrow_mut().rect_manager = rect_manager;
    }

    pub fn width(&self) -> f32 { self.data.borrow().width }
    pub fn set_width(&self, width: f32) { self.data.borrow_mut().width = width; }

    pub fn height(&self) -> f32 { self.data.borrow().height }
    pub fn set_height(&self, height: f32) { self.data.borrow_mut().height = height; }

    pub fn min_x(&self) -> f32 { self.position()[0] - (self.width() * 0.5f32) }
    pub fn min_y(&self) -> f32 { self.position()[1] - (self.height() * 0.5f32) }
    pub fn max_x(&self) -> f32 { self.position()[0] + (self.width() * 0.5f32) }
    pub fn max_y(&self) -> f32 { self.position()[1] + (self.height() * 0.5f32) }

    pub fn local_min_x(&self) -> f32 { self.local_position()[0] - (self.width() * 0.5f32) }
    pub fn local_min_y(&self) -> f32 { self.local_position()[1] - (self.height() * 0.5f32) }
    pub fn local_max_x(&self) -> f32 { self.local_position()[0] + (self.width() * 0.5f32) }
    pub fn local_max_y(&self) -> f32 { self.local_position()[1] + (self.height() * 0.5f32) }

    fn position(&self) -> [f32; 2] {
        if let Some(entity) = self.entity() {
            if let Some(transform3d) = entity.get_component::<Transform3D>() {
                let position = transform3d.position();
                [position[0], position[1]]
            } else if let Some(transform2d) = entity.get_component::<Transform2D>() {
                transform2d.position()
            } else {
                [0f32, 0f32]
            }
        } else {
            [0f32, 0f32]
        }
    }
    fn local_position(&self) -> [f32; 2] {
        if let Some(entity) = self.entity() {
            if let Some(transform3d) = entity.get_component::<Transform3D>() {
                let position = transform3d.local_position();
                [position[0], position[1]]
            } else if let Some(transform2d) = entity.get_component::<Transform2D>() {
                transform2d.local_position()
            } else {
                [0f32, 0f32]
            }
        } else {
            [0f32, 0f32]
        }
    }
}

impl Component for Rect {
    fn id(&self) -> Id {
        Id::of::<Rect>()
    }
    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(RectManager::new())
    }
    fn component_manager_id(&self) -> Id {
        Id::of::<RectManager>()
    }
    fn entity(&self) -> Option<Entity> {
        self.data.borrow().entity.clone()
    }
    fn set_entity(&self, entity: Option<Entity>) {
        self.data.borrow_mut().entity = entity;
    }
}

impl PartialEq<Rect> for Rect {
    fn eq(&self, other: &Rect) -> bool {
        (&*self.data.borrow() as *const _) == (&*other.data.borrow() as *const _)
    }
    fn ne(&self, other: &Rect) -> bool {
        !self.eq(other)
    }
}

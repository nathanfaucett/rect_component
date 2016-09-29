use collections::boxed::Box;

use shared::Shared;
use transform_components::{Transform2D, Transform3D};
use scene_graph::{Entity, Component, ComponentManager, Id};
use rect_manager::RectManager;


struct RectData {
    entity: Option<Entity>,
    width: f32,
    height: f32,
}


#[derive(Clone)]
pub struct Rect {
    data: Shared<RectData>,
}

impl Rect {
    pub fn new() -> Self {
        Rect {
            data: Shared::new(RectData {
                entity: None,
                width: 1f32,
                height: 1f32,
            })
        }
    }

    pub fn width(&self) -> f32 { self.data.width }
    pub fn set_width(&mut self, width: f32) { self.data.width = width; }

    pub fn height(&self) -> f32 { self.data.height }
    pub fn set_height(&mut self, height: f32) { self.data.height = height; }

    pub fn min_x(&mut self) -> f32 { self.position()[0] - (self.width() * 0.5f32) }
    pub fn min_y(&mut self) -> f32 { self.position()[1] - (self.height() * 0.5f32) }
    pub fn max_x(&mut self) -> f32 { self.position()[0] + (self.width() * 0.5f32) }
    pub fn max_y(&mut self) -> f32 { self.position()[1] + (self.height() * 0.5f32) }

    pub fn local_min_x(&mut self) -> f32 { self.local_position()[0] - (self.width() * 0.5f32) }
    pub fn local_min_y(&mut self) -> f32 { self.local_position()[1] - (self.height() * 0.5f32) }
    pub fn local_max_x(&mut self) -> f32 { self.local_position()[0] + (self.width() * 0.5f32) }
    pub fn local_max_y(&mut self) -> f32 { self.local_position()[1] + (self.height() * 0.5f32) }

    fn position(&mut self) -> [f32; 2] {
        if let Some(entity) = self.get_entity() {
            if let Some(ref mut transform3d) = entity.get_component::<Transform3D>() {
                let position = transform3d.get_position();
                [position[0], position[1]]
            } else if let Some(ref mut transform2d) = entity.get_component::<Transform2D>() {
                *transform2d.get_position()
            } else {
                [0f32, 0f32]
            }
        } else {
            [0f32, 0f32]
        }
    }
    fn local_position(&mut self) -> [f32; 2] {
        if let Some(entity) = self.get_entity() {
            if let Some(ref mut transform3d) = entity.get_component::<Transform3D>() {
                let position = transform3d.get_local_position();
                [position[0], position[1]]
            } else if let Some(ref mut transform2d) = entity.get_component::<Transform2D>() {
                *transform2d.get_local_position()
            } else {
                [0f32, 0f32]
            }
        } else {
            [0f32, 0f32]
        }
    }
}

impl Component for Rect {
    fn get_id(&self) -> Id {
        Id::of::<Rect>()
    }
    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(RectManager::new())
    }
    fn get_component_manager_id(&self) -> Id {
        Id::of::<RectManager>()
    }
    fn get_entity(&self) -> Option<Entity> {
        self.data.entity.clone()
    }
    fn set_entity(&mut self, entity: Option<Entity>) {
        self.data.entity = entity;
    }
}

impl PartialEq<Rect> for Rect {
    fn eq(&self, other: &Rect) -> bool {
        (&*self.data as *const _) == (&*other.data as *const _)
    }
    fn ne(&self, other: &Rect) -> bool {
        !self.eq(other)
    }
}

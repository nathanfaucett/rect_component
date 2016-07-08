use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;

use scene_graph::{Scene, Component, ComponentManager, Id};


struct RectManagerData {
    scene: Option<Scene>,
    components: usize,
}


#[derive(Clone)]
pub struct RectManager {
    data: Arc<RefCell<RectManagerData>>,
}

impl RectManager {
    pub fn new() -> Self {
        RectManager {
            data: Arc::new(RefCell::new(RectManagerData {
                scene: None,
                components: 0usize,
            }))
        }
    }
}

impl ComponentManager for RectManager {

    fn id(&self) -> Id { Id::of::<RectManager>() }

    fn scene(&self) -> Option<Scene> {
        match self.data.borrow().scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    fn set_scene(&self, scene: Option<Scene>) {
        self.data.borrow_mut().scene = scene;
    }

    fn order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        self.data.borrow().components == 0usize
    }

    fn destroy(&self) {}
    fn init(&self) {}
    fn update(&self) {}

    fn add_component(&self, _: &Box<Component>) {
        self.data.borrow_mut().components += 1;
    }
    fn remove_component(&self, _: &Box<Component>) {
        self.data.borrow_mut().components -= 1;
    }
}

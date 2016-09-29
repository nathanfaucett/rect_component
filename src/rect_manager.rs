use collections::boxed::Box;

use shared::Shared;
use scene_graph::{Scene, Component, ComponentManager, Id};


struct RectManagerData {
    scene: Option<Scene>,
    components: usize,
}


#[derive(Clone)]
pub struct RectManager {
    data: Shared<RectManagerData>,
}

impl RectManager {
    pub fn new() -> Self {
        RectManager {
            data: Shared::new(RectManagerData {
                scene: None,
                components: 0usize,
            })
        }
    }
}

impl ComponentManager for RectManager {

    fn get_id(&self) -> Id { Id::of::<RectManager>() }

    fn get_scene(&self) -> Option<Scene> {
        match self.data.scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    fn set_scene(&mut self, scene: Option<Scene>) {
        self.data.scene = scene;
    }

    fn get_order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        self.data.components == 0usize
    }

    fn clear(&mut self) {}
    fn init(&mut self) {}
    fn update(&mut self) {}

    fn add_component(&mut self, _: &mut Box<Component>) {
        self.data.components += 1;
    }
    fn remove_component(&mut self, _: &mut Box<Component>) {
        self.data.components -= 1;
    }
}

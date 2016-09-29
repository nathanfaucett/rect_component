#![no_std]


extern crate transform_components;
extern crate rect_component;
extern crate scene_graph;


use transform_components::Transform2D;
use rect_component::Rect;
use scene_graph::{Scene, Entity};


#[test]
fn test_scene() {
    let mut scene = Scene::new();
    let mut entity = Entity::new();

    entity
        .add_component(Transform2D::new())
        .add_component(Rect::new());

    scene.add_entity(&mut entity);

    scene.init();

    let mut entity_rect = entity.get_component::<Rect>().unwrap();

    assert_eq!(entity_rect.min_x(), -0.5f32);
    assert_eq!(entity_rect.min_y(), -0.5f32);
    assert_eq!(entity_rect.max_x(), 0.5f32);
    assert_eq!(entity_rect.max_y(), 0.5f32);

    let mut entity_transform2d = entity.get_component::<Transform2D>().unwrap();
    entity_transform2d.translate(&[0.5f32, 0.5f32]);

    assert_eq!(entity_rect.min_x(), 0f32);
    assert_eq!(entity_rect.min_y(), 0f32);
    assert_eq!(entity_rect.max_x(), 1f32);
    assert_eq!(entity_rect.max_y(), 1f32);
}

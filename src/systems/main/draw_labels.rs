use crate::helpers::names::Names;
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::{self, Text};
use ggez::Context;
use graphics::{DrawParam, WHITE};

pub fn draw_labels_system(world: &World, context: &mut Context) -> Result<()> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Label.to_string(),
        &Names::Size.to_string(),
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let labels = query.get(&Names::Label.to_string()).unwrap();
    let sizes = query.get(&Names::Size.to_string()).unwrap();
    assert!(locations.len() == labels.len() && labels.len() == sizes.len());

    for (index, label) in labels.iter().enumerate() {
        let label: &DataWrapper<Text> = label.cast()?;
        let location: &DataWrapper<Point> = locations[index].cast()?;
        let size: &DataWrapper<f32> = sizes[index].cast()?;

        let font_width = label.borrow().width(context) as f32;

        let mut label_location = *location.borrow();
        label_location.y += *size.borrow();
        label_location.x -= font_width / 2.0;

        graphics::draw(
            context,
            &*label.borrow(),
            DrawParam::default()
                .color(WHITE)
                .dest(label_location.to_array()),
        )?;
    }
    Ok(())
}

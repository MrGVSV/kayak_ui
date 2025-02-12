use bevy::{math::Vec2, sprite::Rect};
use kayak_core::render_primitive::RenderPrimitive;

use crate::{
    render::unified::pipeline::{ExtractQuadBundle, ExtractedQuad, UIQuadType},
    to_bevy_color,
};

pub fn extract_quads(render_primitive: &RenderPrimitive, dpi: f32) -> Vec<ExtractQuadBundle> {
    let (background_color, layout, border_radius) = match render_primitive {
        RenderPrimitive::Quad {
            background_color,
            layout,
            border_radius,
        } => (background_color, layout, border_radius),
        _ => panic!(""),
    };

    vec![ExtractQuadBundle {
        extracted_quad: ExtractedQuad {
            rect: Rect {
                min: Vec2::new(layout.posx, layout.posy),
                max: Vec2::new(layout.posx + layout.width, layout.posy + layout.height) * dpi,
            },
            color: to_bevy_color(background_color),
            vertex_index: 0,
            char_id: 0,
            z_index: layout.z_index,
            font_handle: None,
            quad_type: UIQuadType::Quad,
            type_index: 0,
            border_radius: *border_radius,
            image: None,
            uv_max: None,
            uv_min: None,
        },
    }]
}

use std::io::Cursor;

use image::ImageFormat;
use regex::Regex;
use takumi::{
    layout::{
        node::{ContainerNode, NodeKind, TextNode},
        style::{
            AlignItems, Angle, BackgroundImage, BackgroundImages, Color, CssValue, FontWeight,
            GradientStop, JustifyContent, LengthUnit, LinearGradient, StopPosition, Style,
        },
        Viewport,
    },
    rendering::render,
    GlobalContext,
};

#[flutter_rust_bridge::frb(positional)]
pub fn generate_with_name(name: String) -> Vec<u8> {
    let name_to_display = get_name_to_display(name.to_owned());
    let colors = get_gradient_colors(name);
    generate(name_to_display, colors)
}

#[flutter_rust_bridge::frb(positional)]
pub fn generate_with_first_name_last_name(first_name: String, last_name: String) -> Vec<u8> {
    let full_name = format!("{} {}", first_name, last_name);
    let name_to_display = get_name_to_display(full_name.to_owned());
    let colors = get_gradient_colors(full_name.to_owned());
    generate(name_to_display, colors)
}

fn get_name_to_display(name: String) -> String {
    let split_regex = Regex::new(r"\s+|,|-").unwrap();
    let parts = split_regex
        .split(name.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    if parts.len() == 0 {
        return "?".to_string();
    }
    if parts.len() == 1 {
        return parts[0].chars().nth(0).unwrap().to_uppercase().to_string();
    }
    (parts[0].chars().nth(0).unwrap().to_uppercase().to_string()
        + &parts[parts.len() - 1]
            .chars()
            .nth(0)
            .unwrap()
            .to_uppercase()
            .to_string())
        .to_string()
}

fn get_gradient_colors(name: String) -> (Color, Color) {
    let gradients = [
        (
            Color([0xFF, 0x6B, 0x6B, 0xFF]),
            Color([0xFF, 0x8E, 0x53, 0xFF]),
        ),
        (
            Color([0x4E, 0xCD, 0xC4, 0xFF]),
            Color([0x44, 0xA0, 0x8D, 0xFF]),
        ),
        (
            Color([0xA8, 0xE6, 0xCF, 0xFF]),
            Color([0x3D, 0x84, 0xA8, 0xFF]),
        ),
        (
            Color([0xFF, 0xD9, 0x3D, 0xFF]),
            Color([0xFF, 0x6B, 0x6B, 0xFF]),
        ),
        (
            Color([0x6C, 0x5C, 0xE7, 0xFF]),
            Color([0xA2, 0x9B, 0xFE, 0xFF]),
        ),
        (
            Color([0xFD, 0x79, 0xA8, 0xFF]),
            Color([0xFD, 0xCB, 0x6E, 0xFF]),
        ),
        (
            Color([0x74, 0xB9, 0xFF, 0xFF]),
            Color([0x09, 0x84, 0xE3, 0xFF]),
        ),
        (
            Color([0x55, 0xEF, 0xC4, 0xFF]),
            Color([0x00, 0xB8, 0x94, 0xFF]),
        ),
        (
            Color([0xFA, 0xB1, 0xA0, 0xFF]),
            Color([0xE1, 0x70, 0x55, 0xFF]),
        ),
        (
            Color([0xA2, 0x9B, 0xFE, 0xFF]),
            Color([0x6C, 0x5C, 0xE7, 0xFF]),
        ),
    ];

    let digest = md5::compute(name.as_bytes());
    let hashed_string = format!("{:x}", digest);
    let part = &hashed_string[0..8];
    let hash = usize::from_str_radix(part, 16).unwrap();
    let index: usize = hash as usize % gradients.len();
    gradients[index]
}

fn generate(name: String, (start, end): (Color, Color)) -> Vec<u8> {
    let node = NodeKind::Container(ContainerNode {
        children: Some(vec![NodeKind::Text(TextNode {
            text: name,
            style: Style {
                font_size: CssValue::Value(Some(LengthUnit::Rem(12f32))),
                color: CssValue::Value(Color([255, 255, 255, 255])),
                font_weight: CssValue::Value(FontWeight::from(600.0)),
                ..Style::default()
            },
        })]),
        style: Style {
            background_image: CssValue::Value(Some(BackgroundImages(
                vec![BackgroundImage::Linear(LinearGradient {
                    angle: Angle::new(135.0),
                    stops: vec![
                        GradientStop::ColorHint {
                            color: start,
                            hint: Some(StopPosition(LengthUnit::Percentage(0.0))),
                        },
                        GradientStop::ColorHint {
                            color: end,
                            hint: Some(StopPosition(LengthUnit::Percentage(100.0))),
                        },
                    ]
                    .into(),
                })]
                .into(),
            ))),
            width: CssValue::Value(LengthUnit::Rem(32.0)),
            height: CssValue::Value(LengthUnit::Rem(32.0)),
            align_items: CssValue::Value(Some(AlignItems::Center)),
            justify_content: CssValue::Value(Some(JustifyContent::Center)),
            ..Style::default()
        },
    });

    let context = GlobalContext::default();

    let viewport = Viewport::new(512, 512);

    context
        .font_context
        .load_and_store(
            include_bytes!("../../MapleMonoNormalNL-NF-CN-Regular.ttf"),
            None,
            None,
        )
        .unwrap();

    let img = render(viewport, &context, node).unwrap();

    let mut buf: Vec<u8> = Vec::new();

    {
        let mut cursor = Cursor::new(&mut buf);
        img.write_to(&mut cursor, ImageFormat::Png).unwrap();
    }

    buf
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

use std::{borrow::Cow, io::Cursor};

use image::ImageFormat;
use regex::Regex;
use takumi::{
    layout::{
        node::{ContainerNode, NodeKind, TextNode},
        style::{
            AlignItems, Angle, BackgroundImage, Color, ColorInput, CssValue, FontWeight,
            GradientStop, JustifyContent, Length, LinearGradient, StopPosition, Style,
        },
        Viewport,
    },
    rendering::{render, RenderOptionsBuilder},
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
    let name_to_display =
        get_name_to_display_from_parts(first_name.to_owned(), last_name.to_owned());
    let full_name = format!("{}{}", first_name, last_name);
    let colors = get_gradient_colors(full_name);
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

fn is_chinese_char(c: char) -> bool {
    // Check if character is in CJK Unified Ideographs range
    matches!(c,
        '\u{4E00}'..='\u{9FFF}' | // CJK Unified Ideographs
        '\u{3400}'..='\u{4DBF}' | // CJK Unified Ideographs Extension A
        '\u{20000}'..='\u{2A6DF}' | // CJK Unified Ideographs Extension B
        '\u{2A700}'..='\u{2B73F}' | // CJK Unified Ideographs Extension C
        '\u{2B740}'..='\u{2B81F}' | // CJK Unified Ideographs Extension D
        '\u{2B820}'..='\u{2CEAF}' | // CJK Unified Ideographs Extension E
        '\u{F900}'..='\u{FAFF}' | // CJK Compatibility Ideographs
        '\u{2F800}'..='\u{2FA1F}' // CJK Compatibility Ideographs Supplement
    )
}

fn has_chinese_chars(s: &str) -> bool {
    s.chars().any(is_chinese_char)
}

fn get_name_to_display_from_parts(first_name: String, last_name: String) -> String {
    let first_trimmed = first_name.trim();
    let last_trimmed = last_name.trim();

    // Handle empty names
    if first_trimmed.is_empty() && last_trimmed.is_empty() {
        return "?".to_string();
    }
    if first_trimmed.is_empty() {
        let first_char = last_trimmed.chars().nth(0).unwrap();
        return first_char.to_uppercase().to_string();
    }
    if last_trimmed.is_empty() {
        let first_char = first_trimmed.chars().nth(0).unwrap();
        return first_char.to_uppercase().to_string();
    }

    // Check if either name contains Chinese characters
    let has_chinese = has_chinese_chars(first_trimmed) || has_chinese_chars(last_trimmed);

    if has_chinese {
        // For Chinese names, last name typically comes first and is usually one character
        // We want to show the last name character (family name) + first character of first name
        let last_first_char = last_trimmed.chars().nth(0).unwrap();
        let first_first_char = first_trimmed.chars().nth(0).unwrap();

        // If both are Chinese characters, just show them as-is (no uppercase needed)
        if is_chinese_char(last_first_char) && is_chinese_char(first_first_char) {
            format!("{}{}", last_first_char, first_first_char)
        } else {
            // Mixed case - apply uppercase to non-Chinese characters
            format!(
                "{}{}",
                if is_chinese_char(last_first_char) {
                    last_first_char.to_string()
                } else {
                    last_first_char.to_uppercase().to_string()
                },
                if is_chinese_char(first_first_char) {
                    first_first_char.to_string()
                } else {
                    first_first_char.to_uppercase().to_string()
                }
            )
        }
    } else {
        // For English names, show first character of first name + first character of last name
        let first_char = first_trimmed
            .chars()
            .nth(0)
            .unwrap()
            .to_uppercase()
            .to_string();
        let last_char = last_trimmed
            .chars()
            .nth(0)
            .unwrap()
            .to_uppercase()
            .to_string();
        format!("{}{}", first_char, last_char)
    }
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
        children: Some(
            [NodeKind::Text(TextNode {
                text: name,
                style: Some(Style {
                    font_size: CssValue::Value(Some(Length::Rem(12f32))),
                    color: CssValue::Value(ColorInput::Value(Color([255, 255, 255, 255]))),
                    font_weight: CssValue::Value(FontWeight::from(600.0)),
                    ..Style::default()
                }),
                ..Default::default()
            })]
            .into(),
        ),
        style: Some(Style {
            background_image: CssValue::Value(Some(
                [BackgroundImage::Linear(LinearGradient {
                    angle: Angle::new(135.0),
                    stops: [
                        GradientStop::ColorHint {
                            color: ColorInput::Value(start),
                            hint: Some(StopPosition(Length::Percentage(0.0))),
                        },
                        GradientStop::ColorHint {
                            color: ColorInput::Value(end),
                            hint: Some(StopPosition(Length::Percentage(100.0))),
                        },
                    ]
                    .into(),
                })]
                .into(),
            )),
            width: CssValue::Value(Length::Rem(32.0)),
            height: CssValue::Value(Length::Rem(32.0)),
            align_items: CssValue::Value(AlignItems::Center),
            justify_content: CssValue::Value(JustifyContent::Center),
            ..Style::default()
        }),
        ..Default::default()
    });

    let mut context = GlobalContext::default();

    let viewport = Viewport::new(Some(512), Some(512));

    context
        .font_context
        .load_and_store(
            Cow::from(include_bytes!("../../MapleMonoNormalNL-NF-CN-Regular.ttf")),
            None,
            None,
        )
        .unwrap();

    let options = RenderOptionsBuilder::default()
        .viewport(viewport)
        .node(node)
        .global(&context)
        .build()
        .unwrap();

    let img = render(options).unwrap();

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

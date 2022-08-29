use super::constants::{
    COLOR_ACCENT, COLOR_BUTTON_DEFAULT, COLOR_FOREGROUND, FONT_SIZE_LARGE, FONT_SIZE_MEDIUM,
    FONT_SIZE_SMALL,
};
use bevy::prelude::*;

pub fn default_small_button_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: FONT_SIZE_SMALL,
        color: COLOR_FOREGROUND,
    }
}

#[allow(dead_code)]
pub fn default_medium_button_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: FONT_SIZE_MEDIUM,
        color: COLOR_FOREGROUND,
    }
}

#[allow(dead_code)]
pub fn default_large_button_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: FONT_SIZE_LARGE,
        color: COLOR_FOREGROUND,
    }
}

#[allow(dead_code)]
pub fn accent_small_button_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: FONT_SIZE_SMALL,
        color: COLOR_ACCENT,
    }
}

pub fn accent_medium_button_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: FONT_SIZE_MEDIUM,
        color: COLOR_ACCENT,
    }
}

pub fn accent_large_button_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: FONT_SIZE_LARGE,
        color: COLOR_ACCENT,
    }
}

pub fn default_node_bundle_style() -> Style {
    Style {
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::ColumnReverse,
        align_items: AlignItems::Center,
        align_self: AlignSelf::Center,
        ..default()
    }
}

pub fn default_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(40.), Val::Percent(20.)),
            padding: UiRect::all(Val::Px(20.0)),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: COLOR_BUTTON_DEFAULT.into(),
        ..default()
    }
}

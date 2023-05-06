use bevy::prelude::*;

use super::UiAssets;

const FONT_SIZE: f32 = 18.;

#[derive(Component)]
pub struct ClickableButton;

pub fn button_click_animation(
    mut interactions: Query<(&Interaction, &mut Transform), (Changed<Interaction>, With<ClickableButton>)>
) {
    for (interaction, mut transform) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                transform.scale = Vec3::new(0.95, 0.95, 1.);
            },
            _ => {
                transform.scale = Vec3::splat(1.);
            }
        }
    }
}

pub fn get_button(
    commands: &mut Commands,
    size: Size,
    margin: UiRect,
    image: &Handle<Image>,
) -> Entity {
    commands.spawn((
            ClickableButton,
            ButtonBundle {
                style: Style {
                    size,
                    margin,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: UiImage::new(image.clone()),
                ..Default::default()
            }
        ))
        .id()
}

pub fn get_text_bundle(
    text: &str,
    assets: &UiAssets
) -> impl Bundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                color: Color::WHITE,
                font: assets.font.clone(),
                font_size: 18.,
                ..Default::default()
            }
        ),
        ..Default::default() 
    }
}
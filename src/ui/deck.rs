use bevy::prelude::*;
use crate::player::{
    cards::CardHolder,
    Deck, DeckEvent, DeckEventKind
};

use super::{helpers, ReloadUiEvent, UiAssets};

const DECK_HEIGHT: f32 = 150.;
const CARD_WIDTH: f32 = 96.;
const CARD_HEIGHT: f32 = 128.;
const CARD_MARGIN: f32 = 4.;
const CARD_SELECT: f32 = 24.;

#[derive(Component)]
pub struct DeckMenu;

#[derive(Component)]
pub struct CardButton(Entity, bool);

pub fn card_click(
    mut interactions: Query<(&Interaction, &mut CardButton), Changed<Interaction>>,
    mut ev_deck: EventWriter<DeckEvent>,
    mut ev_ui: EventWriter<ReloadUiEvent>
) {
    for (interaction, mut button) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                button.1 = true
            },
            Interaction::Hovered => {
                if button.1 {
                    ev_deck.send(DeckEvent(
                        DeckEventKind::SelectCard(button.0)
                    ));
                    ev_ui.send(super::ReloadUiEvent);
                }
                button.1 = false;
            },
            Interaction::None => button.1 = false
        }
    }
}

pub fn draw_deck(
    mut commands: Commands,
    deck_query: Query<Entity, With<DeckMenu>>,
    assets: Res<UiAssets>,
    deck: Res<Deck>,
    card_query: Query<&CardHolder>
) {
    clear_deck(&mut commands, &deck_query);

    let container = commands.spawn((
            DeckMenu,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::bottom(Val::Px(0.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.), Val::Px(DECK_HEIGHT)),
                    ..Default::default()
                },
                ..Default::default()
            }
        ))
        .id();
        
        for card_entity in deck.cards.iter() {
            let Ok(card_holder) = card_query.get(*card_entity) else { continue };

            // the active card will be shifted upwards a little
            let mut margin = UiRect::all(Val::Px(CARD_MARGIN));
            if Some(*card_entity) == deck.current_card {
                margin.bottom = Val::Px(CARD_SELECT);
            }

            let button = helpers::get_button(
                &mut commands,
                Size::new(Val::Px(CARD_WIDTH), Val::Px(CARD_HEIGHT)),
                margin,
                &assets.textures["card"],
            );

            // add card component to the button
            commands.entity(button).insert(CardButton(*card_entity, false));

            // set button's content
            let content = commands.spawn(
                    helpers::get_text_bundle(&card_holder.0.get_label(), assets.as_ref())
                )
                .id();
            commands.entity(button).add_child(content);

            // parent button to the container
            commands.entity(container).add_child(button);
        }
}

fn clear_deck(
    commands: &mut Commands,
    query: &Query<Entity, With<DeckMenu>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

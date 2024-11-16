use crate::entity::dropped_item::spawn_dropped_item;
use crate::ui::floating::{Floating, FloatingContent};
use crate::{
    asset::GameAssets, constant::MAX_ITEMS_IN_INVENTORY, controller::player::Player,
    entity::actor::Actor, inventory_item::InventoryItem, states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AsepriteSlice, AsepriteSliceUiBundle};

use super::item_information::{SpellInformation, SpellInformationItem};

#[derive(Component)]
struct InventoryItemSlot(usize);

pub fn spawn_inventory(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    width: Val::Px(32.0 * 8.0),
                    height: Val::Px(32.0 * 8.0),
                    // Make the height of the node fill its parent
                    // height: Val::Percent(100.0),
                    // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                    // As the height is set explicitly, this means the width will adjust to match the height
                    // aspect_ratio: Some(1.0),
                    // Use grid layout for this node
                    // display: Display::Grid,
                    // Add 24px of padding around the grid
                    // padding: UiRect::all(Val::Px(0.0)),
                    // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                    // This creates 4 exactly evenly sized columns
                    // grid_template_columns: RepeatedGridTrack::flex(8, 1.0),
                    // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                    // This creates 4 exactly evenly sized rows
                    // grid_template_rows: RepeatedGridTrack::flex(8, 1.0),
                    // Set a 12px gap/gutter between rows and columns
                    // row_gap: Val::Px(2.0),
                    // column_gap: Val::Px(2.0),
                    ..default()
                },
                // background_color: BackgroundColor(Color::hsla(0.0, 0.0, 0.5, 0.2)),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                StateScoped(GameState::MainMenu),
                ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(32.0 * 8.0),
                        height: Val::Px(32.0 * 8.0),
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    z_index: ZIndex::Local(0),
                    ..default()
                },
                AsepriteSliceUiBundle {
                    aseprite: assets.atlas.clone(),
                    slice: "inventory".into(),
                    ..default()
                },
            ));

            for i in 0..MAX_ITEMS_IN_INVENTORY {
                builder.spawn((
                    InventoryItemSlot(i),
                    StateScoped(GameState::MainMenu),
                    Interaction::default(),
                    ImageBundle {
                        style: Style {
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
                            // gird layoutだと、兄弟要素の大きさに左右されてレイアウトが崩れてしまう場合があるので、
                            // Absoluteでずれないようにする
                            position_type: PositionType::Absolute,
                            left: Val::Px((i % 8) as f32 * 32.0),
                            top: Val::Px((i / 8) as f32 * 32.0),
                            ..default()
                        },
                        z_index: ZIndex::Local(1),
                        ..default()
                    },
                    AsepriteSliceUiBundle {
                        aseprite: assets.atlas.clone(),
                        slice: "empty".into(),
                        ..default()
                    },
                ));
            }
        });
}

fn update_inventory_slot(
    query: Query<&Player>,
    mut slot_query: Query<(&InventoryItemSlot, &mut AsepriteSlice, &mut Style)>,
    floating_query: Query<&Floating>,
) {
    if let Ok(player) = query.get_single() {
        let floating = floating_query.single();
        for (slot, mut aseprite, mut style) in slot_query.iter_mut() {
            match floating.0 {
                Some(FloatingContent::Inventory(index)) => {
                    if index == slot.0 {
                        *aseprite = "empty".into();
                        continue;
                    }
                }
                _ => {}
            }

            let item = player.inventory.get(slot.0);
            let slice: &'static str = match item {
                Some(item) => item.get_icon(),
                None => "empty",
            };
            *aseprite = slice.into();

            style.width = Val::Px(
                32.0 * match item {
                    Some(InventoryItem::Wand(_)) => 2.0,
                    _ => 1.0,
                },
            );
        }
    }
}

fn interaction(
    mut interaction_query: Query<
        (&InventoryItemSlot, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut floating_query: Query<&mut Floating>,
    mut player_query: Query<(&mut Player, &mut Actor, &Transform)>,

    mut spell_info_query: Query<&mut SpellInformation>,

    mut commands: Commands,

    assets: Res<GameAssets>,
) {
    for (slot, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok((mut player, mut actor, player_position)) = player_query.get_single_mut()
                {
                    let mut floating = floating_query.single_mut();
                    match floating.0 {
                        None => {
                            *floating = Floating(Some(FloatingContent::Inventory(slot.0)));
                        }
                        Some(FloatingContent::Inventory(index)) => {
                            if index == slot.0 {
                                *floating = Floating(None);
                            } else {
                                match (player.inventory.get(index), player.inventory.get(slot.0)) {
                                    (Some(floating_item), None) => {
                                        if player.inventory.is_settable(slot.0, floating_item) {
                                            player.inventory.set(slot.0, Some(floating_item));
                                            player.inventory.set(index, None);
                                            *floating = Floating(None);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Some(FloatingContent::WandSpell(wand_index, spell_index)) => match actor
                            .wands[wand_index]
                        {
                            None => {
                                *floating = Floating(None);
                            }
                            Some(ref mut wand) => {
                                let spell = wand.slots[spell_index];
                                player
                                    .inventory
                                    .set(slot.0, spell.and_then(|s| Some(InventoryItem::Spell(s))));
                                wand.slots[spell_index] = None;
                                actor.wands[wand_index] = Some(wand.clone());
                                *floating = Floating(None);
                            }
                        },
                        Some(FloatingContent::Wand(wand_index)) => {
                            if let Some(ref wand) = actor.wands[wand_index] {
                                let current = player.inventory.get(slot.0);
                                player
                                    .inventory
                                    .set(slot.0, Some(InventoryItem::Wand(wand.wand_type)));

                                // 杖に入っていた呪文はインベントリの空きスロットに入れる
                                let mut not_inserted = Vec::new();
                                for slot in wand.slots {
                                    if let Some(spell) = slot {
                                        if !player.inventory.insert(InventoryItem::Spell(spell)) {
                                            not_inserted.push(spell);
                                        }
                                    }
                                }

                                // インベントリに入らなかった分は床にばらまかれる
                                for spell in not_inserted {
                                    // drop items
                                    spawn_dropped_item(
                                        &mut commands,
                                        &assets,
                                        player_position.translation.x,
                                        player_position.translation.y,
                                        InventoryItem::Spell(spell),
                                    );
                                }

                                actor.wands[wand_index] = None;

                                match current {
                                    None => {
                                        *floating = Floating(None);
                                    }
                                    Some(_) => {
                                        *floating =
                                            Floating(Some(FloatingContent::Inventory(slot.0)));
                                    }
                                }
                            }
                        }
                        Some(FloatingContent::Equipment(index)) => {
                            let equipment = player.equipments[index].unwrap();
                            if player
                                .inventory
                                .try_set(slot.0, InventoryItem::Equipment(equipment))
                            {
                                player.equipments[index] = None;
                                *floating = Floating(None);
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                let mut spell_info = spell_info_query.single_mut();
                if let Ok((player, actor, _)) = player_query.get_single() {
                    let floating = floating_query.single_mut();
                    let floating_item = floating.get_item(&player, &actor);
                    if player.inventory.is_settable_optional(slot.0, floating_item) {
                        *color = Color::hsla(0.0, 0.0, 0.5, 0.95).into();
                        *spell_info = match player.inventory.get(slot.0) {
                            Some(slot_item) => SpellInformation(Some(
                                SpellInformationItem::InventoryItem(slot_item),
                            )),
                            None => SpellInformation(None),
                        };
                    } else {
                        *spell_info = SpellInformation(None);
                        *color = Color::hsla(0.0, 0.0, 0.0, 0.0).into();
                    }
                } else {
                    *spell_info = SpellInformation(None);
                    *color = Color::hsla(0.0, 0.0, 0.0, 0.0).into();
                }
            }
            Interaction::None => {
                *color = Color::hsla(0.0, 0.0, 0.0, 0.0).into();
            }
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_inventory_slot, interaction).run_if(in_state(GameState::InGame)),
        );
    }
}

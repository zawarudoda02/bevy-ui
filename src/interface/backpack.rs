use crate::robot::backpack::BackPack;
use crate::states::LifeCycleSets;
use bevy::prelude::*;
use robotics_lib::world::tile::Content;

#[derive(Component)]
pub struct BackPackUiMarker;

#[derive(Component)]
pub struct BackpackElementContent {
    content: Content,
}
impl BackpackElementContent {
    fn new(c: Content) -> Self {
        Self {
            content: c.to_default(),
        }
    }
    fn eq(&self, c: Content) -> bool {
        self.content.eq(&c)
    }
    fn print(&self, q: usize) -> String {
        let content_string = match self.content {
            Content::Rock(_) => "Rock",
            Content::Tree(_) => "Tree",
            Content::Garbage(_) => "Garbage",
            Content::Fire => "Fire",
            Content::Coin(_) => "Coin",
            Content::Bin(_) => "Bin",
            Content::Crate(_) => "Crate",
            Content::Bank(_) => "Bank",
            Content::Water(_) => "Water",
            Content::Market(_) => "Market",
            Content::Fish(_) => "Fish",
            Content::Building => "Building",
            Content::Bush(_) => "Bush",
            Content::JollyBlock(_) => "Jollyblock",
            Content::Scarecrow => "Scarecrow",
            Content::None => "",
        };
        format!("{} : {}", content_string, q)
    }
}

#[derive(Component)]
pub struct BackpackElementMarker;

pub fn spawn_backpack_ui() -> Box<dyn FnOnce(&mut ChildBuilder)> {
    let text = "Backpack";
    Box::new(move |parent| {
        parent.spawn(
            (TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            )),
        );
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        height: Val::Percent(50.),

                        ..default()
                    },
                    background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                    ..default()
                },
                BackPackUiMarker,
            ))
            .with_children(spawn_backpack_content_ui());
    })
}

pub fn spawn_backpack_content_ui() -> Box<dyn FnOnce(&mut ChildBuilder)> {
    Box::new(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },

                ..default()
            })
            .with_children(|parent| {
                for i in [
                    Content::Rock(0),
                    Content::Tree(0),
                    Content::Garbage(0),
                    Content::Fire,
                    Content::Coin(0),
                    Content::Bin(0..1),
                    Content::Crate(0..1),
                    Content::Bank(0..1),
                    Content::Water(0),
                    Content::Market(0),
                    Content::Fish(0),
                    Content::Building,
                    Content::Bush(0),
                    Content::JollyBlock(0),
                    Content::Scarecrow,
                ] {
                    let c = i.to_default();
                    let b_element_content = BackpackElementContent::new(c);
                    parent.spawn((
                        TextBundle::from_section(
                            b_element_content.print(0),
                            TextStyle {
                                font_size: 20.,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        b_element_content,
                        BackpackElementMarker,
                    ));
                }
            });
    })
}
pub fn update_backpack_ui(
    backpack: Res<BackPack>,
    mut query: Query<(&mut Text, &mut BackpackElementContent), With<BackpackElementMarker>>,
) {
    let backpack_contents = backpack.get_all();
    for (mut text, mut content) in &mut query {
        for (i, quantity) in backpack_contents.iter() {
            if content.eq(i.clone()) {
                text.sections[0].value = content.print(*quantity);
            }
        }
    }
}
pub struct BackPackUiPlugin;
impl Plugin for BackPackUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_backpack_ui.in_set(LifeCycleSets::Robot));
    }
}

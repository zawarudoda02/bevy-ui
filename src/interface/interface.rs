use super::backpack::spawn_backpack_ui;
use super::world::spawn_weather_ui;
use crate::interface::robot::spawn_robot_ui;
use crate::interface::world::spawn_hovered_tile;
use crate::states::UiStates;
use bevy::prelude::*;

#[derive(Component)]
struct ViewPortMarker;

fn spawn_ui(mut commands: Commands) {
    //spawning root node
    commands
        .spawn(NodeBundle {
            node: Default::default(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        })
        .with_children(spawn_main_containers());
}

fn spawn_main_containers() -> Box<dyn FnOnce(&mut ChildBuilder)> {
    Box::new(|parent| {
        //leftbox
        parent
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(40.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::rgb_u8(35, 35, 38).into(),

                ..default()
            })
            .with_children(spawn_backpack_ui())
            .with_children(spawn_hovered_tile());
        //viewport
        parent.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            ViewPortMarker,
        ));
        //rightbox
        parent
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(40.0),

                    row_gap: Val::Percent(10.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::rgb_u8(35, 35, 38).into(),
                ..default()
            })
            .with_children(spawn_robot_ui())
            .with_children(spawn_weather_ui());
    })
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiStates::Setup), (spawn_ui, apply_deferred).chain())
            .add_systems(OnEnter(UiStates::End), simulation_finished);
    }
}

fn simulation_finished(mut commands: Commands, query: Query<Entity, With<ViewPortMarker>>) {
    let entity = query.get_single().unwrap();
    commands.entity(entity).with_children(|parent| {
        let mut text_bundle = TextBundle::from_section(
            "Simulation Finished",
            TextStyle {
                font: Default::default(),
                font_size: 30.0,
                color: Color::WHITE,
            },
        );
        text_bundle.visibility = Visibility::Visible;
        text_bundle.background_color = BackgroundColor(Color::BLACK);
        parent.spawn(text_bundle);
    });
}

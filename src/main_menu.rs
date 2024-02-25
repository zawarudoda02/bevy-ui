use bevy::a11y::accesskit::Action::Default;
use bevy::prelude::*;
use std::path::Path;
use std::process::Command;
use std::string::ToString;
use std::thread;
use std::time::Duration;

use crate::states::{UiStates, UiSystemSet};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
/*
const SERVER_IP :&str = "127.0.0.1:42597";
const PEPE_AI_PATH:&str = "..\\advanced_programming_ai\\target\\debug\\advanced_programming_ai.exe";
const PEPE_ARGS: [&str;4] = ["--address",SERVER_IP,"--side","128"];

 */
fn spawn_title_text(commands: &mut Commands) -> Entity {
    let text = "Robot UI in Bevy!";

    commands
        .spawn(
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 100.0,
                    color: Color::WHITE,
                    ..default()
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Relative,
                ..default()
            }),
        )
        .id()
}
//spawns button with given text
fn spawn_button(commands: &mut Commands, ai: AiExec) -> Entity {
    let button_text_node = TextBundle::from_section(
        &ai.name,
        TextStyle {
            font_size: 20.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );
    let ui_button = ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        interaction: Interaction::None,
        ..default()
    };

    let button = commands.spawn((ui_button, ai)).id();
    let button_text = commands.spawn(button_text_node).id();
    commands.entity(button).push_children(&[button_text]);
    button
}
#[derive(Component)]
struct ContainerMarker;
fn begin_main_menu(mut commands: Commands) {
    let container_node = NodeBundle {
        style: Style {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,

            justify_content: JustifyContent::Center,

            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    };
    let container = commands.spawn((container_node, ContainerMarker)).id();
    let main_text = spawn_title_text(&mut commands);
    let button1 = spawn_button(
        &mut commands,
        AiExec {
            name: "Vincenzo Pepe".into(),
            windows_script: ".\\pepe.bat".into(),
            linux_script: "./pepe.sh".into(),
        },
    );
    let button2 = spawn_button(
        &mut commands,
        AiExec {
            name: "Jag".into(),
            windows_script: "\\".into(),
            linux_script: "./".into(),
        },
    );
    commands
        .entity(container)
        .push_children(&[main_text, button1, button2]);
}
pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            begin_main_menu.in_set(UiSystemSet::MainMenuBegin),
        )
        .add_systems(Update, button_system.in_set(UiSystemSet::MainMenuCycle))
        .add_systems(
            OnExit(UiStates::MainMenu),
            end_main_menu.in_set(UiSystemSet::MainMenuEnd),
        );
    }
}

#[derive(Component)]
pub struct AiExec {
    pub name: String,
    pub windows_script: String,
    pub linux_script: String,
}
fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &AiExec,
        ),
        (With<Button>, Changed<Interaction>),
    >,
    mut next: ResMut<NextState<UiStates>>,
) {
    for (e, interaction, mut bg_color, children, ai_exec) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                debug!("aoooo");

                let command = {
                    if cfg!(target_os = "windows") {
                        Command::new(&ai_exec.windows_script).spawn()
                    } else {
                        Command::new(&ai_exec.linux_script).spawn()
                    }
                };

                thread::sleep(Duration::from_millis(500));

                next.set(UiStates::AwaitingFirstMessage);
            }
            Interaction::Hovered => *bg_color = Color::DARK_GRAY.into(),
            Interaction::None => *bg_color = NORMAL_BUTTON.into(),
        }
    }
}

fn end_main_menu(mut commands: Commands, text_query: Query<Entity, &ContainerMarker>) {
    let container = text_query.single();
    commands.entity(container).despawn_recursive();
}

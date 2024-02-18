use bevy::prelude::*;



use strum_macros::{EnumIter};
use robotics_lib::world::tile::{Content};
use crate::states::UiStates;


#[derive(Component)]
struct BackPackUiMarker;
#[derive(Component)]
struct EnergyUiMarker;
#[derive(Component)]
struct WeatherUiMarker;
#[derive(Component)]
struct BackpackElementMarker;
#[derive(Component)]
struct CurrentTimeMarker;
#[derive(Component)]
struct CurrentWeatherMarker;
#[derive(Component)]
struct BackpackElementContent {
    content: Content
}
impl BackpackElementContent {
    fn new(c: Content) -> Self {
        Self {
            content: c.to_default()
        }
    }
    fn eq(&self, c: Content) ->bool{
        self.content.eq(&c)
    }
    fn print(&self, q: usize)-> String{
        let content_string = match self.content{
            Content::Rock(_) => {"Rock"}
            Content::Tree(_) => {"Tree"}
            Content::Garbage(_) => {"Garbage"}
            Content::Fire => {"Fire"}
            Content::Coin(_) => {"Coin"}
            Content::Bin(_) => {"Bin"}
            Content::Crate(_) => {"Crate"}
            Content::Bank(_) => {"Bank"}
            Content::Water(_) => {"Water"}
            Content::Market(_) => {"Market"}
            Content::Fish(_) => {"Fish"}
            Content::Building => {"Building"}
            Content::Bush(_) => {"Bush"}
            Content::JollyBlock(_) => {"Jollyblock"}
            Content::Scarecrow => {"Scarecrow"}
            Content::None => {""}
        };
        format!("{} : {}",content_string,q)
    }
}
fn spawn_ui(mut commands: Commands){
    //spawning root node
    commands.spawn(
        NodeBundle{
            node: Default::default(),
            style: Style{
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ).with_children(spawn_main_containers());


}

fn spawn_main_containers() -> Box<dyn FnOnce(&mut ChildBuilder)>{
    Box::new(|parent| {
        //leftbox
        parent.spawn(
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(40.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color:Color::rgb_u8(35,35,38).into(),

                ..default()
            }
        ).with_children(spawn_backpack_ui());
        //viewport
        parent.spawn(
            NodeBundle{
                style: Style{
                    display: Display::Flex,
                    flex_direction:FlexDirection::Column,
                    width:Val::Percent(100.0),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()

            }
        );
        //rightbox
        parent.spawn(
            NodeBundle{
                style: Style{
                    display: Display::Flex,
                    flex_direction:FlexDirection::Column,
                    width:Val::Percent(40.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::rgb_u8(35,35,38).into(),
                ..default()

            }
        ).with_children(spawn_energy_ui()).with_children(spawn_weather_ui());
    })
}
fn spawn_backpack_ui() -> Box<dyn FnOnce(&mut ChildBuilder)> {
    let text = "Backpack";
    Box::new(move |parent| {
        parent.spawn((TextBundle::from_section(
            text,
            TextStyle{
                font_size:20.0,
                color: Color::WHITE,
                ..default()
            }
        ))
        );
        parent.spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                height: Val::Percent(50.),

                ..default()
            },
            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
            ..default()
        },BackPackUiMarker)
        ).with_children(spawn_backpack_content_ui())
        ;

    })
}

fn spawn_backpack_content_ui() -> Box<dyn FnOnce(&mut ChildBuilder)>{

    Box::new(|parent| {
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },

                ..default()
            },
        ).with_children(|parent|
            {


                for i in [Content::Rock(0),Content::Tree(0),Content::Garbage(0),Content::Fire,Content::Coin(0),Content::Bin(0..1),Content::Crate(0..1),Content::Bank(0..1),Content::Water(0),Content::Market(0),Content::Fish(0),Content::Building,Content::Bush(0),Content::JollyBlock(0),Content::Scarecrow,Content::None]{
                    let c = i.to_default();
                    let b_element_content = BackpackElementContent::new(c);
                    parent.spawn((
                        TextBundle::from_section(
                            b_element_content.print(0),
                            TextStyle{
                                font_size:20.,
                                color: Color::WHITE,
                                ..default()
                            }

                        ),
                        b_element_content,
                        BackpackElementMarker
                    )

                    );
                }
            }

        );

    })
}
fn spawn_energy_ui() -> Box<dyn FnOnce(&mut ChildBuilder)>{
    let text = "Energy: 1000";
    Box::new(move |parent| {
        parent.spawn((TextBundle::from_section(
            text,
            TextStyle{
                font_size:20.0,
                color: Color::WHITE,
                ..default()
            }
        ),EnergyUiMarker)
        );

    })
}

fn spawn_weather_ui() -> Box<dyn FnOnce(&mut ChildBuilder)>{
   Box::new( |parent|{
       //current time
       parent.spawn((
           TextBundle::from_section(
               "current time: ",
               TextStyle{
                   font_size:20.0,
                   color: Color::WHITE,
                   ..default()
               }
           ),CurrentTimeMarker)
       );

       parent.spawn((
           TextBundle::from_section(
               "current weather: ",
               TextStyle{
                    font_size:20.0,
                   color:Color::WHITE,
                   ..default()
               }
           ),
           CurrentWeatherMarker
           ));





   }
   )
}
pub struct UiPlugin;
impl Plugin for UiPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiStates::Setup),spawn_ui);
    }
}



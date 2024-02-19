use bevy::prelude::*;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;


use strum_macros::{EnumIter};
use robotics_lib::world::tile::{Content};
use crate::robot::backpack::BackPack;
use crate::robot::res::RobotEnergy;
use crate::states::{LifeCycleSets, UiStates};
use crate::world::res::CurrentWeather;


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
#[derive(Component)]
struct ViewPortMarker;
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
            (
            NodeBundle{
                style: Style{
                    display: Display::Flex,
                    flex_direction:FlexDirection::Column,
                    width:Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()

            },ViewPortMarker)
        );
        //rightbox
        parent.spawn(
            NodeBundle{
                style: Style{
                    display: Display::Flex,
                    flex_direction:FlexDirection::Column,
                    width:Val::Percent(40.0),

                    row_gap: Val::Percent(10.0),
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


                for i in [Content::Rock(0),Content::Tree(0),Content::Garbage(0),Content::Fire,Content::Coin(0),Content::Bin(0..1),Content::Crate(0..1),Content::Bank(0..1),Content::Water(0),Content::Market(0),Content::Fish(0),Content::Building,Content::Bush(0),Content::JollyBlock(0),Content::Scarecrow]{
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
               "Time : ",
               TextStyle{
                   font_size:20.0,
                   color: Color::WHITE,
                   ..default()
               }
           ),CurrentTimeMarker)
       );

       parent.spawn((
           TextBundle::from_section(
               "Weather : ",
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
        app
            .add_systems(OnEnter(UiStates::Setup),(spawn_ui,apply_deferred).chain())
            .add_systems(Update,update_energy_ui.in_set(LifeCycleSets::Robot))
            .add_systems(Update, update_backpack_ui.in_set(LifeCycleSets::Robot))
            .add_systems(Update, (update_weather,update_time).chain().in_set(LifeCycleSets::Robot))
            .add_systems(OnEnter(UiStates::End),simulation_finished);
        ;
    }
}


fn update_energy_ui(energy: Res<RobotEnergy>, mut query: Query<&mut Text,With<EnergyUiMarker>> ) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Energy : {:?}",energy.energy);
}

fn update_backpack_ui(backpack: Res<BackPack>,mut query: Query<(&mut Text, &mut BackpackElementContent), With<BackpackElementMarker>>){
    let backpack_contents = backpack.get_all();
    for(mut text, mut content) in &mut query{
        for (i,quantity) in backpack_contents.iter(){
            if content.eq(i.clone()){
                text.sections[0].value = content.print(*quantity);
            }
        }
    }

}

fn update_time(weather: Res<CurrentWeather>, mut query: Query<&mut Text,With<CurrentTimeMarker>>){
    if let Some(weather )= &weather.conditions{
        let mut text = query.get_single_mut().unwrap();
        text.sections[0].value = format!("Time : {:?} || {:?}",weather.get_time_of_day(),weather.get_time_of_day_string());

    }
}
fn update_weather(weather: Res<CurrentWeather>, mut query: Query<&mut Text,With<CurrentWeatherMarker>>){
    if let Some(weather )= &weather.conditions{
        let mut text = query.get_single_mut().unwrap();
        text.sections[0].value = format!("Weather : {:?}",weather.get_weather_condition());

    }
}

fn simulation_finished(mut commands: Commands,mut query: Query<Entity,With<ViewPortMarker>> ){
    let entity = query.get_single().unwrap();
    commands.entity(entity).with_children(|parent| {
        let mut  text_bundle = TextBundle::from_section(
            "Simulation Finished",
            TextStyle{
                font: Default::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }
        );
        text_bundle.visibility = Visibility::Visible;
        text_bundle.background_color =BackgroundColor(Color::BLACK);
        parent.spawn(text_bundle);
    });
}
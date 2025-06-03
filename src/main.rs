use bevy::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(PomodoroTimer(Timer::new(
            Duration::from_secs(25 * 60),
            TimerMode::Once,
        )))
        .insert_resource(PomodoroState::Ready)
        .add_systems(Startup, setup)
        .add_systems(Update, (pomodoro_timer_system, button_system))
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct PomodoroTimer(Timer);

#[derive(Resource, PartialEq, Eq, Clone, Copy, Debug)]
enum PomodoroState {
    Ready,
    Running,
    Finished,
}

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
struct PomodoroText;

fn setup(mut cmds: Commands, timer: ResMut<PomodoroTimer>) {
    cmds.spawn(Camera2d::default());

    cmds.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Px(260.0),
                    height: Val::Px(120.0),
                    border: UiRect::all(Val::Px(5.0)),
                    margin: UiRect::all(Val::Px(5.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(
                    PomodoroText,
                    Text::new(remain_time_string(&timer)),
                    TextFont {
                        font_size: 80.0,
                        ..default()
                    },
                    TextLayout {
                        justify: JustifyText::Center,
                        linebreak: LineBreak::NoWrap,
                    },
                    TextColor(Color::WHITE),
                )]
            ),
            (
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(60.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                StartButton,
                Button,
                BorderColor(Color::WHITE),
                BorderRadius::MAX,
                children![(
                    Text::new("Start"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                ),]
            )
        ],
    ));
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut state: ResMut<PomodoroState>,
) {
    match *state {
        PomodoroState::Ready => {
            for (interaction, mut border_color) in interaction_query.iter_mut() {
                match *interaction {
                    Interaction::Pressed => {
                        *state = PomodoroState::Running;
                    }
                    Interaction::Hovered => {
                        *border_color = BorderColor(Color::srgb(0.5, 0.5, 0.5));
                    }
                    Interaction::None => {
                        *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.3));
                    }
                }
            }
        }
        PomodoroState::Running => {
            // Do nothing, timer is running
        }
        PomodoroState::Finished => {
            // Do nothing, waiting for reset
        }
    }
}

fn pomodoro_timer_system(
    time: Res<Time>,
    mut timer: ResMut<PomodoroTimer>,
    mut text_query: Query<&mut Text, With<PomodoroText>>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<PomodoroState>,
) {
    match *state {
        PomodoroState::Ready => {
            // Do nothing, waiting for button click
        }
        PomodoroState::Running => {
            if !timer.tick(time.delta()).just_finished() {
                if let Ok(mut text) = text_query.single_mut() {
                    text.0 = remain_time_string(&timer);
                }
            } else {
                *state = PomodoroState::Finished;

                if let Ok(mut text) = text_query.single_mut() {
                    text.0 = "Time's up!".to_string();
                }

                let a = asset_server.load::<AudioSource>("alarm.wav");
                cmds.spawn(AudioPlayer(a));
            }
        }
        PomodoroState::Finished => {
            // Timer is finished, waiting for reset
        }
    }
}

fn remain_time_string(timer: &PomodoroTimer) -> String {
    let secs = timer.remaining().as_secs();
    let min = secs / 60;
    let sec = secs % 60;
    format!("{:02}:{:02}", min, sec)
}

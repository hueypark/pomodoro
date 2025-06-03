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
            Duration::from_secs(5),
            TimerMode::Once,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, pomodoro_timer_system)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct PomodoroTimer(Timer);

fn setup(mut cmds: Commands) {
    cmds.spawn((Camera2d, Transform::default(), GlobalTransform::default()));
    cmds.spawn((
        PomodoroText,
        Text2d::new(""),
        TextFont {
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::default(),
    ));
}

#[derive(Component)]
struct PomodoroText;

fn pomodoro_timer_system(
    time: Res<Time>,
    mut timer: ResMut<PomodoroTimer>,
    mut query: Query<&mut Text2d, With<PomodoroText>>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut end: Local<bool>,
) {
    if *end {
        return;
    }

    if !timer.tick(time.delta()).just_finished() {
        if timer.finished() {
            return;
        }
        let secs = timer.remaining().as_secs();
        let min = secs / 60;
        let sec = secs % 60;
        if let Ok(mut text) = query.single_mut() {
            text.0 = format!("{:02}:{:02}", min, sec);
        }

        return;
    }

    *end = true;

    if let Ok(mut text) = query.single_mut() {
        text.0 = "Time's up!".to_string();
    }

    let a = asset_server.load::<AudioSource>("alarm.wav");
    cmds.spawn(AudioPlayer(a));
}

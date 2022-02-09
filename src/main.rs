use bevy::{
    prelude::*,
};

#[derive(Component)]
struct Troop {}

fn mouse_click_system(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut query: Query<(&Troop, &mut Transform)>,
) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        let size = Vec2::new(win.width() as f32, win.height() as f32);
        let default_orthographic_pos = size / 2.0;

        // convert mouse cursor position to world position
        let mut mouse_pos = win.cursor_position().unwrap();
        mouse_pos = mouse_pos - default_orthographic_pos;
        println!("{:?}", mouse_pos);

        let (_troop, mut transform) = query.single_mut();
        let translation = &mut transform.translation;
        translation.x = mouse_pos.x;
        translation.y = mouse_pos.y;
    }
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Troop {});
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.4, 0.9)))
        .add_startup_system(startup)
        .add_system(mouse_click_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

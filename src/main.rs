use std::io;
use std::cmp::Ordering;
use rand::Rng;
use bevy::{prelude::*, text::Text2dBounds};
use bevy::render::camera::RenderTarget;

#[derive(Default)]
struct Player {
    transform: Transform,
    entity: Option<Entity>
}

#[derive(Default)]
struct Game {
    player: Player,
}

#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct AnimateTranslation;
#[derive(Component)]
struct AnimateRotation;
#[derive(Component)]
struct AnimateScale;
#[derive(Component)]
struct DebugConsole;
#[derive(Component)]
struct MouseClickEffect;

const CLICK_EFFECT_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const MAIN_COLOR: Color = Color::rgb(1.0, 0.58, 0.27);
const SUB_COLOR: Color = Color::rgb(0.82, 0.82, 0.45);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    let font = asset_server.load("fonts/Mabinogi_Classic_TTF.ttf");
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;

    // 2d camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    // Demonstrate changing translation
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("translation 크크", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        })
        .insert(AnimateTranslation);
    // Demonstrate changing rotation
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("rotation 회전", text_style.clone()).with_alignment(text_alignment),
            ..default()
        })
        .insert(AnimateRotation);
    // Demonstrate changing scale
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("scale 야호", text_style.clone()).with_alignment(text_alignment),
            ..default()
        })
        .insert(AnimateScale);

    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_translation(box_position.extend(0.0)),
        ..default()
    });
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("mouse position: ", text_style),
        text_2d_bounds: Text2dBounds {
            // Wrap text in the rectangle
            size: box_size,
        },
        // We align text to the top-left, so this transform is the top-left corner of our text. The
        // box is centered at box_position, so it is necessary to move by half of the box size to
        // keep the text in the box.
        transform: Transform::from_xyz(
            box_position.x - box_size.x / 2.0,
            box_position.y + box_size.y / 2.0,
            1.0,
        ),
        ..default()
    }).insert(DebugConsole);

    game.player.transform = Transform::from_translation(Vec3::new(200.0, 0.0, 200.0));

    game.player.entity = Some(
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                texture: asset_server.load("sprites/frog_1.png"),
                ..default()
            })
            .id(),
    );

    commands
        .spawn()
        .insert(MouseClickEffect)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                scale: CLICK_EFFECT_SIZE,
                ..default()
            },
            visibility: Visibility {
                is_visible: false,
            },
            ..default()
        });
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(animate_translation)
        .add_system(animate_rotation)
        .add_system(animate_scale)
        .add_system(player_render)
        .add_system(my_cursor_system)
        .run();
    println!("숫자를 맞춰보자!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let random_char = rand::random::<char>();
    
    loop {
        println!("정답이라고 생각하는 숫자를 입력하세요. {}", random_char);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다. 뭘 쓴거야 이 멍청아!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력한 값: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    
    }
    
}

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Text, (With<Text>, With<DebugConsole>)>,
    mut cursor_effect: Query<(&mut Transform, &mut Visibility), With<MouseClickEffect>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();
    let (mut effect_transform, mut effect_visibility) = cursor_effect.single_mut();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        for mut text in &mut query {
            text.sections[0].value = format!("Mouse pos: ({},{})", world_pos.x, world_pos.y);
        }

        if mouse_input.pressed(MouseButton::Left) {
            effect_transform.translation.x = world_pos.x;
            effect_transform.translation.y = world_pos.y;
            effect_visibility.is_visible = true;
        }

        if mouse_input.just_released(MouseButton::Left) {
            effect_visibility.is_visible = false;
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        game.player.transform.translation.z -= 1.2;
    }
    if keyboard_input.pressed(KeyCode::S) {
        game.player.transform.translation.z += 1.2;
    }
    if keyboard_input.pressed(KeyCode::A) {
        game.player.transform.translation.x -= 1.2;
    }
    if keyboard_input.pressed(KeyCode::D) {
        game.player.transform.translation.x += 1.2;
    }
}

fn jump(
    
    mut game: ResMut<Game>,
    tartget_pos: Vec2,
) {
    
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32 - 400.0;
        transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.seconds_since_startup().cos() as f32);
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in &mut query {
        transform.translation = Vec3::new(400.0, 0.0, 0.0);
        transform.scale = Vec3::splat((time.seconds_since_startup().sin() as f32 + 1.1) * 2.0);
    }
}

fn player_render(game: ResMut<Game>, mut transforms: Query<&mut Transform>) {
    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(
            game.player.transform.translation.x,
            -game.player.transform.translation.z,
            0.0),
        scale: Vec3::splat(0.2),
        ..default()
    };
}

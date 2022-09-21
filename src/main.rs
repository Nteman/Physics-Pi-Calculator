use bevy::{prelude::*};
use bevy_prototype_lyon::prelude::*;

//Set equal to the desired number of digits after the decimal point '.'
// *Note this value can't be greater than 4, it causes the program to panic
// at an overflow error
const DIGITS: u32 = 3; 

// DO NOT CHANGE
const HUND: i32 = 100;

// Visuals
const BIG_SIZE: f32 = 200.0;
const SMALL_SIZE: f32 = 70.0;

// Has to be > 0
const SMALL_MASS: f32 = 1.0;

// Has to be < 0
const BIG_VEL: f32 = -0.1;

#[derive(Component)]
struct CollisionText {}

struct WinSize {
    w: f32,
    // h: f32 | No need for height since we're working on the x axis!
}

#[derive(Component)]
struct Physics {
    vel: f32,
    mass: f32,
}
#[derive(Component)]
struct SmallBox {}

#[derive(Component)]
struct BigBox {}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            title: "Pi Calculator".to_string(),
            width: 800.0,
            height: 600.0,
            resizable: false,
            ..Default::default()
        })
        .add_event::<i32>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup_system)
        .add_system(move_big_box_system)
        .add_system(collision_detection_system)
        .add_system(wall_collision_detection_system)
        .add_system(collision_counter_system)
        .run()
}

fn startup_system(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    // Add 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());

    //Window Size
    let window = windows.get_primary().unwrap();
    let (win_width, win_height) = (window.width(), window.height());

    // Create WinSize Resource
    let win_size = WinSize {
        w: win_width,
        // h: win_height,
    };

    // Rectangle shapes from bevy_prototype_lyon
    let small_box = shapes::Rectangle {
        extents: Vec2::new(SMALL_SIZE, SMALL_SIZE),
        origin: RectangleOrigin::BottomRight,
    };
    let big_box = shapes::Rectangle {
        extents: Vec2::new(BIG_SIZE, BIG_SIZE),
        origin: RectangleOrigin::BottomLeft,
    };

    // Spawn the small box situated at the left side of the screen
    commands.spawn_bundle(GeometryBuilder::build_as(
        &small_box,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 2.0),
        },
        Transform {
            translation: Vec3::new(-win_width * 0.25, -win_height * 0.5, 0.0),
            ..Default::default()
        }
    )).insert(Physics {
        vel: 0.0,
        mass: SMALL_MASS
    }).insert(SmallBox {});
    
    // Calculate the mass of the big box
    // *Note it's mass has to be equal to: SMALL_MASS * 100^n, 
    // where 'n' is the number of digits after the decimal point.
    let big_mass: f32 = SMALL_MASS * HUND.pow(DIGITS) as f32; 

    // Spawn the big box situated at the right side of the screen
    commands.spawn_bundle(GeometryBuilder::build_as(
        &big_box,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 2.0),
        },
        Transform {
            translation: Vec3::new(win_width * 0.0, -win_height * 0.5, 2.0),
            ..Default::default()
        }
    )).insert(Physics {
        vel: BIG_VEL,
        mass: big_mass
    }).insert(BigBox {});

    let font: Handle<Font> = asset_server.load("static/Comfortaa-Bold.ttf");
    let text_alignment = TextAlignment::TOP_RIGHT;

    // Counter text
    commands.spawn_bundle(
            TextBundle::from_section(
                "0",
                TextStyle {
                    font,
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ) 
            .with_text_alignment(text_alignment)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(500.0),
                    right: Val::Px(20.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(CollisionText {});

    
    // Insert the window resource 
    commands.insert_resource(win_size);
}

fn move_big_box_system(
    mut query: Query<(&mut Transform, &Physics)>,
) {
    for (mut transform, physics) in query.iter_mut() {
       transform.translation.x += physics.vel; 
    }
}

fn collision_detection_system(
    mut big_box_query: Query<(&mut Transform, &mut Physics), With<BigBox>>,
    mut small_box_query: Query<(&mut Transform, &mut Physics), Without<BigBox>>,
    mut collisions_event_writer: EventWriter<i32>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (transform1, mut physics1) in big_box_query.iter_mut() {
        for (transform2, mut physics2) in small_box_query.iter_mut() {
            if transform1.translation.x <= transform2.translation.x {

                collisions_event_writer.send(1);

                // The following line is not performance efficient!!
                // it can be commented out if needed
                audio.play(asset_server.load("clack4.ogg"));

                let vel_after_collision = |one: &Mut<'_, Physics>, other: &Mut<'_, Physics>| -> f32 {
                    let sum_m = one.mass + other.mass;
                    let mut new_v = (one.mass - other.mass)/sum_m * one.vel;
                    new_v += (2.0 * other.mass / sum_m) * other.vel;
                    new_v
                };

                let v1 = vel_after_collision(&physics1, &physics2);
                let v2 = vel_after_collision(&physics2, &physics1);

                physics1.vel = v1;
                physics2.vel = v2;
            }
        }
    }
}

fn wall_collision_detection_system(
    window: Res<WinSize>,
    mut query: Query<(&Transform, &mut Physics), With<SmallBox>>,
    mut collisions_event_writer: EventWriter<i32>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (&transform, mut physics) in query.iter_mut() {
        if transform.translation.x - SMALL_SIZE <= -window.w * 0.5 {
            physics.vel = physics.vel.abs();
            collisions_event_writer.send(1);

            // The following line is not performance efficient!
            // it can be commented out if needed
            audio.play(asset_server.load("clack4.ogg"));
        }
    }
}

fn collision_counter_system(
    mut text_query: Query<&mut Text, With<CollisionText>>,
    mut collisions_event_reader: EventReader<i32>
) {
    for mut text in text_query.iter_mut() {
        for event in collisions_event_reader.iter() {

            let old_counter = &text.sections[0].value.parse::<i32>().unwrap();
            let new_counter = old_counter + event;

            text.sections[0].value = new_counter.to_string();
        }
    }
}

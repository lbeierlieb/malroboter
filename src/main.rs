//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

#[derive(Component)]
struct Movable {
    start_pos : Vec3,
    end_pos : Vec3,
    t : f32,
    dt : f32
}

impl Movable {
    fn update(&mut self) {
        self.t += self.dt;

        if (self.t > 1.0 && self.dt > 0.0)
        || (self.t < 0.0 && self.dt < 0.0){
            self.dt *= -1.0;
        }
    }

    fn current(&self) -> Vec3 {
        self.start_pos + self.t * (self.end_pos - self.start_pos)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(moving)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cubes
    let childblock = commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 1.0, 0.5))),
        material: materials.add(Color::rgb(0.6, 0.7, 0.8).into()),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..default()
    }, Movable { start_pos: Vec3::new(-0.25, 1.5, 0.0), end_pos: Vec3::new(0.25, 1.5, 0.0), t: 0.0, dt: 0.001 })).id();
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 2.0, 1.0))),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }, Movable { start_pos: Vec3::new(0.0, 1.0, 0.0), end_pos: Vec3::new(1.0, 1.0, 2.0), t: 0.0, dt: 0.0007 }))
    .add_child(childblock);
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-6.0, 3.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn moving (
    keyboard: Res<Input<KeyCode>>,
    mut movables: Query<(&mut Transform,&mut Movable)>,
) {
    if keyboard.just_pressed(KeyCode::Space) { return; }

    for m in &mut movables {
        let (mut player_position, mut movable) = m;
        movable.update();
        player_position.translation = movable.current();
    }
}


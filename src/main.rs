use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, plugin::RapierPhysicsPlugin};
use std::f32::consts::PI;

#[derive(Component)]
struct Ball {
    size: BallSize,
}

fn spawn_explosion(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
) {
    // Spawn a bunch of particles in different directions
    for i in 0..20 {  // 20 particles
        let angle = (i as f32 / 20.0) * PI * 2.0;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * 200.0; // Speed of particles
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(10.0, 10.0)),  // Small particles
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            ExplosionParticle {
                lifetime: Timer::from_seconds(0.5, TimerMode::Once),
            },
            // Add physics components
            RigidBody::Dynamic,
            Velocity::linear(velocity),  // Initial velocity from explosion
            Collider::ball(5.0),        // Circular collider (radius = half the sprite size)
            Restitution::coefficient(0.7), // Make them bouncy
            Friction::coefficient(0.1),    // Low friction to make them slide
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
        ));
    }
}

fn update_explosion_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut Sprite, &mut ExplosionParticle)>,
) {
    for (entity, mut transform, mut sprite, mut particle) in &mut particles {
        particle.lifetime.tick(time.delta());
        
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            // Fade out by adjusting alpha and scale
            
            let life_percent = particle.lifetime.elapsed_secs() / particle.lifetime.duration().as_secs_f32();
            sprite.color.set_alpha(life_percent);
            transform.scale = Vec3::splat(life_percent);
        }
    }
}

#[derive(Component)]
struct BallPreview;

#[derive(Copy, Clone, PartialEq)]
enum BallSize {
    One,
    Two,
    Three,
}
static MAX_BALL_SIZE : BallSize = BallSize::Three;


#[derive(Component)]
struct CollisionEffect {
    timer: Timer,
    initial_scale: Vec3,
}

#[derive(Component)]
struct ExplosionParticle {
    lifetime: Timer,
}


#[derive(Component)]
struct BackgroundStrip {
    hue: f32,         // Current hue
    speed: f32,       // How fast this strip changes
    width: f32,       // Width of the strip
}

fn setup_background(mut commands: Commands) {
    // Create several vertical strips
    let num_strips = 10;
    let strip_width = 500.0 / num_strips as f32;
    
    for i in 0..num_strips {
        let x_pos = -250.0 + (i as f32 * strip_width) + (strip_width / 2.0);
        
        commands.spawn((
            BackgroundStrip {
                hue: (i as f32 / num_strips as f32) * 360.0, // Starting hue in degrees
                speed: 0.3, // Reduced speed since we're using degrees now
                width: strip_width,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::hsl(0.0, 1.0, 0.5),
                    custom_size: Some(Vec2::new(strip_width, 600.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, 0.0, -1.0),
                ..default()
            },
        ));
    }
}
fn animate_background(
    time: Res<Time>,
    mut strips: Query<(&mut Sprite, &mut BackgroundStrip)>,
) {
    for (mut sprite, mut strip) in &mut strips {
        // Update the hue (now using degrees)
        strip.hue += strip.speed * 360.0 * time.delta_seconds();
        if strip.hue > 360.0 {
            strip.hue -= 360.0;
        }
        
        // Update the color (hue in degrees)
        sprite.color = Color::hsl(strip.hue, 1.0, 0.5);
    }
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (500.0, 600.0).into(),
                title: "Ball Drop".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -500.0),
            ..RapierConfiguration::new(1.0)
        })
        .add_systems(Startup, (setup, setup_preview, setup_background))  // Add setup_preview
        .add_systems(Update, (
            spawn_ball,
            handle_ball_collisions,
            update_preview,
            animate_background,
            handle_collision_effects,
            update_explosion_particles,
        ))
        .run();
}


// New system to create the preview ball
fn setup_preview(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_size = 40.0; // Size for BallSize::One
    
    commands.spawn((
        BallPreview,
        SpriteBundle {
            texture: asset_server.load("happy_sprite.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(ball_size, ball_size)),
                color: Color::srgba(1.0, 1.0, 1.0, 0.5), // 50% transparent
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0), // Slightly in front
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
}

// New system to update the preview position
fn update_preview(
    mut preview_query: Query<(&mut Transform, &mut Visibility), With<BallPreview>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();
    
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Ok((mut transform, mut visibility)) = preview_query.get_single_mut() {
            transform.translation.x = world_position.x;
            transform.translation.y = 300.0;
            *visibility = Visibility::Visible;
        }
    } else {
        if let Ok((_, mut visibility)) = preview_query.get_single_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

fn spawn_ball_at(
    commands: &mut Commands,
    asset_server: &AssetServer,
    size: BallSize,
    position: Vec3,
) -> Entity {
    let ball_size = match size {
        BallSize::One => 40.0,
        BallSize::Two => 60.0,
        BallSize::Three => 80.0,
    };

    commands.spawn((
        Ball { size },
        SpriteBundle {
            texture: asset_server.load("happy_sprite.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(ball_size, ball_size)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(ball_size / 2.0),
        Restitution::coefficient(0.7),
        Friction::coefficient(0.0),
        // Add initial collision effect
        CollisionEffect {
            timer: Timer::from_seconds(0.3, TimerMode::Once),
            initial_scale: Vec3::ONE,
        }
    )).id()
}


// Add new system to handle the effects
fn handle_collision_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut CollisionEffect)>,
) {
    for (entity, mut transform, mut effect) in &mut query {
        effect.timer.tick(time.delta());
        
        if effect.timer.finished() {
            // Remove the effect component when done
            commands.entity(entity).remove::<CollisionEffect>();
            // Reset scale
            transform.scale = effect.initial_scale;
        } else {
            // Calculate pulse effect
            let percent = effect.timer.elapsed().as_secs_f32() /  effect.timer.duration().as_secs_f32();
            let pulse = 1.0 + (1.0 - percent) * 0.3; // 30% size increase that fades out
            transform.scale = effect.initial_scale * pulse;
        }
    }
}


fn setup(mut commands: Commands) {
    // Add 2D camera
    commands.spawn(Camera2dBundle::default());

    // Add ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(500.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(250.0, 10.0),
    ));

     // Left wall
     commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(20.0, 600.0)),
                ..default()
            },
            transform: Transform::from_xyz(-250.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.0, 300.0),
    ));

    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(20.0, 600.0)),
                ..default()
            },
            transform: Transform::from_xyz(250.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.0, 300.0),
    ));
}
fn spawn_ball(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();
        
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            spawn_ball_at(
                &mut commands,
                &asset_server,
                BallSize::One,
                Vec3::new(world_position.x, 300.0, 0.0)
            );
        }
    }
}

fn handle_ball_collisions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    rapier_context: Res<RapierContext>,
    query: Query<(Entity, &Ball, &Transform)>,
) {
    for pair in rapier_context.contact_pairs() {
        let entity1 = pair.collider1();
        let entity2 = pair.collider2();

        if let (Ok((e1, ball1, transform1)), Ok((e2, ball2, transform2))) = 
            (query.get(entity1), query.get(entity2)) 
        {
            if ball1.size == ball2.size {
                if ball1.size == MAX_BALL_SIZE {
                    let position = (transform1.translation + transform2.translation) / 2.0;
                    spawn_explosion(&mut commands, position, Color::rgba(0.5, 0.0, 0.0, 5.0));
                    commands.entity(e1).despawn();
                    commands.entity(e2).despawn();
                    continue;
                }

                let midpoint = (transform1.translation + transform2.translation) / 2.0;
                
                let new_size = match ball1.size {
                    BallSize::One => BallSize::Two,
                    BallSize::Two => BallSize::Three,
                    BallSize::Three => BallSize::Three,
                };
                
                commands.entity(e1).despawn();
                commands.entity(e2).despawn();
                
                let new_ball = spawn_ball_at(&mut commands, &asset_server, new_size, midpoint);

                // Add a glow effect to the new ball
                commands.entity(new_ball).insert(CollisionEffect {
                    timer: Timer::from_seconds(0.3, TimerMode::Once),
                    initial_scale: Vec3::ONE,
                });

            }
        }
    }
}

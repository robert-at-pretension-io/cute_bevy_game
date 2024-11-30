use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::time::Duration;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource)]
struct DangerZone {
    warning_timer: Timer,
    height: f32,
    is_warning: bool,
    flash_timer: Timer,
    show_warning: bool,
}

impl Default for DangerZone {
    fn default() -> Self {
        Self {
            warning_timer: Timer::from_seconds(3.0, TimerMode::Once),
            height: 200.0,
            is_warning: false,
            flash_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            show_warning: false,
        }
    }
}

#[derive(Component)]
struct DangerZoneWarning;

#[derive(Component)]
struct GameOverText;
use bevy_rapier2d::{na::ComplexField, plugin::RapierPhysicsPlugin, prelude::*};
use std::f32::consts::PI;

use rand::Rng;

/// A utility struct for generating random colors
pub struct ColorGenerator;

impl ColorGenerator {
    /// Generate a random SRGBA color
    pub fn random_srgba() -> Srgba {
        let mut rng = rand::thread_rng();
        Srgba::new(
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            1.0, // Full opacity by default
        )
    }

    /// Generate a random HSLA color
    pub fn random_hsla() -> Hsla {
        let mut rng = rand::thread_rng();
        Hsla::new(
            rng.gen_range(0.0..360.0), // Hue: 0-360 degrees
            rng.gen_range(0.0..=1.0),   // Saturation: 0-1
            rng.gen_range(0.0..=1.0),   // Lightness: 0-1
            1.0,                        // Alpha: full opacity
        )
    }

    /// Generate a random LinearRGBA color
    pub fn random_linear_rgba() -> LinearRgba {
        // Generate SRGBA first and convert to linear
        let srgb = Self::random_srgba();
        LinearRgba::from(srgb)
    }

    /// Generate a random vibrant color in SRGBA
    pub fn random_vibrant_srgba() -> Srgba {
        let mut rng = rand::thread_rng();
        let hsla = Hsla::new(
            rng.gen_range(0.0..360.0), // Random hue
            rng.gen_range(0.8..=1.0),   // High saturation
            rng.gen_range(0.4..=0.6),   // Medium lightness
            1.0,                        // Full opacity
        );
        Srgba::from(hsla)
    }

    /// Generate a random pastel color in SRGBA
    pub fn random_pastel_srgba() -> Srgba {
        let mut rng = rand::thread_rng();
        let hsla = Hsla::new(
            rng.gen_range(0.0..360.0), // Random hue
            rng.gen_range(0.3..=0.5),   // Lower saturation
            rng.gen_range(0.8..=0.9),   // High lightness
            1.0,                        // Full opacity
        );
        Srgba::from(hsla)
    }
}

#[derive(Component)]
struct Ball {
    size: BallSize,
    glow_phase: f32,
    color_phase: f32,
    pulse_phase: f32,
}

#[derive(Resource)]
struct VisualEffects {
    glow_speed: f32,
    color_speed: f32,
    pulse_speed: f32,
    glow_intensity: f32,
    pulse_magnitude: f32,
}

impl Default for VisualEffects {
    fn default() -> Self {
        Self {
            glow_speed: 2.0,
            color_speed: 0.5,
            pulse_speed: 1.5,
            glow_intensity: 0.3,
            pulse_magnitude: 0.1,
        }
    }
}

fn spawn_explosion(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Further reduced particles and made count more consistent
    let num_particles = rng.gen_range(20..40);
    
    for _ in 0..num_particles {
        let angle = rng.gen::<f32>() * PI * 2.0;
        let speed = rng.gen_range(50.0..600.0); // Wider speed range for more variety
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
        
        // More varied sizes with exponential distribution for visual interest
        let size = rng.gen_range(1.0..5.0).powf(2.0) + 1.0;
        let new_color = color.to_srgba();
        
        // Vary the color components directly
        let varied_color = Color::srgba(
            new_color.red * rng.gen_range(0.8..1.2),
            new_color.green * rng.gen_range(0.8..1.2),
            new_color.blue * rng.gen_range(0.8..1.2),
            new_color.alpha
        );
        
        // Shorter lifetimes for better performance
        let lifetime = rng.gen_range(0.2..0.6);
        
        // Use only ball colliders for better performance
        let collider = Collider::ball(size / 2.0);
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: varied_color,
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            ExplosionParticle {
                lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
            },
            RigidBody::Dynamic,
            Velocity::linear(velocity),
            collider,
            Restitution::coefficient(0.5), // Fixed restitution
            Friction::coefficient(0.2),    // Fixed friction
            Damping {
                linear_damping: 0.8,      // Fixed damping values
                angular_damping: 0.8,
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

// Base size for scaling all balls
const BASE_BALL_SIZE: f32 = 45.0;

#[derive(Copy, Clone, PartialEq)]
enum BallSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

impl BallSize {
    fn size(&self) -> f32 {
        let ratio = match self {
            BallSize::Tiny => 1.0,
            BallSize::Small => 1.75,
            BallSize::Medium => 2.5,
            BallSize::Large => 3.5,
            BallSize::Huge => 4.5,
        };
        BASE_BALL_SIZE * ratio
    }

    fn sprite_path(&self) -> &'static str {
        match self {
            BallSize::Tiny => "sad_sprite.png",
            BallSize::Small => "angry_sprite.png", 
            BallSize::Medium => "surprise_sprite.png",
            BallSize::Large => "happy_sprite.png",
            BallSize::Huge => "love_sprite.png",
        }
    }
    
    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..5) {
            0 => BallSize::Tiny,
            1 => BallSize::Small,
            2 => BallSize::Medium,
            3 => BallSize::Large,
            _ => BallSize::Huge,
        }
    }
}


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



#[derive(Resource)]
struct GameSounds {
    collision: Handle<AudioSource>,
    pop: Handle<AudioSource>,
    warning: Handle<AudioSource>,
    game_over: Handle<AudioSource>,
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
        .insert_state::<GameState>(GameState::Playing)
        .insert_resource(DangerZone::default())
        .insert_resource(VisualEffects::default())
        .add_systems(Startup, (
            setup,
            setup_preview,
            setup_background,
            setup_audio,
            setup_danger_zone,
        ))
        .add_systems(Update, (
            spawn_ball,
            handle_ball_collisions,
            update_preview,
            animate_background,
            handle_collision_effects,
            update_explosion_particles,
            update_ball_effects,
            check_danger_zone,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(Update, handle_game_over.run_if(in_state(GameState::GameOver)))
        .run();
}


// New system to create the preview ball
fn setup_preview(mut commands: Commands, asset_server: Res<AssetServer>) {
    let initial_size = BallSize::Medium;
    let ball_size = initial_size.size();
    
    commands.spawn((
        BallPreview,
        SpriteBundle {
            texture: asset_server.load(initial_size.sprite_path()),
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
    let ball_size = size.size();

    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Random initial velocity
    let velocity = Vec2::new(
        rng.gen_range(-5.0..5.0),  // Random x velocity
        rng.gen_range(-5.0..5.0)     // Random y velocity
    );

    // Random initial angular velocity (rotation)
    let angular_velocity = rng.gen_range(-5.0..5.0);

    commands.spawn((
        Ball { 
            size,
            glow_phase: rng.gen_range(0.0..std::f32::consts::TAU),
            color_phase: rng.gen_range(0.0..std::f32::consts::TAU),
            pulse_phase: rng.gen_range(0.0..std::f32::consts::TAU),
        },
        SpriteBundle {
            texture: asset_server.load(size.sprite_path()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(ball_size, ball_size)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        RigidBody::Dynamic,
        Velocity {
            linvel: velocity,
            angvel: angular_velocity,
        },
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
                BallSize::random(),
                Vec3::new(world_position.x, 300.0, 0.0)
            );
        }
    }
}

fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let collision = asset_server.load("whoop_squish.ogg");
    let pop = asset_server.load("whoop_squish.ogg");
    let warning = asset_server.load("whoop_squish.ogg");
    let game_over = asset_server.load("whoop_squish.ogg");
    
    commands.insert_resource(GameSounds {
        collision,
        pop,
        warning,
        game_over,
    });
}

fn setup_danger_zone(mut commands: Commands) {
    // Red warning zone at the top
    commands.spawn((
        DangerZoneWarning,
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, 0.2),
                custom_size: Some(Vec2::new(500.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 250.0, 0.0),
            ..default()
        },
    ));
}

fn check_danger_zone(
    time: Res<Time>,
    mut danger_zone: ResMut<DangerZone>,
    ball_query: Query<&Transform, With<Ball>>,
    mut warning_query: Query<&mut Sprite, With<DangerZoneWarning>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_sounds: Res<GameSounds>,
    mut commands: Commands,
) {
    let balls_in_danger = ball_query
        .iter()
        .any(|transform| transform.translation.y > danger_zone.height);

    danger_zone.flash_timer.tick(time.delta());
    if danger_zone.flash_timer.just_finished() {
        danger_zone.show_warning = !danger_zone.show_warning;
    }

    if balls_in_danger {
        if !danger_zone.is_warning {
            danger_zone.is_warning = true;
            danger_zone.warning_timer.reset();
            // Play warning sound
            commands.spawn(AudioBundle {
                source: game_sounds.warning.clone(),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            });
        }
        danger_zone.warning_timer.tick(time.delta());

        // Update warning zone visibility
        if let Ok(mut sprite) = warning_query.get_single_mut() {
            sprite.color.set_alpha(if danger_zone.show_warning { 0.4 } else { 0.1 });
        }

        if danger_zone.warning_timer.finished() {
            next_state.set(GameState::GameOver);
            commands.spawn(AudioBundle {
                source: game_sounds.game_over.clone(),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            });
        }
    } else {
        danger_zone.is_warning = false;
        danger_zone.warning_timer.reset();
        
        // Reset warning zone visibility
        if let Ok(mut sprite) = warning_query.get_single_mut() {
            sprite.color.set_alpha(0.2);
        }
    }
}

fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        GameOverText,
        TextBundle::from_section(
            "Game Over!\nPress SPACE to restart",
            TextStyle {
                font_size: 50.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(250.0),
            top: Val::Px(300.0),
            ..default()
        }),
    ));
}

fn update_ball_effects(
    time: Res<Time>,
    effects: Res<VisualEffects>,
    mut query: Query<(&mut Ball, &mut Transform, &mut Sprite)>,
) {
    for (mut ball, mut transform, mut sprite) in query.iter_mut() {
        // Update phases
        ball.glow_phase += effects.glow_speed * time.delta_seconds();
        ball.color_phase += effects.color_speed * time.delta_seconds();
        ball.pulse_phase += effects.pulse_speed * time.delta_seconds();

        // Glow effect (alpha oscillation)
        let glow = (1.0 + effects.glow_intensity * ball.glow_phase.sin()) * 0.8;
        sprite.color.set_alpha(glow);

        // Color cycling (subtle hue shift)
        let hue_shift = (ball.color_phase.sin() * 20.0).to_radians(); // 20 degree shift
        let mut color = sprite.color;

        color.set_hue(color.hue() + hue_shift);

        sprite.color = color;

        // Size pulsing
        let scale = 1.0 + effects.pulse_magnitude * ball.pulse_phase.sin();
        transform.scale = Vec3::splat(scale);

        // Gentle rotation
        transform.rotate_z(0.2 * time.delta_seconds());
    }
}

fn handle_game_over(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    balls: Query<Entity, With<Ball>>,
    game_over_text: Query<Entity, With<GameOverText>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Remove all balls
        for entity in balls.iter() {
            commands.entity(entity).despawn();
        }
        
        // Remove game over text
        for entity in game_over_text.iter() {
            commands.entity(entity).despawn();
        }
        
        // Reset to playing state
        next_state.set(GameState::Playing);
    }
}

fn handle_ball_collisions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    rapier_context: Res<RapierContext>, 
    query: Query<(Entity, &Ball, &Transform)>,
    game_sounds: Res<GameSounds>,
) {
    for pair in rapier_context.contact_pairs() {
        let entity1 = pair.collider1();
        let entity2 = pair.collider2();

        if let (Ok((e1, ball1, transform1)), Ok((e2, ball2, transform2))) = 
            (query.get(entity1), query.get(entity2)) 
        {
            // Couldn't get this working
            // if let Some(manifold) = pair.manifolds().next() {
            //     let relative_vel = manifold.relative_velocity();
            //     let normal = manifold.normal();
            //     let impact_vel = relative_vel.dot(normal).abs();
            //     println!("Impact velocity: {}", impact_vel);
                
            //     // Only play sound for significant impacts (not gentle touches or resting contacts)
            //     if impact_vel > 50.0 {

            //     }
            // }
        

            if ball1.size == ball2.size {
                if ball1.size == BallSize::Huge {
                    let position = (transform1.translation + transform2.translation) / 2.0;
                    spawn_explosion(&mut commands, position, Color::srgba(0.5, 0.0, 0.0, 1.0));
                    commands.spawn(AudioBundle {
                        source: game_sounds.pop.clone(),
                        settings: PlaybackSettings::DESPAWN,
                        ..default()
                    });
                    commands.entity(e1).despawn();
                    commands.entity(e2).despawn();
                    continue;
                }

                let midpoint = (transform1.translation + transform2.translation) / 2.0;
                
                let new_size = match ball1.size {
                    BallSize::Tiny => BallSize::Small,
                    BallSize::Small => BallSize::Medium,
                    BallSize::Medium => BallSize::Large,
                    BallSize::Large => BallSize::Huge,
                    BallSize::Huge => BallSize::Huge,
                };
                
                commands.entity(e1).despawn();
                commands.entity(e2).despawn();
                
                let new_ball = spawn_ball_at(&mut commands, &asset_server, new_size, midpoint);

                commands.spawn(AudioBundle {
                    source: game_sounds.collision.clone(),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });

                // Add a glow effect to the new ball
                commands.entity(new_ball).insert(CollisionEffect {
                    timer: Timer::from_seconds(0.3, TimerMode::Once),
                    initial_scale: Vec3::ONE,
                });

            }
        }
    }
}

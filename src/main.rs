use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
    Win,
    Settings,
}

#[derive(Resource)]
struct Score {
    current: u32,
    high_score: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            current: 0,
            high_score: 0,
        }
    }
}

#[derive(Resource)]
struct Settings {
    volume: f32,
    sound_enabled: bool,
    glow_intensity: f32,
    glow_speed: f32,
    pulse_magnitude: f32,
    pulse_speed: f32,
    color_speed: f32,
    background_animation_speed: f32,
    background_strip_count: i32,
    background_saturation: f32,
    background_brightness: f32,
    explosion_intensity: f32,
    screen_shake_intensity: f32,
    screen_shake_decay: f32,
    is_fullscreen: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            volume: 0.5,
            sound_enabled: true,
            glow_intensity: 0.02,
            glow_speed: 0.2,
            pulse_magnitude: 0.01,
            pulse_speed: 0.2,
            color_speed: 0.05,
            background_animation_speed: 0.5,
            background_strip_count: 10,
            background_saturation: 1.0,
            background_brightness: 0.5,
            explosion_intensity: 0.5,
            screen_shake_intensity: 0.5,
            screen_shake_decay: 3.0,
            is_fullscreen: false,
        }
    }
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
struct ScoreText;

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
    variant: BallVariant,
    glow_phase: f32,
    color_phase: f32,
    pulse_phase: f32,
}


fn spawn_explosion(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
    effects: &VisualEffects,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Scale particle count with explosion intensity
    let base_particles = (10.0 + settings.explosion_intensity * 30.0) as i32;
    let num_particles = rng.gen_range(base_particles..base_particles + 10);
    
    for _ in 0..num_particles {
        let angle = rng.gen::<f32>() * PI * 2.0;
        // Scale particle speed with explosion intensity
        let speed = rng.gen_range(50.0..200.0 + 800.0 * settings.explosion_intensity);
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
                velocity: velocity,
                rotation_speed: rng.gen_range(-3.0..3.0),
                initial_color: varied_color,
            },
            RigidBody::Dynamic,
            Velocity::linear(velocity),
            collider,
            Restitution::coefficient(0.5), // Fixed restitution
            Friction::coefficient(0.5),    // Increased friction
            Damping {
                linear_damping: 0.8,      // Fixed damping values
                angular_damping: 0.8,
            },
        ));
    }
}


fn update_screen_shake(
    time: Res<Time>,
    mut shake_state: ResMut<ScreenShakeState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    settings: Res<Settings>,
) {
    let mut camera_transform = camera_query.single_mut();
    
    // Clamp trauma between 0 and 1
    shake_state.trauma = shake_state.trauma.clamp(0.0, 1.0);
    
    // Decay trauma over time
    shake_state.trauma = (shake_state.trauma - settings.screen_shake_decay * time.delta_seconds())
        .max(0.0);
    
    // Calculate shake amount with quadratic falloff
    let shake_amount = shake_state.trauma * shake_state.trauma * settings.screen_shake_intensity;
    
    if shake_amount > 0.0 {
        let time = time.elapsed_seconds();
        
        // Scale the shake effect based on screen size (assuming 500x600 window)
        // Scale shake intensity based on visual effects settings
        let screen_scale = 8.0 * (0.5 + settings.glow_intensity * 2.0); // Scales from 4.0 to 12.0 based on effects
        
        camera_transform.translation.x = shake_amount * screen_scale * (
            (time * 15.0 + 0.0).sin() + 
            0.5 * (time * 27.0 + 1.3).sin() +
            0.25 * (time * 45.0 + 2.6).sin()
        );
        
        camera_transform.translation.y = shake_amount * screen_scale * (
            (time * 17.0 + 3.9).sin() + 
            0.5 * (time * 32.0 + 5.2).sin() +
            0.25 * (time * 52.0 + 6.5).sin()
        );
        
        camera_transform.rotation = Quat::from_rotation_z(
            shake_amount * 0.03 * (time * 25.0).sin()
        );
    } else {
        camera_transform.translation = Vec3::ZERO;
        camera_transform.rotation = Quat::IDENTITY;
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
            if commands.get_entity(entity).is_some() {
                commands.entity(entity).despawn();
            }
        } else {
            let life_percent = 1.0 - (particle.lifetime.elapsed_secs() / particle.lifetime.duration().as_secs_f32());
                
            // Update position based on velocity
            transform.translation += particle.velocity.extend(0.0) * time.delta_seconds();
                
            // Update rotation
            transform.rotate_z(particle.rotation_speed * time.delta_seconds());
                
            // Update color and scale with life
            sprite.color = particle.initial_color.with_alpha(life_percent);
            transform.scale = Vec3::splat(life_percent);
        }
    }
}


// Base size for scaling all balls
const BASE_BALL_SIZE: f32 = 45.0;

#[derive(Copy, Clone, PartialEq)]
enum BallVariant {
    // Tier 1
    Sad,
    Angry,
    Surprised,
    
    // Tier 2
    Embarrassed,
    Happy,
    Joyful,
    
    // Tier 3
    Spite,
    Love,
    Pride,
    
    // Tier 4
    Rage,
    
    // Victory
    Win,
}

impl BallVariant {
    fn size(&self) -> f32 {
        let ratio = match self {
            // Tier 1 (smallest)
            BallVariant::Sad => 0.8,
            BallVariant::Angry => 1.2,
            BallVariant::Surprised => 1.6,
            
            // Tier 2 (medium)
            BallVariant::Embarrassed => 2.2,
            BallVariant::Happy => 2.8,
            BallVariant::Joyful => 3.4,
            
            // Tier 3 (large)
            BallVariant::Spite => 4.0,
            BallVariant::Love => 4.5,
            BallVariant::Pride => 5.0,
            
            // Tier 4 (extra large)
            BallVariant::Rage => 7.0,
            
            // Victory size
            BallVariant::Win => 8.5,
        };
        BASE_BALL_SIZE * ratio
    }

    fn sprite_path(&self) -> &'static str {
        match self {
            BallVariant::Sad => "sad_sprite.png",
            BallVariant::Angry => "angry_sprite.png",
            BallVariant::Surprised => "surprise_sprite.png",
            BallVariant::Embarrassed => "embarassed_sprite.png",
            BallVariant::Happy => "happy_sprite.png",
            BallVariant::Joyful => "joyful_sprite.png",
            BallVariant::Spite => "spite_sprite.png",
            BallVariant::Love => "love_sprite.png",
            BallVariant::Pride => "pride_sprite.png",
            BallVariant::Rage => "rage_sprite.png",
            BallVariant::Win => "win_sprite.png",
        }
    }

    fn next_variant(&self) -> Option<Self> {
        match self {
            // Tier 1 combinations
            BallVariant::Sad => Some(BallVariant::Angry),
            BallVariant::Angry => Some(BallVariant::Surprised),
            BallVariant::Surprised => Some(BallVariant::Embarrassed),
            
            // Tier 2 combinations
            BallVariant::Embarrassed => Some(BallVariant::Happy),
            BallVariant::Happy => Some(BallVariant::Joyful),
            BallVariant::Joyful => Some(BallVariant::Spite),
            
            // Tier 3 combinations
            BallVariant::Spite => Some(BallVariant::Love),
            BallVariant::Love => Some(BallVariant::Pride),
            BallVariant::Pride => Some(BallVariant::Rage),
            
            // Tier 4 combinations
            BallVariant::Rage => Some(BallVariant::Win),
            
            // Win is the final form
            BallVariant::Win => None,
        }
    }

    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // List all variants with exponentially decreasing weights based on size
        let variants = [
            (BallVariant::Sad, 100.0),  // Most common
            (BallVariant::Angry, 80.0),
            (BallVariant::Surprised, 60.0),
            (BallVariant::Embarrassed, 25.0),
            (BallVariant::Happy, 10.0),
            (BallVariant::Joyful, 4.0),
            (BallVariant::Spite, 1.0),
            (BallVariant::Love, 0.4),
            (BallVariant::Pride, 0.1),
            (BallVariant::Rage, 0.01),  // Extremely rare
        ];
        
        // Calculate total weight
        let total_weight: f32 = variants.iter().map(|(_, weight)| weight).sum();
        
        // Generate random value
        let mut value = rng.gen::<f32>() * total_weight;
        
        // Select variant based on weights
        for (variant, weight) in variants.iter() {
            value -= weight;
            if value <= 0.0 {
                return *variant;
            }
        }
        
        // Fallback to smallest ball if something goes wrong
        BallVariant::Sad
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
    velocity: Vec2,
    rotation_speed: f32,
    initial_color: Color,
}


#[derive(Resource)]
struct ScreenShakeState {
    trauma: f32,
    decay: f32,
}

impl Default for ScreenShakeState {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            decay: 3.0, // Faster decay for snappier feel
        }
    }
}


#[derive(Component)]
struct BackgroundStrip {
    hue: f32,         // Current hue
    speed: f32,       // How fast this strip changes
    width: f32,       // Width of the strip
}

fn setup_background(mut commands: Commands, settings: Res<Settings>) {
    // Create several vertical strips based on settings
    let num_strips = settings.background_strip_count;
    let strip_width = 500.0 / num_strips as f32;
    
    for i in 0..num_strips {
        let x_pos = -250.0 + (i as f32 * strip_width) + (strip_width / 2.0);
        
        commands.spawn((
            BackgroundStrip {
                hue: (i as f32 / num_strips as f32) * 360.0, // Starting hue in degrees
                speed: 0.05, // Base speed that will be multiplied by effects.color_speed
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
    settings: Res<Settings>,
    settings: Res<Settings>,
) {
    for (mut sprite, mut strip) in &mut strips {
        // Update the hue based on visual effects settings
        strip.hue += strip.speed * effects.background_animation_speed * 360.0 * time.delta_seconds();
        while strip.hue > 360.0 {
            strip.hue -= 360.0;
        }
        
        // Update the color with settings-based saturation and brightness
        sprite.color = Color::hsl(strip.hue, settings.background_saturation, settings.background_brightness);
    }
}



#[derive(Resource)]
struct GameSounds {
    collision: Handle<AudioSource>,
    pop: Handle<AudioSource>,
    warning: Handle<AudioSource>,
    game_over: Handle<AudioSource>,
}

fn toggle_settings_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Settings),
            GameState::Settings => next_state.set(GameState::Playing),
            _ => {},
        }
    }
}

fn main() {
    
    App::new()
        .insert_resource(Settings::default())
        .insert_resource(Score::default())
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
        .insert_resource(ScreenShakeState::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -1200.0),
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
        ).run_if(not(in_state(GameState::Settings))))
        .add_systems(Update, (
            update_preview,
            animate_background,
            handle_collision_effects,
            update_explosion_particles,
            update_screen_shake,
            check_danger_zone,
        ).after(handle_ball_collisions).run_if(in_state(GameState::Playing)))
        .add_systems(Update, update_ball_effects.run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(OnEnter(GameState::Win), setup_win_screen)
        .add_systems(Update, handle_game_over.run_if(in_state(GameState::GameOver)))
        .add_systems(Update, handle_win_screen.run_if(in_state(GameState::Win)))
        .add_systems(Update, toggle_settings_menu)
        .add_systems(OnEnter(GameState::Settings), setup_settings_menu)
        .add_systems(OnExit(GameState::Settings), cleanup_settings_menu)
        .add_systems(Update, (
            settings_menu_interaction,
            update_button_colors,
            apply_settings_changes,
        ).run_if(in_state(GameState::Settings)))
        .run();

#[derive(Component)]
struct SettingsMenu;

#[derive(Component, PartialEq, Clone, Copy)]
enum SettingButton {
    SoundToggle,
    LowEffects,
    NormalEffects,
    HighEffects,
}


#[derive(Resource, PartialEq, Clone, Copy)]
struct SelectedEffectsSetting(SettingButton);

fn get_current_effects_level(settings: &Settings) -> SettingButton {
    match (settings.glow_intensity, settings.pulse_magnitude, settings.color_speed) {
        (0.02, 0.01, 0.05) => SettingButton::LowEffects,
        (0.05, 0.02, 0.1) => SettingButton::NormalEffects,
        (0.08, 0.03, 0.15) => SettingButton::HighEffects,
        _ => SettingButton::NormalEffects, // Default to normal if values don't match exactly
    }
}


fn setup_settings_menu(mut commands: Commands, settings: Res<Settings>) {
    // Initialize the selected effects setting based on current settings
    commands.insert_resource(SelectedEffectsSetting(get_current_effects_level(&settings)));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ..default()
            },
            SettingsMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Settings Menu\nPress ESC to return",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Sound Toggle Button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(if settings.sound_enabled {
                        Color::srgb(0.2, 0.8, 0.2)
                    } else {
                        Color::srgb(0.8, 0.2, 0.2)
                    }),
                    ..default()
                },
                SettingButton::SoundToggle,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    if settings.sound_enabled { "Sound: ON" } else { "Sound: OFF" },
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Visual Effects Header
            parent.spawn(TextBundle::from_section(
                "Visual Effects:",
                TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));


            // Preset Buttons Header
            parent.spawn(TextBundle::from_section(
                "Preset Effects Levels:",
                TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Preset Effects Button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(
                        if get_current_effects_level(&settings) == SettingButton::LowEffects {
                            Color::srgb(0.2, 0.8, 0.2) // Green for selected
                        } else {
                            Color::srgb(0.4, 0.4, 0.4) // Gray for unselected
                        }
                    ),
                    ..default()
                },
                SettingButton::LowEffects,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Low",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Normal Effects Button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(
                        if get_current_effects_level(&settings) == SettingButton::NormalEffects {
                            Color::srgb(0.2, 0.8, 0.2) // Green for selected
                        } else {
                            Color::srgb(0.4, 0.4, 0.4) // Gray for unselected
                        }
                    ),
                    ..default()
                },
                SettingButton::NormalEffects,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Normal",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // High Effects Button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(
                        if get_current_effects_level(&settings) == SettingButton::HighEffects {
                            Color::srgb(0.2, 0.8, 0.2) // Green for selected
                        } else {
                            Color::srgb(0.4, 0.4, 0.4) // Gray for unselected
                        }
                    ),
                    ..default()
                },
                SettingButton::HighEffects,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "High",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
}


fn settings_menu_interaction(
    mut commands: Commands,
    mut settings: ResMut<Settings>,
    mut interaction_query: Query<
        (&Interaction, Option<&SettingButton>, &mut BackgroundColor, &Children),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    _windows: Query<&Window>,
) {
    for (interaction, button, mut color, children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(button) = button {
                match button {
                    SettingButton::SoundToggle => {
                    settings.sound_enabled = !settings.sound_enabled;
                    *color = BackgroundColor(if settings.sound_enabled {
                        Color::srgb(0.2, 0.8, 0.2)
                    } else {
                        Color::srgb(0.8, 0.2, 0.2)
                    });
                    // Update button text
                    if let Some(child) = children.first() {
                        if let Ok(mut text) = text_query.get_mut(*child) {
                            text.sections[0].value = if settings.sound_enabled {
                                "Sound: ON".to_string()
                            } else {
                                "Sound: OFF".to_string()
                            };
                        }
                    }
                }
                    SettingButton::LowEffects => {
                        settings.glow_intensity = 0.01;   // Extremely subtle glow
                        settings.pulse_magnitude = 0.005; // Minimal pulse
                        settings.color_speed = 0.05;      // Very slow, almost static
                        settings.background_animation_speed = 0.1; // Slow background
                        settings.background_strip_count = 5;      // Fewer strips
                        settings.background_saturation = 0.5;     // Muted colors
                        settings.background_brightness = 0.3;     // Darker background
                        settings.explosion_intensity = 0.2;       // Small explosions
                        settings.screen_shake_intensity = 0.2;    // Minimal shake
                        settings.screen_shake_decay = 4.0;        // Fast decay
                        commands.insert_resource(SelectedEffectsSetting(*button));
                    }
                    SettingButton::NormalEffects => {
                        settings.glow_intensity = 0.05;   // Default moderate glow
                        settings.pulse_magnitude = 0.02;  // Default subtle pulse
                        settings.color_speed = 0.15;      // Default moderate speed
                        settings.background_animation_speed = 0.5; // Normal background
                        settings.background_strip_count = 10;     // Normal strips
                        settings.background_saturation = 1.0;     // Normal saturation
                        settings.background_brightness = 0.5;     // Normal brightness
                        settings.explosion_intensity = 0.5;       // Medium explosions
                        settings.screen_shake_intensity = 0.5;    // Medium shake
                        settings.screen_shake_decay = 3.0;        // Normal decay
                        commands.insert_resource(SelectedEffectsSetting(*button));
                    }
                    SettingButton::HighEffects => {
                        settings.glow_intensity = 0.02;    // Keep glow subtle
                        settings.pulse_magnitude = 0.008;  // Very subtle size changes
                        settings.color_speed = 2.0;        // Super fast color changes
                        settings.background_animation_speed = 2.0; // Very fast background
                        settings.background_strip_count = 20;     // Many strips
                        settings.background_saturation = 1.8;     // Very saturated
                        settings.background_brightness = 0.8;     // Brighter
                        settings.explosion_intensity = 4.0;       // MASSIVE explosions
                        settings.screen_shake_intensity = 5.0;    // EXTREME shake
                        settings.screen_shake_decay = 1.0;        // Very slow decay
                        commands.insert_resource(SelectedEffectsSetting(*button));
                    }
                }
            }
        }
    }
}

fn update_button_colors(
    selected: Res<SelectedEffectsSetting>,
    mut query: Query<(&SettingButton, &mut BackgroundColor)>,
) {
    for (button, mut color) in &mut query {
        if matches!(button, SettingButton::LowEffects | SettingButton::NormalEffects | SettingButton::HighEffects) {
            *color = BackgroundColor(
                if *button == selected.0 {
                    Color::srgb(0.2, 0.8, 0.2)
                } else {
                    Color::srgb(0.4, 0.4, 0.4)
                }
            );
        }
    }
}


fn cleanup_settings_menu(
    mut commands: Commands,
    query: Query<Entity, With<SettingsMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
}


// New system to create the preview ball
#[derive(Component)]
struct BallPreview {
    next_size: BallVariant,
}

fn setup_preview(mut commands: Commands, asset_server: Res<AssetServer>) {
    let next_size = BallVariant::random();
    let ball_size = next_size.size();
    
    commands.spawn((
        BallPreview { next_size },
        SpriteBundle {
            texture: asset_server.load(next_size.sprite_path()),
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
    mut preview_query: Query<(&mut Transform, &mut Visibility, &BallPreview)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();
    
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Ok((mut transform, mut visibility, preview)) = preview_query.get_single_mut() {
            transform.translation.x = world_position.x;
            // Position higher based on ball size to prevent clipping
            transform.translation.y = 300.0 - (preview.next_size.size() / 2.0) - 10.0;
            *visibility = Visibility::Visible;
        }
    } else {
        if let Ok((_, mut visibility, _)) = preview_query.get_single_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

fn spawn_ball_at(
    commands: &mut Commands,
    asset_server: &AssetServer,
    variant: BallVariant,
    position: Vec3,
) -> Entity {
    let ball_size = variant.size();

    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Very subtle initial velocity
    let velocity = Vec2::new(
        rng.gen_range(-1.0..1.0),  // Minimal x velocity
        rng.gen_range(-1.0..1.0)   // Minimal y velocity
    );

    // Very subtle initial rotation
    let angular_velocity = rng.gen_range(-0.2..0.2);

    commands.spawn((
        Ball { 
            variant,
            glow_phase: rng.gen_range(0.0..std::f32::consts::TAU),
            color_phase: rng.gen_range(0.0..std::f32::consts::TAU),
            pulse_phase: rng.gen_range(0.0..std::f32::consts::TAU),
        },
        SpriteBundle {
            texture: asset_server.load(variant.sprite_path()),
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
        Restitution::coefficient(0.3), // Less bouncy
        Friction::coefficient(0.8), // More friction to help settle faster
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


fn setup(mut commands: Commands, score: Res<Score>) {
    // Score display
    commands.spawn((
        ScoreText,
        TextBundle::from_section(
            format!("Score: {}\nHigh Score: {}", score.current, score.high_score),
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        }),
    ));
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
    mut preview_query: Query<(&mut BallPreview, &mut Handle<Image>, &mut Sprite)>,
    game_state: Res<State<GameState>>,
) {
    if mouse.just_pressed(MouseButton::Left) && *game_state.get() == GameState::Playing {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();
        
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Get the size from preview and spawn that ball
            if let Ok((mut preview, mut texture, mut sprite)) = preview_query.get_single_mut() {
                let ball_size = preview.next_size.size();
                let safe_margin = ball_size / 2.0 + 5.0; // Add 5 pixels extra margin
                
                // Clamp x position to prevent wall intersection
                let x_pos = world_position.x.clamp(-240.0 + safe_margin, 240.0 - safe_margin);
                
                spawn_ball_at(
                    &mut commands,
                    &asset_server,
                    preview.next_size,
                    Vec3::new(x_pos, 300.0, 0.0)
                );
                
                // Generate next preview
                preview.next_size = BallVariant::random();
                let ball_size = preview.next_size.size();
                
                // Update preview appearance
                *texture = asset_server.load(preview.next_size.sprite_path());
                sprite.custom_size = Some(Vec2::new(ball_size, ball_size));
            }
        }
    }
}

fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let collision = asset_server.load("whoop_squish.ogg");
    // let pop = asset_server.load("pop.wav");
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
    settings: Res<Settings>,
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
            if settings.sound_enabled {
                commands.spawn(AudioBundle {
                    source: game_sounds.warning.clone(),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
            }
        }
        danger_zone.warning_timer.tick(time.delta());

        // Update warning zone visibility
        if let Ok(mut sprite) = warning_query.get_single_mut() {
            sprite.color.set_alpha(if danger_zone.show_warning { 0.4 } else { 0.1 });
        }

        if danger_zone.warning_timer.finished() {
            next_state.set(GameState::GameOver);
            if settings.sound_enabled {
                commands.spawn(AudioBundle {
                    source: game_sounds.game_over.clone(),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
            }
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

fn setup_game_over(mut commands: Commands) {
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
            left: Val::Auto,
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
            margin: UiRect::all(Val::Auto),
            ..default()
        }),
    ));
}

fn update_ball_effects(
    time: Res<Time>,
    settings: Res<Settings>,
    mut query: Query<(&mut Ball, &mut Transform, &mut Sprite)>,
) {
    for (mut ball, mut transform, mut sprite) in query.iter_mut() {
        // Update phases
        ball.glow_phase += settings.glow_speed * time.delta_seconds();
        ball.color_phase += settings.color_speed * time.delta_seconds();
        ball.pulse_phase += settings.pulse_speed * time.delta_seconds();

        // Glow effect (alpha oscillation)
        let glow = (1.0 + settings.glow_intensity * ball.glow_phase.sin()) * 0.9;
        sprite.color.set_alpha(glow.clamp(0.3, 1.0)); // Prevent balls from becoming too transparent

        // Color cycling (enhanced hue shift)
        let hue_shift = (ball.color_phase.sin() * 45.0 * settings.color_speed).to_radians(); // Up to 45 degree shift
        let mut color = sprite.color;
        color.set_hue(color.hue() + hue_shift);
        // Also modify saturation slightly for more vibrant effects at high settings
        // color.set_saturation((color.saturation() + effects.glow_intensity * 0.2).clamp(0.5, 1.0));
        // sprite.color = color;

        // Size pulsing (with enhanced effect at high settings)
        let pulse_effect = settings.pulse_magnitude * (1.0 + settings.glow_intensity);
        let scale = 1.0 + pulse_effect * ball.pulse_phase.sin();
        transform.scale = Vec3::splat(scale);
    }
}

fn handle_game_over(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    balls: Query<Entity, With<Ball>>,
    game_over_text: Query<Entity, With<GameOverText>>,
    mut score: ResMut<Score>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Remove all balls
        for entity in balls.iter() {
            if commands.get_entity(entity).is_some() {
                commands.entity(entity).despawn();
            }
        }
        
        // Remove game over text
        for entity in game_over_text.iter() {
            commands.entity(entity).despawn();
        }
        
        // Reset score and state
        score.current = 0;
        if let Ok(mut text) = score_text_query.get_single_mut() {
            text.sections[0].value = format!("Score: {}\nHigh Score: {}", score.current, score.high_score);
        }
        next_state.set(GameState::Playing);
    }
}


fn handle_ball_collisions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    rapier_context: Res<RapierContext>, 
    query: Query<(Entity, &Ball, &Transform)>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
    mut score: ResMut<Score>,
    game_sounds: Res<GameSounds>,
    settings: Res<Settings>,
    mut next_state: ResMut<NextState<GameState>>,
    settings: Res<Settings>,
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

            if ball1.variant == ball2.variant {
                let position = (transform1.translation + transform2.translation) / 2.0;
                    
                if let Some(next_variant) = ball1.variant.next_variant() {
                    // Only despawn if the entities still exist
                    if let Some(mut entity1) = commands.get_entity(e1) {
                        entity1.despawn();
                    }
                    if let Some(mut entity2) = commands.get_entity(e2) {
                        entity2.despawn();
                    }

                    // Calculate score based on ball variant 
                    let score_value = match ball1.variant {
                        BallVariant::Sad => 10,
                        BallVariant::Angry => 20,
                        BallVariant::Surprised => 30,
                        BallVariant::Embarrassed => 50,
                        BallVariant::Happy => 80,
                        BallVariant::Joyful => 120,
                        BallVariant::Spite => 200,
                        BallVariant::Love => 300,
                        BallVariant::Pride => 500,
                        BallVariant::Rage => 1000,
                        BallVariant::Win => 5000,
                    };
                    
                    score.current += score_value;
                    score.high_score = score.high_score.max(score.current);
                    
                    // Update score display
                    if let Ok(mut text) = score_text_query.get_single_mut() {
                        text.sections[0].value = format!("Score: {}\nHigh Score: {}", score.current, score.high_score);
                    }

                    if next_variant == BallVariant::Win {
                        // Create the Ultimate ball
                        let new_ball = spawn_ball_at(&mut commands, &asset_server, next_variant, position);
                        commands.entity(new_ball).insert(CollisionEffect {
                            timer: Timer::from_seconds(0.3, TimerMode::Once),
                            initial_scale: Vec3::ONE,
                        });
                            
                        // Trigger win effects
                        // Add screen shake
                        commands.insert_resource(ScreenShakeState {
                            trauma: ball1.variant.size() / BASE_BALL_SIZE * 0.3,
                            decay: 2.0,
                        });
                        // Spawn enhanced explosion
                        let explosion_color = Color::srgba(1.0, 0.5, 0.0, 1.0);
                        spawn_explosion(&mut commands, position, explosion_color, &settings);
                    
                        if settings.sound_enabled {
                            commands.spawn(AudioBundle {
                                source: game_sounds.pop.clone(),
                                settings: PlaybackSettings::DESPAWN,
                                ..default()
                            });
                        }
                            
                        // Trigger win state
                        next_state.set(GameState::Win);
                    } else {
                        // Normal combination
                        let new_ball = spawn_ball_at(&mut commands, &asset_server, next_variant, position);

                        // Add screen shake effect
                        let trauma = ball1.variant.size() / BASE_BALL_SIZE * 0.5; // Reduced multiplier
                        // println!("Setting shake trauma: {:.3} for ball size: {}", trauma, ball1.variant.size());
                        commands.insert_resource(ScreenShakeState {
                            trauma,
                            decay: 1.5, // Slower decay
                        });

                        // Add explosion effect
                        let explosion_color = Color::srgba(1.0, 0.5, 0.0, 1.0);
                        spawn_explosion(&mut commands, position, explosion_color, &effects);
                        
                        commands.entity(new_ball).insert(CollisionEffect {
                            timer: Timer::from_seconds(0.3, TimerMode::Once),
                            initial_scale: Vec3::ONE,
                        });
                            
                        if settings.sound_enabled {
                            commands.spawn(AudioBundle {
                                source: game_sounds.collision.clone(),
                                settings: PlaybackSettings::DESPAWN,
                                ..default()
                            });
                        }

                        commands.entity(new_ball).insert(CollisionEffect {
                            timer: Timer::from_seconds(0.3, TimerMode::Once),
                            initial_scale: Vec3::ONE,
                        });

                    }
                }

            }
        }
    }
}


#[derive(Component)]
struct WinText;

fn setup_win_screen(mut commands: Commands) {
    commands.spawn((
        WinText,
        TextBundle::from_section(
            "You Won!\nPress SPACE to play again",
            TextStyle {
                font_size: 50.0,
                color: Color::srgb(1.0, 0.84, 0.0), // Gold color in RGB
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Auto,
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
            margin: UiRect::all(Val::Auto),
            ..default()
        }),
    ));
}

fn handle_win_screen(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    balls: Query<Entity, With<Ball>>,
    win_text: Query<Entity, With<WinText>>,
    mut score: ResMut<Score>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Remove all balls
        for entity in balls.iter() {
            commands.entity(entity).despawn();
        }
        
        // Remove win text
        for entity in win_text.iter() {
            commands.entity(entity).despawn();
        }
        
        // Reset score and state
        score.current = 0;
        if let Ok(mut text) = score_text_query.get_single_mut() {
            text.sections[0].value = format!("Score: {}\nHigh Score: {}", score.current, score.high_score);
        }
        next_state.set(GameState::Playing);
    }
}

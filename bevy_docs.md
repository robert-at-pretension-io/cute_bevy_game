 Docs.rs
 bevy-0.14.2 
 
 Go to latest version
 Platform 
 Feature flags
Rust
 
Find crate
logo
bevy
0.14.2
ButtonInput
Sections
Usage
Multiple systems
Performance
Window focus
Examples
Note
Methods
all_pressed
any_just_pressed
any_just_released
any_pressed
clear
clear_just_pressed
clear_just_released
get_just_pressed
get_just_released
get_pressed
just_pressed
just_released
press
pressed
release
release_all
reset
reset_all
Trait Implementations
Clone
Debug
Default
FromReflect
GetTypeRegistration
Reflect
Resource
Struct
TypePath
Typed
Auto Trait Implementations
Freeze
RefUnwindSafe
Send
Sync
Unpin
UnwindSafe
Blanket Implementations
Any
AsBindGroupShaderType<U>
Borrow<T>
BorrowMut<T>
CloneToUninit
ConditionalSend
Downcast
Downcast<T>
DowncastSync
Duplex<S>
DynamicTypePath
From<T>
FromSample<S>
FromWorld
GetField
GetPath
Instrument
Into<U>
IntoEither
IntoSample<T>
NoneValue
Pointable
ReadPrimitive<R>
Same
Settings
ToOwned
ToSample<U>
TryFrom<U>
TryInto<U>
TypeData
Upcast<T>
VZip<V>
WasmNotSend
WasmNotSendSync
WasmNotSync
WithSubscriber
In bevy::input
Modules
common_conditions
gamepad
gestures
keyboard
mouse
prelude
touch
Structs
Axis
ButtonInput
InputPlugin
InputSystem
Enums
ButtonState
Type ‘S’ or ‘/’ to search, ‘?’ for more options…
?
Settings
Struct bevy::input::ButtonInputCopy item path
source · [−]
pub struct ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{ /* private fields */ }
A “press-able” input of type T.

Usage
This type can be used as a resource to keep the current state of an input, by reacting to events from the input. For a given input value:

ButtonInput::pressed will return true between a press and a release event.
ButtonInput::just_pressed will return true for one frame after a press event.
ButtonInput::just_released will return true for one frame after a release event.
Multiple systems
In case multiple systems are checking for ButtonInput::just_pressed or ButtonInput::just_released but only one should react, for example when modifying a Resource, you should consider clearing the input state, either by:

Using ButtonInput::clear_just_pressed or ButtonInput::clear_just_released instead.
Calling ButtonInput::clear or ButtonInput::reset immediately after the state change.
Performance
For all operations, the following conventions are used:

n is the number of stored inputs.
m is the number of input arguments passed to the method.
*-suffix denotes an amortized cost.
~-suffix denotes an expected cost.
See Rust’s std::collections doc on performance for more details on the conventions used here.

ButtonInput operations	Computational complexity
ButtonInput::any_just_pressed	O(m)~
ButtonInput::any_just_released	O(m)~
ButtonInput::any_pressed	O(m)~
ButtonInput::get_just_pressed	O(n)
ButtonInput::get_just_released	O(n)
ButtonInput::get_pressed	O(n)
ButtonInput::just_pressed	O(1)~
ButtonInput::just_released	O(1)~
ButtonInput::pressed	O(1)~
ButtonInput::press	O(1)~*
ButtonInput::release	O(1)~*
ButtonInput::release_all	O(n)~*
ButtonInput::clear_just_pressed	O(1)~
ButtonInput::clear_just_released	O(1)~
ButtonInput::reset_all	O(n)
ButtonInput::clear	O(n)
Window focus
ButtonInput<KeyCode> is tied to window focus. For example, if the user holds a button while the window loses focus, ButtonInput::just_released will be triggered. Similarly if the window regains focus, ButtonInput::just_pressed will be triggered. Currently this happens even if the focus switches from one Bevy window to another (for example because a new window was just spawned).

ButtonInput<GamepadButton> is independent of window focus.

Examples
Reading and checking against the current set of pressed buttons:


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            print_gamepad.run_if(resource_changed::<ButtonInput<GamepadButton>>),
        )
        .add_systems(
            Update,
            print_mouse.run_if(resource_changed::<ButtonInput<MouseButton>>),
        )
        .add_systems(
            Update,
            print_keyboard.run_if(resource_changed::<ButtonInput<KeyCode>>),
        )
        .run();
}

fn print_gamepad(gamepad: Res<ButtonInput<GamepadButton>>) {
    println!("Gamepad: {:?}", gamepad.get_pressed().collect::<Vec<_>>());
}

fn print_mouse(mouse: Res<ButtonInput<MouseButton>>) {
    println!("Mouse: {:?}", mouse.get_pressed().collect::<Vec<_>>());
}

fn print_keyboard(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight])
        && keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
        && keyboard.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight])
        && keyboard.pressed(KeyCode::KeyL)
    {
        println!("On Windows this opens LinkedIn.");
    } else {
        println!("keyboard: {:?}", keyboard.get_pressed().collect::<Vec<_>>());
    }
}
Accepting input from multiple devices:


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            something_used.run_if(
                input_just_pressed(KeyCode::KeyE)
                    .or_else(input_just_pressed(GamepadButton::new(
                        Gamepad::new(0),
                        GamepadButtonType::West,
                    ))),
            ),
        )
        .run();
}

fn something_used() {
    println!("Generic use-ish button pressed.");
}
Note
When adding this resource for a new input type, you should:

Call the ButtonInput::press method for each press event.
Call the ButtonInput::release method for each release event.
Call the ButtonInput::clear method at each frame start, before processing events.
Note: Calling clear from a ResMut will trigger change detection. It may be preferable to use DetectChangesMut::bypass_change_detection to avoid causing the resource to always be marked as changed.

Implementations
source
impl<T> ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
source
pub fn press(&mut self, input: T)
Registers a press for the given input.

source
pub fn pressed(&self, input: T) -> bool
Returns true if the input has been pressed.

Examples found in repository?
examples/ecs/generic_system.rs (line 78)
fn transition_to_in_game_system(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}
More examples
source
pub fn any_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool
Returns true if any item in inputs has been pressed.

Examples found in repository?
examples/input/keyboard_modifiers.rs (line 14)
fn keyboard_input_system(input: Res<ButtonInput<KeyCode>>) {
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    if ctrl && shift && input.just_pressed(KeyCode::KeyA) {
        info!("Just pressed Ctrl + Shift + A!");
    }
}
More examples
source
pub fn all_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool
Returns true if all items in inputs have been pressed.

source
pub fn release(&mut self, input: T)
Registers a release for the given input.

source
pub fn release_all(&mut self)
Registers a release for all currently pressed inputs.

source
pub fn just_pressed(&self, input: T) -> bool
Returns true if the input has been pressed during the current frame.

Note: This function does not imply information regarding the current state of ButtonInput::pressed or ButtonInput::just_released.

Examples found in repository?
examples/app/logs.rs (line 36)
fn panic_on_p(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyP) {
        panic!("P pressed, panicking");
    }
}
More examples
source
pub fn any_just_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool
Returns true if any item in inputs has been pressed during the current frame.

source
pub fn clear_just_pressed(&mut self, input: T) -> bool
Clears the just_pressed state of the input and returns true if the input has just been pressed.

Future calls to ButtonInput::just_pressed for the given input will return false until a new press event occurs.

source
pub fn just_released(&self, input: T) -> bool
Returns true if the input has been released during the current frame.

Note: This function does not imply information regarding the current state of ButtonInput::pressed or ButtonInput::just_pressed.

Examples found in repository?
examples/input/keyboard_input.rs (line 21)
fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        info!("'A' just pressed");
    }
    if keyboard_input.just_released(KeyCode::KeyA) {
        info!("'A' just released");
    }
}
More examples
source
pub fn any_just_released(&self, inputs: impl IntoIterator<Item = T>) -> bool
Returns true if any item in inputs has just been released.

source
pub fn clear_just_released(&mut self, input: T) -> bool
Clears the just_released state of the input and returns true if the input has just been released.

Future calls to ButtonInput::just_released for the given input will return false until a new release event occurs.

source
pub fn reset(&mut self, input: T)
Clears the pressed, just_pressed and just_released data of the input.

source
pub fn reset_all(&mut self)
Clears the pressed, just_pressed, and just_released data for every input.

See also ButtonInput::clear for simulating elapsed time steps.

source
pub fn clear(&mut self)
Clears the just pressed and just released data for every input.

See also ButtonInput::reset_all for a full reset.

source
pub fn get_pressed(&self) -> impl ExactSizeIterator
An iterator visiting every pressed input in arbitrary order.

source
pub fn get_just_pressed(&self) -> impl ExactSizeIterator
An iterator visiting every just pressed input in arbitrary order.

Note: Returned elements do not imply information regarding the current state of ButtonInput::pressed or ButtonInput::just_released.

Examples found in repository?
examples/ecs/component_hooks.rs (line 106)
fn trigger_hooks(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    index: Res<MyComponentIndex>,
) {
    for (key, entity) in index.iter() {
        if !keys.pressed(*key) {
            commands.entity(*entity).remove::<MyComponent>();
        }
    }
    for key in keys.get_just_pressed() {
        commands.spawn(MyComponent(*key));
    }
}
source
pub fn get_just_released(&self) -> impl ExactSizeIterator
An iterator visiting every just released input in arbitrary order.

Note: Returned elements do not imply information regarding the current state of ButtonInput::pressed or ButtonInput::just_pressed.

Trait Implementations
source
impl<T> Clone for ButtonInput<T>
where
    T: Clone + Copy + Eq + Hash + Send + Sync + 'static,
source
fn clone(&self) -> ButtonInput<T>
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<T> Debug for ButtonInput<T>
where
    T: Debug + Copy + Eq + Hash + Send + Sync + 'static,
source
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
source
impl<T> Default for ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
source
fn default() -> ButtonInput<T>
Returns the “default value” for a type. Read more
source
impl<T> FromReflect for ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static + TypePath,
    ButtonInput<T>: Any + Send + Sync,
    HashSet<T>: FromReflect + TypePath + RegisterForReflection,
source
fn from_reflect(reflect: &(dyn Reflect + 'static)) -> Option<ButtonInput<T>>
Constructs a concrete instance of Self from a reflected value.
source
fn take_from_reflect(
    reflect: Box<dyn Reflect>,
) -> Result<Self, Box<dyn Reflect>>
Attempts to downcast the given value to Self using, constructing the value using from_reflect if that fails. Read more
source
impl<T> GetTypeRegistration for ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static + TypePath,
    ButtonInput<T>: Any + Send + Sync,
    HashSet<T>: FromReflect + TypePath + RegisterForReflection,
source
fn get_type_registration() -> TypeRegistration
Returns the default TypeRegistration for this type.
source
fn register_type_dependencies(registry: &mut TypeRegistry)
Registers other types needed by this type. Read more
source
impl<T> Reflect for ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static + TypePath,
    ButtonInput<T>: Any + Send + Sync,
    HashSet<T>: FromReflect + TypePath + RegisterForReflection,
source
fn get_represented_type_info(&self) -> Option<&'static TypeInfo>
Returns the TypeInfo of the type represented by this value. Read more
source
fn into_any(self: Box<ButtonInput<T>>) -> Box<dyn Any>
Returns the value as a Box<dyn Any>.
source
fn as_any(&self) -> &(dyn Any + 'static)
Returns the value as a &dyn Any.
source
fn as_any_mut(&mut self) -> &mut (dyn Any + 'static)
Returns the value as a &mut dyn Any.
source
fn into_reflect(self: Box<ButtonInput<T>>) -> Box<dyn Reflect>
Casts this type to a boxed reflected value.
source
fn as_reflect(&self) -> &(dyn Reflect + 'static)
Casts this type to a reflected value.
source
fn as_reflect_mut(&mut self) -> &mut (dyn Reflect + 'static)
Casts this type to a mutable reflected value.
source
fn clone_value(&self) -> Box<dyn Reflect>
Clones the value as a Reflect trait object. Read more
source
fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>>
Performs a type-checked assignment of a reflected value to this value. Read more
source
fn try_apply(
    &mut self,
    value: &(dyn Reflect + 'static),
) -> Result<(), ApplyError>
Tries to apply a reflected value to this value. Read more
source
fn reflect_kind(&self) -> ReflectKind
Returns a zero-sized enumeration of “kinds” of type. Read more
source
fn reflect_ref(&self) -> ReflectRef<'_>
Returns an immutable enumeration of “kinds” of type. Read more
source
fn reflect_mut(&mut self) -> ReflectMut<'_>
Returns a mutable enumeration of “kinds” of type. Read more
source
fn reflect_owned(self: Box<ButtonInput<T>>) -> ReflectOwned
Returns an owned enumeration of “kinds” of type. Read more
source
fn reflect_partial_eq(&self, value: &(dyn Reflect + 'static)) -> Option<bool>
Returns a “partial equality” comparison result. Read more
source
fn apply(&mut self, value: &(dyn Reflect + 'static))
Applies a reflected value to this value. Read more
source
fn reflect_hash(&self) -> Option<u64>
Returns a hash of the value (which includes the type). Read more
source
fn debug(&self, f: &mut Formatter<'_>) -> Result<(), Error>
Debug formatter for the value. Read more
source
fn serializable(&self) -> Option<Serializable<'_>>
Returns a serializable version of the value. Read more
source
fn is_dynamic(&self) -> bool
Indicates whether or not this type is a dynamic type. Read more
source
impl<T> Struct for ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static + TypePath,
    ButtonInput<T>: Any + Send + Sync,
    HashSet<T>: FromReflect + TypePath + RegisterForReflection,
source
fn field(&self, name: &str) -> Option<&(dyn Reflect + 'static)>
Returns a reference to the value of the field named name as a &dyn Reflect.
source
fn field_mut(&mut self, name: &str) -> Option<&mut (dyn Reflect + 'static)>
Returns a mutable reference to the value of the field named name as a &mut dyn Reflect.
source
fn field_at(&self, index: usize) -> Option<&(dyn Reflect + 'static)>
Returns a reference to the value of the field with index index as a &dyn Reflect.
source
fn field_at_mut(&mut self, index: usize) -> Option<&mut (dyn Reflect + 'static)>
Returns a mutable reference to the value of the field with index index as a &mut dyn Reflect.
source
fn name_at(&self, index: usize) -> Option<&str>
Returns the name of the field with index index.
source
fn field_len(&self) -> usize
Returns the number of fields in the struct.
source
fn iter_fields(&self) -> FieldIter<'_> ⓘ
Returns an iterator over the values of the reflectable fields for this struct.
source
fn clone_dynamic(&self) -> DynamicStruct
Clones the struct into a DynamicStruct.
source
impl<T> TypePath for ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static + TypePath,
    ButtonInput<T>: Any + Send + Sync,
source
fn type_path() -> &'static str
Returns the fully qualified path of the underlying type. Read more
source
fn short_type_path() -> &'static str
Returns a short, pretty-print enabled path to the type. Read more
source
fn type_ident() -> Option<&'static str>
Returns the name of the type, or None if it is anonymous. Read more
source
fn crate_name() -> Option<&'static str>
Returns the name of the crate the type is in, or None if it is anonymous. Read more
source
fn module_path() -> Option<&'static str>
Returns the path to the module the type is in, or None if it is anonymous. Read more
source
impl<T> Typed for ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static + TypePath,
    ButtonInput<T>: Any + Send + Sync,
    HashSet<T>: FromReflect + TypePath + RegisterForReflection,
source
fn type_info() -> &'static TypeInfo
Returns the compile-time info for the underlying type.
source
impl<T> Resource for ButtonInput<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
    ButtonInput<T>: Send + Sync + 'static,
Auto Trait Implementations
impl<T> Freeze for ButtonInput<T>
impl<T> RefUnwindSafe for ButtonInput<T>
where
    T: RefUnwindSafe,
impl<T> Send for ButtonInput<T>
impl<T> Sync for ButtonInput<T>
impl<T> Unpin for ButtonInput<T>
where
    T: Unpin,
impl<T> UnwindSafe for ButtonInput<T>
where
    T: UnwindSafe,
Blanket Implementations
source
impl<T> Any for T
where
    T: 'static + ?Sized,
source
impl<T, U> AsBindGroupShaderType<U> for T
where
    U: ShaderType,
    &'a T: for<'a> Into<U>,
source
impl<T> Borrow<T> for T
where
    T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
    T: ?Sized,
source
impl<T> CloneToUninit for T
where
    T: Clone,
source
impl<T> Downcast<T> for T
source
impl<T> Downcast for T
where
    T: Any,
source
impl<T> DowncastSync for T
where
    T: Any + Send + Sync,
source
impl<T> DynamicTypePath for T
where
    T: TypePath,
source
impl<T> From<T> for T
source
impl<S> FromSample<S> for S
source
impl<T> FromWorld for T
where
    T: Default,
source
impl<S> GetField for S
where
    S: Struct,
source
impl<T> GetPath for T
where
    T: Reflect + ?Sized,
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
    U: From<T>,
source
impl<T> IntoEither for T
source
impl<F, T> IntoSample<T> for F
where
    T: FromSample<F>,
source
impl<T> NoneValue for T
where
    T: Default,
source
impl<T> Pointable for T
source
impl<R, P> ReadPrimitive<R> for P
where
    R: Read + ReadEndian<P>,
    P: Default,
source
impl<T> Same for T
source
impl<T> ToOwned for T
where
    T: Clone,
source
impl<T, U> ToSample<U> for T
where
    U: FromSample<T>,
source
impl<T, U> TryFrom<U> for T
where
    U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
source
impl<T> TypeData for T
where
    T: 'static + Send + Sync + Clone,
source
impl<T> Upcast<T> for T
source
impl<V, T> VZip<V> for T
where
    V: MultiLane<T>,
source
impl<T> WithSubscriber for T
source
impl<T> ConditionalSend for T
where
    T: Send,
source
impl<S, T> Duplex<S> for T
where
    T: FromSample<S> + ToSample<S>,
source
impl<T> Settings for T
where
    T: 'static + Send + Sync,
source
impl<T> WasmNotSend for T
where
    T: Send,
source
impl<T> WasmNotSendSync for T
where
    T: WasmNotSend + WasmNotSync,
source
impl<T> WasmNotSync for T
where
    T: Sync,


Skip to main content
Rapier Logo
Rapier
Docs
Demos
Community
Blog
Donate ♥
Dimforge


Rapier Logo
Rapier
About Rapier
User Guides
Rust
Bevy Plugin
Getting started
Rigid-bodies
Colliders
Joints
Character controller
Scene queries
Advanced collision-detection
Common mistakes
JavaScript
API Documentation
User GuidesBevy PluginRigid-bodies
Rigid-bodies
The real-time simulation of rigid-bodies subjected to forces and contacts is the main feature of a physics engine for video-games, robotics, or animation. Rigid-bodies are typically used to simulate the dynamics of non-deformable solids as well as to integrate the trajectory of solids which velocities are controlled by the user (e.g. moving platforms). On the other hand, rigid-bodies are not enough to simulate, e.g., cars, ragdolls, or robotic systems, as those use-cases require adding restrictions on the relative motion between their parts using joints.

Note that rigid-bodies are only responsible for the dynamics and kinematics of the solid. Colliders can be attached to a rigid-body to specify its shape and enable collision-detection. A rigid-body without collider attached to it will not be affected by contacts (because there is no shape to compute contact against).

Creation and insertion
A rigid-body is created by adding the RigidBody component to an entity. Other components like Transform, Velocity, Ccd, etc. can be added for further customization of the rigid-body.

info
The following example shows several initialization of components to customize rigid-body being built. The input values are just random so using this example as-is will not lead to a useful result.

Example 2D
Example 3D
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

commands
    .spawn(RigidBody::Dynamic)
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
    .insert(Velocity {
        linvel: Vec2::new(1.0, 2.0),
        angvel: 0.2,
    })
    .insert(GravityScale(0.5))
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled());


info
Typically, the inertia and center of mass are automatically set to the inertia and center of mass resulting from the shapes of the colliders attached to the rigid-body. But they can also be set manually.

Rigid-body type
There are four types of rigid-bodies, identified by the RigidBody component:

RigidBodyType::Dynamic: Indicates that the body is affected by external forces and contacts.
RigidBodyType::Fixed: Indicates the body cannot move. It acts as if it has an infinite mass and will not be affected by any force. It will continue to collide with dynamic bodies but not with fixed nor with kinematic bodies. This is typically used for the ground or for temporarily freezing a body.
RigidBodyType::KinematicPositionBased: Indicates that the body position must not be altered by the physics engine. The user is free to set its next position and the body velocity will be deduced at each update accordingly to ensure a realistic behavior of dynamic bodies in contact with it. This is typically used for moving platforms, elevators, etc.
RigidBodyType::KinematicVelocityBased: Indicates that the body velocity must not be altered by the physics engine. The user is free to set its velocity and the next body position will be deduced at each update accordingly to ensure a realistic behavior of dynamic bodies in contact with it. This is typically used for moving platforms, elevators, etc.
Both position-based and velocity-based kinematic bodies are mostly the same. Choosing between both is mostly a matter of preference between position-based control and velocity-based control.

info
The whole point of kinematic bodies is to let the user have total control over their trajectory. This means that kinematic bodies will simply ignore any contact force and go through walls and the ground. In other words: if you tell the kinematic to go somewhere, it will go there, no questions asked.

Taking obstacles into account needs to be done manually either by using scene queries to detect nearby obstacles, or by using the built-in character controller.

Position
The position of a rigid-body represents its location (translation) in 2D or 3D world-space, as well as its orientation (rotation). stored in the standard Bevy Transform component.

The position of a rigid-body can be set when creating it. It can also be set after its creation as illustrated below.

warning
Directly changing the position of a rigid-body is equivalent to teleporting it: this is a not a physically realistic action! Teleporting a dynamic or kinematic bodies may result in odd behaviors especially if it teleports into a space occupied by other objects. For dynamic bodies, forces, impulses, or velocity modification should be preferred. For kinematic bodies, see the discussion after the examples below.

Example 2D
Example 3D
commands
    .spawn(RigidBody::Dynamic)
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))


/* Change the position inside of a system. */
fn modify_body_translation(mut positions: Query<&mut Transform, With<RigidBody>>) {
    for mut position in positions.iter_mut() {
        position.translation.y += 0.1;
    }
}


In order to move a dynamic rigid-body it is strongly discouraged to set its position directly as it may results in weird behaviors: it's as if the rigid-body teleports itself, which is a non-physical behavior. For dynamic bodies, it is recommended to either set its velocity or to apply forces or impulses.

For velocity-based kinematic bodies, it is recommended to set its velocity instead of setting its position directly. For position-based kinematic bodies, it is recommended to modify its Transform (changing its velocity won’t have any effect). This will let the physics engine compute the fictitious velocity of the kinematic body for more realistic intersections with other rigid-bodies.

Velocity
The velocity of a dynamic rigid-body controls how fast it is moving in time. The velocity is applied at the center-of-mass of the rigid-body, and is composed of two independent parts:

The linear velocity is specified as a vector representing the direction and magnitude of the movement.
In 3D, the angular velocity is given as a vector representing the rotation axis multiplied by the rotation angular speed in rad/s (axis-angle representation). In 2D, the angular velocity is given as a real representing the angular speed in rad/s.
info
The velocity is only relevant to dynamic rigid-bodies. It has no effect on fixed rigid-bodies, and the velocity of kinematic rigid-bodies are automatically computed at each timestep based on their next kinematic positions.

The velocity of a rigid-body is automatically updated by the physics pipeline after taking forces, contacts, and joints into account. It can be set when the rigid-body is created or after its creation:

Example 2D
Example 3D
/* Set the velocities when the rigid-body is created. */
commands.spawn(RigidBody::Dynamic).insert(Velocity {
    linvel: Vec2::new(0.0, 2.0),
    angvel: 0.4,
});

/* Set the velocities inside of a system. */
fn modify_body_velocity(mut velocities: Query<&mut Velocity>) {
    for mut vel in velocities.iter_mut() {
        vel.linvel = Vec2::new(0.0, 2.0);
        vel.angvel = 0.4;
    }
}


Alternatively, the velocity of a dynamic rigid-body can be altered indirectly by applying a force or an impulse.

Gravity
Gravity is such a common force that it is implemented as a special case (even if it could easily be implemented by the user using force application). The gravity is given by the field RapierConfiguration::gravity of the resource RapierConfiguration and can be modified at will. Note however that a change of gravity won't automatically wake-up the sleeping bodies so keep in mind that you may want to wake them up manually before a gravity change.

note
Because fixed and kinematic bodies are immune to forces, they are not affected by gravity.

info
A rigid-body with no mass will not be affected by gravity either. So if your rigid-body doesn't fall when you expected it to, make sure it has a mass set explicitly, or has at least one collider with non-zero density attached to it.

It is possible to change the way gravity affects a specific rigid-body by setting the rigid-body's gravity scale to a value other than 1.0. The magnitude of the gravity applied to this body will be multiplied by this scaling factor. Therefore, a gravity scale set to 0.0 will disable gravity for the rigid-body whereas a gravity scale set to 2.0 will make it twice as strong. A negative value will flip the direction of the gravity for this rigid-body.

This gravity scale factor can be set when the rigid-body is created or after its creation:

/* Set the gravity scale when the rigid-body is created. */
commands.spawn(RigidBody::Dynamic).insert(GravityScale(2.0));

/* Set the gravity scale inside of a system. */
fn modify_body_gravity_scale(mut grav_scale: Query<&mut GravityScale>) {
    for mut grav_scale in grav_scale.iter_mut() {
        grav_scale.0 = 2.0;
    }
}


Forces and impulses
In addition to gravity, it is possible to add custom forces (or torques) or apply impulses (or torque impulses) to dynamic rigid-bodies in order to make them move in specific ways. Forces affect the rigid-body's acceleration whereas impulses affect the rigid-body's velocity. They are both based on the familiar equations:

Forces: the acceleration change is equal to the force divided by the mass: 
Δ
a
=
m
−
1
f
Δa=m 
−1
 f
Impulses: the velocity change is equal to the impulse divided by the mass: 
Δ
v
=
m
−
1
i
Δv=m 
−1
 i
Forces can be added, and impulses can be applied, to a rigid-body when it is created or after its creation. Added forces are persistent across simulation steps, and can be cleared manually.

Example 2D
Example 3D
commands
    .spawn(RigidBody::Dynamic)
    .insert(ExternalForce {
        force: Vec2::new(1000.0, 2000.0),
        torque: 140.0,
    })
    .insert(ExternalImpulse {
        impulse: Vec2::new(100.0, 200.0),
        torque_impulse: 14.0,
    });

/* Apply forces and impulses inside of a system. */
fn apply_forces(
    mut ext_forces: Query<&mut ExternalForce>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
) {
    // Apply forces.
    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = Vec2::new(1000.0, 2000.0);
        ext_force.torque = 0.4;
    }

    // Apply impulses.
    for mut ext_impulse in ext_impulses.iter_mut() {
        ext_impulse.impulse = Vec2::new(100.0, 200.0);
        ext_impulse.torque_impulse = 0.4;
    }
}

info
Keep in mind that a dynamic rigid-body with a zero mass won't be affected by a linear force/impulse, and a rigid-body with a zero angular inertia won't be affected by torques/torque impulses. So if your force doesn't appear to do anything, make sure that:

The rigid-body is dynamic.
It is strong enough to make the rigid-body move (try a very large value and see if it does something).
The rigid-body has a non-zero mass or angular inertia either because they were set explicitly, or because they were computed automatically from colliders with non-zero densities.
Mass properties
The mass properties of a rigid-body is composed of three parts:

The mass which determines the resistance of the rigid-body wrt. linear movements. A high mass implies that larger forces are needed to make the rigid-body translate.
The angular inertia determines the resistance of the rigid-body wrt. the angular movements. A high angular inertia implies that larger torques are needed to make the rigid-body rotate.
The center-of-mass determines relative to what points torques are applied to the rigid-body.
note
Zero is a special value for masses and angular inertia. A mass equal to zero is interpreted as an infinite mass. An angular inertia equal to zero is interpreted as an infinite angular inertia. Therefore, a rigid-body with a mass equal to zero will not be affected by any force, and a rigid-body with an angular inertia equal to zero will not be affected by any torque.

Computing the mass and angular-inertia can often be difficult because they depend on the geometric shape of the object being simulated. This is why they are automatically computed by Rapier when a collider is attached to the rigid-body: the collider add its own mass and angular-inertia contribution (computed based on the collider's shape and density) to the rigid-body it is attached to:

// When the collider is attached, the rigid-body's mass and angular
// inertia will be automatically updated to take the collider into account.
commands
    .spawn(RigidBody::Dynamic)
    .insert(Collider::ball(1.0))
    // The default density is 1.0, we are setting 2.0 for this example.
    .insert(ColliderMassProperties::Density(2.0));


Alternatively, it is possible to set the mass properties of a rigid-body when it is created. Keep in mind that this won't prevent the colliders' contributions to be added to these values. So make sure to set the attached colliders' densities to zero if you want your explicit values to be the final mass-properties values.

/* Set the additional mass properties when the rigid-body bundle is created. */
commands
    .spawn(RigidBody::Dynamic)
    .insert(AdditionalMassProperties::Mass(10.0));


/* Change the additional mass-properties inside of a system. */
fn modify_body_mass_props(mut mprops: Query<&mut AdditionalMassProperties>) {
    for mut mprops in mprops.iter_mut() {
        *mprops = AdditionalMassProperties::Mass(100.0);
    }
}


Locking translations/rotations
It is sometimes useful to prevent a rigid-body from rotating or translating. One typical use-case for locking rotations is to prevent a player modeled as a dynamic rigid-body from tilting. These kind of degree-of-freedom restrictions could be achieved by joints, but locking translations/rotations of a single rigid-body wrt. the cartesian coordinate axes can be done in a much more efficient and numerically stable way. That's why rigid-bodies have dedicated flags for this.

Example 2D
Example 3D
/* Lock translations and/or rotations when the rigid-body bundle is created. */
commands
    .spawn(RigidBody::Dynamic)
    .insert(LockedAxes::TRANSLATION_LOCKED);


/* Lock translations and/or rotations inside of a system. */
fn modify_body_locked_flags(mut locked_axes: Query<&mut LockedAxes>) {
    for mut locked_axes in locked_axes.iter_mut() {
        *locked_axes = LockedAxes::ROTATION_LOCKED;
    }
}


Damping
Damping lets you slow down a rigid-body automatically. This can be used to achieve a wide variety of effects like fake air friction. Each rigid-body is given a linear damping coefficient (affecting its linear velocity) and an angular damping coefficient (affecting its angular velocity). Larger values of the damping coefficients lead to a stronger slow-downs. Their default values are 0.0 (no damping at all).

This damping coefficients can be set when the rigid-body is created or after its creation:

/* Set damping when the rigid-body bundle is created. */
commands.spawn(RigidBody::Dynamic).insert(Damping {
    linear_damping: 0.5,
    angular_damping: 1.0,
});

/* Set damping inside of a system. */
fn modify_body_damping(mut dampings: Query<&mut Damping>) {
    for mut rb_damping in dampings.iter_mut() {
        rb_damping.linear_damping = 0.5;
        rb_damping.angular_damping = 1.0;
    }
}

Dominance
Dominance is a non-realistic, but sometimes useful, feature. It can be used to make one rigid-body immune to forces originating from contacts with some other bodies. For example this can be used to model a player represented as a dynamic rigid-body that cannot be "pushed back" by any, or some, other dynamic rigid-bodies part of the environment.

Each rigid-body is part of a dominance group in [-127; 127] (the default group is 0). If the colliders from two rigid-bodies are in contact, the one with the highest dominance will act as if it has an infinite mass, making it immune to the contact forces the other body would apply on it. If both bodies are part of the same dominance group, then their contacts will work in the usual way (both are affected by opposite forces with the same magnitude).

For example, if a dynamic body A is in the dominance group 10, and a dynamic body B in the dominance group -20, then a contact between a collider attached to A and a collider attached B will result in A remaining immobile and B being pushed by A (independently from their mass).

info
A non-dynamic rigid-body is always considered as being part of a dominance group greater than any dynamic rigid-body. This means that dynamic/fixed and dynamic/kinematic contacts will continue to work normally, independently from the dominance group they were given by the user.

The dominance group can be set when the rigid-body is created or after its creation:

/* Set dominance when the rigid-body bundle is created. */
commands
    .spawn(RigidBody::Dynamic)
    .insert(Dominance::group(10));

/* Set dominance inside of a system. */
fn modify_body_dominance(mut dominances: Query<&mut Dominance>) {
    for mut rb_dominance in dominances.iter_mut() {
        rb_dominance.groups = 10;
    }
}


Continuous collision detection
Continuous Collision Detection (CCD) is used to make sure that fast-moving objects don't miss any contacts (a problem usually called tunneling). This is done by using motion-clamping, i.e., each fast-moving rigid-body with CCD enabled will be stopped at the time where their first contact happen, taking their continuous motion into account. This will result in some "time loss" for that rigid-body. This loss of time can be reduced by increasing the maximum number of CCD substeps executed (the default being 1) in the IntegrationParameters (by changing the RapierContext::integration_parameters::max_ccd_substeps field).

Rapier implements nonlinear CCD, meaning that it takes into account both the angular and translational motion of the rigid-body.

info
CCD takes action only if the CCD-enabled rigid-body is moving fast relative to another rigid-body. Therefore it is useless to enable CCD on fixed rigid-bodies and rigid-bodies that are expected to move slowly.

By default, CCD is disabled for all the rigid-bodies because it requires additional computations. It can be enabled when creating a rigid-body or after its creation:

/* Enable CCD when the rigid-body bundle is created. */
commands.spawn(RigidBody::Dynamic).insert(Ccd::enabled());

/* Enable CCD inside of a system. */
fn modify_body_ccd(mut ccds: Query<&mut Ccd>) {
    for mut rb_ccd in ccds.iter_mut() {
        rb_ccd.enabled = true;
    }
}

Sleeping
When a dynamic rigid-body doesn't move (or moves very slowly) during a few seconds, it will be marked as sleeping by the physics pipeline. Rigid-bodies marked as sleeping are no longer simulated by the physics engine until they are woken up. That way the physics engine doesn't waste any computational resources simulating objects that don't actually move. They are woken up automatically whenever another non-sleeping rigid-body starts interacting with them (either with a joint, or with one of its attached colliders generating contacts).

When using the bevy_rapier plugin, rigid-bodies are also automatically woken up whenever one of the components of the rigid-body is modified (to apply forces, change its position, etc.) They will not be awaken automatically when changing the gravity though. So you may sometimes want to wake a rigid-body manually by setting the component field Sleeping::sleeping to true.

Previous
Getting started
Next
Colliders
Creation and insertion
Rigid-body type
Position
Velocity
Gravity
Forces and impulses
Mass properties
Locking translations/rotations
Damping
Dominance
Continuous collision detection
Sleeping
Resources
Documentation
Demos 2D
Demos 3D
Community
Stack Overflow
Discord
More
GitHub
Dimforge EURL Logo
Built by Dimforge, EURL.
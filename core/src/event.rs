//! Handle events of a user interface.
use crate::keyboard;
use crate::mouse;
use crate::touch;
use crate::window;

/// A user interface event.
///
/// _**Note:** This type is largely incomplete! If you need to track
/// additional events, feel free to [open an issue] and share your use case!_
///
/// [open an issue]: https://github.com/iced-rs/iced/issues
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// A keyboard event
    Keyboard(keyboard::Event),

    /// A mouse event
    Mouse(mouse::Event),

    /// A window event
    Window(window::Event),

    /// A touch event
    Touch(touch::Event),

    /// A platform specific event
    PlatformSpecific(PlatformSpecific),

    /// An arbitrary timing event that fires every frame, regardless of input. Generally used for
    /// animated components.
    ///
    /// Iced does not directly initiate this event, as its source depends on the environment in
    /// which Iced is running. For example, in an ECS environment there will generally be a system
    /// responsible for piping the timing information via this event. It is up to the individual app
    /// to determine how and when to send this event.
    Tick(TickEvent),
}

/// A platform specific event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformSpecific {
    /// A MacOS specific event
    MacOS(MacOS),
}

/// Describes an event specific to MacOS
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacOS {
    /// Triggered when the app receives an URL from the system
    ///
    /// _**Note:** For this event to be triggered, the executable needs to be properly [bundled]!_
    ///
    /// [bundled]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html#//apple_ref/doc/uid/10000123i-CH101-SW19
    ReceivedUrl(String),
}

/// A timing event that fires every frame, regardless of input. Used for animation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickEvent {
    /// Seconds elapsed since last tick.
    pub delta_seconds: f32,

    /// Total seconds elapsed since the app was started.
    pub total_seconds: f32,
}

impl TickEvent {
    /// Creates a new ['TickEvent'].
    pub fn new(delta_seconds: f32, total_seconds: f32) -> Self {
        Self { delta_seconds: delta_seconds, total_seconds }
    }
}

/// The status of an [`Event`] after being processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Event`] was **NOT** handled by any widget.
    Ignored,

    /// The [`Event`] was handled and processed by a widget.
    Captured,
}

impl Status {
    /// Merges two [`Status`] into one.
    ///
    /// `Captured` takes precedence over `Ignored`:
    ///
    /// ```
    /// use iced_core::event::Status;
    ///
    /// assert_eq!(Status::Ignored.merge(Status::Ignored), Status::Ignored);
    /// assert_eq!(Status::Ignored.merge(Status::Captured), Status::Captured);
    /// assert_eq!(Status::Captured.merge(Status::Ignored), Status::Captured);
    /// assert_eq!(Status::Captured.merge(Status::Captured), Status::Captured);
    /// ```
    pub fn merge(self, b: Self) -> Self {
        match self {
            Status::Ignored => b,
            Status::Captured => Status::Captured,
        }
    }
}

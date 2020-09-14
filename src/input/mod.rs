//! Input, the abstraction
//!
//! # Usage
//!
//! The primary ANF input system is just a state:
//! handle input:
//!
//! ```
//! use anf::input::{Key, Keyboard};
//!
//! fn handle_input(kbd: &Keyboard) {
//!     if kbd.is_pressed(kbd::Enter) {
//!         // do something
//!     }
//! }
//! ```
//!
//! Why is it not event-based? Because it's for emitting higher-level input events like UI commands,
//! where it's natural to be updated every frame. Same for the virtual input ([`vinput`]).
//!
//! * TODO: explain why I think it's simpler to pull input when emitting higher-level input events
//! * TODO: mention Android where `update` it not allowed (?)`
//!
//! # Integration
//!
//! Call the lifecycle methods of input objects:
//!
//! ```
//! use anf::prelude::*;
//! use anf::input::Keyboard;
//! use sdl2::event::Event;
//!
//! struct SampleState {
//!      kbd: Keyboard,
//! }
//!
//! impl AnfGame for SampleState {
//!     fn event(&mut self, ev: &Event) {
//!         self.kbd.listen_sdl_event(ev);
//!     }
//!     fn on_next_frame(&mut self) {
//!         self.kbd.on_next_frame();
//!     }
//! }
//! ```

mod keyboard;
use num_enum::TryFromPrimitive;

pub mod vinput;
pub use keyboard::Keyboard;

/// ANF key code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u32)]
pub enum Key {
    None = 0,
    /// Backspace
    Back = 8,
    Tab = 9,
    Enter = 13,
    CapsLock = 20,
    Escape = 27,
    Space = 32,
    PageUp = 33,
    PageDown = 34,
    End = 35,
    Home = 36,
    Left = 37,
    Up = 38,
    Right = 39,
    Down = 40,
    Select = 41,
    Print = 42,
    Execute = 43,
    PrintScreen = 44,
    Insert = 45,
    Delete = 46,
    Help = 47,
    /// Digit 0
    D0 = 48,
    D1 = 49,
    D2 = 50,
    D3 = 51,
    D4 = 52,
    D5 = 53,
    D6 = 54,
    D7 = 55,
    D8 = 56,
    D9 = 57,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftWindows = 91,
    RightWindows = 92,
    Apps = 93,
    Sleep = 95,
    NumPad0 = 96,
    NumPad1 = 97,
    NumPad2 = 98,
    NumPad3 = 99,
    NumPad4 = 100,
    NumPad5 = 101,
    NumPad6 = 102,
    NumPad7 = 103,
    NumPad8 = 104,
    NumPad9 = 105,
    Multiply = 106,
    Add = 107,
    Separator = 108,
    Subtract = 109,
    Decimal = 110,
    Divide = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    F13 = 124,
    F14 = 125,
    F15 = 126,
    F16 = 127,
    F17 = 128,
    F18 = 129,
    F19 = 130,
    F20 = 131,
    F21 = 132,
    F22 = 133,
    F23 = 134,
    F24 = 135,
    NumLock = 144,
    Scroll = 145,
    LeftShift = 160,
    RightShift = 161,
    LeftControl = 162,
    RightControl = 163,
    LeftAlt = 164,
    RightAlt = 165,
    BrowserBack = 166,
    BrowserForward = 167,
    BrowserRefresh = 168,
    BrowserStop = 169,
    BrowserSearch = 170,
    BrowserFavorites = 171,
    BrowserHome = 172,
    VolumeMute = 173,
    VolumeDown = 174,
    VolumeUp = 175,
    MediaNextTrack = 176,
    MediaPreviousTrack = 177,
    MediaStop = 178,
    MediaPlayPause = 179,
    LaunchMail = 180,
    SelectMedia = 181,
    LaunchApplication1 = 182,
    LaunchApplication2 = 183,
    /// The OEM Semicolon key on a US standard keyboard.
    OemSemicolon = 186,
    OemPlus = 187,
    OemComma = 188,
    OemMinus = 189,
    OemPeriod = 190,
    OemQuestion = 191,
    OemTilde = 192,
    OemOpenBrackets = 219,
    OemPipe = 220,
    OemCloseBrackets = 221,
    OemQuotes = 222,
    Oem8 = 223,
    OemBackslash = 226,
    ProcessKey = 229,
    Attn = 246,
    Crsel = 247,
    Exsel = 248,
    EraseEof = 249,
    Play = 250,
    Zoom = 251,
    Pa1 = 253,
    OemClear = 254,
    ChatPadGreen = 0xCA,
    ChatPadOrange = 0xCB,
    Pause = 0x13,
    ImeConvert = 0x1c,
    ImeNoConvert = 0x1d,
    Kana = 0x15,
    Kanji = 0x19,
    OemAuto = 0xf3,
    OemCopy = 0xf2,
    /// OEM Enlarge Window key.
    OemEnlW = 0xf4,
}

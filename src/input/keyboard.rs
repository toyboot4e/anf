//! Re-exported to super module

use std::convert::TryFrom;

use crate::input::Key;
pub use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod, Scancode},
};
use std::collections::HashMap;

/// Full-feature keyboard state
#[derive(Debug)]
pub struct Keyboard {
    /// SDL2 keycode to ANF keycode
    s2f: HashMap<Keycode, Key>,
    kbd: Double<self::KeyboardStateSnapshot>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            s2f: self::gen_key_translation(),
            kbd: Double::default(),
        }
    }
}

/// Interface
impl Keyboard {
    pub fn is_key_down(&self, key: Key) -> bool {
        self.kbd.a.is_down(key)
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        self.kbd.a.is_up(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.kbd.b.is_up(key) && self.kbd.a.is_down(key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.kbd.b.is_down(key) && self.kbd.a.is_up(key)
    }
}

/// Lifecycle
impl Keyboard {
    pub fn listen_sdl_event(&mut self, ev: &Event) {
        match ev {
            Event::KeyDown {
                keycode: Some(sdl_key),
                ..
            } => {
                self.on_key_down(*sdl_key);
            }
            Event::KeyUp {
                keycode: Some(sdl_key),
                ..
            } => {
                self.on_key_up(*sdl_key);
            }
            _ => {}
        }
    }

    /// Prepare for next frame
    pub fn on_next_frame(&mut self) {
        self.kbd.b.bits = self.kbd.a.bits;
    }
}

impl Keyboard {
    fn on_key_down(&mut self, sdl_key: Keycode) {
        let anf_key = match self.s2f.get(&sdl_key) {
            Some(key) => key.clone(),
            None => return,
        };
        self.kbd.a.on_key_down(anf_key);
    }

    fn on_key_up(&mut self, sdl_key: Keycode) {
        let anf_key = match self.s2f.get(&sdl_key) {
            Some(key) => key.clone(),
            None => return,
        };
        self.kbd.a.on_key_up(anf_key);
    }
}

#[derive(Debug)]
pub struct Double<T> {
    a: T,
    b: T,
}

impl<T: Default> Default for Double<T> {
    fn default() -> Self {
        Self {
            a: T::default(),
            b: T::default(),
        }
    }
}

/// 256 bits that represent if the key is up or down
///
/// Compare two snapshots to see if the key is pressed or released.
///
/// Based on: http://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetParallel
#[derive(Debug, Default)]
pub struct KeyboardStateSnapshot {
    pub bits: [u32; 8],
}

impl KeyboardStateSnapshot {
    // fn from_keys(akeys: &[Keycode]) -> Self {}

    pub fn on_key_down(&mut self, key: Key) {
        let mask = 1 << ((key as u32) & 0x1f);
        let ix = key as usize >> 5;
        self.bits[ix] |= mask;
    }

    pub fn on_key_up(&mut self, key: Key) {
        let mask = 1 << ((key as u32) & 0x1f);
        let ix = key as usize >> 5;
        self.bits[ix] &= !mask;
    }

    pub fn is_down(&self, key: Key) -> bool {
        let mask: u32 = 1 << ((key as u32) & 0x1f);
        let ix = key as usize >> 5;
        (self.bits[ix] & mask) != 0
    }

    pub fn is_up(&self, key: Key) -> bool {
        !self.is_down(key)
    }

    pub fn pressed_keys(&self) -> Vec<Key> {
        let count = self
            .bits
            .iter()
            .map(|bits| Self::count_bits(*bits) as usize)
            .sum();

        if count == 0 {
            return Vec::new();
        }

        let mut keys = Vec::with_capacity(count);

        let mut ix = 0;
        for bits in self.bits.iter() {
            if *bits != 0 {
                ix = Self::store_keys(*bits, 0 * 32, &mut keys, ix);
            }
        }

        keys
    }
}

impl KeyboardStateSnapshot {
    /// http://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetParallel
    fn count_bits(key: u32) -> u32 {
        let mut v = key as u32;
        v = v - ((v >> 1) & 0x55555555);
        v = (v & 0x33333333) + ((v >> 2) & 0x33333333);
        ((v + (v >> 4) & 0xF0F0F0F) * 0x1010101) >> 24
    }

    fn store_keys(keys: u32, offset: u32, pressed_keys: &mut [Key], mut ix: usize) -> usize {
        for i in 0..32 {
            if (keys & (1 << i)) != 0 {
                pressed_keys[ix] = Key::try_from(offset + i).unwrap();
                ix += 1;
            }
        }
        ix
    }
}

pub fn gen_key_translation() -> HashMap<Keycode, Key> {
    [
        (Keycode::A, Key::A),
        (Keycode::B, Key::B),
        (Keycode::C, Key::C),
        (Keycode::D, Key::D),
        (Keycode::E, Key::E),
        (Keycode::F, Key::F),
        (Keycode::G, Key::G),
        (Keycode::H, Key::H),
        (Keycode::I, Key::I),
        (Keycode::J, Key::J),
        (Keycode::K, Key::K),
        (Keycode::L, Key::L),
        (Keycode::M, Key::M),
        (Keycode::N, Key::N),
        (Keycode::O, Key::O),
        (Keycode::P, Key::P),
        (Keycode::Q, Key::Q),
        (Keycode::R, Key::R),
        (Keycode::S, Key::S),
        (Keycode::T, Key::T),
        (Keycode::U, Key::U),
        (Keycode::V, Key::V),
        (Keycode::W, Key::W),
        (Keycode::X, Key::X),
        (Keycode::Y, Key::Y),
        (Keycode::Z, Key::Z),
        (Keycode::Num0, Key::D0),
        (Keycode::Num1, Key::D1),
        (Keycode::Num2, Key::D2),
        (Keycode::Num3, Key::D3),
        (Keycode::Num4, Key::D4),
        (Keycode::Num5, Key::D5),
        (Keycode::Num6, Key::D6),
        (Keycode::Num7, Key::D7),
        (Keycode::Num8, Key::D8),
        (Keycode::Num9, Key::D9),
        (Keycode::Kp0, Key::NumPad0),
        (Keycode::Kp1, Key::NumPad1),
        (Keycode::Kp2, Key::NumPad2),
        (Keycode::Kp3, Key::NumPad3),
        (Keycode::Kp4, Key::NumPad4),
        (Keycode::Kp5, Key::NumPad5),
        (Keycode::Kp6, Key::NumPad6),
        (Keycode::Kp7, Key::NumPad7),
        (Keycode::Kp8, Key::NumPad8),
        (Keycode::Kp9, Key::NumPad9),
        (Keycode::KpClear, Key::OemClear),
        (Keycode::KpDecimal, Key::Decimal),
        (Keycode::KpDivide, Key::Divide),
        (Keycode::KpEnter, Key::Enter),
        (Keycode::KpMinus, Key::Subtract),
        (Keycode::KpMultiply, Key::Multiply),
        (Keycode::KpPeriod, Key::OemPeriod),
        (Keycode::KpPlus, Key::Add),
        (Keycode::F1, Key::F1),
        (Keycode::F2, Key::F2),
        (Keycode::F3, Key::F3),
        (Keycode::F4, Key::F4),
        (Keycode::F5, Key::F5),
        (Keycode::F6, Key::F6),
        (Keycode::F7, Key::F7),
        (Keycode::F8, Key::F8),
        (Keycode::F9, Key::F9),
        (Keycode::F10, Key::F10),
        (Keycode::F11, Key::F11),
        (Keycode::F12, Key::F12),
        (Keycode::F13, Key::F13),
        (Keycode::F14, Key::F14),
        (Keycode::F15, Key::F15),
        (Keycode::F16, Key::F16),
        (Keycode::F17, Key::F17),
        (Keycode::F18, Key::F18),
        (Keycode::F19, Key::F19),
        (Keycode::F20, Key::F20),
        (Keycode::F21, Key::F21),
        (Keycode::F22, Key::F22),
        (Keycode::F23, Key::F23),
        (Keycode::F24, Key::F24),
        (Keycode::Space, Key::Space),
        (Keycode::Up, Key::Up),
        (Keycode::Down, Key::Down),
        (Keycode::Left, Key::Left),
        (Keycode::Right, Key::Right),
        (Keycode::LAlt, Key::LeftAlt),
        (Keycode::RAlt, Key::RightAlt),
        (Keycode::LCtrl, Key::LeftControl),
        (Keycode::RCtrl, Key::RightControl),
        (Keycode::LGui, Key::LeftWindows),
        (Keycode::RGui, Key::RightWindows),
        (Keycode::LShift, Key::LeftShift),
        (Keycode::RShift, Key::RightShift),
        (Keycode::Application, Key::Apps),
        (Keycode::Slash, Key::OemQuestion),
        (Keycode::Backslash, Key::OemBackslash),
        (Keycode::LeftBracket, Key::OemOpenBrackets),
        (Keycode::RightBracket, Key::OemCloseBrackets),
        (Keycode::CapsLock, Key::CapsLock),
        (Keycode::Comma, Key::OemComma),
        (Keycode::Delete, Key::Delete),
        (Keycode::End, Key::End),
        (Keycode::Backspace, Key::Back),
        (Keycode::Return, Key::Enter),
        (Keycode::Escape, Key::Escape),
        (Keycode::Home, Key::Home),
        (Keycode::Insert, Key::Insert),
        (Keycode::Minus, Key::OemMinus),
        (Keycode::NumLockClear, Key::NumLock),
        (Keycode::PageUp, Key::PageUp),
        (Keycode::PageDown, Key::PageDown),
        (Keycode::Pause, Key::Pause),
        (Keycode::Period, Key::OemPeriod),
        (Keycode::Equals, Key::OemPlus),
        (Keycode::PrintScreen, Key::PrintScreen),
        (Keycode::Quote, Key::OemQuotes),
        (Keycode::ScrollLock, Key::Scroll),
        (Keycode::Semicolon, Key::OemSemicolon),
        (Keycode::Sleep, Key::Sleep),
        (Keycode::Tab, Key::Tab),
        (Keycode::Backquote, Key::OemTilde),
        (Keycode::VolumeUp, Key::VolumeUp),
        (Keycode::VolumeDown, Key::VolumeDown),
    ]
    .iter()
    .cloned()
    .collect()
}

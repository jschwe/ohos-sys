//! Conversions from XComponent key events to [`keyboard_types`] values,
//! including US-layout character computation.
//!
//! OpenHarmony currently has no keyboard-layout API and the assumption
//! seems to be US keyboard layout. Until there is an official OH API for
//! keyboard layout, the best we can do is provide tracking and conversion
//! for US layout here (Note: This will move to an abstraction crate in the future)

use std::collections::HashSet;

use crate::{OH_NativeXComponent_KeyAction, OH_NativeXComponent_KeyCode};
use keyboard_types::{Code, Key, KeyState, KeyboardEvent, Location, Modifiers, NamedKey};

impl From<OH_NativeXComponent_KeyCode> for Code {
    fn from(code: OH_NativeXComponent_KeyCode) -> Self {
        use OH_NativeXComponent_KeyCode as OH_KeyCode;
        match code {
            OH_KeyCode::KEY_GRAVE => Code::Backquote,
            OH_KeyCode::KEY_BACKSLASH => Code::Backslash,
            OH_KeyCode::KEY_LEFT_BRACKET => Code::BracketLeft,
            OH_KeyCode::KEY_RIGHT_BRACKET => Code::BracketRight,
            OH_KeyCode::KEY_COMMA => Code::Comma,
            OH_KeyCode::KEY_0 => Code::Digit0,
            OH_KeyCode::KEY_1 => Code::Digit1,
            OH_KeyCode::KEY_2 => Code::Digit2,
            OH_KeyCode::KEY_3 => Code::Digit3,
            OH_KeyCode::KEY_4 => Code::Digit4,
            OH_KeyCode::KEY_5 => Code::Digit5,
            OH_KeyCode::KEY_6 => Code::Digit6,
            OH_KeyCode::KEY_7 => Code::Digit7,
            OH_KeyCode::KEY_8 => Code::Digit8,
            OH_KeyCode::KEY_9 => Code::Digit9,

            OH_KeyCode::KEY_EQUALS => Code::Equal,
            OH_KeyCode::KEY_RO => Code::IntlRo,
            OH_KeyCode::KEY_YEN => Code::IntlYen,

            OH_KeyCode::KEY_A => Code::KeyA,
            OH_KeyCode::KEY_B => Code::KeyB,
            OH_KeyCode::KEY_C => Code::KeyC,
            OH_KeyCode::KEY_D => Code::KeyD,
            OH_KeyCode::KEY_E => Code::KeyE,
            OH_KeyCode::KEY_F => Code::KeyF,
            OH_KeyCode::KEY_G => Code::KeyG,
            OH_KeyCode::KEY_H => Code::KeyH,
            OH_KeyCode::KEY_I => Code::KeyI,
            OH_KeyCode::KEY_J => Code::KeyJ,
            OH_KeyCode::KEY_K => Code::KeyK,
            OH_KeyCode::KEY_L => Code::KeyL,
            OH_KeyCode::KEY_M => Code::KeyM,
            OH_KeyCode::KEY_N => Code::KeyN,
            OH_KeyCode::KEY_O => Code::KeyO,
            OH_KeyCode::KEY_P => Code::KeyP,
            OH_KeyCode::KEY_Q => Code::KeyQ,
            OH_KeyCode::KEY_R => Code::KeyR,
            OH_KeyCode::KEY_S => Code::KeyS,
            OH_KeyCode::KEY_T => Code::KeyT,
            OH_KeyCode::KEY_U => Code::KeyU,
            OH_KeyCode::KEY_V => Code::KeyV,
            OH_KeyCode::KEY_W => Code::KeyW,
            OH_KeyCode::KEY_X => Code::KeyX,
            OH_KeyCode::KEY_Y => Code::KeyY,
            OH_KeyCode::KEY_Z => Code::KeyZ,

            OH_KeyCode::KEY_MINUS => Code::Minus,
            OH_KeyCode::KEY_PERIOD => Code::Period,
            OH_KeyCode::KEY_APOSTROPHE => Code::Quote,
            OH_KeyCode::KEY_SEMICOLON => Code::Semicolon,
            OH_KeyCode::KEY_SLASH => Code::Slash,
            OH_KeyCode::KEY_ALT_LEFT => Code::AltLeft,
            OH_KeyCode::KEY_ALT_RIGHT => Code::AltRight,
            OH_KeyCode::KEY_DEL => Code::Backspace,
            OH_KeyCode::KEY_CAPS_LOCK => Code::CapsLock,
            OH_KeyCode::KEY_MENU => Code::ContextMenu,
            OH_KeyCode::KEY_CTRL_LEFT => Code::ControlLeft,
            OH_KeyCode::KEY_CTRL_RIGHT => Code::ControlRight,
            OH_KeyCode::KEY_ENTER => Code::Enter,
            OH_KeyCode::KEY_META_LEFT => Code::MetaLeft,
            OH_KeyCode::KEY_META_RIGHT => Code::MetaRight,
            OH_KeyCode::KEY_SHIFT_LEFT => Code::ShiftLeft,
            OH_KeyCode::KEY_SHIFT_RIGHT => Code::ShiftRight,
            OH_KeyCode::KEY_SPACE => Code::Space,
            OH_KeyCode::KEY_TAB => Code::Tab,
            // Code::Convert
            // Lang1-5
            // NonConvert
            OH_KeyCode::KEY_FORWARD_DEL => Code::Delete,
            OH_KeyCode::KEY_MOVE_END => Code::End,
            OH_KeyCode::KEY_HELP => Code::Help,
            OH_KeyCode::KEY_HOME => Code::Home,
            OH_KeyCode::KEY_MOVE_HOME => Code::Home,
            OH_KeyCode::KEY_INSERT => Code::Insert,
            OH_KeyCode::KEY_PAGE_DOWN => Code::PageDown,
            OH_KeyCode::KEY_PAGE_UP => Code::PageUp,
            OH_KeyCode::KEY_DPAD_DOWN => Code::ArrowDown,
            OH_KeyCode::KEY_DPAD_UP => Code::ArrowUp,
            OH_KeyCode::KEY_DPAD_LEFT => Code::ArrowLeft,
            OH_KeyCode::KEY_DPAD_RIGHT => Code::ArrowRight,

            OH_KeyCode::KEY_NUM_LOCK => Code::NumLock,
            OH_KeyCode::KEY_NUMPAD_0 => Code::Numpad0,
            OH_KeyCode::KEY_NUMPAD_1 => Code::Numpad1,
            OH_KeyCode::KEY_NUMPAD_2 => Code::Numpad2,
            OH_KeyCode::KEY_NUMPAD_3 => Code::Numpad3,
            OH_KeyCode::KEY_NUMPAD_4 => Code::Numpad4,
            OH_KeyCode::KEY_NUMPAD_5 => Code::Numpad5,
            OH_KeyCode::KEY_NUMPAD_6 => Code::Numpad6,
            OH_KeyCode::KEY_NUMPAD_7 => Code::Numpad7,
            OH_KeyCode::KEY_NUMPAD_8 => Code::Numpad8,
            OH_KeyCode::KEY_NUMPAD_9 => Code::Numpad9,
            OH_KeyCode::KEY_NUMPAD_ADD => Code::NumpadAdd,
            OH_KeyCode::KEY_NUMPAD_COMMA => Code::NumpadComma,
            OH_KeyCode::KEY_NUMPAD_DIVIDE => Code::NumpadDivide,
            OH_KeyCode::KEY_NUMPAD_DOT => Code::NumpadDecimal,
            OH_KeyCode::KEY_NUMPAD_ENTER => Code::NumpadEnter,
            OH_KeyCode::KEY_NUMPAD_EQUALS => Code::NumpadEqual,
            OH_KeyCode::KEY_NUMPAD_LEFT_PAREN => Code::NumpadParenLeft,
            OH_KeyCode::KEY_NUMPAD_RIGHT_PAREN => Code::NumpadParenRight,
            OH_KeyCode::KEY_NUMPAD_MULTIPLY => Code::NumpadMultiply,
            // OH_KeyCode::KEY_NUMPAD_PLUSMINUS => Code::Numpad
            OH_KeyCode::KEY_NUMPAD_SUBTRACT => Code::NumpadSubtract,

            OH_KeyCode::KEY_ESCAPE => Code::Escape,
            OH_KeyCode::KEY_FN => Code::Fn,
            // apparently no fn lock
            OH_KeyCode::KEY_SYSRQ => Code::PrintScreen,
            OH_KeyCode::KEY_PRINT => Code::PrintScreen,
            OH_KeyCode::KEY_SCROLL_LOCK => Code::ScrollLock,
            OH_KeyCode::KEY_BREAK => Code::Pause,
            OH_KeyCode::KEY_FORWARD => Code::BrowserForward,
            // Code Browser*
            OH_KeyCode::KEY_MEDIA_EJECT => Code::Eject,
            OH_KeyCode::KEY_ENVELOPE => Code::LaunchMail,
            // Code Launch*
            OH_KeyCode::KEY_MEDIA_PLAY_PAUSE => Code::MediaPlayPause,
            OH_KeyCode::KEY_PLAYPAUSE => Code::MediaPlayPause,
            OH_KeyCode::KEY_MEDIA_PAUSE => Code::MediaPause,
            OH_KeyCode::KEY_MEDIA_PLAY => Code::MediaPlay,
            OH_KeyCode::KEY_MEDIA_NEXT => Code::MediaTrackNext,
            OH_KeyCode::KEY_MEDIA_PREVIOUS => Code::MediaTrackPrevious,
            OH_KeyCode::KEY_MEDIA_STOP => Code::MediaStop,
            OH_KeyCode::KEY_MEDIA_RECORD => Code::MediaRecord,
            OH_KeyCode::KEY_MEDIA_REWIND => Code::MediaRewind,
            OH_KeyCode::KEY_MEDIA_FAST_FORWARD => Code::MediaFastForward,

            OH_KeyCode::KEY_POWER => Code::Power,
            OH_KeyCode::KEY_SLEEP => Code::Sleep,

            OH_KeyCode::KEY_VOLUME_DOWN => Code::AudioVolumeDown,
            OH_KeyCode::KEY_VOLUME_UP => Code::AudioVolumeUp,
            OH_KeyCode::KEY_VOLUME_MUTE => Code::AudioVolumeMute,

            OH_KeyCode::KEY_WAKEUP => Code::WakeUp,
            // Hyper, Super, Turbo, Abort, Resume
            OH_KeyCode::KEY_SUSPEND => Code::Suspend,
            OH_KeyCode::KEY_AGAIN => Code::Again,
            OH_KeyCode::KEY_COPY => Code::Copy,
            OH_KeyCode::KEY_CUT => Code::Cut,
            OH_KeyCode::KEY_FIND => Code::Find,
            OH_KeyCode::KEY_OPEN => Code::Open,
            OH_KeyCode::KEY_PASTE => Code::Paste,
            OH_KeyCode::KEY_PROPS => Code::Props,
            // Select
            OH_KeyCode::KEY_UNDO => Code::Undo,
            OH_KeyCode::KEY_HIRAGANA => Code::Hiragana,
            OH_KeyCode::KEY_KATAKANA => Code::Katakana,
            OH_KeyCode::KEY_F1 => Code::F1,
            OH_KeyCode::KEY_F2 => Code::F2,
            OH_KeyCode::KEY_F3 => Code::F3,
            OH_KeyCode::KEY_F4 => Code::F4,
            OH_KeyCode::KEY_F5 => Code::F5,
            OH_KeyCode::KEY_F6 => Code::F6,
            OH_KeyCode::KEY_F7 => Code::F7,
            OH_KeyCode::KEY_F8 => Code::F8,
            OH_KeyCode::KEY_F9 => Code::F9,
            OH_KeyCode::KEY_F10 => Code::F10,
            OH_KeyCode::KEY_F11 => Code::F11,
            OH_KeyCode::KEY_F12 => Code::F12,
            OH_KeyCode::KEY_F13 => Code::F13,
            OH_KeyCode::KEY_F14 => Code::F14,
            OH_KeyCode::KEY_F15 => Code::F15,
            OH_KeyCode::KEY_F16 => Code::F16,
            OH_KeyCode::KEY_F17 => Code::F17,
            OH_KeyCode::KEY_F18 => Code::F18,
            OH_KeyCode::KEY_F19 => Code::F19,
            OH_KeyCode::KEY_F20 => Code::F20,
            OH_KeyCode::KEY_F21 => Code::F21,
            OH_KeyCode::KEY_F22 => Code::F22,
            OH_KeyCode::KEY_F23 => Code::F23,
            OH_KeyCode::KEY_F24 => Code::F24,
            OH_KeyCode::KEY_BRIGHTNESS_DOWN => Code::BrightnessDown,
            OH_KeyCode::KEY_BRIGHTNESS_UP => Code::BrightnessUp,
            // DisplayToggleIntExt?
            // OH_KeyCode::KEY_KBD_LAYOUT_NEXT => Code::KeyboardLayoutSelect
            OH_KeyCode::KEY_MUTE => Code::MicrophoneMuteToggle,
            OH_KeyCode::KEY_APPSELECT => Code::SelectTask,
            // todo: verify
            OH_KeyCode::KEY_CYCLEWINDOWS => Code::ShowAllWindows,
            // OH_KeyCode::KEY_ZOOMIN => Code::
            OH_KeyCode::KEY_UNKNOWN => Code::Unidentified,
            _ => Code::Unidentified,
        }
    }
}

pub struct UnknownKeyState {}

impl TryFrom<OH_NativeXComponent_KeyAction> for KeyState {
    type Error = UnknownKeyState;

    fn try_from(value: OH_NativeXComponent_KeyAction) -> Result<Self, Self::Error> {
        use OH_NativeXComponent_KeyAction as KeyAction;
        match value {
            KeyAction::OH_NATIVEXCOMPONENT_KEY_ACTION_UP => Ok(KeyState::Up),
            KeyAction::OH_NATIVEXCOMPONENT_KEY_ACTION_DOWN => Ok(KeyState::Down),
            _ => Err(UnknownKeyState {}),
        }
    }
}

/// Logical key value for key codes that never produce a character.
///
/// Character keys (including numpad keys, whose value depends on NumLock) are
/// resolved through [`us_char`] first; this map only covers the remaining
/// non-printable keys.
pub fn named_key(code: OH_NativeXComponent_KeyCode) -> Option<NamedKey> {
    use OH_NativeXComponent_KeyCode as OH_KeyCode;
    let key = match code {
        OH_KeyCode::KEY_FN | OH_KeyCode::KEY_FUNCTION => NamedKey::Fn,
        OH_KeyCode::KEY_HOME => NamedKey::GoHome,
        OH_KeyCode::KEY_BACK => NamedKey::GoBack,
        OH_KeyCode::KEY_MEDIA_PLAY_PAUSE | OH_KeyCode::KEY_PLAYPAUSE => NamedKey::MediaPlayPause,
        OH_KeyCode::KEY_MEDIA_STOP => NamedKey::MediaStop,
        OH_KeyCode::KEY_MEDIA_NEXT => NamedKey::MediaTrackNext,
        OH_KeyCode::KEY_MEDIA_PREVIOUS => NamedKey::MediaTrackPrevious,
        OH_KeyCode::KEY_MEDIA_REWIND => NamedKey::MediaRewind,
        OH_KeyCode::KEY_MEDIA_FAST_FORWARD => NamedKey::MediaFastForward,
        OH_KeyCode::KEY_VOLUME_UP => NamedKey::AudioVolumeUp,
        OH_KeyCode::KEY_VOLUME_DOWN => NamedKey::AudioVolumeDown,
        OH_KeyCode::KEY_POWER => NamedKey::Power,
        OH_KeyCode::KEY_CAMERA => NamedKey::Camera,
        OH_KeyCode::KEY_VOLUME_MUTE => NamedKey::AudioVolumeMute,
        OH_KeyCode::KEY_MUTE => NamedKey::MicrophoneVolumeMute,
        OH_KeyCode::KEY_BRIGHTNESS_UP => NamedKey::BrightnessUp,
        OH_KeyCode::KEY_BRIGHTNESS_DOWN => NamedKey::BrightnessDown,
        OH_KeyCode::KEY_DPAD_UP => NamedKey::ArrowUp,
        OH_KeyCode::KEY_DPAD_DOWN => NamedKey::ArrowDown,
        OH_KeyCode::KEY_DPAD_LEFT => NamedKey::ArrowLeft,
        OH_KeyCode::KEY_DPAD_RIGHT => NamedKey::ArrowRight,
        OH_KeyCode::KEY_DPAD_CENTER => NamedKey::Select,
        OH_KeyCode::KEY_ALT_LEFT | OH_KeyCode::KEY_ALT_RIGHT => NamedKey::Alt,
        OH_KeyCode::KEY_SHIFT_LEFT | OH_KeyCode::KEY_SHIFT_RIGHT => NamedKey::Shift,
        OH_KeyCode::KEY_TAB => NamedKey::Tab,
        OH_KeyCode::KEY_SYM => NamedKey::Symbol,
        OH_KeyCode::KEY_EXPLORER => NamedKey::LaunchWebBrowser,
        OH_KeyCode::KEY_ENVELOPE => NamedKey::LaunchMail,
        OH_KeyCode::KEY_ENTER | OH_KeyCode::KEY_NUMPAD_ENTER => NamedKey::Enter,
        OH_KeyCode::KEY_DEL => NamedKey::Backspace,
        OH_KeyCode::KEY_MENU => NamedKey::ContextMenu,
        OH_KeyCode::KEY_PAGE_UP => NamedKey::PageUp,
        OH_KeyCode::KEY_PAGE_DOWN => NamedKey::PageDown,
        OH_KeyCode::KEY_ESCAPE => NamedKey::Escape,
        OH_KeyCode::KEY_FORWARD_DEL => NamedKey::Delete,
        OH_KeyCode::KEY_CTRL_LEFT | OH_KeyCode::KEY_CTRL_RIGHT => NamedKey::Control,
        OH_KeyCode::KEY_CAPS_LOCK => NamedKey::CapsLock,
        OH_KeyCode::KEY_SCROLL_LOCK => NamedKey::ScrollLock,
        OH_KeyCode::KEY_META_LEFT | OH_KeyCode::KEY_META_RIGHT => NamedKey::Meta,
        OH_KeyCode::KEY_SYSRQ | OH_KeyCode::KEY_PRINT => NamedKey::PrintScreen,
        OH_KeyCode::KEY_BREAK => NamedKey::Pause,
        OH_KeyCode::KEY_MOVE_HOME => NamedKey::Home,
        OH_KeyCode::KEY_MOVE_END => NamedKey::End,
        OH_KeyCode::KEY_INSERT => NamedKey::Insert,
        OH_KeyCode::KEY_FORWARD => NamedKey::BrowserForward,
        OH_KeyCode::KEY_MEDIA_PLAY => NamedKey::MediaPlay,
        OH_KeyCode::KEY_MEDIA_PAUSE => NamedKey::MediaPause,
        OH_KeyCode::KEY_MEDIA_CLOSE => NamedKey::MediaClose,
        OH_KeyCode::KEY_MEDIA_EJECT => NamedKey::Eject,
        OH_KeyCode::KEY_MEDIA_RECORD => NamedKey::MediaRecord,
        OH_KeyCode::KEY_F1 => NamedKey::F1,
        OH_KeyCode::KEY_F2 => NamedKey::F2,
        OH_KeyCode::KEY_F3 => NamedKey::F3,
        OH_KeyCode::KEY_F4 => NamedKey::F4,
        OH_KeyCode::KEY_F5 => NamedKey::F5,
        OH_KeyCode::KEY_F6 => NamedKey::F6,
        OH_KeyCode::KEY_F7 => NamedKey::F7,
        OH_KeyCode::KEY_F8 => NamedKey::F8,
        OH_KeyCode::KEY_F9 => NamedKey::F9,
        OH_KeyCode::KEY_F10 => NamedKey::F10,
        OH_KeyCode::KEY_F11 => NamedKey::F11,
        OH_KeyCode::KEY_F12 => NamedKey::F12,
        OH_KeyCode::KEY_F13 => NamedKey::F13,
        OH_KeyCode::KEY_F14 => NamedKey::F14,
        OH_KeyCode::KEY_F15 => NamedKey::F15,
        OH_KeyCode::KEY_F16 => NamedKey::F16,
        OH_KeyCode::KEY_F17 => NamedKey::F17,
        OH_KeyCode::KEY_F18 => NamedKey::F18,
        OH_KeyCode::KEY_F19 => NamedKey::F19,
        OH_KeyCode::KEY_F20 => NamedKey::F20,
        OH_KeyCode::KEY_F21 => NamedKey::F21,
        OH_KeyCode::KEY_F22 => NamedKey::F22,
        OH_KeyCode::KEY_F23 => NamedKey::F23,
        OH_KeyCode::KEY_F24 => NamedKey::F24,
        OH_KeyCode::KEY_NUM_LOCK => NamedKey::NumLock,
        OH_KeyCode::KEY_SLEEP => NamedKey::Standby,
        OH_KeyCode::KEY_WAKEUP => NamedKey::WakeUp,
        OH_KeyCode::KEY_HELP => NamedKey::Help,
        OH_KeyCode::KEY_AGAIN => NamedKey::Again,
        OH_KeyCode::KEY_PROPS => NamedKey::Props,
        OH_KeyCode::KEY_UNDO => NamedKey::Undo,
        OH_KeyCode::KEY_COPY => NamedKey::Copy,
        OH_KeyCode::KEY_OPEN => NamedKey::Open,
        OH_KeyCode::KEY_PASTE => NamedKey::Paste,
        OH_KeyCode::KEY_FIND => NamedKey::Find,
        OH_KeyCode::KEY_CUT => NamedKey::Cut,
        OH_KeyCode::KEY_HIRAGANA => NamedKey::Hiragana,
        OH_KeyCode::KEY_KATAKANA => NamedKey::Katakana,
        OH_KeyCode::KEY_APPSELECT => NamedKey::AppSwitch,
        _ => return None,
    };
    Some(key)
}

/// Navigation key a numpad digit or dot key acts as while NumLock is off.
fn numpad_nav_key(code: Code) -> Option<NamedKey> {
    let key = match code {
        Code::Numpad0 => NamedKey::Insert,
        Code::Numpad1 => NamedKey::End,
        Code::Numpad2 => NamedKey::ArrowDown,
        Code::Numpad3 => NamedKey::PageDown,
        Code::Numpad4 => NamedKey::ArrowLeft,
        Code::Numpad5 => NamedKey::Clear,
        Code::Numpad6 => NamedKey::ArrowRight,
        Code::Numpad7 => NamedKey::Home,
        Code::Numpad8 => NamedKey::ArrowUp,
        Code::Numpad9 => NamedKey::PageUp,
        Code::NumpadDecimal => NamedKey::Delete,
        _ => return None,
    };
    Some(key)
}

/// `location` value for a key code. `NumLock` itself is `Standard`, per the
/// UI Events specification.
pub fn location(code: OH_NativeXComponent_KeyCode) -> Location {
    use OH_NativeXComponent_KeyCode as OH_KeyCode;
    match code {
        OH_KeyCode::KEY_ALT_LEFT
        | OH_KeyCode::KEY_SHIFT_LEFT
        | OH_KeyCode::KEY_CTRL_LEFT
        | OH_KeyCode::KEY_META_LEFT => Location::Left,
        OH_KeyCode::KEY_ALT_RIGHT
        | OH_KeyCode::KEY_SHIFT_RIGHT
        | OH_KeyCode::KEY_CTRL_RIGHT
        | OH_KeyCode::KEY_META_RIGHT => Location::Right,
        OH_KeyCode::KEY_NUMPAD_PLUSMINUS => Location::Numpad,
        code if (OH_KeyCode::KEY_NUMPAD_0.0..=OH_KeyCode::KEY_NUMPAD_RIGHT_PAREN.0)
            .contains(&code.0) =>
        {
            Location::Numpad
        }
        _ => Location::Standard,
    }
}

/// US-layout `(unshifted, shifted)` character pair for a physical key.
///
/// Derived from the `KEY_UNICODE_TRANSFORMATION` table of the OpenHarmony
/// multimodalinput service (multimodalinput_input @ 02182f08, Apache-2.0 like
/// this repo), re-keyed by [`Code`]. Deviations from upstream: Space is added
/// (absent upstream, so the platform mapper types nothing for it), and numpad
/// keys carry their character here while [`us_char`] gates the digit and dot
/// keys on NumLock (upstream ignores NumLock and gives numpad keys no shifted
/// character at all).
fn us_char_pair(code: Code) -> Option<(char, char)> {
    let pair = match code {
        Code::Digit0 => ('0', ')'),
        Code::Digit1 => ('1', '!'),
        Code::Digit2 => ('2', '@'),
        Code::Digit3 => ('3', '#'),
        Code::Digit4 => ('4', '$'),
        Code::Digit5 => ('5', '%'),
        Code::Digit6 => ('6', '^'),
        Code::Digit7 => ('7', '&'),
        Code::Digit8 => ('8', '*'),
        Code::Digit9 => ('9', '('),
        Code::KeyA => ('a', 'A'),
        Code::KeyB => ('b', 'B'),
        Code::KeyC => ('c', 'C'),
        Code::KeyD => ('d', 'D'),
        Code::KeyE => ('e', 'E'),
        Code::KeyF => ('f', 'F'),
        Code::KeyG => ('g', 'G'),
        Code::KeyH => ('h', 'H'),
        Code::KeyI => ('i', 'I'),
        Code::KeyJ => ('j', 'J'),
        Code::KeyK => ('k', 'K'),
        Code::KeyL => ('l', 'L'),
        Code::KeyM => ('m', 'M'),
        Code::KeyN => ('n', 'N'),
        Code::KeyO => ('o', 'O'),
        Code::KeyP => ('p', 'P'),
        Code::KeyQ => ('q', 'Q'),
        Code::KeyR => ('r', 'R'),
        Code::KeyS => ('s', 'S'),
        Code::KeyT => ('t', 'T'),
        Code::KeyU => ('u', 'U'),
        Code::KeyV => ('v', 'V'),
        Code::KeyW => ('w', 'W'),
        Code::KeyX => ('x', 'X'),
        Code::KeyY => ('y', 'Y'),
        Code::KeyZ => ('z', 'Z'),
        Code::Comma => (',', '<'),
        Code::Period => ('.', '>'),
        Code::Backquote => ('`', '~'),
        Code::Minus => ('-', '_'),
        Code::Equal => ('=', '+'),
        Code::BracketLeft => ('[', '{'),
        Code::BracketRight => (']', '}'),
        Code::Backslash => ('\\', '|'),
        Code::Semicolon => (';', ':'),
        Code::Quote => ('\'', '"'),
        Code::Slash => ('/', '?'),
        Code::Space => (' ', ' '),
        Code::Numpad0 => ('0', '0'),
        Code::Numpad1 => ('1', '1'),
        Code::Numpad2 => ('2', '2'),
        Code::Numpad3 => ('3', '3'),
        Code::Numpad4 => ('4', '4'),
        Code::Numpad5 => ('5', '5'),
        Code::Numpad6 => ('6', '6'),
        Code::Numpad7 => ('7', '7'),
        Code::Numpad8 => ('8', '8'),
        Code::Numpad9 => ('9', '9'),
        Code::NumpadDivide => ('/', '/'),
        Code::NumpadMultiply => ('*', '*'),
        Code::NumpadSubtract => ('-', '-'),
        Code::NumpadAdd => ('+', '+'),
        Code::NumpadDecimal => ('.', '.'),
        _ => return None,
    };
    Some(pair)
}

fn is_letter(code: Code) -> bool {
    matches!(
        code,
        Code::KeyA
            | Code::KeyB
            | Code::KeyC
            | Code::KeyD
            | Code::KeyE
            | Code::KeyF
            | Code::KeyG
            | Code::KeyH
            | Code::KeyI
            | Code::KeyJ
            | Code::KeyK
            | Code::KeyL
            | Code::KeyM
            | Code::KeyN
            | Code::KeyO
            | Code::KeyP
            | Code::KeyQ
            | Code::KeyR
            | Code::KeyS
            | Code::KeyT
            | Code::KeyU
            | Code::KeyV
            | Code::KeyW
            | Code::KeyX
            | Code::KeyY
            | Code::KeyZ
    )
}

fn is_numpad_digit_or_decimal(code: Code) -> bool {
    matches!(
        code,
        Code::Numpad0
            | Code::Numpad1
            | Code::Numpad2
            | Code::Numpad3
            | Code::Numpad4
            | Code::Numpad5
            | Code::Numpad6
            | Code::Numpad7
            | Code::Numpad8
            | Code::Numpad9
            | Code::NumpadDecimal
    )
}

fn is_numpad_operator(code: Code) -> bool {
    matches!(
        code,
        Code::NumpadDivide | Code::NumpadMultiply | Code::NumpadSubtract | Code::NumpadAdd
    )
}

/// Character produced by a physical key on the US layout, following the
/// platform's own `KeyCodeToUnicode` logic: letters use the shifted character
/// when Shift XOR CapsLock, other keys when Shift. `None` when the key
/// produces no character.
///
/// Deliberate deviation from the platform (which ignores NumLock): numpad
/// digit and dot keys produce their character only while `num_lock` is true
/// and `None` otherwise ([`KeyEventConverter`] then maps them to the
/// corresponding navigation keys); the numpad operator keys (divide, multiply,
/// subtract, add) produce their character regardless of NumLock, and Shift
/// does not affect the numpad.
pub fn us_char(code: Code, shift: bool, caps_lock: bool, num_lock: bool) -> Option<char> {
    let (unshifted, shifted) = us_char_pair(code)?;
    if is_numpad_digit_or_decimal(code) {
        return num_lock.then_some(unshifted);
    }
    if is_numpad_operator(code) {
        return Some(unshifted);
    }
    let use_shifted = if is_letter(code) {
        shift != caps_lock
    } else {
        shift
    };
    Some(if use_shifted { shifted } else { unshifted })
}

/// Modifier and lock state as reported by the platform alongside a key event.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ModifierState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    /// `None` when the platform cannot report Meta; [`KeyEventConverter`]
    /// then substitutes the state it tracks from `KEY_META_LEFT` /
    /// `KEY_META_RIGHT` transitions.
    pub meta: Option<bool>,
    pub caps_lock: bool,
    pub num_lock: bool,
    pub scroll_lock: bool,
}

/// Stateful translator from XComponent key events to
/// [`keyboard_types::KeyboardEvent`].
///
/// The state tracks which key codes are down (to derive `repeat`) and the Meta
/// keys (used when [`ModifierState::meta`] is `None`). Feed it every key event
/// of one physical keyboard, in order. Returns `None` for an unknown key
/// action.
///
/// Note: this stateful helper logically belongs in a safe XComponent wrapper
/// crate and will move there when such an abstraction crate is created; it
/// lives here until then.
///
/// ```
/// use keyboard_types::{Code, Key};
/// use xcomponent_sys::keyboard_types_compat::{KeyEventConverter, ModifierState};
/// use xcomponent_sys::{OH_NativeXComponent_KeyAction, OH_NativeXComponent_KeyCode};
///
/// let mut converter = KeyEventConverter::new();
/// let shifted = ModifierState {
///     shift: true,
///     ..Default::default()
/// };
/// let event = converter
///     .convert(
///         OH_NativeXComponent_KeyAction::OH_NATIVEXCOMPONENT_KEY_ACTION_DOWN,
///         OH_NativeXComponent_KeyCode::KEY_A,
///         shifted,
///     )
///     .unwrap();
/// assert_eq!(event.key, Key::Character("A".to_string()));
/// assert_eq!(event.code, Code::KeyA);
/// ```
#[derive(Clone, Debug, Default)]
pub struct KeyEventConverter {
    pressed: HashSet<OH_NativeXComponent_KeyCode>,
    meta_left: bool,
    meta_right: bool,
}

impl KeyEventConverter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn convert(
        &mut self,
        action: OH_NativeXComponent_KeyAction,
        key_code: OH_NativeXComponent_KeyCode,
        modifiers: ModifierState,
    ) -> Option<KeyboardEvent> {
        use OH_NativeXComponent_KeyCode as OH_KeyCode;
        let state = KeyState::try_from(action).ok()?;
        let down = state == KeyState::Down;
        if key_code == OH_KeyCode::KEY_META_LEFT {
            self.meta_left = down;
        } else if key_code == OH_KeyCode::KEY_META_RIGHT {
            self.meta_right = down;
        }
        let repeat = if down {
            !self.pressed.insert(key_code)
        } else {
            self.pressed.remove(&key_code);
            false
        };

        let meta = modifiers.meta.unwrap_or(self.meta_left || self.meta_right);
        let mut flags = Modifiers::empty();
        flags.set(Modifiers::SHIFT, modifiers.shift);
        flags.set(Modifiers::CONTROL, modifiers.ctrl);
        flags.set(Modifiers::ALT, modifiers.alt);
        flags.set(Modifiers::META, meta);
        flags.set(Modifiers::CAPS_LOCK, modifiers.caps_lock);
        flags.set(Modifiers::NUM_LOCK, modifiers.num_lock);
        flags.set(Modifiers::SCROLL_LOCK, modifiers.scroll_lock);

        let code = Code::from(key_code);
        let key = match us_char(
            code,
            modifiers.shift,
            modifiers.caps_lock,
            modifiers.num_lock,
        ) {
            Some(character) => Key::Character(character.to_string()),
            None => Key::Named(
                numpad_nav_key(code)
                    .or_else(|| named_key(key_code))
                    .unwrap_or(NamedKey::Unidentified),
            ),
        };

        Some(KeyboardEvent {
            state,
            key,
            code,
            location: location(key_code),
            modifiers: flags,
            repeat,
            is_composing: false,
        })
    }
}

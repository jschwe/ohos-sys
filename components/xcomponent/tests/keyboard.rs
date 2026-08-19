#![cfg(feature = "keyboard-types")]

use keyboard_types::webdriver::{send_keys, Event};
use keyboard_types::{Code, Key, KeyState, KeyboardEvent, Location, Modifiers, NamedKey};
use xcomponent_sys::keyboard_types_compat::{us_char, KeyEventConverter, ModifierState};
use xcomponent_sys::{OH_NativeXComponent_KeyAction, OH_NativeXComponent_KeyCode};

use OH_NativeXComponent_KeyAction as OH_KeyAction;
use OH_NativeXComponent_KeyCode as OH_KeyCode;

/// Codes whose character output intentionally diverges from the WebDriver US
/// layout. WebDriver types numpad digits and the decimal point
/// unconditionally; this crate requires NumLock and produces the numpad
/// navigation keys otherwise. (The upstream OHOS table gives these keys no
/// character at all — see `us_char` for the documented deviation.)
const DIVERGENT_CODES: &[Code] = &[
    Code::Numpad0,
    Code::Numpad1,
    Code::Numpad2,
    Code::Numpad3,
    Code::Numpad4,
    Code::Numpad5,
    Code::Numpad6,
    Code::Numpad7,
    Code::Numpad8,
    Code::Numpad9,
    Code::NumpadDecimal,
];

/// WebDriver codepoints for the numpad keys that type a character.
const WEBDRIVER_NUMPAD_KEYS: [char; 15] = [
    '\u{E01A}', '\u{E01B}', '\u{E01C}', '\u{E01D}', '\u{E01E}', '\u{E01F}', '\u{E020}', '\u{E021}',
    '\u{E022}', '\u{E023}', '\u{E024}', '\u{E025}', '\u{E027}', '\u{E028}', '\u{E029}',
];

/// The `(code, shift)` pair of the keydown that types `raw` per WebDriver.
fn webdriver_keydown(raw: char) -> (char, Code, bool) {
    for event in send_keys(&raw.to_string()) {
        if let Event::Keyboard(keyboard) = event {
            if keyboard.state == KeyState::Down {
                if let Key::Character(ref s) = keyboard.key {
                    let mut chars = s.chars();
                    let typed = chars.next().expect("empty character key");
                    assert_eq!(chars.next(), None);
                    return (
                        typed,
                        keyboard.code,
                        keyboard.modifiers.contains(Modifiers::SHIFT),
                    );
                }
            }
        }
    }
    panic!("no character keydown for {raw:?}");
}

#[test]
fn printable_ascii_matches_webdriver() {
    for c in '\u{20}'..'\u{7f}' {
        let (typed, code, shifted) = webdriver_keydown(c);
        assert_eq!(typed, c);
        assert!(
            !DIVERGENT_CODES.contains(&code),
            "unexpected divergent code for {c:?}"
        );
        assert_eq!(
            us_char(code, shifted, false, false),
            Some(c),
            "code {code:?}, shift {shifted}"
        );
    }
}

#[test]
fn numpad_matches_webdriver_modulo_allowlist() {
    for raw in WEBDRIVER_NUMPAD_KEYS {
        let (typed, code, shifted) = webdriver_keydown(raw);
        assert!(!shifted);
        assert_eq!(
            us_char(code, false, false, true),
            Some(typed),
            "code {code:?} with NumLock"
        );
        let without_num_lock = us_char(code, false, false, false);
        if DIVERGENT_CODES.contains(&code) {
            assert_eq!(without_num_lock, None, "code {code:?}");
        } else {
            assert_eq!(without_num_lock, Some(typed), "code {code:?}");
        }
    }
}

#[test]
fn all_letters_have_both_cases() {
    for c in 'a'..='z' {
        let (_, code, _) = webdriver_keydown(c);
        assert_eq!(us_char(code, false, false, false), Some(c));
        assert_eq!(
            us_char(code, true, false, false),
            Some(c.to_ascii_uppercase())
        );
    }
}

fn down(converter: &mut KeyEventConverter, code: OH_KeyCode, mods: ModifierState) -> KeyboardEvent {
    converter
        .convert(
            OH_KeyAction::OH_NATIVEXCOMPONENT_KEY_ACTION_DOWN,
            code,
            mods,
        )
        .unwrap()
}

fn up(converter: &mut KeyEventConverter, code: OH_KeyCode, mods: ModifierState) -> KeyboardEvent {
    converter
        .convert(OH_KeyAction::OH_NATIVEXCOMPONENT_KEY_ACTION_UP, code, mods)
        .unwrap()
}

fn character(s: &str) -> Key {
    Key::Character(s.to_string())
}

#[test]
fn letter_with_shift_and_caps_lock() {
    let mut converter = KeyEventConverter::new();
    let plain = ModifierState::default();
    let shift = ModifierState {
        shift: true,
        ..plain
    };
    let caps = ModifierState {
        caps_lock: true,
        ..plain
    };
    let caps_shift = ModifierState {
        shift: true,
        caps_lock: true,
        ..plain
    };
    assert_eq!(
        down(&mut converter, OH_KeyCode::KEY_A, plain).key,
        character("a")
    );
    let shifted = up(&mut converter, OH_KeyCode::KEY_A, shift);
    assert_eq!(shifted.key, character("A"));
    assert!(shifted.modifiers.contains(Modifiers::SHIFT));
    assert_eq!(
        down(&mut converter, OH_KeyCode::KEY_A, caps).key,
        character("A")
    );
    assert_eq!(
        up(&mut converter, OH_KeyCode::KEY_A, caps_shift).key,
        character("a")
    );
}

#[test]
fn digit_row_with_shift() {
    let mut converter = KeyEventConverter::new();
    let plain = ModifierState::default();
    let shift = ModifierState {
        shift: true,
        ..plain
    };
    assert_eq!(
        down(&mut converter, OH_KeyCode::KEY_1, plain).key,
        character("1")
    );
    assert_eq!(
        up(&mut converter, OH_KeyCode::KEY_1, shift).key,
        character("!")
    );
    assert_eq!(
        down(&mut converter, OH_KeyCode::KEY_2, plain).key,
        character("2")
    );
    assert_eq!(
        up(&mut converter, OH_KeyCode::KEY_2, shift).key,
        character("@")
    );
}

#[test]
fn numpad_digit_follows_num_lock() {
    let mut converter = KeyEventConverter::new();
    let num_lock = ModifierState {
        num_lock: true,
        ..Default::default()
    };
    let with = down(&mut converter, OH_KeyCode::KEY_NUMPAD_1, num_lock);
    assert_eq!(with.key, character("1"));
    assert_eq!(with.code, Code::Numpad1);
    assert_eq!(with.location, Location::Numpad);
    assert!(with.modifiers.contains(Modifiers::NUM_LOCK));
    let without = up(
        &mut converter,
        OH_KeyCode::KEY_NUMPAD_1,
        ModifierState::default(),
    );
    assert_eq!(without.key, Key::Named(NamedKey::End));
    assert_eq!(without.code, Code::Numpad1);
    assert_eq!(without.location, Location::Numpad);
}

#[test]
fn non_printables_produce_named_keys() {
    let mut converter = KeyEventConverter::new();
    let plain = ModifierState::default();
    let cases = [
        (OH_KeyCode::KEY_ENTER, NamedKey::Enter, Code::Enter),
        (OH_KeyCode::KEY_DEL, NamedKey::Backspace, Code::Backspace),
        (OH_KeyCode::KEY_DPAD_UP, NamedKey::ArrowUp, Code::ArrowUp),
        (
            OH_KeyCode::KEY_DPAD_DOWN,
            NamedKey::ArrowDown,
            Code::ArrowDown,
        ),
        (
            OH_KeyCode::KEY_DPAD_LEFT,
            NamedKey::ArrowLeft,
            Code::ArrowLeft,
        ),
        (
            OH_KeyCode::KEY_DPAD_RIGHT,
            NamedKey::ArrowRight,
            Code::ArrowRight,
        ),
        (OH_KeyCode::KEY_ESCAPE, NamedKey::Escape, Code::Escape),
        (OH_KeyCode::KEY_MOVE_HOME, NamedKey::Home, Code::Home),
        (OH_KeyCode::KEY_MOVE_END, NamedKey::End, Code::End),
        (OH_KeyCode::KEY_F5, NamedKey::F5, Code::F5),
    ];
    for (key_code, named, code) in cases {
        let event = down(&mut converter, key_code, plain);
        assert_eq!(event.key, Key::Named(named));
        assert_eq!(event.code, code);
        up(&mut converter, key_code, plain);
    }
    let unknown = down(&mut converter, OH_NativeXComponent_KeyCode(9999), plain);
    assert_eq!(unknown.key, Key::Named(NamedKey::Unidentified));
    assert_eq!(unknown.code, Code::Unidentified);
}

#[test]
fn modifier_key_locations() {
    let mut converter = KeyEventConverter::new();
    let plain = ModifierState::default();
    assert_eq!(
        down(&mut converter, OH_KeyCode::KEY_SHIFT_LEFT, plain).location,
        Location::Left
    );
    assert_eq!(
        down(&mut converter, OH_KeyCode::KEY_SHIFT_RIGHT, plain).location,
        Location::Right
    );
    assert_eq!(
        down(&mut converter, OH_KeyCode::KEY_NUM_LOCK, plain).location,
        Location::Standard
    );
}

#[test]
fn meta_tracked_from_key_transitions() {
    let mut converter = KeyEventConverter::new();
    let plain = ModifierState::default();
    let meta_down = down(&mut converter, OH_KeyCode::KEY_META_LEFT, plain);
    assert_eq!(meta_down.key, Key::Named(NamedKey::Meta));
    assert_eq!(meta_down.code, Code::MetaLeft);
    assert!(meta_down.modifiers.contains(Modifiers::META));
    assert!(down(&mut converter, OH_KeyCode::KEY_A, plain)
        .modifiers
        .contains(Modifiers::META));
    up(&mut converter, OH_KeyCode::KEY_A, plain);
    let meta_up = up(&mut converter, OH_KeyCode::KEY_META_LEFT, plain);
    assert!(!meta_up.modifiers.contains(Modifiers::META));
    assert!(!down(&mut converter, OH_KeyCode::KEY_A, plain)
        .modifiers
        .contains(Modifiers::META));
}

#[test]
fn reported_meta_overrides_tracking() {
    let mut converter = KeyEventConverter::new();
    let plain = ModifierState::default();
    down(&mut converter, OH_KeyCode::KEY_META_LEFT, plain);
    let reported_off = ModifierState {
        meta: Some(false),
        ..plain
    };
    assert!(!down(&mut converter, OH_KeyCode::KEY_A, reported_off)
        .modifiers
        .contains(Modifiers::META));
}

#[test]
fn repeat_derived_from_held_keys() {
    let mut converter = KeyEventConverter::new();
    let plain = ModifierState::default();
    assert!(!down(&mut converter, OH_KeyCode::KEY_A, plain).repeat);
    assert!(down(&mut converter, OH_KeyCode::KEY_A, plain).repeat);
    assert!(!up(&mut converter, OH_KeyCode::KEY_A, plain).repeat);
    assert!(!down(&mut converter, OH_KeyCode::KEY_A, plain).repeat);
}

#[test]
fn space_types_a_character() {
    let mut converter = KeyEventConverter::new();
    let event = down(
        &mut converter,
        OH_KeyCode::KEY_SPACE,
        ModifierState::default(),
    );
    assert_eq!(event.key, character(" "));
    assert_eq!(event.code, Code::Space);
}

#[test]
fn unknown_action_returns_none() {
    let mut converter = KeyEventConverter::new();
    assert!(converter
        .convert(
            OH_KeyAction::OH_NATIVEXCOMPONENT_KEY_ACTION_UNKNOWN,
            OH_KeyCode::KEY_A,
            ModifierState::default(),
        )
        .is_none());
}

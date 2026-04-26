//! Shared keymap types and builder for text editing components.

use std::collections::HashMap;

use floem_editor_core::command::{EditCommand, MoveCommand};
use ui_events::keyboard::{Key, Modifiers, NamedKey};

pub const CURSOR_BLINK_INTERVAL_MS: u64 = 500;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Edit(EditCommand),
    Move(MoveCommand),
    SelectAll,
    Copy,
    Cut,
    Paste,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct KeyPress {
    pub key: Key,
    pub modifiers: Modifiers,
}

pub struct KeymapBuilder {
    keymaps: HashMap<KeyPress, Command>,
}

impl KeymapBuilder {
    pub fn new() -> Self { Self { keymaps: HashMap::new() } }
    pub fn with_common_bindings(mut self) -> Self {
        #[cfg(target_os = "macos")] let cmd_or_ctrl = Modifiers::META;
        #[cfg(not(target_os = "macos"))] let cmd_or_ctrl = Modifiers::CONTROL;

        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::default() }, Command::Move(MoveCommand::Left));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::default() }, Command::Move(MoveCommand::Right));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::ALT }, Command::Move(MoveCommand::WordBackward));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::ALT }, Command::Move(MoveCommand::WordForward));
        #[cfg(not(target_os = "macos"))]
        {
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::CONTROL }, Command::Move(MoveCommand::WordBackward));
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::CONTROL }, Command::Move(MoveCommand::WordForward));
        }
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::default() }, Command::Edit(EditCommand::DeleteBackward));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::default() }, Command::Edit(EditCommand::DeleteForward));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::ALT }, Command::Edit(EditCommand::DeleteWordBackward));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::ALT }, Command::Edit(EditCommand::DeleteWordForward));
        #[cfg(not(target_os = "macos"))]
        {
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::CONTROL }, Command::Edit(EditCommand::DeleteWordBackward));
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::CONTROL }, Command::Edit(EditCommand::DeleteWordForward));
        }
        #[cfg(target_os = "macos")]
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::META }, Command::Edit(EditCommand::DeleteToBeginningOfLine));
        self.keymaps.insert(KeyPress { key: Key::Character("a".into()), modifiers: cmd_or_ctrl }, Command::SelectAll);
        self.keymaps.insert(KeyPress { key: Key::Character("c".into()), modifiers: cmd_or_ctrl }, Command::Copy);
        self.keymaps.insert(KeyPress { key: Key::Character("x".into()), modifiers: cmd_or_ctrl }, Command::Cut);
        self.keymaps.insert(KeyPress { key: Key::Character("v".into()), modifiers: cmd_or_ctrl }, Command::Paste);
        self
    }
    pub fn with_single_line_bindings(mut self) -> Self {
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Home), modifiers: Modifiers::default() }, Command::Move(MoveCommand::DocumentStart));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::End), modifiers: Modifiers::default() }, Command::Move(MoveCommand::DocumentEnd));
        #[cfg(target_os = "macos")]
        {
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::META }, Command::Move(MoveCommand::DocumentStart));
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::META }, Command::Move(MoveCommand::DocumentEnd));
        }
        self
    }
    pub fn with_multi_line_bindings(mut self) -> Self {
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowUp), modifiers: Modifiers::default() }, Command::Move(MoveCommand::Up));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowDown), modifiers: Modifiers::default() }, Command::Move(MoveCommand::Down));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Home), modifiers: Modifiers::default() }, Command::Move(MoveCommand::LineStart));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::End), modifiers: Modifiers::default() }, Command::Move(MoveCommand::LineEnd));
        #[cfg(target_os = "macos")]
        {
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::META }, Command::Move(MoveCommand::LineStart));
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::META }, Command::Move(MoveCommand::LineEnd));
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowUp), modifiers: Modifiers::META }, Command::Move(MoveCommand::DocumentStart));
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowDown), modifiers: Modifiers::META }, Command::Move(MoveCommand::DocumentEnd));
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Home), modifiers: Modifiers::CONTROL }, Command::Move(MoveCommand::DocumentStart));
            self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::End), modifiers: Modifiers::CONTROL }, Command::Move(MoveCommand::DocumentEnd));
        }
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Enter), modifiers: Modifiers::default() }, Command::Edit(EditCommand::InsertNewLine));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Tab), modifiers: Modifiers::default() }, Command::Edit(EditCommand::InsertTab));
        self
    }
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    pub fn with_emacs_bindings(mut self, multiline: bool) -> Self {
        self.keymaps.insert(KeyPress { key: Key::Character("h".into()), modifiers: Modifiers::CONTROL }, Command::Edit(EditCommand::DeleteBackward));
        self.keymaps.insert(KeyPress { key: Key::Character("d".into()), modifiers: Modifiers::CONTROL }, Command::Edit(EditCommand::DeleteForward));
        self.keymaps.insert(KeyPress { key: Key::Character("a".into()), modifiers: Modifiers::CONTROL }, Command::Move(if multiline { MoveCommand::LineStart } else { MoveCommand::DocumentStart }));
        self.keymaps.insert(KeyPress { key: Key::Character("e".into()), modifiers: Modifiers::CONTROL }, Command::Move(if multiline { MoveCommand::LineEnd } else { MoveCommand::DocumentEnd }));
        self.keymaps.insert(KeyPress { key: Key::Character("f".into()), modifiers: Modifiers::CONTROL }, Command::Move(MoveCommand::Right));
        self.keymaps.insert(KeyPress { key: Key::Character("b".into()), modifiers: Modifiers::CONTROL }, Command::Move(MoveCommand::Left));
        self.keymaps.insert(KeyPress { key: Key::Character("k".into()), modifiers: Modifiers::CONTROL }, Command::Edit(EditCommand::DeleteToEndOfLine));
        if multiline {
            self.keymaps.insert(KeyPress { key: Key::Character("n".into()), modifiers: Modifiers::CONTROL }, Command::Move(MoveCommand::Down));
            self.keymaps.insert(KeyPress { key: Key::Character("p".into()), modifiers: Modifiers::CONTROL }, Command::Move(MoveCommand::Up));
        }
        self
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    pub fn with_emacs_bindings(self, _multiline: bool) -> Self { self }
    pub fn build(self) -> Keymap { Keymap { keymaps: self.keymaps } }
}

impl Default for KeymapBuilder { fn default() -> Self { Self::new() } }

pub struct Keymap { pub keymaps: HashMap<KeyPress, Command> }

impl Keymap {
    pub fn single_line() -> Self {
        KeymapBuilder::new().with_common_bindings().with_single_line_bindings().with_emacs_bindings(false).build()
    }
    pub fn multi_line() -> Self {
        KeymapBuilder::new().with_common_bindings().with_multi_line_bindings().with_emacs_bindings(true).build()
    }
    pub fn chat_mode() -> Self {
        let mut keymap = KeymapBuilder::new().with_common_bindings().with_multi_line_bindings().with_emacs_bindings(true).build();
        keymap.keymaps.remove(&KeyPress { key: Key::Named(NamedKey::Enter), modifiers: Modifiers::default() });
        keymap.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Enter), modifiers: Modifiers::SHIFT }, Command::Edit(EditCommand::InsertNewLine));
        keymap
    }
    pub fn get(&self, key: &Key, modifiers: &Modifiers) -> Option<&Command> {
        let keypress = KeyPress { key: key.clone(), modifiers: *modifiers };
        self.keymaps.get(&keypress).or_else(|| {
            let mut modified = keypress.clone();
            modified.modifiers.set(Modifiers::SHIFT, false);
            self.keymaps.get(&modified)
        })
    }
}

//! Shared keymap types and builder for text editing components.
//!
//! This module provides common types and utilities used by both TextInput and TextArea.

use std::collections::HashMap;

use floem_editor_core::command::{EditCommand, MoveCommand};
use ui_events::keyboard::{Key, Modifiers, NamedKey};

/// Cursor blink interval in milliseconds
pub const CURSOR_BLINK_INTERVAL_MS: u64 = 500;

/// A command that can be executed on a text editor
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Edit(EditCommand),
    Move(MoveCommand),
    /// Select all text
    SelectAll,
    /// Copy selected text to clipboard
    Copy,
    /// Cut selected text to clipboard
    Cut,
    /// Paste text from clipboard
    Paste,
}

/// A key press with modifiers
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct KeyPress {
    pub key: Key,
    pub modifiers: Modifiers,
}

/// Builder for creating keymaps with various binding sets.
pub struct KeymapBuilder {
    keymaps: HashMap<KeyPress, Command>,
}

impl KeymapBuilder {
    /// Create a new empty keymap builder.
    pub fn new() -> Self {
        Self {
            keymaps: HashMap::new(),
        }
    }

    /// Add common bindings shared by all text editors.
    pub fn with_common_bindings(mut self) -> Self {
        #[cfg(target_os = "macos")]
        let cmd_or_ctrl = Modifiers::META;
        #[cfg(not(target_os = "macos"))]
        let cmd_or_ctrl = Modifiers::CONTROL;

        // Basic navigation
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::default() }, Command::Move(MoveCommand::Left));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::default() }, Command::Move(MoveCommand::Right));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::ALT }, Command::Move(MoveCommand::WordBackward));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::ALT }, Command::Move(MoveCommand::WordForward));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::default() }, Command::Edit(EditCommand::DeleteBackward));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::default() }, Command::Edit(EditCommand::DeleteForward));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::ALT }, Command::Edit(EditCommand::DeleteWordBackward));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::ALT }, Command::Edit(EditCommand::DeleteWordForward));
        self.keymaps.insert(KeyPress { key: Key::Character("a".into()), modifiers: cmd_or_ctrl }, Command::SelectAll);
        self.keymaps.insert(KeyPress { key: Key::Character("c".into()), modifiers: cmd_or_ctrl }, Command::Copy);
        self.keymaps.insert(KeyPress { key: Key::Character("x".into()), modifiers: cmd_or_ctrl }, Command::Cut);
        self.keymaps.insert(KeyPress { key: Key::Character("v".into()), modifiers: cmd_or_ctrl }, Command::Paste);
        self
    }

    pub fn with_single_line_bindings(mut self) -> Self {
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::Home), modifiers: Modifiers::default() }, Command::Move(MoveCommand::DocumentStart));
        self.keymaps.insert(KeyPress { key: Key::Named(NamedKey::End), modifiers: Modifiers::default() }, Command::Move(MoveCommand::DocumentEnd));
        self
    }

    pub fn with_multi_line_bindings(mut self) -> Self { self }
    pub fn with_emacs_bindings(self, _ml: bool) -> Self { self }
    pub fn build(self) -> Keymap { Keymap { keymaps: self.keymaps } }
}

impl Default for KeymapBuilder { fn default() -> Self { Self::new() } }

pub struct Keymap { pub keymaps: HashMap<KeyPress, Command> }

impl Keymap {
    pub fn single_line() -> Self {
        KeymapBuilder::new().with_common_bindings().with_single_line_bindings().with_emacs_bindings(false).build()
    }
    pub fn multi_line() -> Self { todo!() }
    pub fn chat_mode() -> Self { todo!() }
    pub fn get(&self, key: &Key, mods: &Modifiers) -> Option<&Command> {
        self.keymaps.get(&KeyPress { key: key.clone(), modifiers: *mods })
    }
}

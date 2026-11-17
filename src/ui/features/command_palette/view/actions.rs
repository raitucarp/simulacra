//! Keyboard actions of the palette (bound by the central keymap in its search field).

use gpui_kit::actions;

actions!(command_palette, [SelectPrevious, SelectNext, ConfirmSelection, DismissPalette, NextGroup, PreviousGroup]);

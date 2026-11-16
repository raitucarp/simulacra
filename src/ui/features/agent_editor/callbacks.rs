//! Callback type aliases for the agent editor and their shared (`Rc`) bundle.

use super::config_tab::{AgentConfigTab, LabMode};
use crate::domain::{Profile, ProfileId};
use gpui_kit::*;
use std::rc::Rc;

pub type SelectProfileCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type SelectConfigTabCallback = Box<dyn Fn(AgentConfigTab, &mut Window, &mut App) + 'static>;
pub type SelectSubTabCallback = Box<dyn Fn(usize, &mut Window, &mut App) + 'static>;
pub type SwitchLabModeCallback = Box<dyn Fn(LabMode, &mut Window, &mut App) + 'static>;
pub type UpdateProfileCallback = Box<dyn Fn(Profile, &mut Window, &mut App) + 'static>;
pub type SaveTextInputsCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type CreateProfileCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type DeleteProfileCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type PickAvatarCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type ActivatePersonaCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type TriggerAgentSimCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;

/// All workspace callbacks wrapped in `Rc` so many child elements can share them.
#[derive(Clone)]
pub(crate) struct SharedCallbacks {
    pub select_profile: Rc<SelectProfileCallback>,
    pub select_tab: Rc<SelectConfigTabCallback>,
    pub select_sub_tab: Rc<SelectSubTabCallback>,
    pub switch_mode: Rc<SwitchLabModeCallback>,
    pub update_profile: Rc<UpdateProfileCallback>,
    pub save_text: Rc<SaveTextInputsCallback>,
    pub create: Rc<CreateProfileCallback>,
    pub delete: Rc<DeleteProfileCallback>,
    pub pick_avatar: Rc<PickAvatarCallback>,
    pub activate_persona: Rc<ActivatePersonaCallback>,
    pub trigger_agent: Rc<TriggerAgentSimCallback>,
}

//! The network switcher's menu (M1): every network with its documents and agents, the
//! active one ticked, then "Create network". It is a kit popup menu (arrow keys, Enter,
//! Escape, scrolling) the shell opens from the switcher or the "Switch network" command.

mod consts;

pub use consts::MENU_ANCHOR_ID;

use crate::domain::{Network, NetworkId};
use crate::i18n::{t_count, COUNT_SEPARATOR};
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{MenuItem, MenuSection, MenuSpec, MenuText};
use gpui_kit::{App, Pixels, Window};
use std::rc::Rc;

pub type SelectNetworkCallback = Rc<dyn Fn(NetworkId, &mut Window, &mut App) + 'static>;
pub type CreateNetworkCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// Localized "N docs · M agents" caption for a network.
fn docs_and_agents_caption(network: &Network) -> String {
    [t_count("sidebar.doc_count", network.doc_count), t_count("sidebar.member_count", network.agent_count)].join(COUNT_SEPARATOR)
}

/// The network menu, `active` ticked, at least `width` wide.
pub fn network_menu_spec(networks: &[Network], active: Option<&NetworkId>, width: Pixels, on_select: &SelectNetworkCallback, on_create: &CreateNetworkCallback) -> MenuSpec {
    let items = networks.iter().map(|network| {
        let (on_select, id) = (on_select.clone(), network.id.clone());
        MenuItem::new("network.select", MenuText::plain(network.name.clone()))
            .detail(MenuText::plain(docs_and_agents_caption(network)))
            .checked(Some(&network.id) == active)
            .on_click(move |window, cx| on_select(id.clone(), window, cx))
    });
    let on_create = on_create.clone();
    let create = MenuItem::new("network.create", MenuText::key("workspace.create_network")).icon(LucideIcon::Plus).on_click(move |window, cx| on_create(window, cx));
    MenuSpec::new().section(MenuSection::labelled("workspace.switch_network").entries(items)).section(MenuSection::new().entry(create)).min_width(width).scrollable()
}

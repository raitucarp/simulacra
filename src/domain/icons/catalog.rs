//! The icon sets of the pinned package (S12/S13): what Settings › Symbols lists before
//! anything is downloaded. Each set shows a neutral style label, its licence, its icon count
//! and size; the project's name and links are attribution, shown only on the Licences
//! screen (decision #19).

/// How a set's icons look; the label Settings › Symbols shows (`symbols.style.<key>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetStyle {
    Line,
    Outline,
    Solid,
    Rounded,
    Colour,
    Logos,
    Developer,
    Game,
    Weather,
}

impl SetStyle {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Outline => "outline",
            Self::Solid => "solid",
            Self::Rounded => "rounded",
            Self::Colour => "colour",
            Self::Logos => "logos",
            Self::Developer => "developer",
            Self::Game => "game",
            Self::Weather => "weather",
        }
    }
}

/// One icon set of the package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconSet {
    /// The set's folder in the package and in the store (`sym:<id>/<name>`).
    pub id: &'static str,
    pub style: SetStyle,
    pub icons: u32,
    /// The size of the set's module in the package, about what the set takes once stored.
    pub bytes: u64,
    /// The licence's SPDX identifier.
    pub licence: &'static str,
    /// Attribution (Licences screen only): the project's name, site and licence text.
    pub project: &'static str,
    pub project_url: &'static str,
    pub licence_url: &'static str,
}

macro_rules! sets {
    ($($id:literal $style:ident $icons:literal $bytes:literal $licence:literal $project:literal $site:literal $text:literal;)+) => {
        /// Every set of the pinned package, in the package's manifest order.
        pub const ICON_SETS: &[IconSet] = &[$(IconSet {
            id: $id, style: SetStyle::$style, icons: $icons, bytes: $bytes, licence: $licence,
            project: $project, project_url: $site, licence_url: $text,
        },)+];
    };
}

sets! {
    "ci" Outline 288 248300 "MPL-2.0" "Circum Icons" "https://circumicons.com/" "https://github.com/Klarr-Agency/Circum-Icons/blob/main/LICENSE";
    "fa" Solid 1611 1365071 "CC-BY-4.0" "Font Awesome 5" "https://fontawesome.com/" "https://creativecommons.org/licenses/by/4.0/";
    "fa6" Solid 2058 1723849 "CC-BY-4.0" "Font Awesome 6" "https://fontawesome.com/" "https://creativecommons.org/licenses/by/4.0/";
    "io" Outline 696 479051 "MIT" "Ionicons 4" "https://ionicons.com/" "https://github.com/ionic-team/ionicons/blob/master/LICENSE";
    "io5" Outline 1332 849233 "MIT" "Ionicons 5" "https://ionicons.com/" "https://github.com/ionic-team/ionicons/blob/master/LICENSE";
    "md" Solid 4341 2141152 "Apache-2.0" "Material Design icons" "http://google.github.io/material-design-icons/" "https://github.com/google/material-design-icons/blob/master/LICENSE";
    "ti" Rounded 336 269668 "CC-BY-SA-3.0" "Typicons" "http://s-ings.com/typicons/" "https://creativecommons.org/licenses/by-sa/3.0/";
    "go" Developer 264 174570 "MIT" "Github Octicons icons" "https://octicons.github.com/" "https://github.com/primer/octicons/blob/master/LICENSE";
    "fi" Line 287 131120 "MIT" "Feather" "https://feathericons.com/" "https://github.com/feathericons/feather/blob/master/LICENSE";
    "lu" Line 1541 793072 "ISC" "Lucide" "https://lucide.dev/" "https://github.com/lucide-icons/lucide/blob/main/LICENSE";
    "gi" Game 4040 6945096 "CC-BY-3.0" "Game Icons" "https://game-icons.net/" "https://creativecommons.org/licenses/by/3.0/";
    "wi" Weather 219 414117 "OFL-1.1" "Weather Icons" "https://erikflowers.github.io/weather-icons/" "http://scripts.sil.org/OFL";
    "di" Logos 192 534700 "MIT" "Devicons" "https://vorillaz.github.io/devicons/" "https://opensource.org/licenses/MIT";
    "ai" Outline 831 734197 "MIT" "Ant Design Icons" "https://github.com/ant-design/ant-design-icons" "https://opensource.org/licenses/MIT";
    "bs" Solid 2754 1721102 "MIT" "Bootstrap Icons" "https://github.com/twbs/icons" "https://opensource.org/licenses/MIT";
    "ri" Line 3229 2248738 "Apache-2.0" "Remix Icon" "https://github.com/Remix-Design/RemixIcon" "http://www.apache.org/licenses/";
    "fc" Colour 329 318435 "MIT" "Flat Color Icons" "https://github.com/icons8/flat-color-icons" "https://opensource.org/licenses/MIT";
    "gr" Line 637 504870 "Apache-2.0" "Grommet-Icons" "https://github.com/grommet/grommet-icons" "http://www.apache.org/licenses/";
    "hi" Outline 460 201699 "MIT" "Heroicons" "https://github.com/tailwindlabs/heroicons" "https://opensource.org/licenses/MIT";
    "hi2" Outline 972 611784 "MIT" "Heroicons 2" "https://github.com/tailwindlabs/heroicons" "https://opensource.org/licenses/MIT";
    "si" Logos 3446 5196016 "CC0-1.0" "Simple Icons" "https://simpleicons.org/" "https://creativecommons.org/publicdomain/zero/1.0/";
    "sl" Line 189 212432 "MIT" "Simple Line Icons" "https://thesabbir.github.io/simple-line-icons/" "https://github.com/thesabbir/simple-line-icons/blob/master/LICENSE.md";
    "im" Solid 491 344185 "CC-BY-4.0" "IcoMoon Free" "https://github.com/Keyamoon/IcoMoon-Free" "https://github.com/Keyamoon/IcoMoon-Free/blob/master/License.txt";
    "bi" Solid 1634 839919 "MIT" "BoxIcons" "https://github.com/atisawd/boxicons" "https://github.com/atisawd/boxicons/blob/master/LICENSE";
    "cg" Line 704 498962 "MIT" "css.gg" "https://github.com/astrit/css.gg" "https://opensource.org/licenses/MIT";
    "vsc" Developer 612 766303 "CC-BY-4.0" "VS Code Icons" "https://github.com/microsoft/vscode-codicons" "https://creativecommons.org/licenses/by/4.0/";
    "tb" Line 6146 3522067 "MIT" "Tabler Icons" "https://github.com/tabler/tabler-icons" "https://opensource.org/licenses/MIT";
    "tfi" Line 352 238079 "MIT" "Themify Icons" "https://github.com/lykmapipo/themify-icons" "https://github.com/thecreation/standard-icons/blob/master/modules/themify-icons/LICENSE";
    "rx" Line 332 432664 "MIT" "Radix Icons" "https://icons.radix-ui.com" "https://github.com/radix-ui/icons/blob/master/LICENSE";
    "pi" Line 9072 5663588 "MIT" "Phosphor Icons" "https://github.com/phosphor-icons/core" "https://github.com/phosphor-icons/core/blob/main/LICENSE";
    "lia" Line 1544 2163831 "MIT" "Icons8 Line Awesome" "https://icons8.com/line-awesome" "https://github.com/icons8/line-awesome/blob/master/LICENSE.md";
}

/// The set with `id`.
pub fn icon_set(id: &str) -> Option<&'static IconSet> {
    ICON_SETS.iter().find(|set| set.id == id)
}

/// The set's number in Settings › Symbols (1-based, manifest order), part of its label.
pub fn set_number(id: &str) -> Option<usize> {
    ICON_SETS.iter().position(|set| set.id == id).map(|index| index + 1)
}

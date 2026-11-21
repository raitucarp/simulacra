# Third-party notices

## Theme palettes (`assets/themes/*.json`)

The bundled theme palettes adapt the published color values of the following open-source
color schemes. Only colors were taken (no code, fonts or artwork); each palette was reduced
to Simulacra's fourteen base colors and, where needed, its muted text was nudged to meet
WCAG AA contrast. The `credit` field of every JSON file names its upstream source.

| Palette(s) | Upstream project | Author / owner | License |
| --- | --- | --- | --- |
| Ayu Light, Ayu Mirage | [ayu-theme](https://github.com/ayu-theme/ayu-colors) | Ayu Theme contributors | MIT |
| Catppuccin Latte, Frappé, Macchiato, Mocha | [catppuccin](https://github.com/catppuccin/catppuccin) | Catppuccin contributors | MIT |
| Dracula | [Dracula Theme](https://github.com/dracula/dracula-theme) | Zeno Rocha | MIT |
| Everforest | [everforest](https://github.com/sainnhe/everforest) | sainnhe | MIT |
| Graphite Dark, Graphite Light (adapted from GitHub Dark and Light) | [primer/github-vscode-theme](https://github.com/primer/github-vscode-theme) | GitHub, Inc. | MIT |
| Gruvbox Dark, Gruvbox Light | [gruvbox](https://github.com/morhetz/gruvbox) | Pavel Pertsev | MIT |
| Kanagawa | [kanagawa.nvim](https://github.com/rebelot/kanagawa.nvim) | rebelot | MIT |
| Monokai | Monokai color scheme | Wimer Hazenberg | Color values only (widely redistributed palette) |
| Night Owl | [night-owl-vscode-theme](https://github.com/sdras/night-owl-vscode-theme) | Sarah Drasner | MIT |
| Nord | [nord](https://github.com/nordtheme/nord) | Arctic Ice Studio & Sven Greb | MIT |
| One Dark, One Light | [Atom one-dark / one-light syntax](https://github.com/atom/atom) | GitHub, Inc. (Atom) | MIT |
| Rosé Pine, Rosé Pine Dawn | [rose-pine](https://github.com/rose-pine/rose-pine-theme) | Rosé Pine contributors | MIT |
| Solarized Dark, Solarized Light | [solarized](https://github.com/altercation/solarized) | Ethan Schoonover | MIT |
| Synthwave '84 | [synthwave-vscode](https://github.com/robb0wen/synthwave-vscode) | Robb Owen | MIT |
| Tokyo Night, Tokyo Night Day | [tokyo-night-vscode-theme](https://github.com/enkia/tokyo-night-vscode-theme) | enkia | MIT |

Simulacra Midnight, Volcanic Zinc and Midnight Slate are original Simulacra palettes.

The MIT License requires that the copyright notice and permission notice be included in
all copies or substantial portions of the software. For each MIT-licensed project above:

> Permission is hereby granted, free of charge, to any person obtaining a copy of this
> software and associated documentation files (the "Software"), to deal in the Software
> without restriction, including without limitation the rights to use, copy, modify,
> merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
> permit persons to whom the Software is furnished to do so, subject to the following
> conditions:
>
> The above copyright notice and this permission notice shall be included in all copies
> or substantial portions of the Software.
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
> CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR
> THE USE OR OTHER DEALINGS IN THE SOFTWARE.

## Emoji data (`src/domain/emoji/table.rs`)

The emoji names, groups, versions and skin-tone variants of the reaction picker are
generated from `emoji-test.txt`, **Unicode emoji version 17.0** (dated 2025-08-04, part of
Unicode Technical Standard #51), by the `emoji-table` tool (`src/bin/emoji_table.rs`, see
`src/domain/emoji/generate`). The input was the copy of that file shipped inside the
`unicode-width` 0.2.2 crate's test data; nothing was downloaded. Only data was taken (code
points, names, group and subgroup names, emoji versions); the table is regenerated with
`cargo run --features codegen --bin emoji-table -- <path to emoji-test.txt>`.

The data is © 2025 Unicode, Inc. and is used under the Unicode License v3:

> UNICODE LICENSE V3
>
> COPYRIGHT AND PERMISSION NOTICE
>
> Copyright © 1991-2025 Unicode, Inc.
>
> NOTICE TO USER: Carefully read the following legal agreement. BY DOWNLOADING, INSTALLING,
> COPYING OR OTHERWISE USING DATA FILES, AND/OR SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE
> TO BE BOUND BY, ALL OF THE TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO
> NOT DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.
>
> Permission is hereby granted, free of charge, to any person obtaining a copy of data files
> and any associated documentation (the "Data Files") or software and any associated
> documentation (the "Software") to deal in the Data Files or Software without restriction,
> including without limitation the rights to use, copy, modify, merge, publish, distribute,
> and/or sell copies of the Data Files or Software, and to permit persons to whom the Data
> Files or Software are furnished to do so, provided that either (a) this copyright and
> permission notice appear with all copies of the Data Files or Software, or (b) this
> copyright and permission notice appear in associated Documentation.
>
> THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
>
> IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE BE LIABLE FOR ANY
> CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES WHATSOEVER RESULTING
> FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
> TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
> FILES OR SOFTWARE.
>
> Except as contained in this notice, the name of a copyright holder shall not be used in
> advertising or otherwise to promote the sale, use or other dealings in these Data Files or
> Software without prior written authorization of the copyright holder.
>
> SPDX-License-Identifier: Unicode-3.0

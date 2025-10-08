0.2.10 - 08 Oct 2025
===================
- feat: Support v1alpha server reflection
- fix: Show correct wireman version with `--version`

0.2.9 - 04 Oct 2025
===================
- feat: Support bracketed paste (ctrl+y or cmd+y). See https://github.com/preiter93/wireman/issues/31

0.2.8 - 15 Aug 2025
===================
- feat: show history tabs on headers page
- ui: style help dialog with rounded borders

0.2.7 - 26 Mar 2025
===================
- feat: Wayland clipboard compatibility (@ChausseBenjamin)
- feat: Add keymapping to yank request/response

0.2.6 - 21 Jan 2025
===================
- feat: Add `--config` flag to allow user specifying a custom config. This is helpful if you have different projects. You could define a config value in each project.
- feat: Add `--local-protos` flag to use local protos instead of the values from the config map.

0.2.5 - 11 Jan 2025
===================
- fix: Do not allow unselect fully on headers page
- refac: Use once instead of once_cell for clibpboard
- chore: Update README
- chore: Update dependencies

0.2.4 - 01 Jan 2025
===================
- Complete UI rework

0.2.3 - 24 Dec 2024
===================
- Server side Streaming

0.2.2 - 23 Dec 2024
===================
- Switch auth headers with L/H
- Allow color parameters in skin configuration
- Support inline editing of config file
- Add a placeholder text if there are no proto services

0.2.1 - 23 Nov 2024
===================
- Add a setup cli

0.2.0 - 21 Okt 2024
===================
- Bump ratatui version (v0.29)

0.1.5 - 07 Sep 2024
===================
- Resize request and response windows with `+`/`-`

0.1.4 - 07 Sep 2024
===================
- Bump edtui to v0.8: Supports line wrapping
- Change default theme background color

0.1.3 - 15 Sep 2024
===================
- Support server reflection. Type ctrl+r on the selection page.
- Display search query in editor status line

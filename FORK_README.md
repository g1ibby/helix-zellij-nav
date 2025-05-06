# Helix Editor with Zellij Navigation Integration

This fork of the Helix editor adds seamless navigation between Helix windows and Zellij panes/tabs, creating a unified navigation experience across your terminal environment.

## Features

This fork adds four new commands that extend Helix's window navigation to seamlessly integrate with Zellij:

| Command                | Description                               |
|------------------------|-------------------------------------------|
| `jump_view_right_zj`   | Jump to right split or Zellij pane/tab    |
| `jump_view_left_zj`    | Jump to left split or Zellij pane/tab     |
| `jump_view_up_zj`      | Jump to split above or Zellij pane        |
| `jump_view_down_zj`    | Jump to split below or Zellij pane        |

These commands first attempt to navigate within Helix splits. If no split exists in the requested direction, the navigation command is passed to Zellij, allowing for seamless movement between editor windows and terminal panes.

## How It Works

When you reach the edge of your Helix windows and try to navigate further:

*   Standard Helix commands stop at the boundary.
*   The Zellij-enhanced commands (`*_zj`) will continue the navigation into Zellij panes and tabs.

## Files Modified

This fork makes minimal changes to the Helix codebase:

*   Modified: `helix-term/src/commands.rs` - Added command registrations
*   Added: `helix-term/src/commands/zellij.rs` - New module containing Zellij integration code

## Installation

To install this fork:
```bash
# Make sure you've removed any existing Helix installation

# Clone the repository
git clone https://github.com/g1ibby/helix-zellij-nav.git
cd helix-zellij-nav

# Build with optimizations
cargo build --profile opt --locked

# Install (adjust paths as needed for your system)
mkdir -p /usr/local/lib/helix
cp -r runtime /usr/local/lib/helix/
cp target/opt/hx /usr/local/bin/hx
```
## Configuration

### Helix Configuration

Add these key bindings to your `~/.config/helix/config.toml`:
```toml
[keys.normal]
A-up = "jump_view_up_zj"
A-down = "jump_view_down_zj"
A-left = "jump_view_left_zj"
A-right = "jump_view_right_zj"
```

This maps Alt+Arrow keys to the Zellij-integrated navigation commands.

### Recommended Zellij Configuration

For the best experience, we recommend using the `zellij-autolock` plugin to prevent accidental keypresses from affecting Zellij when working in Helix.

Add this to your `~/.config/zellij/config.kdl`:
```kdl
plugins {
    autolock location="file:~/.config/zellij/plugins/zellij-autolock.wasm" {
        // Enabled at start?
        is_enabled true
        // Lock when any of these programs open
        triggers "nvim|git|fzf|zoxide|atuin|hx|lazygit"
        // Reaction to input occurs after this many seconds (default=0.3)
        // (An existing scheduled reaction prevents additional reactions)
        reaction_seconds "0.5"
        // Print to Zellij log? (default=false)
        print_to_log true
    }
}
// Load this "headless" plugin on start
load_plugins {
    autolock
}
```

You'll need to install the autolock plugin first:
```bash
mkdir -p ~/.config/zellij/plugins
curl -L https://github.com/fresh2dev/zellij-autolock/releases/latest/download/zellij-autolock.wasm -o ~/.config/zellij/plugins/zellij-autolock.wasm
```
## Usage

*   Start Zellij: `zellij`
*   Open Helix in a Zellij pane: `hx`
*   Create some splits in Helix and some panes in Zellij.
*   Use your configured keybindings (e.g., Alt+Arrow keys) to navigate seamlessly between all windows.

When you reach the edge of your Helix environment, navigation will automatically continue to Zellij panes and tabs.
## Maintenance Strategy

This fork follows a specific maintenance strategy to stay in sync with upstream Helix:

*   The `main` branch tracks the upstream Helix repository.
*   The `zellij-integration` branch contains our Zellij navigation feature.
*   Regular updates from upstream are merged into `master` and then rebased into `zellij-integration`.

This ensures that you always have access to the latest Helix features and bug fixes while maintaining the Zellij integration.

## Contributing

Contributions are welcome! If you have ideas for improving the Zellij integration or encounter any issues, please open an issue or submit a pull request.

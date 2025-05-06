use super::*;
use crate::commands::Context;
use helix_view::tree;

fn jump_view_with_zellij(cx: &mut Context, direction: tree::Direction) {
    let current_view = cx.editor.tree.focus;
    if let Some(id) = cx
        .editor
        .tree
        .find_split_in_direction(current_view, direction)
    {
        cx.editor.focus(id)
    } else {
        let shell = cx.editor.config().shell.clone();
        let cmd = match direction {
            tree::Direction::Right => "zellij action move-focus-or-tab right",
            tree::Direction::Left => "zellij action move-focus-or-tab left",
            tree::Direction::Up => "zellij action move-focus up",
            tree::Direction::Down => "zellij action move-focus down",
        };

        if let Err(err) = shell_impl(&shell, cmd, None) {
            log::debug!("Shell command failed: {}", err);
        }
    }
}

// Command functions that will be referenced in the command registry
pub fn jump_view_right_zj(cx: &mut Context) {
    jump_view_with_zellij(cx, tree::Direction::Right)
}

pub fn jump_view_left_zj(cx: &mut Context) {
    jump_view_with_zellij(cx, tree::Direction::Left)
}

pub fn jump_view_up_zj(cx: &mut Context) {
    jump_view_with_zellij(cx, tree::Direction::Up)
}

pub fn jump_view_down_zj(cx: &mut Context) {
    jump_view_with_zellij(cx, tree::Direction::Down)
}

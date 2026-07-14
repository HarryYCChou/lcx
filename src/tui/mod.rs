//! Interactive terminal UI. `lcx` with no subcommand lands here.
//!
//! - `home`: the unified main screen — action menu + profile on the left, the
//!   problem search/list on the right. Opening a problem launches the solve view.
//! - `solve`: two-pane description + code preview to edit, test, and submit.
//!
//! The orchestrator (`run`) sets up the terminal once and hands control to the
//! home screen, which runs until the user quits.

pub mod home;
pub mod login;
pub mod markup;
pub mod solve;

use std::io::{self, Stdout};

use anyhow::{Context, Result};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Terminal;

use crate::cache::Cache;
use crate::client::LeetCodeClient;
use crate::config::Config;

pub(crate) type Backend = CrosstermBackend<Stdout>;

/// Entry point for the interactive TUI.
pub fn run(cfg: Config, client: LeetCodeClient, cache: Cache) -> Result<()> {
    let mut terminal = enter()?;
    let result = main_loop(&mut terminal, cfg, client, cache);
    leave(&mut terminal)?;
    result
}

fn main_loop(
    terminal: &mut Terminal<Backend>,
    mut cfg: Config,
    client: LeetCodeClient,
    cache: Cache,
) -> Result<()> {
    // Prompt for login at startup if there are no credentials or they fail to
    // verify. Cancelling here quits; F4 enters offline (cached) browsing.
    if needs_login(&cfg, &client) {
        match login::run(terminal, &cfg, true)? {
            login::LoginOutcome::Saved { cfg: new_cfg, .. } => cfg = new_cfg,
            login::LoginOutcome::Offline => {}
            login::LoginOutcome::Cancelled => return Ok(()),
        }
    }
    drop(client); // the home screen rebuilds clients from `cfg` on demand

    home::run(terminal, &mut cfg, cache)
}

/// Whether we should show the login modal: no saved credentials, or saved ones
/// that don't verify against LeetCode.
fn needs_login(cfg: &Config, client: &LeetCodeClient) -> bool {
    if !cfg.is_authenticated() {
        return true;
    }
    !matches!(block_on(client.whoami()), Ok(Some(_)))
}

/// Drive a network future to completion from inside the synchronous event loop.
/// Requires the multi-threaded tokio runtime (see `main`).
pub(crate) fn block_on<F: std::future::Future>(fut: F) -> F::Output {
    tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(fut))
}

/// Enter raw mode + the alternate screen and build a terminal.
fn enter() -> Result<Terminal<Backend>> {
    enable_raw_mode().context("enabling raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("entering alternate screen")?;
    let mut terminal =
        Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal")?;
    terminal.clear().context("clearing terminal")?;
    Ok(terminal)
}

/// Leave the alternate screen and disable raw mode (also used to suspend before
/// shelling out to an external editor).
pub(crate) fn leave(terminal: &mut Terminal<Backend>) -> Result<()> {
    disable_raw_mode().context("disabling raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("leaving alternate screen")?;
    terminal.show_cursor().context("showing cursor")?;
    Ok(())
}

/// Re-enter raw mode + the alternate screen on an existing terminal (used after
/// shelling out to an external editor).
pub(crate) fn resume(terminal: &mut Terminal<Backend>) -> Result<()> {
    enable_raw_mode().context("re-enabling raw mode")?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)
        .context("re-entering alternate screen")?;
    terminal.clear().context("clearing terminal")?;
    Ok(())
}

/// Build a help line where every `<KEY>` token is highlighted with a background
/// color so the hotkeys stand out from the surrounding description text.
pub(crate) fn help_line(text: &str) -> Line<'static> {
    let key_style = Style::default()
        .fg(Color::Black)
        .bg(Color::Rgb(255, 161, 22))
        .add_modifier(Modifier::BOLD);

    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find('<') {
        match rest[start..].find('>') {
            Some(off) => {
                let end = start + off + 1;
                if start > 0 {
                    spans.push(Span::raw(rest[..start].to_string()));
                }
                spans.push(Span::styled(rest[start..end].to_string(), key_style));
                rest = &rest[end..];
            }
            None => break,
        }
    }
    if !rest.is_empty() {
        spans.push(Span::raw(rest.to_string()));
    }
    Line::from(spans)
}

/// Remove ANSI SGR escape sequences so colored output renders cleanly in the
/// TUI (the shared formatters emit terminal color codes).
pub(crate) fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for nc in chars.by_ref() {
                    if nc.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

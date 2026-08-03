use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

// file imports using crate
use crate::{
    app::{App, Focus, Screen},
    layout::{LayoutMode, mode_for},
    theme::Theme,
};

/// Height of the top bar and bottom bar in both compact and standard layouts.
const BAR_HEIGHT: u16 = 3;
/// Minimum height left for the body after the header and footer are drawn.
const MIN_BODY_HEIGHT: u16 = 10;

/// Draws one complete frame from the current application state.
///
/// This function is pure: it reads from `app` and uses ratatui layout math, but
/// it never performs I/O or introduces random values. That makes it safe to
/// render into a `TestBackend` for snapshot tests later.
pub fn render(frame: &mut Frame, app: &App) {
    // Choose the layout mode from the live terminal size. Caching the mode or
    // any rectangles inside `App` would make them stale after a resize.
    match mode_for(frame.area()) {
        LayoutMode::TooSmall => render_size_warning(frame, app.theme),
        LayoutMode::Compact => render_compact(frame, app),
        LayoutMode::Standard => render_standard(frame, app),
    }

    render_overlay(frame, app);
}

/// Renders a full-screen warning when the terminal is too small to play.
fn render_size_warning(frame: &mut Frame, theme: Theme) {
    let area = frame.area();

    let message = Text::from(vec![
        Line::from("Storyforge needs at least 80 columns x 24 rows."),
        Line::from(format!("Current size: {} x {}", area.width, area.height)),
        Line::from("Resize the terminal or press q to quit."),
    ]);

    let paragraph = Paragraph::new(message)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .block(
            Block::default()
                .title("Storyforge")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.danger)),
        );

    frame.render_widget(paragraph, area);
}

/// Compact layout for smaller terminals: header, combined story/actions area,
/// one selected detail tab, and footer.
fn render_compact(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let [header_area, body_area, footer_area] = vertical_main_layout(area);

    render_header(frame, header_area, app);
    render_compact_body(frame, body_area, app);
    render_footer(frame, footer_area, app);
}

/// Standard layout for full-size terminals: header, body split into a story
/// panel and a four-column detail panel, and footer.
fn render_standard(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let [header_area, body_area, footer_area] = vertical_main_layout(area);

    render_header(frame, header_area, app);
    render_standard_body(frame, body_area, app);
    render_footer(frame, footer_area, app);
}

/// Reserved hook for future modal overlays.
///
/// Kept as a no-op so the main `render` dispatch does not need to change when
/// overlays are added.
fn render_overlay(_frame: &mut Frame, _app: &App) {}

/// Splits the full frame into header, body, and footer.
///
/// The body gets the remaining space, but we enforce a minimum so tiny
/// terminals fall back to the `TooSmall` warning before reaching this code.
fn vertical_main_layout(area: Rect) -> [Rect; 3] {
    let parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(BAR_HEIGHT),
            Constraint::Min(MIN_BODY_HEIGHT),
            Constraint::Length(BAR_HEIGHT),
        ])
        .split(area);

    [parts[0], parts[1], parts[2]]
}

/// Top bar with the app title and keyboard hints.
fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;

    let hint = Line::from(vec![
        Span::styled("q/Esc", Style::default().fg(theme.focus)),
        Span::styled(" quit  ", Style::default().fg(theme.muted)),
        Span::styled("c/l/m/?", Style::default().fg(theme.focus)),
        Span::styled(" screens  ", Style::default().fg(theme.muted)),
        Span::styled("i", Style::default().fg(theme.focus)),
        Span::styled(" inv  ", Style::default().fg(theme.muted)),
        Span::styled("j/k", Style::default().fg(theme.focus)),
        Span::styled(" focus  ", Style::default().fg(theme.muted)),
        Span::styled("t", Style::default().fg(theme.focus)),
        Span::styled(" theme", Style::default().fg(theme.muted)),
    ]);

    let title = format!("Storyforge - {}", app.theme_id.name());

    let block = Block::default()
        .title(title.as_str())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .border_type(BorderType::Rounded);

    let header = Paragraph::new(hint)
        .alignment(Alignment::Left)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .block(block);

    frame.render_widget(header, area);
}

/// Bottom bar showing discrete spell-slot and sorcery-point counts.
fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let mut spans: Vec<Span> = Vec::new();

    // Build the compact spell-slot summary.
    spans.push(Span::styled("SLOTS ", Style::default().fg(theme.muted)));
    let mut any_slot = false;
    for level in 0..9 {
        let max = app.spell_slots_max[level];
        let temp = app.spell_slots_temp[level];
        if max == 0 && temp == 0 {
            continue;
        }

        any_slot = true;

        let current = app.spell_slots_current[level];
        spans.push(Span::styled(
            format!("{}:{current}/{max}", level + 1),
            Style::default().fg(if current == 0 {
                theme.danger
            } else {
                theme.text
            }),
        ));
        if temp > 0 {
            spans.push(Span::styled(
                format!("(+{temp})"),
                Style::default().fg(theme.success),
            ));
        }
        spans.push(Span::styled("  ", Style::default().fg(theme.muted)));
    }

    if !any_slot {
        spans.push(Span::styled("none", Style::default().fg(theme.muted)));
    }

    // Sorcery points only appear for characters that have the feature.
    if let Some((current, max)) = app.sorcery_points {
        spans.push(Span::styled(" | ", Style::default().fg(theme.muted)));
        spans.push(Span::styled("SP ", Style::default().fg(theme.muted)));
        spans.push(Span::styled(
            format!("{current}/{max}"),
            Style::default().fg(if current == 0 {
                theme.danger
            } else {
                theme.text
            }),
        ));
    }

    // Add a short control reminder after the spell resources.
    spans.push(Span::styled(" | ", Style::default().fg(theme.muted)));
    spans.push(Span::styled("j/k", Style::default().fg(theme.focus)));
    spans.push(Span::styled(" focus  ", Style::default().fg(theme.muted)));
    spans.push(Span::styled("w/a/s/d", Style::default().fg(theme.focus)));
    spans.push(Span::styled(" move  ", Style::default().fg(theme.muted)));
    spans.push(Span::styled("t", Style::default().fg(theme.focus)));
    spans.push(Span::styled(" theme", Style::default().fg(theme.muted)));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent));

    let footer = Paragraph::new(Line::from(spans))
        .style(Style::default().fg(theme.text).bg(theme.background))
        .block(block);

    frame.render_widget(footer, area);
}

/// Compact body: story on top, then the currently selected tab.
///
/// A small actions preview is rendered between them unless the selected tab
/// itself is already showing Actions, so the same content never appears twice.
fn render_compact_body(frame: &mut Frame, area: Rect, app: &App) {
    let actions_in_tab = app.selected_tab.is_multiple_of(4);

    // Resolve the main body area first. When the inventory sidebar is open it
    // takes a slice of `area` and we render the sidebar immediately.
    let main_area = if app.inventory_open {
        let [main_area, inventory_area] = inventory_split(area, mode_for(frame.area()));
        render_inventory_panel(frame, inventory_area, app);
        main_area
    } else {
        area
    };

    if actions_in_tab {
        let [story_area, tab_area]: [Rect; 2] =
            Layout::vertical([Constraint::Percentage(40), Constraint::Min(8)]).areas(main_area);

        render_story_panel(frame, story_area, app);
        render_selected_tab(frame, tab_area, app);
    } else {
        // `Layout::areas()` returns `[Rect; N]` through a const generic. The
        // compiler can't infer `N` from the constraints alone, so the type
        // annotation has to be explicit. Using an array here also gives a
        // compile-time guarantee that the layout produces exactly three regions.
        let [story_area, actions_area, tab_area]: [Rect; 3] = Layout::vertical([
            Constraint::Percentage(40),
            Constraint::Length(5),
            Constraint::Min(8),
        ])
        .areas(main_area);

        render_story_panel(frame, story_area, app);
        render_actions_panel(frame, actions_area, app);
        render_selected_tab(frame, tab_area, app);
    }
}

/// Standard body: large story panel on top, four-column detail grid below.
fn render_standard_body(frame: &mut Frame, area: Rect, app: &App) {
    if app.inventory_open {
        let [main_area, inventory_area] = inventory_split(area, mode_for(frame.area()));

        let [story_area, detail_area]: [Rect; 2] =
            Layout::vertical([Constraint::Percentage(45), Constraint::Percentage(55)])
                .areas(main_area);

        render_story_panel(frame, story_area, app);
        render_detail_grid(frame, detail_area, app);
        render_inventory_panel(frame, inventory_area, app);
    } else {
        let [story_area, detail_area]: [Rect; 2] =
            Layout::vertical([Constraint::Percentage(45), Constraint::Percentage(55)]).areas(area);

        render_story_panel(frame, story_area, app);
        render_detail_grid(frame, detail_area, app);
    }
}

/// Splits a body area into a main region and an inventory sidebar.
///
/// The sidebar width follows the requested ratios:
/// - Standard layout: one third of the width.
/// - Compact layout: one half of the width.
/// - Too-small layout: seven eighths of the width.
///
/// The too-small branch is a fallback. The `TooSmall` warning normally
/// renders instead of the body, so this branch is rarely visible.
fn inventory_split(area: Rect, mode: LayoutMode) -> [Rect; 2] {
    let inventory_width = match mode {
        LayoutMode::Standard => area.width / 3,
        LayoutMode::Compact => area.width / 2,
        LayoutMode::TooSmall => area.width * 7 / 8,
    };
    let main_width = area.width.saturating_sub(inventory_width);

    Layout::horizontal([
        Constraint::Length(main_width),
        Constraint::Length(inventory_width),
    ])
    .areas(area)
}

/// Renders the inventory sidebar with a temporary placeholder item list.
fn render_inventory_panel(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let focused = app.focus == Focus::Inventory;

    let mut lines = vec![String::new()];
    for (index, item) in app.inventory_items.iter().enumerate() {
        let marker = if index == app.inventory_selected {
            ">"
        } else {
            " "
        };
        lines.push(format!("{marker} {item}"));
    }
    lines.push(String::new());
    lines.push("[w/a/s/d] Move  [i] Close".to_owned());

    render_pane(frame, area, "Inventory", lines.join("\n"), theme, focused);
}

/// Main narrative panel. Shows placeholder text per screen until the story
/// content model is wired in.
fn render_story_panel(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let focused = app.focus == Focus::Story;

    let text = match app.screen {
        Screen::Story => "The academy doors stand open. What do you do?",
        Screen::Character => "Character sheet will appear here.",
        Screen::Journal => "Journal entries will appear here.",
        Screen::Map => "Map will appear here.",
        Screen::Help => "Help text will appear here.",
    };

    render_pane(frame, area, "Story", text, theme, focused);
}

/// Four-column detail grid used in standard mode.
fn render_detail_grid(frame: &mut Frame, area: Rect, app: &App) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    render_actions_panel(frame, columns[0], app);
    render_character_panel(frame, columns[1], app);
    render_quest_panel(frame, columns[2], app);
    render_log_panel(frame, columns[3], app);
}

/// Renders the actions pane.
///
/// When the Story screen is active this pane displays the currently available
/// story choices. The selected choice is read directly from the game engine's
/// state so rendering remains a pure operation.
///
/// For every other screen this pane falls back to showing the application's
/// keyboard shortcuts.
///
/// Note that this function never dispatches commands or mutates game state.
/// It simply reads the current state and renders it.
fn render_actions_panel(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let focused = app.focus == Focus::Actions;

    // While we're still wiring the content pipeline, the story choices are
    // hard-coded. A later guide will load these from the active scene.
    let text = if app.screen == Screen::Story {
        // Read the selected choice directly from the gameplay engine.
        let selected = app.engine.state().selected_choice;

        // Prefix the currently selected option with '>' so the player can see
        // which choice the keyboard is controlling.
        let first = if selected == 0 {
            "> Ask about the sealed gate."
        } else {
            "  Ask about the sealed gate."
        };

        let second = if selected == 1 {
            "> Look for another route."
        } else {
            "  Look for another route."
        };

        format!("\n{first}\n{second}\n\n[j/k] Choose  [Enter] Confirm")
    } else {
        "\n[c] Character\n[l] Journal\n[m] Map\n[?] Help\n[Esc] Back/Quit".to_owned()
    };

    render_pane(frame, area, "Actions", text, theme, focused);
}

/// Character summary pane. The full Magic tab will show all nine slot levels,
/// temporary slots, and long-rest recovery; this preview just lists the basics.
fn render_character_panel(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let focused = app.focus == Focus::Character;

    let mut lines = vec!["".into()];
    lines.push("Level 1 caster".into());
    lines.push("HP 22/22".into());
    lines.push("AC 13".into());
    lines.push("".into());
    lines.push(spell_resource_line(app, theme));

    render_pane(frame, area, "Character", Text::from(lines), theme, focused);
}

/// Quest tracker pane.
fn render_quest_panel(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let focused = app.focus == Focus::Quest;

    render_pane(
        frame,
        area,
        "Quests",
        "\n- Attend orientation\n- Meet your mentor\n- Explore the grounds",
        theme,
        focused,
    );
}

/// Recent event log pane.
fn render_log_panel(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let focused = app.focus == Focus::Log;

    render_pane(frame, area, "Log", "\nStoryforge is awake.", theme, focused);
}

/// Renders one compact tab based on the current selection.
fn render_selected_tab(frame: &mut Frame, area: Rect, app: &App) {
    match app.selected_tab % 4 {
        0 => render_actions_panel(frame, area, app),
        1 => render_character_panel(frame, area, app),
        2 => render_quest_panel(frame, area, app),
        _ => render_log_panel(frame, area, app),
    }
}

/// Shared helper for drawing a bordered pane with an optional focus highlight.
fn render_pane(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    content: impl Into<Text<'static>>,
    theme: Theme,
    focused: bool,
) {
    // Use the full palette when this panel is focused. Dim every color when it
    // is not, so the focused panel stands out.
    let theme = if focused { theme } else { theme.dim() };
    let border_color = if focused { theme.focus } else { theme.accent };

    let block = Block::default()
        .title(title)
        .title_style(Style::default().fg(border_color))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .block(block);

    frame.render_widget(paragraph, area);
}

/// Formats the spell-slot summary as discrete counts.
///
/// Used by both the compact footer and the character preview.
fn spell_resource_line(app: &App, theme: Theme) -> Line<'static> {
    let mut spans: Vec<Span> = vec![Span::styled("Slots ", Style::default().fg(theme.muted))];

    for level in 0..9 {
        let max = app.spell_slots_max[level];
        let temp = app.spell_slots_temp[level];
        if max == 0 && temp == 0 {
            continue;
        }

        let current = app.spell_slots_current[level];
        let ordinal = ordinal_name(level + 1);
        spans.push(Span::styled(
            format!("{ordinal} {current}/{max}"),
            Style::default().fg(if current == 0 {
                theme.danger
            } else {
                theme.text
            }),
        ));
        if temp > 0 {
            spans.push(Span::styled(
                format!("(+{temp})"),
                Style::default().fg(theme.success),
            ));
        }
        spans.push(Span::styled("  ", Style::default().fg(theme.muted)));
    }

    if let Some((current, max)) = app.sorcery_points {
        spans.push(Span::styled("SP ", Style::default().fg(theme.muted)));
        spans.push(Span::styled(
            format!("{current}/{max}"),
            Style::default().fg(if current == 0 {
                theme.danger
            } else {
                theme.text
            }),
        ));
    }

    Line::from(spans)
}

/// Short ordinal names for the nine spell-slot levels.
fn ordinal_name(n: usize) -> &'static str {
    match n {
        1 => "1st",
        2 => "2nd",
        3 => "3rd",
        4 => "4th",
        5 => "5th",
        6 => "6th",
        7 => "7th",
        8 => "8th",
        9 => "9th",
        _ => "?",
    }
}

use crate::api::{strip_html, time_ago};
use crate::app::{App, Screen};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
};

const ORANGE: Color = Color::Rgb(255, 102, 0);
const CREAM: Color = Color::Rgb(236, 228, 214);
const TITLE: Color = Color::Rgb(214, 208, 198);
const DIM: Color = Color::Rgb(154, 146, 134);
const FAINT: Color = Color::Rgb(96, 88, 78);
const RULE: Color = Color::Rgb(78, 56, 34);
const SELECT_BG: Color = Color::Rgb(48, 28, 10);

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::Story => draw_stories(frame, app),
        Screen::Comments => draw_comments(frame, app),
    }
}

fn draw_stories(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
        Constraint::Length(2),
    ])
    .split(area);

    let visible = (chunks[1].height as usize / 2).max(1);
    app.page_size = visible;

    let width = chunks[0].width as usize;
    let pos = if app.stories.is_empty() {
        0
    } else {
        app.selected_story + 1
    };
    let header = spread(
        vec![
            Span::styled(
                " HN ",
                Style::default()
                    .fg(Color::Black)
                    .bg(ORANGE)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(app.feed.label().to_lowercase(), Style::default().fg(CREAM)),
        ],
        vec![
            Span::styled(pos.to_string(), Style::default().fg(ORANGE)),
            Span::styled(
                format!(" / {} ", app.stories.len()),
                Style::default().fg(DIM),
            ),
        ],
        width,
    );
    frame.render_widget(ruled(Paragraph::new(header), Borders::BOTTOM), chunks[0]);

    if app.stories.is_empty() {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "  No stories loaded.",
                Style::default().fg(DIM),
            ))),
            chunks[1],
        );
    } else {
        let title_width = (chunks[1].width as usize).saturating_sub(7);
        let items: Vec<ListItem> = app
            .stories
            .iter()
            .enumerate()
            .skip(app.story_offset)
            .take(visible)
            .map(|(index, story)| story_row(index, story, index == app.selected_story, title_width))
            .collect();
        frame.render_widget(List::new(items), chunks[1]);
    }

    let can_open = app
        .stories
        .get(app.selected_story)
        .and_then(|story| story.url.as_ref())
        .is_some();
    frame.render_widget(
        ruled(
            Paragraph::new(hints(&[
                ("↑↓", "move", true),
                ("enter", "comments", true),
                ("o", "open", can_open),
                ("q", "quit", true),
            ])),
            Borders::TOP,
        ),
        chunks[2],
    );
}

fn story_row(
    index: usize,
    story: &crate::api::Story,
    selected: bool,
    title_width: usize,
) -> ListItem<'static> {
    let marker = if selected { "▸" } else { " " };
    let marker_style = Style::default().fg(if selected { ORANGE } else { FAINT });
    let rank_style = Style::default().fg(if selected { ORANGE } else { DIM });
    let title_style = if selected {
        Style::default().fg(CREAM).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TITLE)
    };

    let title = Line::from(vec![
        Span::styled(format!(" {marker} "), marker_style),
        Span::styled(format!("{:02}  ", index + 1), rank_style),
        Span::styled(truncate(&story.title, title_width), title_style),
    ]);

    let score = story.score.unwrap_or(0);
    let author = story.by.as_deref().unwrap_or("unknown");
    let comments = story.descendants.unwrap_or(0);
    let mut meta = format!(
        " pts · {author} · {} · {comments} comments",
        ago(story.time)
    );
    if let Some(url) = story.url.as_deref() {
        let host = domain(url);
        if !host.is_empty() {
            meta.push_str(" · ");
            meta.push_str(&host);
        }
    }
    let meta = Line::from(vec![
        Span::raw("       "),
        Span::styled(
            score.to_string(),
            Style::default().fg(if selected {
                ORANGE
            } else {
                Color::Rgb(196, 112, 48)
            }),
        ),
        Span::styled(
            truncate(&meta, title_width.saturating_sub(score.to_string().len())),
            Style::default().fg(DIM),
        ),
    ]);

    let mut item = ListItem::new(vec![title, meta]);
    if selected {
        item = item.style(Style::default().bg(SELECT_BG));
    }
    item
}

fn draw_comments(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(2),
    ])
    .split(frame.area());

    let story = app.stories.get(app.selected_story);
    let width = chunks[0].width as usize;
    let title = story
        .map(|story| truncate(&story.title, width.saturating_sub(4)))
        .unwrap_or_else(|| "comments".to_string());
    let meta = story
        .map(|story| {
            let mut parts = vec![
                format!("{} pts", story.score.unwrap_or(0)),
                story.by.clone().unwrap_or_else(|| "unknown".to_string()),
                ago(story.time),
                format!("{} comments", story.descendants.unwrap_or(0)),
            ];
            if let Some(url) = story.url.as_deref() {
                let host = domain(url);
                if !host.is_empty() {
                    parts.push(host);
                }
            }
            truncate(&parts.join(" · "), width.saturating_sub(4))
        })
        .unwrap_or_default();

    let header = Text::from(vec![
        Line::from(vec![
            Span::styled(
                " ‹ ",
                Style::default().fg(ORANGE).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                title,
                Style::default().fg(CREAM).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(format!("   {meta}"), Style::default().fg(DIM))),
    ]);
    frame.render_widget(ruled(Paragraph::new(header), Borders::BOTTOM), chunks[0]);

    let body = chunks[1];
    let lines = comment_lines(app, body.width as usize);
    let viewport = body.height as usize;
    let max_scroll = lines.len().saturating_sub(viewport);
    app.comment_offset = app.comment_offset.min(max_scroll);
    app.comment_viewport = viewport;
    app.comment_line_count = lines.len();

    let overflows = lines.len() > viewport && body.width > 1;
    let bar_area = if overflows {
        let split = Layout::horizontal([Constraint::Min(0), Constraint::Length(1)]).split(body);
        // Rewrap one column narrower so the thumb does not cover the last glyph.
        let lines = comment_lines(app, split[0].width as usize);
        app.comment_line_count = lines.len();
        let max_scroll = lines.len().saturating_sub(viewport);
        app.comment_offset = app.comment_offset.min(max_scroll);
        render_comment_body(frame, lines, app.comment_offset, split[0]);
        Some(split[1])
    } else {
        render_comment_body(frame, lines, app.comment_offset, body);
        None
    };

    if let Some(bar_area) = bar_area {
        let mut state = ScrollbarState::new(app.comment_line_count).position(app.comment_offset);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .thumb_style(Style::default().fg(ORANGE))
                .track_symbol(Some("│"))
                .track_style(Style::default().fg(RULE)),
            bar_area,
            &mut state,
        );
    }

    let can_open = story.and_then(|story| story.url.as_ref()).is_some();
    frame.render_widget(
        ruled(
            Paragraph::new(hints(&[
                ("↑↓", "scroll", true),
                ("esc", "stories", true),
                ("o", "open", can_open),
                ("q", "quit", true),
            ])),
            Borders::TOP,
        ),
        chunks[2],
    );
}

fn render_comment_body(frame: &mut Frame, lines: Vec<Line<'static>>, offset: usize, area: Rect) {
    let offset = u16::try_from(offset).unwrap_or(u16::MAX);
    frame.render_widget(Paragraph::new(Text::from(lines)).scroll((offset, 0)), area);
}

fn comment_lines(app: &App, width: usize) -> Vec<Line<'static>> {
    if app.comments.is_empty() {
        return vec![Line::from(Span::styled(
            "  No comments on this story.",
            Style::default().fg(DIM),
        ))];
    }

    let text_width = width.saturating_sub(4).max(1);
    let mut lines = Vec::new();
    for comment in &app.comments {
        let author = comment.by.as_deref().unwrap_or("[deleted]");
        let when = ago(comment.time);
        lines.push(spread(
            vec![
                Span::raw("  "),
                Span::styled(
                    author.to_string(),
                    Style::default().fg(ORANGE).add_modifier(Modifier::BOLD),
                ),
            ],
            vec![
                Span::styled(when, Style::default().fg(FAINT)),
                Span::raw(" "),
            ],
            width,
        ));

        let body = comment_body(comment);
        let wrapped = wrap_text(&body, text_width);
        let body_style = if body == "[deleted]" {
            Style::default().fg(FAINT).add_modifier(Modifier::ITALIC)
        } else {
            Style::default().fg(CREAM)
        };
        for line in wrapped {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(line, body_style),
            ]));
        }
        lines.push(Line::raw(""));
    }
    lines
}

fn comment_body(comment: &crate::api::Comment) -> String {
    if comment.deleted.unwrap_or(false) || comment.dead.unwrap_or(false) {
        return "[deleted]".to_string();
    }
    match comment.text.as_deref() {
        Some(text) if !text.trim().is_empty() => strip_html(text),
        _ => "[deleted]".to_string(),
    }
}

fn ruled(paragraph: Paragraph<'_>, borders: Borders) -> Paragraph<'_> {
    paragraph.block(
        Block::default()
            .borders(borders)
            .border_style(Style::default().fg(RULE)),
    )
}

fn hints(keys: &[(&str, &str, bool)]) -> Line<'static> {
    let mut spans = vec![Span::raw(" ")];
    for (index, (key, action, enabled)) in keys.iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled("   ", Style::default().fg(FAINT)));
        }
        let key_style = if *enabled {
            Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(FAINT)
        };
        let action_style = Style::default().fg(if *enabled { DIM } else { FAINT });
        spans.push(Span::styled((*key).to_string(), key_style));
        spans.push(Span::styled(format!(" {action}"), action_style));
    }
    Line::from(spans)
}

fn spread(left: Vec<Span<'static>>, right: Vec<Span<'static>>, width: usize) -> Line<'static> {
    let left_w: usize = left.iter().map(Span::width).sum();
    let right_w: usize = right.iter().map(Span::width).sum();
    let gap = width.saturating_sub(left_w.saturating_add(right_w));
    let mut spans = left;
    spans.push(Span::raw(" ".repeat(gap)));
    spans.extend(right);
    Line::from(spans)
}

fn ago(time: Option<i32>) -> String {
    time.map(time_ago).unwrap_or_else(|| "—".to_string())
}

fn domain(url: &str) -> String {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    let host = rest.split(['/', '?', '#']).next().unwrap_or(rest);
    host.strip_prefix("www.").unwrap_or(host).to_string()
}

fn truncate(text: &str, max: usize) -> String {
    if max == 0 || Span::raw(text).width() <= max {
        return text.to_string();
    }
    let budget = max.saturating_sub(1);
    let mut out = String::new();
    for ch in text.chars() {
        let next = format!("{out}{ch}");
        if Span::raw(&next).width() > budget {
            break;
        }
        out.push(ch);
    }
    out.push('…');
    out
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![String::new()];
    }

    let mut lines = Vec::new();
    for raw in text.split('\n') {
        let raw = raw.trim();
        if raw.is_empty() {
            lines.push(String::new());
            continue;
        }

        let mut current = String::new();
        let mut current_w = 0usize;
        for word in raw.split_whitespace() {
            let word_w = Span::raw(word).width();
            if word_w > width {
                if !current.is_empty() {
                    lines.push(std::mem::take(&mut current));
                    current_w = 0;
                }
                push_hard_break(word, width, &mut lines, &mut current, &mut current_w);
                continue;
            }
            if current.is_empty() {
                current = word.to_string();
                current_w = word_w;
            } else if current_w + 1 + word_w <= width {
                current.push(' ');
                current.push_str(word);
                current_w += 1 + word_w;
            } else {
                lines.push(std::mem::take(&mut current));
                current = word.to_string();
                current_w = word_w;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }

    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn push_hard_break(
    word: &str,
    width: usize,
    lines: &mut Vec<String>,
    current: &mut String,
    current_w: &mut usize,
) {
    for ch in word.chars() {
        let ch_w = Span::raw(ch.to_string()).width().max(1);
        if *current_w + ch_w > width && !current.is_empty() {
            lines.push(std::mem::take(current));
            *current_w = 0;
        }
        current.push(ch);
        *current_w += ch_w;
    }
}

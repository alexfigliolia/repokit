use std::sync::LazyLock;

use colored::Color as ColoredColor;
use inquire::ui::{Color, ErrorMessageRenderConfig, IndexPrefix, RenderConfig, StyleSheet, Styled};

use crate::logger::logger::Logger;

pub static PRE_RUNTIME_INQUIRE_THEME: LazyLock<RenderConfig> = LazyLock::new(|| RenderConfig {
    new_line_prefix: None,
    prompt_prefix: Styled::new("").with_fg(Color::LightBlue),
    answered_prompt_prefix: Styled::new("").with_fg(Color::LightGreen),
    prompt: StyleSheet::empty(),
    default_value: StyleSheet::empty(),
    placeholder: StyleSheet::new().with_fg(Color::DarkGrey),
    help_message: StyleSheet::empty().with_fg(Color::LightBlue),
    text_input: StyleSheet::empty(),
    error_message: ErrorMessageRenderConfig::default_colored(),
    password_mask: '*',
    answer: StyleSheet::empty().with_fg(Color::LightBlue),
    answer_from_new_line: false,
    canceled_prompt_indicator: Styled::new("Cancelled").with_fg(Color::LightBlue),
    highlighted_option_prefix: Styled::new(" ").with_fg(Color::LightBlue),
    unhighlighted_option_prefix: Styled::new("").with_fg(Color::LightBlue),
    scroll_up_prefix: Styled::new("^"),
    scroll_down_prefix: Styled::new("v"),
    selected_checkbox: Styled::new("[x]").with_fg(Color::LightGreen),
    unselected_checkbox: Styled::new("[ ]"),
    option_index_prefix: IndexPrefix::None,
    option: StyleSheet::empty(),
    selected_option: Some(StyleSheet::new().with_fg(Color::LightBlue)),
});

pub struct InquireTheme;

impl InquireTheme {
    pub fn create<'a>() -> RenderConfig<'a> {
        let prefix_color =
            Logger::with_theme(|theme| InquireTheme::to_inquirer_color(theme.colors.prefix_color));
        let highlight_color = Logger::with_theme(|theme| {
            InquireTheme::to_inquirer_color(theme.colors.highlight_color)
        });
        RenderConfig {
            new_line_prefix: PRE_RUNTIME_INQUIRE_THEME.new_line_prefix,
            prompt_prefix: PRE_RUNTIME_INQUIRE_THEME
                .prompt_prefix
                .with_fg(prefix_color),
            answered_prompt_prefix: PRE_RUNTIME_INQUIRE_THEME
                .answered_prompt_prefix
                .with_fg(Color::LightGreen),
            prompt: PRE_RUNTIME_INQUIRE_THEME.prompt,
            default_value: PRE_RUNTIME_INQUIRE_THEME.default_value,
            placeholder: PRE_RUNTIME_INQUIRE_THEME
                .placeholder
                .with_fg(Color::DarkGrey),
            help_message: PRE_RUNTIME_INQUIRE_THEME
                .help_message
                .with_fg(highlight_color),
            text_input: PRE_RUNTIME_INQUIRE_THEME.text_input,
            error_message: PRE_RUNTIME_INQUIRE_THEME.error_message,
            password_mask: PRE_RUNTIME_INQUIRE_THEME.password_mask,
            answer: PRE_RUNTIME_INQUIRE_THEME.answer.with_fg(highlight_color),
            answer_from_new_line: PRE_RUNTIME_INQUIRE_THEME.answer_from_new_line,
            canceled_prompt_indicator: PRE_RUNTIME_INQUIRE_THEME
                .canceled_prompt_indicator
                .with_fg(highlight_color),
            highlighted_option_prefix: PRE_RUNTIME_INQUIRE_THEME
                .highlighted_option_prefix
                .with_fg(highlight_color),
            unhighlighted_option_prefix: PRE_RUNTIME_INQUIRE_THEME
                .unhighlighted_option_prefix
                .with_fg(highlight_color),
            scroll_up_prefix: PRE_RUNTIME_INQUIRE_THEME.scroll_up_prefix,
            scroll_down_prefix: PRE_RUNTIME_INQUIRE_THEME.scroll_down_prefix,
            selected_checkbox: PRE_RUNTIME_INQUIRE_THEME
                .selected_checkbox
                .with_fg(Color::LightGreen),
            unselected_checkbox: PRE_RUNTIME_INQUIRE_THEME.unselected_checkbox,
            option_index_prefix: PRE_RUNTIME_INQUIRE_THEME.option_index_prefix,
            option: PRE_RUNTIME_INQUIRE_THEME.option,
            selected_option: Some(
                PRE_RUNTIME_INQUIRE_THEME
                    .selected_option
                    .unwrap()
                    .with_fg(highlight_color),
            ),
        }
    }

    fn to_inquirer_color(color: ColoredColor) -> Color {
        match color {
            ColoredColor::Black => Color::Black,
            ColoredColor::Red => Color::LightRed,
            ColoredColor::Green => Color::LightGreen,
            ColoredColor::Yellow => Color::LightYellow,
            ColoredColor::Blue => Color::LightBlue,
            ColoredColor::Magenta => Color::LightMagenta,
            ColoredColor::Cyan => Color::LightCyan,
            ColoredColor::White => Color::White,
            ColoredColor::BrightBlack => Color::Black,
            ColoredColor::BrightRed => Color::LightRed,
            ColoredColor::BrightGreen => Color::LightGreen,
            ColoredColor::BrightYellow => Color::LightYellow,
            ColoredColor::BrightBlue => Color::LightBlue,
            ColoredColor::BrightMagenta => Color::LightMagenta,
            ColoredColor::BrightCyan => Color::LightCyan,
            ColoredColor::BrightWhite => Color::White,
            ColoredColor::AnsiColor(u8) => Color::AnsiValue(u8),
            ColoredColor::TrueColor { r, g, b } => Color::rgb(r, g, b),
        }
    }
}

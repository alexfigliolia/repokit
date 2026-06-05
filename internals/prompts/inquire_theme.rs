use std::sync::LazyLock;

use inquire::ui::{Color, ErrorMessageRenderConfig, IndexPrefix, RenderConfig, StyleSheet, Styled};

pub static PRE_RUNTIME_INQUIRE_THEME: LazyLock<RenderConfig> = LazyLock::new(|| RenderConfig {
    new_line_prefix: None,
    prompt_prefix: Styled::new("?").with_fg(Color::LightGreen),
    answered_prompt_prefix: Styled::new(">").with_fg(Color::LightGreen),
    prompt: StyleSheet::empty(),
    default_value: StyleSheet::empty(),
    placeholder: StyleSheet::new().with_fg(Color::DarkGrey),
    help_message: StyleSheet::empty().with_fg(Color::LightBlue),
    text_input: StyleSheet::empty(),
    error_message: ErrorMessageRenderConfig::default_colored(),
    password_mask: '*',
    answer: StyleSheet::empty().with_fg(Color::LightBlue),
    answer_from_new_line: false,
    canceled_prompt_indicator: Styled::new("<canceled>").with_fg(Color::DarkRed),
    highlighted_option_prefix: Styled::new(">").with_fg(Color::LightBlue),
    unhighlighted_option_prefix: Styled::new(" ").with_fg(Color::LightBlue),
    scroll_up_prefix: Styled::new("^"),
    scroll_down_prefix: Styled::new("v"),
    selected_checkbox: Styled::new("[x]").with_fg(Color::LightGreen),
    unselected_checkbox: Styled::new("[ ]"),
    option_index_prefix: IndexPrefix::None,
    option: StyleSheet::empty(),
    selected_option: Some(StyleSheet::new().with_fg(Color::LightBlue)),
});

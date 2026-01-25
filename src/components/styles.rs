use iced::Theme;
use iced::widget::container;

/// Estilo para o container do header principal
pub fn header_container(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 1.0,
            color: iced::Color::from_rgba(0.3, 0.3, 0.3, 0.4),
            radius: 6.0.into(),
        },
        background: Some(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.015).into()),
        ..container::Style::default()
    }
}

/// Estilo para card de erro
pub fn error_card(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 1.0,
            color: iced::Color::from_rgb(0.8, 0.0, 0.0),
            radius: 4.0.into(),
        },
        background: Some(iced::Color::from_rgba(0.8, 0.0, 0.0, 0.1).into()),
        ..container::Style::default()
    }
}

/// Estilo para card de configuração (timeout)
pub fn config_card(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 1.0,
            color: iced::Color::from_rgba(0.3, 0.3, 0.3, 0.3),
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

/// Estilo para container de request (tabs + conteúdo)
pub fn request_container(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 1.0,
            color: iced::Color::from_rgba(0.3, 0.3, 0.3, 0.4),
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

/// Estilo para placeholder "No response yet"
pub fn empty_state_card(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 1.0,
            color: iced::Color::from_rgba(0.3, 0.3, 0.3, 0.25),
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

/// Estilo para container de response
pub fn response_container(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 1.0,
            color: iced::Color::from_rgb(0.2, 0.2, 0.2),
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

/// Estilo para badge de status (sucesso, erro, warning)
pub fn status_badge(color: iced::Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme: &Theme| container::Style {
        border: iced::Border {
            width: 1.0,
            color,
            radius: 4.0.into(),
        },
        background: Some(iced::Color::from_rgba(color.r, color.g, color.b, 0.1).into()),
        ..container::Style::default()
    }
}

/// Estilo para card de timing
pub fn timing_card(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 1.0,
            color: iced::Color::from_rgb(0.3, 0.3, 0.3),
            radius: 4.0.into(),
        },
        ..container::Style::default()
    }
}

/// Estilo para container do histórico (sidebar)
pub fn history_container(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 0.0,
            color: iced::Color::TRANSPARENT,
            radius: 0.0.into(),
        },
        background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15).into()),
        ..container::Style::default()
    }
}

/// Estilo para item individual do histórico
pub fn history_item(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 1.0,
            color: iced::Color::from_rgba(0.25, 0.25, 0.25, 0.5),
            radius: 4.0.into(),
        },
        ..container::Style::default()
    }
}

/// Estilo para badge de método HTTP
pub fn method_badge(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Color::from_rgb(0.2, 0.4, 0.8).into()),
        border: iced::Border {
            radius: 3.0.into(),
            ..Default::default()
        },
        ..container::Style::default()
    }
}

/// Estilo para badge de status no histórico
pub fn status_badge_border(color: iced::Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme: &Theme| container::Style {
        border: iced::Border {
            width: 1.0,
            color,
            radius: 3.0.into(),
        },
        ..container::Style::default()
    }
}

/// Estilo para border do text input de body
pub fn body_input_border(_theme: &Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            width: 1.0,
            color: iced::Color::from_rgba(0.3, 0.3, 0.3, 0.2),
            radius: 4.0.into(),
        },
        ..container::Style::default()
    }
}

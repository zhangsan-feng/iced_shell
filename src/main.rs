mod gui;

use iced::{Font, Size, Theme};
use iced::settings as application_settings;
use iced::window::Settings as window_settings;
use iced::window;
use iced::window::{icon, Icon};
use crate::gui::layout::ApplicationGui;

fn main() -> iced::Result {
    let mut win_settings = window_settings::default();
    win_settings.size = Size::new(1200.0, 600.0);
    win_settings.transparent = true;
    win_settings.position = window::Position::Centered;
    
    let icon_bytes = include_bytes!("icons/title_icon.png");
    let img = image::load_from_memory(icon_bytes).expect("REASON").into_rgba8();
    let (width, height) = (img.width(), img.height());
    let rgba_data = img.into_raw();
    win_settings.icon = Some(
        Icon::from(icon::from_rgba(rgba_data, width, height).unwrap())
    );

    let mut app_settings = application_settings::Settings::default();
    app_settings.default_font = Font::with_name("Microsoft YaHei");
    
    

    iced::application(
        "多窗口应用程序",
        ApplicationGui::update, 
        ApplicationGui::view,
    )
        .subscription(ApplicationGui::subscription)
        .theme(|_| Theme::Light)
        .window(win_settings)
        .settings(app_settings)
        .run()
}
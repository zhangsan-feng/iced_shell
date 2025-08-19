use iced::widget::{column, container, row, text, button, pick_list};
use iced::{alignment, window, Element, Length, Task};
use iced::advanced::graphics::text::cosmic_text::Command;
use iced::window::Settings;
use iced_aw::menu::{self, Item, Menu};
use iced_aw::{menu_bar, menu_items};
use iced_fonts::required::{icon_to_string, RequiredIcons};
use crate::gui::top_navigation::session_window::SessionWindow;

#[derive(Debug, Clone, Copy)]
pub enum TopNavigationMessage {
    None,
    SessionWindowOpen,
    WindowOpened(window::Id),

}

pub struct TopNavigation {
    session_window:SessionWindow,
}


impl TopNavigation {
    pub fn new() -> Self {
        TopNavigation {
            session_window:SessionWindow::new()
        }
    }

    pub fn update(&mut self, message: TopNavigationMessage) -> Task<TopNavigationMessage> {
        match message {
            TopNavigationMessage::None =>{Task::none()}
            TopNavigationMessage::SessionWindowOpen => {
                

                let (_id, open) = window::open(window::Settings {
                    position: window::Position::Centered,
                    size: iced::Size::new(800.0, 400.0), 
                    resizable: false,
                    decorations: true,
                    ..window::Settings::default()
                });
                
                
                open.map(TopNavigationMessage::WindowOpened)
            }
            TopNavigationMessage::WindowOpened(id) => {
               
                println!("新窗口已打开，ID: {:?}", id);
                Task::none()
              
            }
        }
    }

    pub fn view(&self) -> Element<'_, TopNavigationMessage> {
        let menu_tpl_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);

        let mb = menu_bar!(
            (debug_button_s("会话"), {
                
                let sub1 = menu_tpl_1(menu_items!(
                    (debug_button("Item", TopNavigationMessage::None))
                    (debug_button("Item", TopNavigationMessage::None) )
                    (debug_button("Item", TopNavigationMessage::None) )
                    (debug_button("Item", TopNavigationMessage::None) )
                    (debug_button("Item", TopNavigationMessage::None) )
                )).width(220.0);

                menu_tpl_1(menu_items!(
                    (debug_button("新会话", TopNavigationMessage::SessionWindowOpen))
                    (debug_button("Item", TopNavigationMessage::None) )
                    (submenu_button("A sub menu"), sub1)
                    (debug_button("Item", TopNavigationMessage::None) )
                    (debug_button("Item", TopNavigationMessage::None) )
                    (debug_button("Item", TopNavigationMessage::None) )
                )).width(140.0)
            })
        );
    

        
    container(
            row![
            mb
            ].spacing(10),
        ).into()
    }

}
fn base_button<'a>(
    content: impl Into<Element<'a, TopNavigationMessage>>,
    msg: TopNavigationMessage,
) -> button::Button<'a, TopNavigationMessage> {
    button(content)
        .padding([4, 8])
        .style(iced::widget::button::primary)
        .on_press(msg)
}

fn labeled_button(
    label: &str,
    msg: TopNavigationMessage,
) -> button::Button<TopNavigationMessage, iced::Theme, iced::Renderer> {
    base_button(text(label).align_y(alignment::Vertical::Center), msg)
}

fn debug_button(label: &str, event:TopNavigationMessage) -> button::Button<TopNavigationMessage, iced::Theme, iced::Renderer> {
    labeled_button(label, event).width(Length::Fill)
}

fn debug_button_s(label: &str) -> button::Button<TopNavigationMessage, iced::Theme, iced::Renderer> {
    labeled_button(label,  TopNavigationMessage::None).width(Length::Shrink)
}

fn submenu_button(label: &str) -> button::Button<TopNavigationMessage, iced::Theme, iced::Renderer> {
    base_button(
        row![
            text(label)
                .width(Length::Fill)
                .align_y(alignment::Vertical::Center),
            text(icon_to_string(RequiredIcons::CaretRightFill))
                .width(Length::Shrink)
                .align_y(alignment::Vertical::Center),
        ]
            .align_y(iced::Alignment::Center),
         TopNavigationMessage::None,
    )
        .width(Length::Fill)
}
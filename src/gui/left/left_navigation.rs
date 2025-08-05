use iced::widget::{column, container, row, text, button, pick_list};
use iced::{Element,};


#[derive(Debug, Clone, Copy)]
pub enum LeftNavigationMessage {
    PageChange(usize),
}

pub struct LeftNavigation {
    pub current_page: usize,

}




impl LeftNavigation {
    pub fn new() -> Self {
        LeftNavigation {
            current_page: 0,

        }
    }

    pub fn update(&mut self, message: LeftNavigationMessage) {
        match message {
            LeftNavigationMessage::PageChange(page) => {
                self.current_page = page;
            }
        }
    }

    pub fn view(&self) -> Element<'_, LeftNavigationMessage> {

        let nav_items = vec!["首页", "设置", "关于"];


        let navigation = container(
            column(
                nav_items
                    .iter()
                    .enumerate()
                    .map(|(i, &item)| {
                        button(text(item).size(20))
                            .width(iced::Length::Fill)
                            .style(if i == self.current_page {
                                iced::widget::button::primary  
                            } else {
                                iced::widget::button::secondary  
                            })

                            .on_press(LeftNavigationMessage::PageChange(i))
                            .into()
                    }).collect::<Vec<_>>()

            )
        )
            .width(iced::Length::Fixed(200.0))
            .height(iced::Length::Fill);
            // .padding(5);
        
        container(
            column![
                navigation
            ]
        ).into()
    }

}

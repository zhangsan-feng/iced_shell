use iced::widget::{column, container, row, text, button, pick_list};
use iced::{Element,};





#[derive(Debug, Clone, Copy)]
pub enum TopNavigationMessage {

}

pub struct TopNavigation {
  
}




impl TopNavigation {
    pub fn new() -> Self {
        TopNavigation {
          
        }
    }

    pub fn update(&mut self, message: TopNavigationMessage) {
        match message {

        }
    }

    pub fn view(&self) -> Element<'_, TopNavigationMessage> {
        container(
            column![
                text!(" ")
            ]
        ).into()
    }

}

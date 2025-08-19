use std::collections::HashMap;
use std::time::Duration;
use iced::widget::{column, container, row, text, Space, rule};
use iced::{window, Element, Length, Subscription, Task};
use crate::gui::left_navigation::left_navigation::{LeftNavigation, LeftNavigationMessage};
use crate::gui::top_navigation::top_navigation::{TopNavigation, TopNavigationMessage};



pub struct ApplicationGui {
    top_navigation: TopNavigation,
    left_navigation: LeftNavigation,


}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    TopNavigationMsg(TopNavigationMessage),
    LeftNavigationMsg(LeftNavigationMessage),
    Tick

}

impl ApplicationGui {
    fn new() -> Self {
        ApplicationGui { 
            top_navigation:TopNavigation::new(),
            left_navigation: LeftNavigation::new(),

        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(15)).map(|_| {Message::Tick})
        // iced::keyboard
        // iced::mouse
        // iced::
    }

    pub fn update(&mut self, message: Message) ->Task<Message> {
        match message {
            Message::TopNavigationMsg(msg)=>{
                self.top_navigation.update(msg).map(Message::TopNavigationMsg)
            }
            Message::LeftNavigationMsg(msg)=>{
                self.left_navigation.update(msg).map(Message::LeftNavigationMsg)
            }
      
            Message::Tick =>{
                println!("tick");
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        
        
        container(column![
            row![
                  self.top_navigation.view().map(Message::TopNavigationMsg),
            ].padding(5),

            rule::Rule::horizontal(5),
            row![
                column![
                    // self.left_navigation.view().map(Message::LeftNavigationMsg),
                 
                ].width(Length::Fixed(200.0)),
                Space::with_width(Length::Fixed(5.0)),
                rule::Rule::vertical(5),
                // Space::with_width(Length::Fill),
                // Space::with_height(Length::Fill),
                column![
                    
            

                ].width(Length::Fill).padding(5),
            ].padding(5),
        ]).padding(5).into()
    }
}

impl Default for ApplicationGui {
    fn default() -> Self {
        ApplicationGui::new()
    }
}
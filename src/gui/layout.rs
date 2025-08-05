
use iced::widget::{column, container, row, text,  Space, rule};
use iced::{Element,  Length};
use crate::gui::left::left_navigation::{LeftNavigation, LeftNavigationMessage};
use crate::gui::top::top_navigation::{TopNavigation, TopNavigationMessage};
use crate::gui::content::sys_info::{SysInfo, SysInfoMessage};


pub struct ApplicationGui {
    top_navigation: TopNavigation,
    left_navigation: LeftNavigation,
    sys_info:SysInfo,
    
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    TopNavigationMsg(TopNavigationMessage),
    LeftNavigationMsg(LeftNavigationMessage),
    SysInfoMsg(SysInfoMessage)
}

impl ApplicationGui {
    fn new() -> Self {
        ApplicationGui { 
            top_navigation:TopNavigation::new(),
            left_navigation: LeftNavigation::new(),
            sys_info:SysInfo::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TopNavigationMsg(msg)=>{
                self.top_navigation.update(msg)
            }
            Message::LeftNavigationMsg(msg)=>{
                self.left_navigation.update(msg)
            }
            Message::SysInfoMsg(msg)=>{
                self.sys_info.update(msg)
            }   
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        
        let content = container(
            match self.left_navigation.current_page { 
                0 => self.sys_info.view().map(Message::SysInfoMsg),
                _=>         container(text("2")).into()
            }
        )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill);
        ;
        
        container(column![
            row![
                  self.top_navigation.view().map(Message::TopNavigationMsg),
            ].padding(5),

            rule::Rule::horizontal(5),
            row![
                column![
                    self.left_navigation.view().map(Message::LeftNavigationMsg),
                 
                ].width(Length::Fixed(200.0)),
                Space::with_width(Length::Fixed(5.0)),
                rule::Rule::vertical(5),
                // Space::with_width(Length::Fill),
                // Space::with_height(Length::Fill),
                column![
                    
                    content,

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
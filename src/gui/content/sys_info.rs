use iced::Element;
use iced::widget::{container, text};

#[derive(Debug, Clone, Copy)]
pub enum SysInfoMessage{
    
}


#[derive(Debug, Clone, Copy)]
pub struct SysInfo{
    
}

impl SysInfo {
    pub fn new()->Self{
        SysInfo{
            
        }
    }
    
    pub fn update(& mut self, message:SysInfoMessage){
        
    }
    pub fn view(&self)-> Element<'_,SysInfoMessage>{
        container(
            text("")
        ).into()
    }
}
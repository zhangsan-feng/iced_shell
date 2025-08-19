
use iced::widget::{button, column, container, text, text_input, row};
use iced::{Element, Length, Task};

#[derive(Debug, Clone)]
pub enum SessionWindowMessage {
    InputChanged(String),
    SendMessage,
}

pub struct SessionWindow {
    input_text: String,
    messages: Vec<String>,
}

impl SessionWindow {
    pub fn new() -> Self {
        Self {
            input_text: String::new(),
            messages: vec!["欢迎来到会话窗口".to_string()],
        }
    }

    pub fn update(&mut self, message: SessionWindowMessage) -> Task<SessionWindowMessage> {
        match message {
            SessionWindowMessage::InputChanged(value) => {
                self.input_text = value;
                Task::none()
            }
            SessionWindowMessage::SendMessage => {
                if !self.input_text.is_empty() {
                    self.messages.push(format!("用户: {}", self.input_text));
                    self.input_text.clear();
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, SessionWindowMessage> {
        let messages_view = column(
            self.messages
                .iter()
                .map(|msg| {
                    container(text(msg))
                        .padding(5)
                        .width(Length::Fill)
                        .into()
                })
                .collect::<Vec<_>>()
        )
            .spacing(5);

        let input_area = row![
            text_input("输入消息...", &self.input_text)
                .on_input(SessionWindowMessage::InputChanged)
                .on_submit(SessionWindowMessage::SendMessage)
                .width(Length::Fill),
            button("发送")
                .on_press(SessionWindowMessage::SendMessage)
        ]
            .spacing(10);

        container(
            column![
                container(text("会话窗口").size(20))
                    .padding(10)
                    .width(Length::Fill),
                container(messages_view)
                    .padding(10)
                    .height(Length::Fill),
                container(input_area)
                    .padding(10)
                    .width(Length::Fill)
            ]
        )
            .padding(10)
            .into()
    }
}
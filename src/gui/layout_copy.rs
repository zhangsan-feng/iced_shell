use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{column, container, responsive, 
                   row, text, vertical_space,horizontal_space, Space};
use iced::{Element, Fill, Theme, Color, Border, Background, theme, Length};




#[derive(Debug, Clone, Copy, PartialEq)]
enum PaneType {
    LeftPanel,
    RightPanel,
}

#[derive(Clone, Copy)]
struct Pane {
    pane_type: PaneType,
}

impl Pane {
    fn new(pane_type: PaneType) -> Self {
        Self { pane_type }
    }
}


pub struct ApplicationGui {
    panes: pane_grid::State<Pane>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Resized(pane_grid::ResizeEvent),
}

impl ApplicationGui {
    fn new() -> Self {

        let (mut panes, left_pane) = pane_grid::State::new(Pane::new(PaneType::LeftPanel));

        if let Some((_, split)) = panes.split(
            pane_grid::Axis::Vertical,
            left_pane,
            Pane::new(PaneType::RightPanel),
        ) {

            panes.resize(split, 0.3);
        }
        
        ApplicationGui { panes }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let pane_grid = PaneGrid::new(&self.panes, |_, pane, _| {
            // 根据面板类型创建不同的内容
            let content = match pane.pane_type {
                PaneType::LeftPanel => responsive(|_| view_left_panel_content()),
                PaneType::RightPanel => responsive(|_| view_right_panel_content()),
            };

            pane_grid::Content::new(content)   
                .style(
                    style::pane_active
               )
        })
        .width(Fill)
        .height(Fill)
        .spacing(5)
        .on_resize(10, Message::Resized);
        
        let top = container(
            column![text("123")]
        ).width(Length::Fill).height(Length::Fixed(50.0)).padding(5).style(
            style::pane_active
        );
        
        container(column![
            top,
            Space::with_height(5),
            vertical_space().height(0),
            pane_grid, 
        ]).padding(5).style( style::pane_active).into()
    }
}

impl Default for ApplicationGui {
    fn default() -> Self {
        ApplicationGui::new()
    }
}


// 创建左面板内容
fn view_left_panel_content<'a>() -> Element<'a, Message> {
   container( column![text("左侧面板内容")])
       .padding(10)
       .into()
}

// 创建右面板内容
fn view_right_panel_content<'a>() -> Element<'a, Message> {
    container(column![text("右侧面板内容")])
        .padding(10)
        .into()
}

// pub fn pane_style(theme: &Theme) -> pane_grid::Style {
//     // 根据主题动态选择颜色
//     let is_dark = matches!(theme, Theme::Dark);
//     
//     let hover_bg = if is_dark {
//         Color::from_rgba(1.0, 1.0, 1.0, 0.1) // 暗主题用白色半透明
//     } else {
//         Color::from_rgba(0.0, 0.0, 0.0, 0.1) // 亮主题用黑色半透明
//     };
//     
//     let border_color = if is_dark {
//         Color::from_rgb(0.8, 0.8, 0.8) // 暗主题用亮色边框
//     } else {
//         Color::from_rgb(0.2, 0.2, 0.2) // 亮主题用深色边框
//     };
// 
//     pane_grid::Style {
//         hovered_region: pane_grid::Highlight {
//             background: Background::Color(hover_bg),
//             border: Border{
//                 color: border_color,
//                 width: 2.0,
//                 radius: Radius::from(3.0),
//             },
//         },
//         picked_split: pane_grid::Line {
//             color: Color::from_rgb(0.0, 0.6, 1.0), // 蓝色在任何背景都明显
//             width: 4.0,
//         },
//         hovered_split: pane_grid::Line {
//             color: Color::from_rgb(0.4, 0.4, 0.4), // 中等灰色
//             width: 3.0,
//         },
//     }
// }


mod style {
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn title_bar_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.background.strong.color,
                ..Border::default()
            },
            
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.primary.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
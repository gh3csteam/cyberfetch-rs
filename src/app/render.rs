use ratatui::{
    Frame, layout::{Alignment, Constraint, Layout}, style::{Color, Style, Stylize}, symbols, text::{Line, Span}, widgets::{Block, BorderType, Borders, Padding, Paragraph}
};

const MUTED: Color = Color::Rgb(60, 70, 85);
const ACCENT: Color = Color::LightCyan;

use super::app::{
    App
};

fn panel(title: &str) -> Block<'static>{
    Block::bordered()
    .border_type(BorderType::Rounded)
    .border_style(MUTED) 
    .title(Line::from(format!(" {title} ")).fg(ACCENT).bold())
    .bold()
}       


pub fn draw(frame: &mut Frame, _app: &mut App){
    let [header, top, mid, keybinds] = Layout::vertical(
        [
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Length(3)
        ]
    ).areas(frame.area());

    let [logo, mem] = Layout::horizontal(
        [
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ]
    ).areas(top);

    let [art, procceses, ports] = Layout::horizontal(
        [
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),

        ]
    ).areas(mid);

    let title = Paragraph::new("λ >>_ cyberfetch system monitor _<< λ")
    .block(panel("cyberfetch"))
    .alignment(Alignment::Center)
    .style(ACCENT);

    (*frame).render_widget(title, header);
    (*frame).render_widget(panel("system"), logo);
    (*frame).render_widget(panel("memory"), mem);
    (*frame).render_widget(panel("art"), art);
    (*frame).render_widget(panel("procceses"), procceses);
    (*frame).render_widget(panel("net info"), ports);
    (*frame).render_widget(panel("keybinds")
    .title_bottom(Line::from("q - Quit").right_aligned().fg(ACCENT)),keybinds);

}
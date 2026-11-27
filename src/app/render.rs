use ratatui::{
    Frame, layout::{Alignment, Constraint, Direction, Flex::Center, Layout}, macros::horizontal, style::{Color, Style, Stylize}, symbols, text::{Line, Span}, widgets::{Bar, BarChart, Block, BorderType, Borders, Chart, Dataset, Padding, Paragraph, Wrap}
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

fn bl_panel() -> Block<'static>{
    Block::default()
    .border_style(MUTED) 
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

    let [data, procceses, ports] = Layout::horizontal(
        [
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),

        ]
    ).areas(mid);

    let [cpu_graph, diskload, tempo] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]
).areas(data);

    let [username, systeminfo] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1)
    ]).areas(logo);

    let [ascii_art, info] = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(60)

    ]).areas(systeminfo);

    let title = Paragraph::new("λ >>_ cyberfetch system monitor _<< λ")
    .block(panel("cyberfetch"))
    .alignment(Alignment::Center)
    .style(ACCENT);
    
    let usname = &_app.username;

    let [cores, usage] = Layout::horizontal(
        [Constraint::Fill(1), 
        Constraint::Fill(1)
        ]
    )
        .areas(cpu_graph);

    let user_info = Paragraph::new(format!(">_ {}@{}", usname.as_str(), _app.host_name)).block(
        panel("user")
    )
    .alignment(Alignment::Center)
    .style(ACCENT);
;

    let info_text = format!(
        "\nOS:  {}\nKernel:  {}\nHost:  {}\nVer:  {}\nCPU Threads:  {}({})\nCpu Model:  {}\nCpu Architecture:  {}\nShell:  {}",
        _app.system_name,
        _app.kernel_version,
        _app.host_name,
        _app.os_version,
        _app.core_count,
        _app.cpu_usage.len().to_string(),
        _app.cpu_model,
        _app.cpu_architecture,
        _app.shell
    );


    let system_info = Paragraph::new(info_text)
    .block(
        panel("info").padding(Padding::horizontal(1))
    )
    .style(ACCENT);


    let cpu_dataset: Vec<Bar> = _app.cpu_usage
    .iter()
    .map(|f|   
    Bar::new(*f as u64)
    )
    .collect();


    let cpu_chart = BarChart::new(cpu_dataset)
    .direction(Direction::Horizontal)       
    .bar_gap(0)
    .block(bl_panel()
    .padding(Padding { left: (0), right: (0), top: (3), bottom: (2) }))
    .fg(ACCENT);

    let cpu_cores: String = _app.cpu_usage
    .iter()
    .enumerate()
    .map(|(i, _n)| format!("\nCore {:.2}", i+1))
    .collect();

    let enum_cpus = Paragraph::new(cpu_cores)
    .block(bl_panel()
    .padding(Padding { left: (2), right: (2), top: (2), bottom: (2) }))
    .fg(ACCENT);

    let ram_gb = _app.ram_used as f64 / (1024.0 * 1024.0 * 1024.0);
    let ram_max = _app.total_ram as f64 / (1024.0 * 1024.0 * 1024.0);
    let ram_text = format!("memory usage - {:.1}/{:.1} GB", ram_gb, ram_max);

    let ram_info = Paragraph::new(ram_text).block(
        panel("memory")
        .padding(Padding { left: (5), right: (5), top: (15), bottom: (1) })
    )
    .style(ACCENT)
    .alignment(Alignment::Center);

    //system renders
    (*frame).render_widget(title, header);  
    (*frame).render_widget(system_info, info);
    (*frame).render_widget(ram_info, mem);
    (*frame).render_widget(panel("system"), logo);
    (*frame).render_widget(user_info, username);
    (*frame).render_widget(panel("ascii_art"), ascii_art);

    //cpu renders
    (*frame).render_widget(enum_cpus, cores);
    (*frame).render_widget(cpu_chart, usage);
    (*frame).render_widget(panel("Cpu graph"), cpu_graph);

    //disk
    (*frame).render_widget(panel("disks"), diskload);

    //tempo
    (*frame).render_widget(panel("temperature"), tempo);

    //procceses
    (*frame).render_widget(panel("processes"), procceses);

    //net info
    (*frame).render_widget(panel("net info"), ports);

    //foot
    (*frame).render_widget(panel("keybinds")
    .title_bottom(Line::from("q - Quit").right_aligned().fg(ACCENT)),keybinds);

}
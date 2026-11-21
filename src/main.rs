pub mod app;
mod info_collection;
use app::app as frame_app;
fn main(){
    let mut _app = frame_app::App::new();
    let mut terminal = ratatui::init();
    let _ = _app.run(& mut terminal);

}

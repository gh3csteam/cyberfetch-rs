use crate::app::{
    app::{App},
    event
};
use sysinfo::{
    Components,
    Disks,
    Networks,
    System,
};

pub fn update_data(app: &mut App){  
   app.sys.refresh_all();
   app.ram_used = app.sys.used_memory();
}
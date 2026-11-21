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
    let sys = &app.sys;
}
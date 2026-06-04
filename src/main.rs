use input::LibinputInterface;
use input::event::EventTrait;
use libc::{O_ACCMODE, O_RDONLY, O_RDWR, O_WRONLY};
use std::fs::OpenOptions;
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

pub struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_ACCMODE == O_RDONLY) | (flags & O_ACCMODE == O_RDWR))
            .write((flags & O_ACCMODE == O_WRONLY) | (flags & O_ACCMODE == O_RDWR))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }

    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(fd);
    }
}

fn main() {
    let mut input = input::Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    input.dispatch().unwrap();

    let events: Vec<String> = input
        .clone()
        .collect::<Vec<input::Event>>()
        .into_iter()
        .map(|event| event.device())
        .filter(|device| device.has_capability(input::DeviceCapability::Pointer))
        .map(|device| device.name().to_string())
        .collect();

    let trackpoints: Vec<String> = events 
        .clone()
        .into_iter()
        .filter(|name| name.contains("TrackPoint"))
        // .filter(|name| name.contains("TrackPoint") )
        .collect();

    println!("Events: {:#?}", events);
    // println!("Input: {:#?}", input);

    println!("Events: {:#?}", trackpoints);
}

// fn main2() {
//     let mut input = Libinput::new_with_udev(Interface);
//     input.udev_assign_seat("seat0").unwrap();
//     loop {
//         input.dispatch().unwrap();
//         for event in &mut input {
//             println!("Got event: {:?}", event);
//             println!("Device: {:?}", event.device().name());
//         }
//     }
// }

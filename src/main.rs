use std::path::{Path, PathBuf};
use std::sync::Arc;

use log::{info, trace, LevelFilter, SetLoggerError};
use syslog::{BasicLogger, Facility, Formatter3164};

use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[cfg(target_os = "macos")]
use winit::platform::macos::{EventLoopWindowTargetExtMacOS, FileOpenResult};

fn open_file(window: &Window, path: &Path) {
    // TODO place your file opening logic here.
    let filename = path.as_os_str();
    let filename = filename.to_owned().into_string().unwrap();
    let title = format!("> Opened: '{}'", filename);
    window.set_title(&title);
}

fn main() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "winit_open".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();

    info!("hello world");

    let event_loop = EventLoop::new();

    // Systems other than macOS provide the file paths
    // as a program argument.
    let file_path;
    if let Some(arg) = std::env::args().skip(1).next() {
        file_path = Some(arg);
    } else {
        file_path = None;
    }

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Loading")
            .with_inner_size(LogicalSize::new(400.0, 200.0))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap(),
    );

    if let Some(file_path) = file_path {
        open_file(&window, file_path.as_ref());
    }

    #[cfg(target_os = "macos")]
    {
        let window = window.clone();
        event_loop.set_file_open_callback(Some(move |paths: Vec<PathBuf>| {
            for path in paths.iter() {
                open_file(&*window, path.as_ref());
            }
            FileOpenResult::Success
        }));
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        //trace!("Event: {:?}", event);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            _ => (),
        };
    });
}

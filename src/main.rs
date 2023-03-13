mod args;
mod server;
use server::MDServer;
extern crate markdown;
use std::fs;
use args::*;
use clap::Parser;
use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
    },
    webview::WebViewBuilder,
};


fn main() -> wry::Result<()>{
    let args = Args::parse();

    let markdown = fs::read_to_string(&args.path).unwrap();
    let html = markdown::to_html(&markdown);

    let mdserver = MDServer::new(Some(args.path.to_string()));
    mdserver.listen(mdserver.stream);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello World")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_url("localhost:7878")?
        .build()?;
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    
    match event {
        Event::WindowEvent {
        event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
        }
    });
}
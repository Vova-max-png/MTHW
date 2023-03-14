mod args;
mod server;
mod window;
use server::MDServer;
extern crate markdown;
use std::fs::{self, File};
use std::io::Write;
use std::thread::*;
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
use futures::executor::*;
use tokio::{self, *};

#[tokio::main]
async fn main() -> wry::Result<()>{
    let args = Args::parse();

    let markdown = fs::read_to_string(&args.path).unwrap();
    let html = markdown::to_html(&markdown);
    
    let mut output = File::create("index.html")?;
    write!(output, "{}", html)?;

    let mdserver = MDServer::new("index.html".to_string());
    tokio::join!(
        mdserver.listen(mdserver.get_listener()),
    );

    let window = window::create_window(&EventLoop::new());

    window::create_window(&EventLoop::new());
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
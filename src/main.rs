mod args;
mod server;
mod window;
use server::MDServer;
use window::*;
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
async fn main() -> wry::Result<()> {
    let args = Args::parse();

    let markdown = fs::read_to_string(&args.path).unwrap();
    let html = markdown::to_html(&markdown);
    
    let mut output = File::create("index.html")?;
    write!(output, "{}", html)?;

    let mut mdwindow = MDWindow::new(EventLoop::new());

    let mdserver = MDServer::new("127.0.0.1:7878".to_string());
    tokio::join!(
        mdserver.listen(mdserver.get_listener()),
        mdwindow.handle_window(),
    );
    Ok(())
}
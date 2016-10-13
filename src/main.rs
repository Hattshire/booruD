extern crate hyper;
extern crate json;
extern crate conrod;
extern crate piston_window;
extern crate image;

use hyper::Client;
use std::io::Read;
use conrod::{widget, Colorable, color};
use piston_window::{EventLoop, PistonWindow, UpdateEvent};

use std::io::Write;
use std::fs::File;

fn main(){
 // Set Window Size
 const WIDTH: u32 = 800;
 const HEIGHT: u32 = 600;
 
 // Create an OpenGL Context for the window
 let opengl = piston_window::OpenGL::V3_2;
 
 // Create the Window
 let mut window: PistonWindow =
	piston_window::WindowSettings::new("Untitled", [WIDTH, HEIGHT])
	.opengl(opengl).exit_on_esc(true).vsync(true).samples(4).build().expect("");
 // Set window framerate to 60
 window.set_ups(60);
 
 // Build the window 
 let mut ui = conrod::UiBuilder::new().build();	
 
 // Test Danbooru's JSON API
 get_json();
 
 // Window loop
 while let Some(event) = window.next() {
  ui.handle_event(event.clone());
  event.update(|_|{
   widget::Canvas::new().color(color::LIGHT_BLUE);
  });
 }
}

/// Gets a JSON, downloads a thumbnail and writes it to a file 
fn get_json() {
 // First we create a new client
 let client = Client::new();
 
 // We use string_buffer to get the json response and then parse it
 let mut string_buffer : String = String::new();
 
 println!("\n\n\n Printing JSON:\n\n");
 
 // Do a GET request to the posts API 
 let mut response = client.get("http://danbooru.donmai.us/posts.json")
   .send().unwrap();
 
 // Check if the request was successful 
 assert_eq!(response.status, hyper::Ok);
 
 // Read the json response
 response.read_to_string(&mut string_buffer).expect("");
 
 // Parse JSON response
 let parsed = json::parse(&string_buffer).unwrap();
 
 // print JSON elements
 for item in parsed.members() {
  println!("{:#}\n----------------\n\n", item);
 }
 
 for element in parsed.members() {
  // Download a thumbnail to test file download
  let uri = &*format!("http://danbooru.donmai.us/{}", element["preview_file_url"]);
  let mut response = client.get(uri)
    .send().unwrap();
  
  // Read the file to a buffer
  let mut buffer = Vec::new();
  response.read_to_end(&mut buffer).unwrap();
 
  // Write the buffer to a file
  let mut thumb_file = File::create( &*format!( "thumbs/thumb_{}.jpg", element["id"] ) ).unwrap();
  thumb_file.write_all(&buffer as &[u8]).unwrap();
 }   
 
 println!("RDY");
}
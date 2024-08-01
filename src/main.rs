
// import the packages im going to use 

use iced::{widget::text, Sandbox, Settings};
use iced::Element;


fn main() -> iced::Result {
    Editor::run(Settings::default())
}


struct Editor ;

#[derive(Debug)]
enum Message {}

impl Sandbox for Editor {
     type Message = Message;

     fn new() -> Self {
         Self

     }

     fn title(&self) -> String {
         String::from("a cool editor")

     }

     fn update(&mut self, message: Message){
         match message {}
     }

     fn view(&self) -> Element<'_, Message> {
         text("Hello , iced !").into()
     }
}

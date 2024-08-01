
// import the packages im going to use 
use iced::widget::{container,text_editor};
use iced::{widget::text, Sandbox, Settings};
use iced::Element;


fn main() -> iced::Result {
    Editor::run(Settings::default())
}


struct Editor {
    content: text_editor::Content
}

#[derive(Debug ,Clone)]
enum Message {
    Edit(text_editor::Action)
}

impl Sandbox for Editor {
     type Message = Message;

     fn new() -> Self {
         Self{
            content: text_editor::Content::new(),
         }

     }

     fn title(&self) -> String {
         String::from("a cool editor")

     }

     fn update(&mut self, message: Message){
         match message {
            Message::Edit(action) => {
                self.content.edit(action);
            }
         }
     }

     fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content).on_edit(Message::Edit);

        container(input).padding(10).into()
     }
}

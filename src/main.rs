use iced::{executor, widget, window, Element, Length};
use iced::highlighter::{self, Highlighter};
use iced::keyboard;
use iced::theme::{self, Theme};
use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, text, text_editor, tooltip,
    Button, Column, Scrollable, Text
};
use iced::{
    Alignment, Font,
    Subscription,
};
use iced::{
    Application, Command, Settings,
};


mod modal;

// use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;




pub fn main() -> iced::Result {
    Editor::run(Settings {
        fonts: vec![include_bytes!("../fonts/icons.ttf").as_slice().into()],
        default_font: Font::MONOSPACE,
        window: window::Settings {
            size: iced::Size::new(800.0, 700.0), 
            resizable: (true),
            ..window::Settings::default()

        },
        ..Settings::default()
    })
}


#[derive(Debug, Clone)]
enum Message {
    ActionPerformed(text_editor::Action),
    ThemeSelected(highlighter::Theme),
    SysThemeSelected(Theme),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
    CloseCard,
    OpenCard,
    ShowModal,
    HideModal,
}
struct Editor {
    file: Option<PathBuf>,
    content: text_editor::Content,
    highlighter_theme: highlighter::Theme,
    sys_theme: Theme,
    is_loading: bool,
    is_dirty: bool,
    state: State,
    show_modal: bool,
}

#[derive(Debug, Clone)]
struct State {
    card_open: bool,
}


impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                file: None,
                content: text_editor::Content::new(),
                highlighter_theme: highlighter::Theme::Base16Mocha,
                sys_theme: iced::Theme::KanagawaDragon,
                is_loading: true,
                is_dirty: false,
                state: State{ card_open: false },
                show_modal: true,

            },

            Command::batch(vec![
                Command::perform(load_file(default_file()), Message::FileOpened),
                ])
         )
    }

    fn title(&self) -> String {
        String::from("Editor - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ShowModal => {
                self.show_modal = true;
                widget::focus_next()
            }
            Message::HideModal => {
                self.hide_modal();
                Command::none()
            }

            Message::CloseCard | Message::OpenCard =>{
                self.state.card_open = !self.state.card_open;
               // self.card_opened = !self.card_opened;

                Command::none()
            }

            Message::ActionPerformed(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();

                self.content.perform(action);

                Command::none()
            }
            Message::ThemeSelected(theme) => {
                self.highlighter_theme = theme;

                Command::none()
            }
            Message::SysThemeSelected(theme) => {
                self.sys_theme = theme;

                Command::none()
            }
            Message::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    self.content = text_editor::Content::new();
                }

                Command::none()
            }
            Message::OpenFile => {
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;

                    Command::perform(open_file(), Message::FileOpened)
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    self.content = text_editor::Content::with_text(&contents);
                }

                Command::none()
            }
            Message::SaveFile => {
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;

                    Command::perform(
                        save_file(self.file.clone(), self.content.text()),
                        Message::FileSaved,
                    )
                }
            }
            Message::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.file = Some(path);
                    self.is_dirty = false;
                }

                Command::none()
            }

        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("s") if modifiers.command() => {
                Some(Message::SaveFile)
            }
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {

        let controls = row![
            action(new_icon(), "New file", Some(Message::NewFile)),
            action(
                open_icon(),
                "Open file",
                (!self.is_loading).then_some(Message::OpenFile)
            ),
            action(
                save_icon(),
                "Save file",
                self.is_dirty.then_some(Message::SaveFile)
            ),
            horizontal_space(),
            button(text("Show Modal")).on_press(Message::ShowModal),
            pick_list(
                iced::Theme::ALL,
                Some(&self.sys_theme),
                Message::SysThemeSelected
            )
            .text_size(14)
            .padding([5, 10]),
            pick_list(
                highlighter::Theme::ALL,
                Some(self.highlighter_theme),
                Message::ThemeSelected
            )
            .text_size(14)
            .padding([5, 10]),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let status = row![
            text(if let Some(path) = &self.file {
                let path = path.display().to_string();

                if path.len() > 60 {
                    format!("...{}", &path[path.len() - 40..])
                } else {
                    path
                }
            } else {
                String::from("New file")
            }),
            horizontal_space(),
            text({
                let (line, column) = self.content.cursor_position();

                format!("{}:{}", line + 1, column + 1)
            })
        ]
        .spacing(10);

        if self.show_modal {
            let modal = container(
                column![
                    text("Hola como estas")
                ]
            )
            .width(300)
            .padding(10)
            .style(theme::Container::Box);

             let about = modal::modal::Modal::new(controls, modal)
                .on_blur(Message::HideModal);
             about.into()
        } else {        

        column![
            controls,
            text_editor(&self.content)
                .height(Length::Fill)
                .on_action(Message::ActionPerformed)
                .highlight::<Highlighter>(
                    highlighter::Settings {
                        theme: self.highlighter_theme,
                        extension: self
                            .file
                            .as_deref()
                            .and_then(Path::extension)
                            .and_then(std::ffi::OsStr::to_str)
                            .map(str::to_string)
                            .unwrap_or(String::from("rs")),
                    },
                    |highlight, _theme| highlight.to_format()
                ),
            status,
        ]
        .spacing(10)
        .padding(10)
        .into()
        }
    }

    fn theme(&self) -> Theme {
        if self.highlighter_theme.is_dark() {
            self.sys_theme.clone()
        } else {
            Theme::GruvboxLight
        }
    }  
}

impl Editor {
    fn hide_modal(&mut self) {
        self.show_modal = false;
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}

async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

async fn save_file(
    path: Option<PathBuf>,
    contents: String,
) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}

fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).width(30).center_x());

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(theme::Container::Box)
        .into()
    } else {
        action.style(theme::Button::Destructive).into()
    }
}

fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f115}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}
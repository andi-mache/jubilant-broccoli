use iced::executor;
use iced::highlighter::{self, Highlighter};
use iced::keyboard;
use iced::theme::{self, Theme};
use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, text,
    text_editor, tooltip,/* toggler */ Button, Column, Text , Scrollable
};
use iced::{
    Alignment, Application, Command, Element, Font, Length, Settings,
    Subscription,
};
use iced_aw::{helpers::card, style};
use iced_aw::style::CardStyles;

use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn main() -> iced::Result {
    Editor::run(Settings {
        fonts: vec![include_bytes!("../fonts/icons.ttf").as_slice().into()],
        default_font: Font::MONOSPACE,
        ..Settings::default()
    })
}

struct Editor {
    file: Option<PathBuf>,
    content: text_editor::Content,
    //app_theme: Theme,
    highlighter_theme: highlighter::Theme,
    is_loading: bool,
    is_dirty: bool,
    card_open: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ActionPerformed(text_editor::Action),
   // AppThemeChanged(Theme),
    ThemeSelected(highlighter::Theme),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
    CloseCard,
    OpenCard,
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug)]
enum CardExample {
    Loading,
    Loaded(State),
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
                //app_theme: Theme::GruvboxDark,
                highlighter_theme: highlighter::Theme::SolarizedDark,
                is_loading: true,
                is_dirty: false,
                card_open: false,
            },
            Command::perform(load_file(default_file()), Message::FileOpened),
        )
    }

    fn title(&self) -> String {
        String::from("Editor - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
          /*  Message::AppThemeChanged(theme) => {
                self.app_theme = theme;

                Command::none()

            }*/ 



            CardExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = CardExample::Loaded(State { card_open: false })
                }
            }
            CardExample::Loaded(State { card_open }) => match message {
                Message::CloseCard | Message::OpenCard => {
                    *card_open = !*card_open;
                }
                _ => {}
            },



            Message::CloseCard | Message::OpenCard =>{
                self.card_open = !self.card_open;

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

       // let themechanger = toggler();

       CardExample::Loaded(State { card_open }) => {
        let element: Element<Message> = if *card_open {
                card(
                    Text::new("Head X"),
                    Column::new()
                        .push(Text::new("Zombie ipsum reversus ab viral inferno, nam rick grimes malum cerebro. De carne lumbering animata corpora quaeritis. Summus brains sit, morbo vel maleficia? De apocalypsi gorger omero undead survivor dictum mauris. Hi mindless mortuis soulless creaturas, imo evil stalking monstra adventus resi dentevil vultus comedat cerebella viventium. Qui animated corpse, cricket bat max brucks terribilem incessu zomby. The voodoo sacerdos flesh eater, suscitat mortuos comedere carnem virus. Zonbi tattered for solum oculi eorum defunctis go lum cerebro. Nescio brains an Undead zombies. Sicut malus putrid voodoo horror. Nigh tofth eliv ingdead."))
                )
                .foot(Text::new("Foot"))
                .style(CardStyles::Primary)
                .on_close(Message::CloseCard)
                .into()
            } else {
                Button::new(Text::new("Open card"))
                    .on_press(Message::OpenCard)
                    .into()
            };
        }

        let content = Scrollable::new(element);


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
            content,
            pick_list(
                highlighter::Theme::ALL,
                Some(self.highlighter_theme),
                Message::ThemeSelected
            )
            .text_size(14)
            .padding([5, 10])
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
                            .and_then(ffi::OsStr::to_str)
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

    fn theme(&self) -> Theme {
        if self.highlighter_theme.is_dark() {
            Theme::GruvboxDark
        } else {
            Theme::GruvboxLight
        }
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
        action.style(theme::Button::Secondary).into()
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
// import all the needes crates for the project
use iced::highlighter::{self, Highlighter};
use iced::keyboard;
use iced::theme::{self, Theme};
use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, text, text_editor, tooltip,
};
use iced::{executor, widget, window, Element, Length};
use iced::{Alignment, Font, Subscription};
use iced::{Application, Command, Settings};

mod modal;
/// The `mod modal;` statement in the Rust code is used to declare a module named `modal`. This
/// statement tells the Rust compiler to look for a file named `modal.rs` or `modal/mod.rs` and treat
/// its contents as part of the `modal` module. This allows for organizing code into separate modules
/// and files, making the codebase more structured and manageable.
// use std::ffi;

// std modules that help as interact with the system for reading and writing
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// The main function initializes an Editor with specific settings in a Rust program.
pub fn main() -> iced::Result {
    Editor::run(Settings {
        fonts: vec![include_bytes!("../fonts/icons.ttf").as_slice().into()],
        default_font: Font::DEFAULT,
        window: window::Settings {
            size: iced::Size::new(800.0, 700.0),
            resizable: (true),
            min_size: Some(iced::Size::new(600., 500.)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

/// The  code is defining an enum named `Message` in Rust. This enum has several variants
/// representing different types of messages that can be sent or received in a program. Each variant can
/// hold associated data of different types. Some of the variants include `ActionPerformed`,
/// `ThemeSelected`, `NewFile`, `OpenFile`, `FileOpened`, `SaveFile`, `FileSaved`, `CloseCard`,
/// `OpenCard`, `ShowModal`, and `HideModal`. The `Message` enum is derived with `Debug` and `Clone`
/// traits, allowing for debugging and cloning of instances of this enum

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

    ShowAboutModal,
    HideAboutModal,
}
/// The `Editor` struct in Rust represents a text editor with various properties such as file path,
/// content, themes, loading status, dirty status, state, and modal visibility.
///
/// Properties:
///
/// * `file`: The `file` property in the `Editor` struct is an optional `PathBuf` type, which represents
/// the path to the file being edited in the editor. It can be `Some(path)` if a file is open or `None`
/// if no file is currently being edited.
/// * `content`: The `content` property in the `Editor` struct likely represents the text content of the
/// file being edited. It is of type `text_editor::Content`, which suggests that it may contain the text
/// data and possibly other information related to the content of the file.
/// * `highlighter_theme`: The `highlighter_theme` property in the `Editor` struct likely represents the
/// theme used for syntax highlighting in the text editor. This theme would define the colors and styles
/// used to highlight different elements of the code or text being edited. It is a part of the overall
/// configuration of the text editor's
/// * `sys_theme`: The `sys_theme` property in the `Editor` struct likely represents the system theme
/// that the editor is using. This could be a theme that determines the overall look and feel of the
/// editor based on the system settings or preferences.
/// * `is_loading`: The `is_loading` property in the `Editor` struct is a boolean flag that indicates
/// whether the editor is currently in a loading state. It is used to track whether the editor is in the
/// process of loading a file or content.
/// * `is_dirty`: The `is_dirty` property in the `Editor` struct typically represents whether the
/// content in the editor has been modified since it was last saved or opened. It is commonly used to
/// track whether there are unsaved changes in the editor.
/// * `state`: The `state` property in the `Editor` struct represents the current state of the editor.
/// It could be used to track different states such as editing, saving, or any other relevant states
/// that the editor may have.
/// * `show_modal`: The `show_modal` property in the `Editor` struct is likely used to determine whether
/// a modal dialog or window should be displayed in the user interface of the text editor. When
/// `show_modal` is set to `true`, it indicates that a modal is currently active and should be shown to
/// the
struct Editor {
    file: Option<PathBuf>,
    content: text_editor::Content,
    highlighter_theme: highlighter::Theme,
    sys_theme: Theme,
    is_loading: bool,
    is_dirty: bool,
    show_modal: bool,
}

impl Application for Editor {
    /// The above code in Rust is defining type aliases for `Message`, `Theme`, `Executor`, and `Flags`. It
    /// is also specifying that the `Executor` type is using the default implementation from the `executor`
    /// module. The `Flags` type is defined as an empty tuple `()`.
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    /// The function `new` initializes a struct with default values and performs a file loading operation.
    ///
    /// Arguments:
    ///
    /// * `_flags`: The `_flags` parameter in the `new` function appears to be of type `Self::Flags`. This
    /// suggests that it is a associated type within the context of the struct or enum that contains this
    /// function. The actual definition of `Self::Flags` would be found in the same scope as the
    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                file: None,
                content: text_editor::Content::new(),
                highlighter_theme: highlighter::Theme::SolarizedDark,
                sys_theme: iced::Theme::TokyoNightStorm,
                is_loading: true,
                is_dirty: false,
                show_modal: false,
            },
            Command::batch(vec![Command::perform(
                load_file(default_file()),
                Message::FileOpened,
            )]),
        )
    }

    fn title(&self) -> String {
        String::from("Editor - Iced")
    }

    /// The `update` function in Rust handles various message types to update the state of a program, such
    /// as showing or hiding modals, toggling card visibility, selecting themes, and managing file
    /// operations.
    ///
    /// Arguments:
    ///
    /// * `message`: The `message` parameter in the `update` function represents the message that is being
    /// sent to update the state of the application. The function matches the message against different
    /// variants of the `Message` enum and performs corresponding actions based on the message received.
    ///
    /// Returns:
    ///
    /// The `update` function returns a `Command<Message>` based on the `message` received.
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ShowAboutModal => {
                self.show_modal = true;
                widget::focus_next()
            }
            Message::HideAboutModal => {
                self.hide_modal();
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

    /// The function `subscription` listens for key presses and triggers a message to save a file when the
    /// "s" key is pressed with the command modifier.
    ///
    /// Returns:
    ///
    /// The `subscription` function returns a subscription to keyboard events that listens for the "s" key
    /// press with the command key modifier held down. If this specific key combination is detected, it will
    /// return a `Message::SaveFile`, otherwise it will return `None`.
    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("s") if modifiers.command() => Some(Message::SaveFile),
            _ => None,
        })
    }

    /// The function `view` in Rust defines a user interface layout with controls, a text editor, and
    /// status information, including the ability to show a modal dialog.
    ///
    /// Returns:
    ///
    /// The `view` function is returning an `Element<Message>`. The content of the `Element<Message>` being
    /// returned consists of a user interface layout with various components such as controls, a text
    /// editor, and status information. If the `show_modal` flag is set to true, a modal dialog is
    /// displayed on top of the main content.
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
            action(
                save_icon(),
                "aboute moi",
                Some(Message::ShowAboutModal)
            ),
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

        let mwili = text_editor(&self.content)
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
                |highlight, _theme| highlight.to_format(),
            );
        

        let full = column![
                    controls, 
                    mwili, 
                    status,
                    ]
                    .spacing(10)
                    .padding(10);

        let content = container(full);

        if self.show_modal {
            let modal = container(text("bonjour mon ami , je suis adrian , bienvenue "))
                .width(300)
                .padding(10)
                .style(theme::Container::Box);

            modal::modal::Modal::new(content, modal)
                .on_blur(Message::HideAboutModal)
                .into()
        } else {
            content.into()
        }
    }
    /// The `theme` function returns the system theme if the highlighter theme is dark, otherwise it returns
    /// the GruvboxLight theme.
    ///
    /// Returns:
    ///
    /// The `theme` function returns either a clone of `self.sys_theme` if the `highlighter_theme` is dark,
    /// or it returns the `Theme::GruvboxLight` if the `highlighter_theme` is not dark.

    fn theme(&self) -> Theme {
        if self.highlighter_theme.is_dark() {
            self.sys_theme.clone()
        } else {
            Theme::GruvboxLight
        }
    }
}

/// The above code is implementing a method `hide_modal` for a struct `Editor`. This method sets the
/// `show_modal` field of the `Editor` struct to `false`, effectively hiding the modal in the user
/// interface.
impl Editor {
    fn hide_modal(&mut self) {
        self.show_modal = false;
    }
}

/// The above code is defining a Rust enum called `Error` with two variants: `DialogClosed` and
/// `IoError`, which takes an `io::ErrorKind` as a parameter. The enum also derives the `Debug` and
/// `Clone` traits for debugging and cloning purposes.

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

/// The function `default_file` returns a `PathBuf` representing the default file path for a Rust
/// project's main.rs file.
///
/// Returns:
///
/// A `PathBuf` object representing the path to the `main.rs` file within the `src` directory of the
/// Cargo project where this function is being called.
fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}

/// The function `open_file` asynchronously opens a text file using a file dialog and loads its
/// contents.
///
/// Returns:
///
/// The `open_file` function returns a `Result` containing a tuple with two elements:
/// 1. `PathBuf` - representing the path of the picked file.
/// 2. `Arc<String>` - an `Arc` smart pointer to a `String` containing the content of the file.
async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
    
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file.path().to_owned()).await
}

/// The `load_file` function in Rust asynchronously reads the contents of a file at the specified path
/// and returns a tuple containing the path and the file contents wrapped in an `Arc`.
///
/// Arguments:
///
/// * `path`: The `path` parameter in the `load_file` function is of type `PathBuf`, which represents a
/// path to a file or directory in the file system. It is used to specify the location of the file that
/// needs to be loaded and read.
///
/// Returns:
///
/// The function `load_file` returns a `Result` containing a tuple with the file path (`PathBuf`) and
/// the file contents (`Arc<String>`), or an `Error` if there was an issue reading the file.
async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)

        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

/// The `save_file` function in Rust asynchronously saves a file with specified contents to a given path
/// or through a file dialog if no path is provided.
///
/// Arguments:
///
/// * `path`: The `path` parameter in the `save_file` function is an optional `PathBuf` type, which
/// represents the path to the file where the contents will be saved. It can either contain a valid path
/// or be `None`, in which case a file dialog will be opened to allow the user
/// * `contents`: The `contents` parameter in the `save_file` function represents the text content that
/// you want to write to a file. It is of type `String`, which means it is a sequence of characters or
/// text data that you want to save to the file specified by the `path` parameter.
///
/// Returns:
///
/// The `save_file` function returns a `Result` containing either a `PathBuf` if the file was
/// successfully saved, or an `Error` if any errors occurred during the process.

async fn save_file(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
        .set_can_create_directories(true)
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
/// The function `action` creates a button element with optional tooltip functionality in Rust.
///
/// Arguments:
///
/// * `content`: The `content` parameter is of type `impl Into<Element<'a, Message>>`, which means it
/// can accept any value that can be converted into an `Element<'a, Message>`. This parameter is used to
/// define the content of the action, such as text or an icon, that will
/// * `label`: The `label` parameter is a reference to a string slice (`&str`) that represents the label
/// or text associated with the action button.
/// * `on_press`: The `on_press` parameter in the `action` function is an optional message that will be
/// triggered when the button is pressed. If a message is provided, the button will have a tooltip
/// attached to it, displaying the `label` text. If no message is provided, the button will have a
///
/// Returns:
///
/// The `action` function returns an `Element` that represents a button with optional tooltip
/// functionality. If the `on_press` parameter is provided, the button will have a tooltip with the
/// specified label and position. If the `on_press` parameter is not provided, the button will have a
/// destructive style.

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

/// The code defines functions to create icons with specific Unicode codepoints in Rust.
///
/// Returns:
///
/// An Element containing an icon with the specified Unicode codepoint is being returned. The icon is
/// styled using a specific font named "editor-icons".
fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f115}')
}

/// The function `icon` in Rust takes a Unicode codepoint as input and returns an Element with the
/// specified codepoint using a specific font named "editor-icons".
///
/// Arguments:
///
/// * `codepoint`: The `codepoint` parameter is a Unicode code point represented as a `char` type. It is
/// used to specify the specific character or icon that you want to display using the `icon` function.
///
/// Returns:
///
/// The `icon` function is returning an `Element` containing a text element with the specified
/// `codepoint` character rendered using the `ICON_FONT` font.
fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}

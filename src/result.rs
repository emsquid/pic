#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Image(image::error::ImageError),
    Sixel(sixel_rs::status::Error),
    ImageSize(imagesize::ImageError),
    Tempfile(tempfile::PersistError),
    MethodSupport(&'static str),
    ActionSupport(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {err}"),
            Error::Image(err) => write!(f, "Image error: {err}"),
            Error::Sixel(err) => write!(f, "Sixel error: {err:#?}"),
            Error::ImageSize(err) => write!(f, "Image size error: {err}"),
            Error::Tempfile(err) => write!(f, "Tempfile error: {err}"),
            Error::MethodSupport(err) => write!(f, "Method error: {err}"),
            Error::ActionSupport(err) => write!(f, "Action error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::Image(err)
    }
}

impl From<sixel_rs::status::Error> for Error {
    fn from(e: sixel_rs::status::Error) -> Self {
        Error::Sixel(e)
    }
}

impl From<imagesize::ImageError> for Error {
    fn from(err: imagesize::ImageError) -> Self {
        Error::ImageSize(err)
    }
}

impl From<tempfile::PersistError> for Error {
    fn from(err: tempfile::PersistError) -> Self {
        Error::Tempfile(err)
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;

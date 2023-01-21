#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Image(image::error::ImageError),
    Sixel(sixel_rs::status::Error),
    ImageSize(imagesize::ImageError),
    Tempfile(tempfile::PersistError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Image(e) => write!(f, "Image error: {}", e),
            Error::Sixel(e) => write!(f, "Sixel error: {:?}", e),
            Error::ImageSize(e) => write!(f, "Image size error: {}", e),
            Error::Tempfile(e) => write!(f, "Tempfile error: {}", e),
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

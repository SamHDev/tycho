pub enum TychoError {
    Io(std::io::Error)
}

pub type TychoResult<T> = Result<T, TychoError>;
pub type TychoStatus = TychoResult<()>;

impl<T> TychoResult<T> {
    pub(crate) fn digest_io(result: Result<T, std::io::Error>) -> TychoResult<T> {
        match result {
            Ok(value) => Ok(value),
            Err(error) => Err(TychoError::Io(error))
        }
    }
}
impl TychoStatus {
    pub(crate) fn digest_io(result: Result<(), std::io::Error>) -> TychoStatus {
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(TychoError::Io(error))
        }
    }
}
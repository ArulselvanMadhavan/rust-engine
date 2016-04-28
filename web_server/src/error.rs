use request::Request;
use job::FileJob;

/* Wrapper type for a Result that is either a Request object or a RequestError */
pub type RequestResult = Result<Request, RequestError>;

/* Custom error type for when a request is invalid */
#[derive(Debug)]
pub struct RequestError {
    kind: RequestErrorKind,
    pub message: String
}

/* Type of request error */
#[derive(Debug)]
pub enum RequestErrorKind {
    EmptyRequest
}

impl RequestError {
    pub fn new(msg: String, kind: RequestErrorKind) -> RequestError {
        RequestError {
            kind: kind,
            message: msg
        }
    }
}

/* Wrapper type for a Result that is either a FileJob or a FileJobError */
pub type FileJobResult = Result<FileJob, FileJobError>;

/* Custom error type for when a FileJob is invalid */
#[derive(Debug)]
pub struct FileJobError {
    kind: FileJobErrorKind,
    pub message: String
}

/* Type of FileJob error */
#[derive(Debug)]
pub enum FileJobErrorKind {
    EmptyFileJob
}

impl FileJobError {
    pub fn new(msg: String, kind: FileJobErrorKind) -> FileJobError {
        FileJobError {
            kind: kind,
            message: msg
        }
    }
}


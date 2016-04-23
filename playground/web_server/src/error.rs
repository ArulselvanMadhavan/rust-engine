use request::Request;
use job::FileJob;


pub type RequestResult = Result<Request, RequestError>;

#[derive(Debug)]
pub struct RequestError {
    kind: RequestErrorKind,
    pub message: String
}

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

pub type FileJobResult = Result<FileJob, FileJobError>;

#[derive(Debug)]
pub struct FileJobError {
    kind: FileJobErrorKind,
    pub message: String
}

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


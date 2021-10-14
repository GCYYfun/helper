//! Linux error codes
use core::fmt;
use rcore_fs::vfs::FsError;
// use zircon_object::ZxError;

/// Linux Result defination
pub type LxResult<T = ()> = Result<T, LxError>;
/// SysResult Result defination (same as Linux Result)
pub type SysResult = LxResult<usize>;

/// Linux error codes defination
#[allow(dead_code)]
#[repr(isize)]
#[derive(Debug)]
pub enum LxError {
    /// Undefined
    EUNDEF = 0,
    /// Operation not permitted
    EPERM = 1,
    /// No such file or directory
    ENOENT = 2,
    /// No such process
    ESRCH = 3,
    /// Interrupted system call
    EINTR = 4,
    /// I/O error
    EIO = 5,
    /// No such device or address
    ENXIO = 6,
    /// Arg list too long
    E2BIG = 7,
    /// Exec format error
    ENOEXEC = 8,
    /// Bad file number
    EBADF = 9,
    /// No child processes
    ECHILD = 10,
    /// Try again
    EAGAIN = 11,
    /// Out of memory
    ENOMEM = 12,
    /// Permission denied
    EACCES = 13,
    /// Bad address
    EFAULT = 14,
    /// Block device required
    ENOTBLK = 15,
    /// Device or resource busy
    EBUSY = 16,
    /// File exists
    EEXIST = 17,
    /// Cross-device link
    EXDEV = 18,
    /// No such device
    ENODEV = 19,
    /// Not a directory
    ENOTDIR = 20,
    /// Is a directory
    EISDIR = 21,
    /// Invalid argument
    EINVAL = 22,
    /// File table overflow
    ENFILE = 23,
    /// Too many open files
    EMFILE = 24,
    /// Not a tty device
    ENOTTY = 25,
    /// Text file busy
    ETXTBSY = 26,
    /// File too large
    EFBIG = 27,
    /// No space left on device
    ENOSPC = 28,
    /// Illegal seek
    ESPIPE = 29,
    /// Read-only file system
    EROFS = 30,
    /// Too many links
    EMLINK = 31,
    /// Broken pipe
    EPIPE = 32,
    /// Math argument out of domain
    EDOM = 33,
    /// Math result not representable
    ERANGE = 34,
    /// Resource deadlock would occur
    EDEADLK = 35,
    /// Filename too long
    ENAMETOOLONG = 36,
    /// No record locks available
    ENOLCK = 37,
    /// Function not implemented
    ENOSYS = 38,
    /// Directory not empty
    ENOTEMPTY = 39,
    /// Too many symbolic links encountered
    ELOOP = 40,
    /// Identifier removed
    EIDRM = 43,
    /// Socket operation on non-socket
    ENOTSOCK = 88,
    /// Protocol not available
    ENOPROTOOPT = 92,
    /// Protocol family not supported
    EPFNOSUPPORT = 96,
    /// Address family not supported by protocol
    EAFNOSUPPORT = 97,
    /// No buffer space available
    ENOBUFS = 105,
    /// Transport endpoint is already connected
    EISCONN = 106,
    /// Transport endpoint is not connected
    ENOTCONN = 107,
    /// Connection refused
    ECONNREFUSED = 111,
}

#[allow(non_snake_case)]
impl fmt::Display for LxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::LxError::*;
        let explain = match self {
            EPERM => "Operation not permitted",
            ENOENT => "No such file or directory",
            ESRCH => "No such process",
            EINTR => "Interrupted system call",
            EIO => "I/O error",
            ENXIO => "No such device or address",
            E2BIG => "Argument list too long",
            ENOEXEC => "Exec format error",
            EBADF => "Bad file number",
            ECHILD => "No child processes",
            EAGAIN => "Try again",
            ENOMEM => "Out of memory",
            EACCES => "Permission denied",
            EFAULT => "Bad address",
            ENOTBLK => "Block device required",
            EBUSY => "Device or resource busy",
            EEXIST => "File exists",
            EXDEV => "Cross-device link",
            ENODEV => "No such device",
            ENOTDIR => "Not a directory",
            EISDIR => "Is a directory",
            EINVAL => "Invalid argument",
            ENFILE => "File table overflow",
            EMFILE => "Too many open files",
            ENOTTY => "Not a typewriter",
            ETXTBSY => "Text file busy",
            EFBIG => "File too large",
            ENOSPC => "No space left on device",
            ESPIPE => "Illegal seek",
            EROFS => "Read-only file system",
            EMLINK => "Too many links",
            EPIPE => "Broken pipe",
            EDOM => "Math argument out of domain of func",
            ERANGE => "Math result not representable",
            EDEADLK => "Resource deadlock would occur",
            ENAMETOOLONG => "File name too long",
            ENOLCK => "No record locks available",
            ENOSYS => "Function not implemented",
            ENOTEMPTY => "Directory not empty",
            ELOOP => "Too many symbolic links encountered",
            EIDRM => "Identifier removed",
            ENOTSOCK => "Socket operation on non-socket",
            ENOPROTOOPT => "Protocol not available",
            EPFNOSUPPORT => "Protocol family not supported",
            EAFNOSUPPORT => "Address family not supported by protocol",
            ENOBUFS => "No buffer space available",
            EISCONN => "Transport endpoint is already connected",
            ENOTCONN => "Transport endpoint is not connected",
            ECONNREFUSED => "Connection refused",
            _ => "Unknown error",
        };
        write!(f, "{}", explain)
    }
}

impl From<ZxError> for LxError {
    fn from(e: ZxError) -> Self {
        match e {
            ZxError::INVALID_ARGS => LxError::EINVAL,
            ZxError::NOT_SUPPORTED => LxError::ENOSYS,
            ZxError::ALREADY_EXISTS => LxError::EEXIST,
            ZxError::SHOULD_WAIT => LxError::EAGAIN,
            ZxError::PEER_CLOSED => LxError::EPIPE,
            ZxError::BAD_HANDLE => LxError::EBADF,
            _ => unimplemented!("unknown error type: {:?}", e),
        }
    }
}

impl From<FsError> for LxError {
    fn from(error: FsError) -> Self {
        match error {
            FsError::NotSupported => LxError::ENOSYS,
            FsError::NotFile => LxError::EISDIR,
            FsError::IsDir => LxError::EISDIR,
            FsError::NotDir => LxError::ENOTDIR,
            FsError::EntryNotFound => LxError::ENOENT,
            FsError::EntryExist => LxError::EEXIST,
            FsError::NotSameFs => LxError::EXDEV,
            FsError::InvalidParam => LxError::EINVAL,
            FsError::NoDeviceSpace => LxError::ENOMEM,
            FsError::DirRemoved => LxError::ENOENT,
            FsError::DirNotEmpty => LxError::ENOTEMPTY,
            FsError::WrongFs => LxError::EINVAL,
            FsError::DeviceError => LxError::EIO,
            FsError::IOCTLError => LxError::EINVAL,
            FsError::NoDevice => LxError::EINVAL,
            FsError::Again => LxError::EAGAIN,
            FsError::SymLoop => LxError::ELOOP,
            FsError::Busy => LxError::EBUSY,
            FsError::Interrupted => LxError::EINTR,
        }
    }
}

use crate::user::Error;

impl From<Error> for LxError {
    fn from(e: Error) -> Self {
        match e {
            Error::InvalidUtf8 => LxError::EINVAL,
            Error::InvalidPointer => LxError::EFAULT,
            Error::BufferTooSmall => LxError::ENOBUFS,
            Error::InvalidLength => LxError::EINVAL,
            Error::InvalidVectorAddress => LxError::EINVAL,
        }
    }
}

/// The type returned by kernel objects methods.
pub type ZxResult<T = ()> = Result<T, ZxError>;

/// Zircon statuses are signed 32 bit integers. The space of values is
/// divided as follows:
/// - The zero value is for the OK status.
/// - Negative values are defined by the system, in this file.
/// - Positive values are reserved for protocol-specific error values,
///   and will never be defined by the system.
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ZxError {
    /// Success.
    OK = 0,

    // ======= Internal failures =======
    /// The system encountered an otherwise unspecified error
    /// while performing the operation.
    INTERNAL = -1,

    /// The operation is not implemented, supported,
    /// or enabled.
    NOT_SUPPORTED = -2,

    /// The system was not able to allocate some resource
    /// needed for the operation.
    NO_RESOURCES = -3,

    /// The system was not able to allocate memory needed
    /// for the operation.
    NO_MEMORY = -4,

    // -5 used to be ZX_ERR_CALL_FAILED.
    /// The system call was interrupted, but should be
    /// retried.  This should not be seen outside of the VDSO.
    INTERNAL_INTR_RETRY = -6,

    // ======= Parameter errors =======
    /// an argument is invalid, ex. null pointer
    INVALID_ARGS = -10,

    /// A specified handle value does not refer to a handle.
    BAD_HANDLE = -11,

    /// The subject of the operation is the wrong type to
    /// perform the operation.
    /// Example: Attempting a message_read on a thread handle.
    WRONG_TYPE = -12,

    /// The specified syscall number is invalid.
    BAD_SYSCALL = -13,

    /// An argument is outside the valid range for this
    /// operation.
    OUT_OF_RANGE = -14,

    /// A caller provided buffer is too small for
    /// this operation.
    BUFFER_TOO_SMALL = -15,

    // ======= Precondition or state errors =======
    /// operation failed because the current state of the
    /// object does not allow it, or a precondition of the operation is
    /// not satisfied
    BAD_STATE = -20,

    /// The time limit for the operation elapsed before
    /// the operation completed.
    TIMED_OUT = -21,

    /// The operation cannot be performed currently but
    /// potentially could succeed if the caller waits for a prerequisite
    /// to be satisfied, for example waiting for a handle to be readable
    /// or writable.
    /// Example: Attempting to read from a channel that has no
    /// messages waiting but has an open remote will return ZX_ERR_SHOULD_WAIT.
    /// Attempting to read from a channel that has no messages waiting
    /// and has a closed remote end will return ZX_ERR_PEER_CLOSED.
    SHOULD_WAIT = -22,

    /// The in-progress operation (e.g. a wait) has been
    /// canceled.
    CANCELED = -23,

    /// The operation failed because the remote end of the
    /// subject of the operation was closed.
    PEER_CLOSED = -24,

    /// The requested entity is not found.
    NOT_FOUND = -25,

    /// An object with the specified identifier
    /// already exists.
    /// Example: Attempting to create a file when a file already exists
    /// with that name.
    ALREADY_EXISTS = -26,

    /// The operation failed because the named entity
    /// is already owned or controlled by another entity. The operation
    /// could succeed later if the current owner releases the entity.
    ALREADY_BOUND = -27,

    /// The subject of the operation is currently unable
    /// to perform the operation.
    /// Note: This is used when there's no direct way for the caller to
    /// observe when the subject will be able to perform the operation
    /// and should thus retry.
    UNAVAILABLE = -28,

    // ======= Permission check errors =======
    /// The caller did not have permission to perform
    /// the specified operation.
    ACCESS_DENIED = -30,

    // ======= Input-output errors =======
    /// Otherwise unspecified error occurred during I/O.
    IO = -40,

    /// The entity the I/O operation is being performed on
    /// rejected the operation.
    /// Example: an I2C device NAK'ing a transaction or a disk controller
    /// rejecting an invalid command, or a stalled USB endpoint.
    IO_REFUSED = -41,

    /// The data in the operation failed an integrity
    /// check and is possibly corrupted.
    /// Example: CRC or Parity error.
    IO_DATA_INTEGRITY = -42,

    /// The data in the operation is currently unavailable
    /// and may be permanently lost.
    /// Example: A disk block is irrecoverably damaged.
    IO_DATA_LOSS = -43,

    /// The device is no longer available (has been
    /// unplugged from the system, powered down, or the driver has been
    /// unloaded,
    IO_NOT_PRESENT = -44,

    /// More data was received from the device than expected.
    /// Example: a USB "babble" error due to a device sending more data than
    /// the host queued to receive.
    IO_OVERRUN = -45,

    /// An operation did not complete within the required timeframe.
    /// Example: A USB isochronous transfer that failed to complete due to an overrun or underrun.
    IO_MISSED_DEADLINE = -46,

    /// The data in the operation is invalid parameter or is out of range.
    /// Example: A USB transfer that failed to complete with TRB Error
    IO_INVALID = -47,

    // ======== Filesystem Errors ========
    /// Path name is too long.
    BAD_PATH = -50,

    /// Object is not a directory or does not support
    /// directory operations.
    /// Example: Attempted to open a file as a directory or
    /// attempted to do directory operations on a file.
    NOT_DIR = -51,

    /// Object is not a regular file.
    NOT_FILE = -52,

    /// This operation would cause a file to exceed a
    /// filesystem-specific size limit
    FILE_BIG = -53,

    /// Filesystem or device space is exhausted.
    NO_SPACE = -54,

    /// Directory is not empty.
    NOT_EMPTY = -55,

    // ======== Flow Control ========
    // These are not errors, as such, and will never be returned
    // by a syscall or public API.  They exist to allow callbacks
    // to request changes in operation.
    /// Do not call again.
    /// Example: A notification callback will be called on every
    /// event until it returns something other than ZX_OK.
    /// This status allows differentiation between "stop due to
    /// an error" and "stop because the work is done."
    STOP = -60,

    /// Advance to the next item.
    /// Example: A notification callback will use this response
    /// to indicate it did not "consume" an item passed to it,
    /// but by choice, not due to an error condition.
    NEXT = -61,

    /// Ownership of the item has moved to an asynchronous worker.
    ///
    /// Unlike ZX_ERR_STOP, which implies that iteration on an object
    /// should stop, and ZX_ERR_NEXT, which implies that iteration
    /// should continue to the next item, ZX_ERR_ASYNC implies
    /// that an asynchronous worker is responsible for continuing iteration.
    ///
    /// Example: A notification callback will be called on every
    /// event, but one event needs to handle some work asynchronously
    /// before it can continue. ZX_ERR_ASYNC implies the worker is
    /// responsible for resuming iteration once its work has completed.
    ASYNC = -62,

    // ======== Network-related errors ========
    /// Specified protocol is not
    /// supported.
    PROTOCOL_NOT_SUPPORTED = -70,

    /// Host is unreachable.
    ADDRESS_UNREACHABLE = -71,

    /// Address is being used by someone else.
    ADDRESS_IN_USE = -72,

    /// Socket is not connected.
    NOT_CONNECTED = -73,

    /// Remote peer rejected the connection.
    CONNECTION_REFUSED = -74,

    /// Connection was reset.
    CONNECTION_RESET = -75,

    /// Connection was aborted.
    CONNECTION_ABORTED = -76,
}

// use crate::user::Error;

impl From<Error> for ZxError {
    fn from(e: Error) -> Self {
        match e {
            Error::InvalidUtf8 => ZxError::INVALID_ARGS,
            Error::InvalidPointer => ZxError::INVALID_ARGS,
            Error::BufferTooSmall => ZxError::BUFFER_TOO_SMALL,
            Error::InvalidLength => ZxError::INVALID_ARGS,
            Error::InvalidVectorAddress => ZxError::NOT_FOUND,
        }
    }
}

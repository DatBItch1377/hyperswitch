pub use common_utils::errors::{ParsingError, ValidationError};
#[cfg(feature = "email")]
use external_services::email::EmailError;
use hyperswitch_domain_models::errors::api_error_response::ApiErrorResponse;
pub use redis_interface::errors::RedisError;
pub use storage_impl::errors::ApplicationError;
use storage_impl::errors::{RecoveryError, StorageError};

use crate::env::logger::{self, error};

#[derive(Debug, thiserror::Error)]
pub enum ProcessTrackerError {
    #[error("An unexpected flow was specified")]
    UnexpectedFlow,
    #[error("Failed to serialize object")]
    SerializationFailed,
    #[error("Failed to deserialize object")]
    DeserializationFailed,
    #[error("Missing required field")]
    MissingRequiredField,
    #[error("Failed to insert process batch into stream")]
    BatchInsertionFailed,
    #[error("Failed to insert process into stream")]
    ProcessInsertionFailed,
    #[error("The process batch with the specified details was not found")]
    BatchNotFound,
    #[error("Failed to update process batch in stream")]
    BatchUpdateFailed,
    #[error("Failed to delete process batch from stream")]
    BatchDeleteFailed,
    #[error("An error occurred when trying to read process tracker configuration")]
    ConfigurationError,
    #[error("Failed to update process in database")]
    ProcessUpdateFailed,
    #[error("Failed to fetch processes from database")]
    ProcessFetchingFailed,
    #[error("Failed while fetching: {resource_name}")]
    ResourceFetchingFailed { resource_name: String },
    #[error("Failed while executing: {flow}")]
    FlowExecutionError { flow: &'static str },
    #[error("Not Implemented")]
    NotImplemented,
    #[error("Job not found")]
    JobNotFound,
    #[error("Received Error ApiResponseError")]
    EApiErrorResponse,
    #[error("Received Error ClientError")]
    EClientError,
    #[error("Received RecoveryError: {0:?}")]
    ERecoveryError(error_stack::Report<RecoveryError>),
    #[error("Received Error StorageError: {0:?}")]
    EStorageError(error_stack::Report<StorageError>),
    #[error("Received Error RedisError: {0:?}")]
    ERedisError(error_stack::Report<RedisError>),
    #[error("Received Error ParsingError: {0:?}")]
    EParsingError(error_stack::Report<ParsingError>),
    #[error("Validation Error Received: {0:?}")]
    EValidationError(error_stack::Report<ValidationError>),
    #[cfg(feature = "email")]
    #[error("Received Error EmailError: {0:?}")]
    EEmailError(error_stack::Report<EmailError>),
    #[error("Type Conversion error")]
    TypeConversionError,
    #[error("Tenant not found")]
    TenantNotFound,
}

#[macro_export]
macro_rules! error_to_process_tracker_error {
    ($($path: ident)::+ < $st: ident >, $($path2:ident)::* ($($inner_path2:ident)::+ <$st2:ident>) ) => {
        impl From<$($path)::+ <$st>> for ProcessTrackerError {
            fn from(err: $($path)::+ <$st> ) -> Self {
                $($path2)::*(err)
            }
        }
    };

    ($($path: ident)::+  <$($inner_path:ident)::+>, $($path2:ident)::* ($($inner_path2:ident)::+ <$st2:ident>) ) => {
        impl<'a> From< $($path)::+ <$($inner_path)::+> > for ProcessTrackerError {
            fn from(err: $($path)::+ <$($inner_path)::+> ) -> Self {
                $($path2)::*(err)
            }
        }
    };
}
pub trait PTError: Send + Sync + 'static {
    fn to_pt_error(&self) -> ProcessTrackerError;
}

impl<T: PTError> From<T> for ProcessTrackerError {
    fn from(value: T) -> Self {
        value.to_pt_error()
    }
}

impl PTError for ApiErrorResponse {
    fn to_pt_error(&self) -> ProcessTrackerError {
        ProcessTrackerError::EApiErrorResponse
    }
}

impl<T: PTError + std::fmt::Debug + std::fmt::Display> From<error_stack::Report<T>>
    for ProcessTrackerError
{
    fn from(error: error_stack::Report<T>) -> Self {
        logger::error!(?error);
        error.current_context().to_pt_error()
    }
}

error_to_process_tracker_error!(
    error_stack::Report<StorageError>,
    ProcessTrackerError::EStorageError(error_stack::Report<StorageError>)
);

error_to_process_tracker_error!(
    error_stack::Report<RedisError>,
    ProcessTrackerError::ERedisError(error_stack::Report<RedisError>)
);

error_to_process_tracker_error!(
    error_stack::Report<ParsingError>,
    ProcessTrackerError::EParsingError(error_stack::Report<ParsingError>)
);

error_to_process_tracker_error!(
    error_stack::Report<ValidationError>,
    ProcessTrackerError::EValidationError(error_stack::Report<ValidationError>)
);

#[cfg(feature = "email")]
error_to_process_tracker_error!(
    error_stack::Report<EmailError>,
    ProcessTrackerError::EEmailError(error_stack::Report<EmailError>)
);

error_to_process_tracker_error!(
    error_stack::Report<RecoveryError>,
    ProcessTrackerError::ERecoveryError(error_stack::Report<RecoveryError>)
);

use prost_types::FileDescriptorSet;
use tonic::Code;

use crate::{descriptor::ReflectionRequest, error::Error};

mod v1;
mod v1alpha;

/// Returns the file descriptor set from a reflection request.
pub(crate) async fn build_file_descriptor_set(
    request: ReflectionRequest,
) -> Result<FileDescriptorSet, Error> {
    let v1_result = v1::build_file_descriptor_set(request.clone()).await;
    if let Err(Error::GrpcError(status)) = &v1_result {
        if status.code == Code::Unimplemented {
            match v1alpha::build_file_descriptor_set(request).await {
                Ok(alpha_result) => return Ok(alpha_result),
                Err(_) => return v1_result,
            }
        }
    }
    v1_result
}

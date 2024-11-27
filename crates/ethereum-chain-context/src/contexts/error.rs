use core::convert::Infallible;

use alloy::transports::{RpcError, TransportErrorKind};
use beacon_api::errors::Error as BeaconError;
use cgp::prelude::*;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::types::HermesError;

pub struct HandleEthereumChainError;

delegate_components! {
    HandleEthereumChainError {
        HermesError: ReturnError,
        Infallible: HandleInfallible,
        [
            BeaconError,
            RpcError<TransportErrorKind>,
        ]:
            ReportError,
        [
            String,
            <'a> &'a str,
        ]:
            DisplayError,
    }
}

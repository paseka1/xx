// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A `CodeExecutor` specialization which uses natively compiled runtime when the wasm to be
//! executed is equivalent to the natively compiled code.

pub use sc_executor::NativeElseWasmExecutor;

// Declare an instance of the native executor named `XXNetworkExecutorDispatch`.
// Include the wasm binary as the equivalent wasm code.
#[cfg(feature = "xxnetwork")]
pub struct XXNetworkExecutorDispatch;

#[cfg(feature = "xxnetwork")]
impl sc_executor::NativeExecutionDispatch for XXNetworkExecutorDispatch {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        xxnetwork_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        xxnetwork_runtime::native_version()
    }
}

// Declare an instance of the native executor named `CanaryExecutorDispatch`.
// Include the wasm binary as the equivalent wasm code.
#[cfg(feature = "canary")]
pub struct CanaryExecutorDispatch;

#[cfg(feature = "canary")]
impl sc_executor::NativeExecutionDispatch for CanaryExecutorDispatch {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        canary_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        canary_runtime::native_version()
    }
}

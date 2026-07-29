use crate::{LabError, LabResult};
use serde_json::Value;
use std::ffi::OsString;
use std::process::Command;

/// Small abstraction over `bitcoin-cli`.
///
/// Lab functions depend on this trait so they can be tested without a live node.
pub trait RpcClient {
    /// Call a Bitcoin Core RPC and return its raw standard output.
    fn call(&self, wallet: Option<&str>, method: &str, params: &[String]) -> LabResult<String>;
}

/// Real [`RpcClient`] backed by a `bitcoin-cli` child process.
#[derive(Debug, Clone)]
pub struct ProcessRpc {
    binary: OsString,
    base_args: Vec<OsString>,
}

impl Default for ProcessRpc {
    fn default() -> Self {
        Self::new("bitcoin-cli")
    }
}

impl ProcessRpc {
    pub fn new(binary: impl Into<OsString>) -> Self {
        Self {
            binary: binary.into(),
            base_args: Vec::new(),
        }
    }

    /// Add arguments applied to every invocation, for example `-regtest`.
    pub fn with_base_args<I, S>(mut self, arguments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.base_args.extend(arguments.into_iter().map(Into::into));
        self
    }
}

impl RpcClient for ProcessRpc {
    fn call(&self, wallet: Option<&str>, method: &str, params: &[String]) -> LabResult<String> {
        let mut command = Command::new(&self.binary);
        command.args(&self.base_args);

        if let Some(wallet_name) = wallet {
            command.arg(format!("-rpcwallet={wallet_name}"));
        }

        command.arg(method);
        command.args(params);

        let output = command
            .output()
            .map_err(|error| LabError::Spawn(error.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            let message = if stderr.is_empty() { stdout } else { stderr };
            return Err(LabError::Rpc(message));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }
}

/// Parse a Bitcoin CLI response.
///
/// `bitcoin-cli` prints objects, arrays, numbers, and booleans as JSON, but commonly
/// prints string results such as addresses and hashes without surrounding quotes.
pub fn parse_cli_value(raw: &str) -> LabResult<Value> {
    let trimmed = raw.trim();
    match serde_json::from_str(trimmed) {
        Ok(value) => Ok(value),
        Err(_) if !trimmed.is_empty() => Ok(Value::String(trimmed.to_owned())),
        Err(error) => Err(LabError::Parse(error.to_string())),
    }
}

pub fn required_string(value: &Value, field: &'static str) -> LabResult<String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or(LabError::MissingField(field))
}

pub fn required_u64(value: &Value, field: &'static str) -> LabResult<u64> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or(LabError::MissingField(field))
}

pub fn required_f64(value: &Value, field: &'static str) -> LabResult<f64> {
    value
        .get(field)
        .and_then(Value::as_f64)
        .ok_or(LabError::MissingField(field))
}

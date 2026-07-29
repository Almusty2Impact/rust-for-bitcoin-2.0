#![allow(dead_code)]

use rfb_labs_week_1::rpc::RpcClient;
use rfb_labs_week_1::{LabError, LabResult};
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
struct ExpectedCall {
    wallet: Option<String>,
    method: String,
    params: Vec<String>,
    response: LabResult<String>,
}

#[derive(Debug, Default)]
pub struct MockRpc {
    calls: RefCell<VecDeque<ExpectedCall>>,
}

impl MockRpc {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expect_ok(&self, wallet: Option<&str>, method: &str, params: &[&str], response: &str) {
        self.calls.borrow_mut().push_back(ExpectedCall {
            wallet: wallet.map(ToOwned::to_owned),
            method: method.to_owned(),
            params: params.iter().map(|value| (*value).to_owned()).collect(),
            response: Ok(response.to_owned()),
        });
    }

    pub fn expect_rpc_error(
        &self,
        wallet: Option<&str>,
        method: &str,
        params: &[&str],
        message: &str,
    ) {
        self.calls.borrow_mut().push_back(ExpectedCall {
            wallet: wallet.map(ToOwned::to_owned),
            method: method.to_owned(),
            params: params.iter().map(|value| (*value).to_owned()).collect(),
            response: Err(LabError::Rpc(message.to_owned())),
        });
    }

    pub fn assert_done(&self) {
        assert!(
            self.calls.borrow().is_empty(),
            "not every expected RPC call was made: {:?}",
            self.calls.borrow()
        );
    }
}

impl RpcClient for MockRpc {
    fn call(&self, wallet: Option<&str>, method: &str, params: &[String]) -> LabResult<String> {
        let expected = self
            .calls
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| panic!("unexpected RPC call: {wallet:?} {method} {params:?}"));

        assert_eq!(expected.wallet.as_deref(), wallet, "wrong wallet context");
        assert_eq!(expected.method, method, "wrong RPC method");
        assert_eq!(expected.params, params, "wrong RPC parameters");
        expected.response
    }
}

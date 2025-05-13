//! DTOs for the 0x swap API. Full documentation for the API can be found
//! [here](https://docs.0x.org/0x-api-swap/api-references/get-swap-v1-quote).

use {
    crate::{domain::dex, util::serialize},
    bigdecimal::BigDecimal,
    ethereum_types::{H160, U256},
    serde::{Deserialize, Serialize},
    serde_with::serde_as,
};

/// A 0x API quote query parameters.
///
/// See [API](https://docs.0x.org/0x-api-swap/api-references/get-swap-v1-quote)
/// documentation for more detailed information on each parameter.
#[serde_as]
#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    /// Chain ID of the network.
    pub chain_id: u64,

    /// Contract address of a token to sell.
    pub sell_token: H160,

    /// Contract address of a token to buy.
    pub buy_token: H160,

    /// Amount of a token to sell, set in atoms.
    pub sell_amount: U256,

    /// The address which will fill the quote.
    pub taker: H160,

    /// The origin address of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_origin: Option<H160>,

    /// The target gas price for the swap transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,

    /// The slippage percentage in basis points.
    pub slippage_bps: Option<u16>,

    /// List of sources to exclude.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde_as(as = "serialize::CommaSeparated")]
    pub excluded_sources: Vec<String>,
}

/// A 0x slippage amount.
#[derive(Clone, Debug, Serialize)]
pub struct Slippage(BigDecimal);

impl Query {
    pub fn with_domain(self, order: &dex::Order, slippage: &dex::Slippage) -> Self {
        let sell_amount = order.amount.get();
        let slippage_bps = slippage.as_bps();

        Self {
            sell_token: order.sell.0,
            buy_token: order.buy.0,
            sell_amount,
            slippage_bps,
            ..self
        }
    }
}

/// A 0x API transaction data.
#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The address of the contract to call in order to execute the swap.
    pub to: H160,

    /// The swap calldata.
    #[serde_as(as = "serialize::Hex")]
    pub data: Vec<u8>,

    /// The gas limit for the transaction.
    #[serde_as(as = "serialize::U256")]
    pub gas: U256,
}

/// A Ox API quote response.
#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    /// The transaction details for executing the swap.
    pub transaction: Transaction,

    /// The amount of sell token (in atoms) that would be sold in this swap.
    #[serde_as(as = "serialize::U256")]
    pub sell_amount: U256,

    /// The amount of buy token (in atoms) that would be bought in this swap.
    #[serde_as(as = "serialize::U256")]
    pub buy_amount: U256,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Response {
    Ok(Quote),
    Err(Error),
}

impl Response {
    /// Turns the API response into a [`std::result::Result`].
    pub fn into_result(self) -> Result<Quote, Error> {
        match self {
            Response::Ok(quote) => Ok(quote),
            Response::Err(err) => Err(err),
        }
    }
}

#[derive(Deserialize)]
pub struct Error {
    pub code: i64,
    pub reason: String,
}

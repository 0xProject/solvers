use {crate::tests, std::net::SocketAddr};

mod market_order;
mod not_found;
mod options;
mod out_of_price;

/// Creates a temporary file containing the config of the given solver.
pub fn config(solver_addr: &SocketAddr) -> tests::Config {
    tests::Config::String(format!(
        r"
node-url = 'http://localhost:8545'
[dex]
chain-id = '1'
endpoint = 'http://{solver_addr}/swap/allowance-holder/'
api-key = 'SUPER_SECRET_API_KEY'
        ",
    ))
}

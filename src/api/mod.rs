//! Serve a solver engine API.

use {
    crate::domain::solver::Solver,
    axum::extract::DefaultBodyLimit,
    std::{future::Future, net::SocketAddr, sync::Arc},
    tokio::sync::oneshot,
};

mod routes;

pub struct Api {
    pub addr: SocketAddr,
    pub solver: Solver,
}

impl Api {
    pub async fn serve(
        self,
        bind: Option<oneshot::Sender<SocketAddr>>,
        shutdown: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), hyper::Error> {
        let app = axum::Router::new()
            .route("/metrics", axum::routing::get(routes::metrics))
            .route("/healthz", axum::routing::get(routes::healthz))
            .route("/solve", axum::routing::post(routes::solve))
            .layer(DefaultBodyLimit::max(5 * 1024 * 1024))
            .layer(
                tower::ServiceBuilder::new().layer(tower_http::trace::TraceLayer::new_for_http()),
            )
            .layer(DefaultBodyLimit::disable())
            .with_state(Arc::new(self.solver));

        let make_svc = observe::make_service_with_request_tracing!(app);

        let server = axum::Server::bind(&self.addr).serve(make_svc);
        if let Some(bind) = bind {
            let _ = bind.send(server.local_addr());
        }

        server.with_graceful_shutdown(shutdown).await
    }
}

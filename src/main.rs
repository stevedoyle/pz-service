use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::path!("ftp" / u32)
        .and(warp::get())
        .and_then(handlers::calc_zones);

    // Start up the server...
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}

/// The API handler at the end of the filter chain.
mod handlers {
    use std::convert::Infallible;
    use powerzones;

    pub async fn calc_zones(ftp :u32) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let zones = powerzones::calc_power_zones(ftp);
        Ok(warp::reply::json(&zones))
    }
}

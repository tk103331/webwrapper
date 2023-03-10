use salvo::prelude::*;

#[handler]
async fn hello_world() -> &'static str {
    "Hello world"
}
// orginal format
// #[handler]
// async fn hello_world(_req: &mut Request, _depot: &mut Depot, res: &mut Response, _ctrl: &mut FlowCtrl) {
//     res.render("Hello world");
// }

#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world);
    let host = "127.0.0.1:7878";
    println!("listen on {}", host);
    Server::new(TcpListener::bind(host)).serve(router).await;
}

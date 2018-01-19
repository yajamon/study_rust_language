#[derive(Copy, Clone)]
enum Method {
    Get,
    Post,
}
struct Client {
    method: Method,
}
struct ClientBuilder {
    method: Method,
}
impl ClientBuilder {
    fn new() -> Self {
        ClientBuilder {
            method: Method::Get,
        }
    }
    fn method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }
    fn finalize(&self) -> Client {
        Client {
            method: self.method,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let client = ClientBuilder::new().method(Method::Post).finalize();
}

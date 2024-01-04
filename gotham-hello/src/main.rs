use gotham::state::State;

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "Hello world!")
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    
    gotham::start("127.0.0.1:8080", || Ok(say_hello)).unwrap()

}

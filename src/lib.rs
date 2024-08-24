pub mod window;
pub mod engine;
pub mod adapter;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(crate::engine::Engine::run().is_ok())
    }
}

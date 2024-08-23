pub mod window;
pub mod engine;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(crate::engine::Engine::run().is_ok())
    }
}

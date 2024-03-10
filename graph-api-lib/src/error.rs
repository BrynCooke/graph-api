#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported graph feature: {feature}")]
    UnsupportedGraphFeature { feature: &'static str },
}

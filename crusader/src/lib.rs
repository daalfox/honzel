use async_trait::async_trait;

#[async_trait]
pub trait Creatable {
    type Input;
    type Output;
    type Error;

    async fn create(&self, n: Self::Input) -> Result<Self::Output, Self::Error>;
}

#[async_trait]
pub trait Listable {
    type Output;
    type Error;

    async fn list(&self) -> Result<Self::Output, Self::Error>;
}

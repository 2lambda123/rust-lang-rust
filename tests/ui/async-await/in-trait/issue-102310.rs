// check-pass
// edition:2021

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

pub trait SpiDevice {
    #[allow(async_fn_in_trait)]
    async fn transaction<F, R>(&mut self);
}

impl SpiDevice for () {
    async fn transaction<F, R>(&mut self) {}
}

fn main() {}

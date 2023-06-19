#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]


#[rtic::app(
    device = rp_pico::hal::pac,
)]
mod app {

    use panic_probe as _;
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init(local=[
    ])]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        (Shared {}, Local { })
    }

    #[task()]
    async fn heartbeat(ctx: heartbeat::Context) {
    }
}
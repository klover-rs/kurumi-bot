use discord_presence::{Client, Event};

pub async fn discord_rpc() -> anyhow::Result<()> {

    tokio::task::spawn(async {
        let mut drpc = Client::new(1215412721743429673);

        drpc.on_ready(|_ctx| {
            println!("ready?");
        })
        .persist();
    
        drpc.on_activity_join_request(|ctx| {
            println!("Join request: {:?}", ctx.event);
        })
        .persist();
    
        drpc.on_activity_join(|ctx| {
            println!("Joined: {:?}", ctx.event);
        })
        .persist();
    
        drpc.on_activity_spectate(|ctx| {
            println!("Spectate: {:?}", ctx.event);
        })
        .persist();
    
        drpc.start();
    
        let _ = drpc.block_until_event(Event::Ready);
    
        assert!(Client::is_ready());
    
        // Set the activity
        drpc.set_activity(|act| {
            act.state("real")
                .assets(|assets| {
                    assets.large_image("kurumi_dis")
                })
                
                
        })
        .unwrap();
    
        let _ = drpc.block_on();

    });

    Ok(())
    
}
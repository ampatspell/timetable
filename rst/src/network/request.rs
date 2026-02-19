use crate::channel::{CHANNEL, Messages};
use crate::network::parse::parse;
use defmt::info;
use embassy_net::Stack;
use embassy_net::dns::DnsSocket;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_time::{Duration, Timer};
use no_std_strings::str256;
use reqwless::client::HttpClient;

#[derive(Debug, Clone)]
struct RequestFailedError;

async fn request(stack: &Stack<'static>) -> Result<str256, RequestFailedError> {
    let dns = DnsSocket::new(*stack);
    let tcp_state = TcpClientState::<1, 4096, 4096>::new();
    let tcp = TcpClient::new(*stack, &tcp_state);

    let mut client = HttpClient::new(&tcp, &dns);
    let mut buffer = [0u8; 4096];
    let mut http_req = client
        .request(
            reqwless::request::Method::GET,
            "http://timetable.app.amateurinmotion.com",
        )
        .await
        .unwrap();

    let response = http_req.send(&mut buffer).await.unwrap();

    if response.status.is_successful() {
        let body = response.body().read_to_end().await.unwrap();
        let content = str256::from(core::str::from_utf8(body).unwrap());

        return Ok(content);
    }

    Err(RequestFailedError)
}

#[embassy_executor::task]
pub async fn request_task(stack: Stack<'static>) {
    info!("Start request_task");
    loop {
        let result = request(&stack).await;

        match result {
            Ok(s) => {
                let str = s.to_str();
                let payload = parse(str);
                info!("{}", payload);
                CHANNEL.send(Messages::Update { payload }).await;
            }
            Err(_) => {
                info!("Failed to fetch update");
            }
        }

        Timer::after(Duration::from_secs(15)).await;
    }
}

use crate::channel::{CHANNEL, Messages};
use defmt::info;
use embassy_net::Stack;
use embassy_net::dns::DnsSocket;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_time::{Duration, Timer};
use no_std_strings::str256;
use reqwless::client::HttpClient;
use reqwless::request::Method::GET;
use ui::payload::TimeData;

#[derive(Debug, Clone)]
struct RequestFailedError;

async fn request(stack: &Stack<'static>, path: &str) -> Result<str256, RequestFailedError> {
    let dns = DnsSocket::new(*stack);
    let tcp_state = TcpClientState::<1, 4096, 4096>::new();
    let tcp = TcpClient::new(*stack, &tcp_state);
    let mut url_str = str256::from("http://timetable.app.amateurinmotion.com/");
    url_str.push(path);

    let url = url_str.to_str();

    info!("Path {}", path);
    info!("GET {}", url);

    let mut client = HttpClient::new(&tcp, &dns);
    let mut buffer = [0u8; 4096];
    let mut http_req = client.request(GET, url).await.expect("HTTP request");
    let response = http_req.send(&mut buffer).await.expect("HTTP request send");

    if response.status.is_successful() {
        let body = response.body().read_to_end().await.unwrap();
        let content = str256::from(core::str::from_utf8(body).unwrap());

        return Ok(content);
    }

    Err(RequestFailedError)
}

#[embassy_executor::task]
pub async fn time_task(stack: Stack<'static>) {
    info!("Start time_task");
    loop {
        let result = request(&stack, "now").await;
        match result {
            Ok(s) => {
                let body = s.to_str();
                info!("Now:");
                info!("{}", body);
                let time = TimeData::parse(&body);
                info!("{}", time);
                CHANNEL.send(Messages::Time { time }).await;
            }
            Err(_) => {
                info!("Failed to fetch time");
            }
        }
        Timer::after(Duration::from_secs(60 * 60)).await;
    }
}

#[embassy_executor::task]
pub async fn weather_task(stack: Stack<'static>) {
    info!("Start weather_task");
    loop {
        let result = request(
            &stack,
            "weather?lat=56.95570916409245&lng=24.12422103404933",
        )
        .await;
        match result {
            Ok(s) => {
                let body = s.to_str();
                info!("Weather:");
                info!("{}", body)
                // CHANNEL.send(Messages::Time { time }).await;
            }
            Err(_) => {
                info!("Failed to fetch weather");
            }
        }
        Timer::after(Duration::from_secs(5 * 60 * 60)).await;
    }
}

#[embassy_executor::task]
pub async fn timetable_task(stack: Stack<'static>) {
    info!("Start weather_task");
    loop {
        let result = request(&stack, "timetable?route=riga_tram_1&stop=3123&direction=1").await;
        match result {
            Ok(s) => {
                let body = s.to_str();
                info!("Timetable:");
                info!("{}", body)
                // CHANNEL.send(Messages::Time { time }).await;
            }
            Err(_) => {
                info!("Failed to fetch timetable");
            }
        }
        Timer::after(Duration::from_secs(5 * 60 * 60)).await;
    }
}

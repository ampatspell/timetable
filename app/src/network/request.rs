use core::str::from_utf8;

use crate::channel::{NETWORK_CHANNEL, Network};
use crate::network::data::Weather;
use crate::time::Time;
use defmt::info;
use embassy_net::Stack;
use embassy_net::dns::DnsSocket;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_time::{Duration, Timer};
use no_std_strings::{str12, str32, str256};
use reqwless::client::HttpClient;
use reqwless::request::Method::GET;
use ui::payload::BlockPayload;

#[derive(Debug, Clone)]
struct RequestFailedError;

async fn request(stack: &Stack<'static>, path: &str) -> Result<str256, RequestFailedError> {
    let dns = DnsSocket::new(*stack);
    let tcp_state = TcpClientState::<1, 4096, 4096>::new();
    let tcp = TcpClient::new(*stack, &tcp_state);
    let mut url_str = str256::from("http://timetable.app.amateurinmotion.com/");
    url_str.push(path);
    let url = url_str.to_str();

    info!("GET {}", url);

    let mut client = HttpClient::new(&tcp, &dns);
    let mut buffer = [0u8; 4096];

    let http_req = client.request(GET, url).await;
    if http_req.is_err() {
        // match http_req.err().unwrap() {
        //     reqwless::Error::AlreadySent => info!("Already sent"),
        //     reqwless::Error::BufferTooSmall => info!("Buffer too small"),
        //     reqwless::Error::Codec => info!("Codec"),
        //     reqwless::Error::ConnectionAborted => info!("Connection aborted"),
        //     reqwless::Error::Dns => info!("DNS"),
        //     reqwless::Error::IncorrectBodyWritten => info!("Incorrect body"),
        //     reqwless::Error::InvalidUrl(..) => info!("Invalid url"),
        //     reqwless::Error::Network(arg) => info!("Network {:?}", arg),
        // }
        // debug_break();
        info!("HTTP request");
        return Err(RequestFailedError);
    }
    let mut http_req = http_req.unwrap();

    let response = http_req.send(&mut buffer).await;
    if response.is_err() {
        info!("HTTP request send");
        return Err(RequestFailedError);
    }
    let response = response.unwrap();

    if response.status.is_successful() {
        let body = response.body().read_to_end().await;
        if body.is_err() {
            info!("HTTP request read body");
            return Err(RequestFailedError);
        }
        let body = body.unwrap();

        let utf8 = from_utf8(body);
        if utf8.is_err() {
            info!("HTTP body utf8");
            return Err(RequestFailedError);
        }
        let utf8 = utf8.unwrap();

        let content = str256::from(utf8);

        info!("Ok: {}\n{}", path, utf8);

        return Ok(content);
    }

    Err(RequestFailedError)
}

fn parse_message(string: &str) -> BlockPayload {
    let mut iter = string.split("\n").into_iter();
    let icon = str12::from(iter.next().unwrap());
    let lines = [
        str32::from(iter.next().unwrap()),
        str32::from(iter.next().unwrap()),
    ];

    BlockPayload { icon, lines }
}

#[embassy_executor::task]
pub async fn message_task(stack: Stack<'static>) {
    info!("Start time_task");
    loop {
        let result = request(&stack, "message").await;
        match result {
            Ok(s) => {
                let body = s.to_str();
                let message = parse_message(&body);
                NETWORK_CHANNEL.send(Network::Message { message }).await;
                Timer::after(Duration::from_secs(60 * 15)).await;
            }
            Err(_) => {
                info!("Failed to fetch message");
                Timer::after(Duration::from_secs(60)).await;
            }
        }
    }
}

fn parse_time(string: &str) -> Time {
    let mut iter = string.split("\n").into_iter();
    let mut parse = || iter.next().unwrap().parse::<u8>().unwrap();
    let hours = parse();
    let minutes = parse();
    let seconds = parse();

    Time {
        hours,
        minutes,
        seconds,
    }
}

#[embassy_executor::task]
pub async fn time_task(stack: Stack<'static>) {
    info!("Start time_task");
    loop {
        let result = request(&stack, "now").await;
        match result {
            Ok(s) => {
                let body = s.to_str();
                let time = parse_time(&body);
                NETWORK_CHANNEL.send(Network::Time { time }).await;
                Timer::after(Duration::from_secs(60)).await;
            }
            Err(_) => {
                info!("Failed to fetch time");
                Timer::after(Duration::from_secs(15)).await;
            }
        }
    }
}

#[embassy_executor::task]
pub async fn tick_task(_stack: Stack<'static>) {
    loop {
        NETWORK_CHANNEL.send(Network::Tick).await;
        Timer::after(Duration::from_secs(1)).await;
    }
}

fn parse_weather(string: &str) -> Weather {
    let mut iter = string.split("\n").into_iter();
    let icon = str12::from(iter.next().unwrap());
    let temperature = str32::from(iter.next().unwrap());
    let description = str32::from(iter.next().unwrap());
    let uv = str32::from(iter.next().unwrap());
    let sunrise = str32::from(iter.next().unwrap());
    let sunset = str32::from(iter.next().unwrap());

    Weather {
        icon,
        temperature,
        description,
        uv,
        sunrise,
        sunset,
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
                let weather = parse_weather(&body);
                NETWORK_CHANNEL.send(Network::Weather { weather }).await;
                Timer::after(Duration::from_secs(5 * 60)).await;
            }
            Err(_) => {
                info!("Failed to fetch weather");
                Timer::after(Duration::from_secs(60)).await;
            }
        }
    }
}

fn parse_timetable(string: &str) -> [str32; 2] {
    let mut iter = string.split("\n").into_iter();
    let mut parse = || str32::from(iter.next().unwrap());
    [parse(), parse()]
}

#[embassy_executor::task]
pub async fn timetable_task(stack: Stack<'static>) {
    info!("Start timetable_task");
    loop {
        let result = request(&stack, "timetable?route=riga_tram_1&stop=3123&direction=1").await;
        match result {
            Ok(s) => {
                let body = s.to_str();
                let timetable = parse_timetable(&body);
                NETWORK_CHANNEL.send(Network::Timetable { timetable }).await;
            }
            Err(_) => {
                info!("Failed to fetch timetable");
                let timetable = [str32::from("Salūza"), str32::new()];
                NETWORK_CHANNEL.send(Network::Timetable { timetable }).await;
            }
        }
        Timer::after(Duration::from_secs(5)).await;
    }
}

use crate::constants::{WIFI_PASSWORD, WIFI_SSID};
use crate::network::request::request_task;
use defmt::info;
use embassy_executor::Spawner;
use embassy_net::{DhcpConfig, Runner, Stack, StackResources};
use embassy_time::{Duration, Timer};
use esp_hal::peripherals::WIFI;
use esp_hal::rng::Rng;
use esp_radio::Controller;
use esp_radio::wifi::{
    ClientConfig, ModeConfig, ScanConfig, WifiController, WifiDevice, WifiEvent, WifiStaState,
};
use static_cell::StaticCell;

mod request;

static RADIO_INIT: StaticCell<Controller<'static>> = StaticCell::new();

static STACK_RESOURCE: StaticCell<StackResources<3>> = StaticCell::new();

pub async fn wait_for_connection<'a>(stack: &Stack<'a>) {
    info!("Waiting for link to be up");
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    info!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            info!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

pub struct ConfigureNetworkOptions<'a> {
    pub spawner: &'a Spawner,
    pub wifi: WIFI<'static>,
}

pub async fn configure_network<'a>(opts: ConfigureNetworkOptions<'a>) {
    let ConfigureNetworkOptions { spawner, wifi } = opts;

    let radio_init = RADIO_INIT
        .uninit()
        .write(esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller"));

    let (wifi_controller, wifi_interfaces) =
        esp_radio::wifi::new(radio_init, wifi, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    let rng = Rng::new();
    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    let dhcp_config = DhcpConfig::default();

    let config = embassy_net::Config::dhcpv4(dhcp_config);
    let resources = STACK_RESOURCE.uninit().write(StackResources::<3>::new());

    let (stack, runner) = embassy_net::new(wifi_interfaces.sta, config, resources, net_seed);

    spawner.spawn(connection_task(wifi_controller)).ok();
    spawner.spawn(net_task(runner)).ok();

    wait_for_connection(&stack).await;

    spawner.spawn(request_task(stack)).ok();
}

#[embassy_executor::task]
async fn connection_task(mut controller: WifiController<'static>) {
    info!("Start connection_task");
    info!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_radio::wifi::sta_state() {
            WifiStaState::Connected => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = ModeConfig::Client(
                ClientConfig::default()
                    .with_ssid(WIFI_SSID.into())
                    .with_password(WIFI_PASSWORD.into()),
            );
            controller.set_config(&client_config).unwrap();
            info!("Starting wifi");
            controller.start_async().await.unwrap();
            info!("Wifi started!");
        }

        info!("Scan");
        let scan_config = ScanConfig::default().with_max(10);
        let result = controller
            .scan_with_config_async(scan_config)
            .await
            .unwrap();
        for ap in result {
            info!("{:?}", ap);
        }

        info!("About to connect...");
        match controller.connect_async().await {
            Ok(_) => info!("Wifi connected!"),
            Err(e) => {
                info!("Failed to connect to wifi: {:?}", e);
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    info!("Start net_task");
    runner.run().await
}

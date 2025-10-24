use reqwest::blocking::Client;
use serde::Deserialize;
use std::string::ToString;
use std::time::Duration;

pub struct RequestConfig {
    pub host: String,
    pub port: String,
    pub vehicle_name: String,
    pub timeout: Duration,
    pub debugging: bool,
}

impl RequestConfig {
    pub fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: "37337".to_string(),
            vehicle_name: "Current".to_string(),
            timeout: Duration::from_millis(300),
            debugging: false,
        }
    }
    pub fn host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn port(mut self, port: String) -> Self {
        self.port = port;
        self
    }
    pub fn vehicle_name(mut self, vehicle_name: String) -> Self {
        self.vehicle_name = vehicle_name;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    pub fn debugging(mut self, debugging: bool) -> Self {
        self.debugging = debugging;
        self
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiVehicleType {
    #[serde(rename = "ActorName")]
    pub actor_name: String,
    #[serde(rename = "VehicleModel")]
    pub vehicle_model: String,
    #[serde(rename = "IgnitionEnabled")]
    pub ignition_enabled: String,
    #[serde(rename = "EngineStarted")]
    pub engine_started: String,
    #[serde(rename = "WarningLights")]
    pub warning_lights: String,
    #[serde(rename = "PassengerDoorsOpen")]
    pub passenger_doors_open: String,
    #[serde(rename = "FixingBrake")]
    pub fixing_brake: String,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "AllowedSpeed")]
    pub allowed_speed: f32,
    #[serde(rename = "DisplayFuel")]
    pub display_fuel: f32,
    #[serde(rename = "IndicatorState")]
    pub indicator_state: i8,
    #[serde(rename = "AllLamps")]
    pub all_lamps: ApiLamps,
    #[serde(rename = "Buttons", default)]
    pub buttons: Vec<ApiButton>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiLamps {
    #[serde(rename = "LightHeadlight", alias = "LightHeadlight1")]
    pub light_main: f32,
    #[serde(rename = "LightTraveling", alias = "LightTraveling1")]
    pub traveller_light: f32,
    #[serde(rename = "Door Button 1", alias = "ButtonLight Door 1", default)]
    pub front_door_light: f32,
    #[serde(rename = "Door Button 2", alias = "ButtonLight Door 2", default)]
    pub second_door_light: f32,
    #[serde(rename = "Door Button 3", alias = "ButtonLight Door 3", default)]
    pub third_door_light: f32,
    #[serde(rename = "Door Button 4", alias = "ButtonLight Door 4", default)]
    pub fourth_door_light: f32,
    #[serde(rename = "LED StopRequest", default)]
    pub led_stop_request: f32,
    #[serde(rename = "ButtonLight BusStopBrake", default)]
    pub light_stopbrake: f32,
}

#[derive(Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ApiButton {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Tooltip", default)]
    pub tooltip: String,
    #[serde(rename = "State", default)]
    pub state: String,
    #[serde(rename = "Value", default)]
    pub value: String,
    #[serde(rename = "Actions", default)]
    pub actions: Vec<String>,
    #[serde(rename = "States", default)]
    pub states: Vec<String>,
}

impl ApiVehicleType {
    pub fn get_button(&self, name: &str) -> Option<ApiButton> {
        self.buttons.iter().find(|b| b.name == name).cloned()
    }
    pub fn get_button_state(&self, name: &str) -> String {
        self.buttons
            .iter()
            .find(|b| b.name == name)
            .map(|b| b.state.clone())
            .unwrap_or_else(|| "".to_string())
    }

    pub fn get_button_state_contains(&self, part: &str) -> String {
        self.buttons
            .iter()
            .find(|b| b.name.contains(part))
            .map(|b| b.state.clone())
            .unwrap_or_else(|| "".to_string())
    }

    pub fn filtered_buttons(&self, name: &str) -> Vec<ApiButton> {
        self.buttons
            .iter()
            .filter(|b| b.name == name)
            .cloned()
            .collect()
    }

    pub fn retain_buttons_by_name(&mut self, name: &str) {
        self.buttons.retain(|b| b.name == name);
    }

    pub fn buttons_name_state(&self) -> Vec<(String, String)> {
        self.buttons
            .iter()
            .map(|b| (b.name.clone(), b.state.clone()))
            .collect()
    }
}

#[deprecated]
pub fn getapidata(ip: &String, debug: bool) -> Result<ApiVehicleType, Box<dyn std::error::Error>> {
    let request_url = format!("http://{}:37337/Vehicles/Current", ip);

    let timeout = Duration::new(2, 0);
    let client = Client::new();

    if debug {
        eprintln!("Fetching url {} ...", &request_url);
    }

    let response = client.get(&request_url).timeout(timeout).send()?; // wir warten auf die antwort

    if !response.status().is_success() {
        Err("Error: response code")?
    }

    let value = response.json::<serde_json::Value>()?;
    if debug {
        eprintln!(
            "JSON structure:\n{}",
            serde_json::to_string_pretty(&value).unwrap()
        );
    }

    let api_vehicle: ApiVehicleType = serde_json::from_value(value).map_err(|e| {
        eprintln!("Failed to parse API response as JSON: {}", e);
        eprintln!("API endpoint: {}", request_url);
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    if debug {
        println!("{:?}", &api_vehicle);
    }
    Ok(api_vehicle)
}

pub async fn send_telemetry_bus_cmd(
    config: &RequestConfig,
    cmd: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "http://{}:{}/vehicles/{}/{}",
        config.host, config.port, config.vehicle_name, cmd
    );
    if config.debugging {
        println!("send_telemetry_bus_cmd URL: {}", url);
    }

    let _ = reqwest::Client::new()
        .get(url)
        .timeout(config.timeout)
        .send()
        .await?;

    Ok(())
}

pub async fn get_telemetry_data(
    config: &RequestConfig,
    path: &str,
) -> reqwest::Result<serde_json::Value> {
    let url = format!("http://{}:{}/{}", config.host, config.port, path);

    if config.debugging {
        println!("get_telemetry_data URL: {}", url);
    }

    let value = reqwest::Client::new()
        .get(url)
        .timeout(config.timeout)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(value)
}

pub async fn get_current_vehicle_name(config: &RequestConfig) -> String {
    let result = get_telemetry_data(&config, "player").await;

    if result.is_err() {
        return "".to_string();
    }
    let data = result.unwrap();

    if config.debugging {
        println!("get_current_vehicle_name data: {:?}", data);
    }

    let mode: Option<String> = data
        .get("Mode")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let mut mo = "".to_string();
    if let Some(ref m) = mode {
        mo = m.clone();
    }

    if mo != "Vehicle" {
        return "".to_string();
    }

    let current_vehicle: Option<String> = data
        .get("CurrentVehicle")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let mut bus = "".to_string();

    if let Some(ref cv) = current_vehicle {
        bus = cv.clone();
    }
    bus
}

pub async fn get_vehicle(
    config: &RequestConfig,
) -> Result<ApiVehicleType, Box<dyn std::error::Error>> {
    let path = format!("vehicles/{}", config.vehicle_name);

    if config.debugging {
        println!("get_vehicle path: {}", path);
    }

    let body = get_telemetry_data(&config, &path).await?;

    let mut api_vehicle: ApiVehicleType = serde_json::from_value(body).map_err(|e| {
        eprintln!("Failed to parse API response as Vehicle JSON: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    if config.debugging {
        println!("{:?}", &api_vehicle);
    }

    Ok(api_vehicle)
}

pub fn get_button_by_name(data: &serde_json::Value, name: &str) -> String {
    let ret = data
        .get("Buttons")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter().find(|entry| {
                entry
                    .get("Name")
                    .and_then(|n| n.as_str())
                    .map_or(false, |s| s == name)
            })
        })
        .and_then(|entry| entry.get("State"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    ret.unwrap_or_else(|| "".to_string())
}

//! This module handles the raw interaction with The Bus Telemetry API.

use serde::Deserialize;
use std::string::ToString;
use std::time::Duration;

/// Configuration for API requests.
pub struct RequestConfig {
    /// Host address of the telemetry server (default: "127.0.0.1").
    pub host: String,
    /// Port of the telemetry server (default: "37337").
    pub port: String,
    /// Name of the vehicle to query (default: "Current").
    pub vehicle_name: String,
    /// Model of the vehicle to query (default: "Current").
    pub vehicle_model: String,
    /// Request timeout (default: 300ms).
    pub timeout: Duration,
    /// Enable debug logging of URLs and data.
    pub debugging: bool,
}

impl RequestConfig {
    /// Creates a new `RequestConfig` with default values.
    pub fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: "37337".to_string(),
            vehicle_name: "Current".to_string(),
            vehicle_model: "Current".to_string(),
            timeout: Duration::from_millis(300),
            debugging: false,
        }
    }
    /// Sets the host address.
    pub fn host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    /// Sets the port.
    pub fn port(mut self, port: String) -> Self {
        self.port = port;
        self
    }
    /// Sets the vehicle name.
    pub fn vehicle_name(mut self, vehicle_name: String) -> Self {
        self.vehicle_name = vehicle_name;
        self
    }
    /// Sets the vehicle model.
    pub fn vehicle_model(mut self, vehicle_model: String) -> Self {
        self.vehicle_model = vehicle_model;
        self
    }

    /// Sets the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    /// Sets the debugging flag.
    pub fn debugging(mut self, debugging: bool) -> Self {
        self.debugging = debugging;
        self
    }
}

/// World telemetry data.
#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiWorldType {
    /// Name of the current level.
    #[serde(rename = "LevelName")]
    pub level_name: String,
    /// Current date and time in the game.
    #[serde(rename = "DateTime")]
    pub date_time: String,
    /// Time acceleration factor.
    #[serde(rename = "TimeFactor")]
    pub time_factor: f32,
    /// Latitude of the world origin.
    #[serde(rename = "BaseLatitude")]
    pub base_latitude: f64,
    /// Longitude of the world origin.
    #[serde(rename = "BaseLongitude")]
    pub base_longitude: f64,
}

/// Vehicle telemetry data.
#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiVehicleType {
    /// Internal actor name.
    #[serde(rename = "ActorName")]
    pub actor_name: String,
    /// Vehicle model name.
    #[serde(rename = "VehicleModel")]
    pub vehicle_model: String,
    /// Whether the ignition is enabled (string "true"/"false").
    #[serde(rename = "IgnitionEnabled")]
    pub ignition_enabled: String,
    /// Whether the engine is started (string "true"/"false").
    #[serde(rename = "EngineStarted")]
    pub engine_started: String,
    /// Whether warning lights are active (string "true"/"false").
    #[serde(rename = "WarningLights")]
    pub warning_lights: String,
    /// Whether any passenger door is open (string "true"/"false").
    #[serde(rename = "PassengerDoorsOpen")]
    pub passenger_doors_open: String,
    /// Whether the fixing (parking) brake is engaged (string "true"/"false").
    #[serde(rename = "FixingBrake")]
    pub fixing_brake: String,
    /// Current speed in km/h.
    #[serde(rename = "Speed")]
    pub speed: f32,
    /// Allowed speed limit.
    #[serde(rename = "AllowedSpeed")]
    pub allowed_speed: f32,
    /// Fuel level on display (0.0 to 1.0).
    #[serde(rename = "DisplayFuel")]
    pub display_fuel: f32,
    /// Indicator state (-1: left, 0: off, 1: right).
    #[serde(rename = "IndicatorState")]
    pub indicator_state: i8,
    /// Status of all external and internal lamps.
    #[serde(rename = "AllLamps")]
    pub all_lamps: ApiLamps,
    /// List of buttons and their states.
    #[serde(rename = "Buttons", default)]
    pub buttons: Vec<ApiButton>,
}

/// Represents various lamp intensities or states.
#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiLamps {
    /// Main headlight intensity (0.0 to 1.0).
    #[serde(
        rename = "LightHeadlight",
        alias = "LightHeadlight1",
        alias = "Light Headlight"
    )]
    pub light_main: f32,
    /// High beam / traveller light intensity (0.0 or 1.0).
    #[serde(
        rename = "LightTraveling",
        alias = "LightTraveling1",
        alias = "Light Travelling"
    )]
    pub traveller_light: f32,
    /// Front door light state.
    #[serde(rename = "Door Button 1", alias = "ButtonLight Door 1", default)]
    pub front_door_light: f32,
    /// Second door light state.
    #[serde(rename = "Door Button 2", alias = "ButtonLight Door 2", default)]
    pub second_door_light: f32,
    /// Third door light state.
    #[serde(rename = "Door Button 3", alias = "ButtonLight Door 3", default)]
    pub third_door_light: f32,
    /// Fourth door light state.
    #[serde(rename = "Door Button 4", alias = "ButtonLight Door 4", default)]
    pub fourth_door_light: f32,
    /// Stop request LED intensity.
    #[serde(rename = "LED StopRequest", default)]
    pub led_stop_request: f32,
    /// Bus stop brake light intensity.
    #[serde(rename = "ButtonLight BusStopBrake", default)]
    pub light_stopbrake: f32,
}

/// Represents a button in the vehicle and its current state.
#[derive(Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ApiButton {
    /// Button name.
    #[serde(rename = "Name")]
    pub name: String,
    /// Tooltip text for the button.
    #[serde(rename = "Tooltip", default)]
    pub tooltip: String,
    /// Current state of the button (e.g. "on", "off", "Drive", "Neutral").
    #[serde(rename = "State", default)]
    pub state: String,
    /// Numeric value as string, if applicable.
    #[serde(rename = "Value", default)]
    pub value: String,
    /// Possible actions for this button.
    #[serde(rename = "Actions", default)]
    pub actions: Vec<String>,
    /// Possible states for this button.
    #[serde(rename = "States", default)]
    pub states: Vec<String>,
}

impl ApiVehicleType {
    /// Returns the button with the given name, if found.
    pub fn get_button(&self, name: &str) -> Option<ApiButton> {
        self.buttons.iter().find(|b| b.name == name).cloned()
    }
    /// Returns the state of the button with the given name, or an empty string if not found.
    pub fn get_button_state(&self, name: &str) -> String {
        self.buttons
            .iter()
            .find(|b| b.name == name)
            .map(|b| b.state.clone())
            .unwrap_or_else(|| "".to_string())
    }

    /// Returns the state of the first button whose name contains the given part.
    pub fn get_button_state_contains(&self, part: &str) -> String {
        self.buttons
            .iter()
            .find(|b| b.name.contains(part))
            .map(|b| b.state.clone())
            .unwrap_or_else(|| "".to_string())
    }

    /// Returns all buttons with the exact given name.
    pub fn filtered_buttons(&self, name: &str) -> Vec<ApiButton> {
        self.buttons
            .iter()
            .filter(|b| b.name == name)
            .cloned()
            .collect()
    }

    /// Keeps only the buttons with the exact given name in the vehicle.
    pub fn retain_buttons_by_name(&mut self, name: &str) {
        self.buttons.retain(|b| b.name == name);
    }

    /// Returns a vector of tuples containing (name, state) for all buttons.
    pub fn buttons_name_state(&self) -> Vec<(String, String)> {
        self.buttons
            .iter()
            .map(|b| (b.name.clone(), b.state.clone()))
            .collect()
    }
}

/// Sends a command to the vehicle via the telemetry API.
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

/// Fetches raw JSON telemetry data from a specific API path.
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

/// Returns the current vehicle name from the "player" telemetry endpoint.
/// Returns an empty string if the player is not in a vehicle or if the request fails.
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

/// Fetches telemetry data for the vehicle specified in `config`.
pub async fn get_vehicle(
    config: &RequestConfig,
) -> Result<ApiVehicleType, Box<dyn std::error::Error>> {
    let path = format!("vehicles/{}", config.vehicle_name);

    if config.debugging {
        println!("get_vehicle path: {}", path);
    }

    let body = get_telemetry_data(&config, &path).await?;

    let api_vehicle: ApiVehicleType = serde_json::from_value(body).map_err(|e| {
        eprintln!("Failed to parse API response as Vehicle JSON: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    if config.debugging {
        println!("{:?}", &api_vehicle);
    }

    Ok(api_vehicle)
}

/// Fetches world telemetry data (time, weather, etc).
pub async fn get_world(config: &RequestConfig) -> Result<ApiWorldType, Box<dyn std::error::Error>> {
    let path = "world";

    if config.debugging {
        println!("get_world path: {}", path);
    }

    let body = get_telemetry_data(&config, &path).await?;

    let api_world: ApiWorldType = serde_json::from_value(body).map_err(|e| {
        eprintln!("Failed to parse API response as World JSON: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    if config.debugging {
        println!("{:?}", &api_world);
    }
    Ok(api_world)
}

/// Extracts the state of a button by name from a raw JSON value containing a "Buttons" array.
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

//! This crate provides a client for The Bus Telemetry API.
//! It fetches telemetry data from the game "The Bus" and maps it to common vehicle states.

// This file exposes the modules used by both binary targets and integration tests
pub mod api;
pub mod api2vehicle;

pub use api::ApiButton;
pub use api::ApiLamps;
pub use api::ApiVehicleType;
pub use api::ApiWorldType;
pub use api::RequestConfig;
pub use api::get_current_vehicle_name;
pub use api::get_vehicle;
pub use api::get_world;
pub use api::send_telemetry_bus_cmd;
pub use api::get_telemetry_data;
pub use api::get_button_by_name;

pub use api2vehicle::get_vehicle_state_from_api;


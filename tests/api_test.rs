#[cfg(test)]
mod tests {
    use serde_json::json;
    use the_bus_telemetry::api::{get_button_by_name, ApiLamps, ApiVehicleType};

    #[test]
    fn test_api_lamps_deserialization() {
        let json_data = json!({
            "LightHeadlight": 1.0,
            "LightTraveling": 0.5,
            "ButtonLight Door 1": 0.0,
            "ButtonLight Door 2": 1.0,
            "LED StopRequest": 0.0,
            "ButtonLight BusStopBrake": 1.0
        });

        let lamps: ApiLamps = serde_json::from_value(json_data).unwrap();

        assert_eq!(lamps.light_main, 1.0);
        assert_eq!(lamps.traveller_light, 0.5);
        assert_eq!(lamps.front_door_light, 0.0);
        assert_eq!(lamps.second_door_light, 1.0);
        assert_eq!(lamps.led_stop_request, 0.0);
        assert_eq!(lamps.light_stopbrake, 1.0);
    }

    #[test]
    fn test_api_lamps_alias_deserialization() {
        // Test the alias functionality for LightTraveling1
        let json_data = json!({
            "LightHeadlight": 1.0,
            "LightTraveling1": 0.5,  // Using the alias
            "Door Button 1": 0.0,    // Using the alias
            "ButtonLight Door 2": 1.0,
            "LED StopRequest": 0.0,
            "ButtonLight BusStopBrake": 1.0
        });

        let lamps: ApiLamps = serde_json::from_value(json_data).unwrap();

        assert_eq!(lamps.traveller_light, 0.5);
        assert_eq!(lamps.front_door_light, 0.0);
    }

    #[test]
    fn test_api_vehicle_type_deserialization() {
        let json_data = json!({
            "ActorName": "TestVehicle",
            "VehicleModel": "TestVehicleModel",
            "IgnitionEnabled": "True",
            "EngineStarted": "True",
            "WarningLights": "False",
            "PassengerDoorsOpen": "False",
            "FixingBrake": "False",
            "Speed": 50.5,
            "AllowedSpeed": 60.0,
            "DisplayFuel": 75.5,
            "IndicatorState": 0,
            "AllLamps": {
                "LightHeadlight": 1.0,
                "LightTraveling": 0.5,
                "ButtonLight Door 1": 0.0,
                "ButtonLight Door 2": 1.0,
                "LED StopRequest": 0.0,
                "ButtonLight BusStopBrake": 1.0
            }
        });

        let vehicle: ApiVehicleType = serde_json::from_value(json_data).unwrap();

        assert_eq!(vehicle.actor_name, "TestVehicle");
        assert_eq!(vehicle.vehicle_model, "TestVehicleModel");
        assert_eq!(vehicle.ignition_enabled, "True");
        assert_eq!(vehicle.engine_started, "True");
        assert_eq!(vehicle.warning_lights, "False");
        assert_eq!(vehicle.passenger_doors_open, "False");
        assert_eq!(vehicle.fixing_brake, "False");
        assert_eq!(vehicle.speed, 50.5);
        assert_eq!(vehicle.allowed_speed, 60.0);
        assert_eq!(vehicle.display_fuel, 75.5);
        assert_eq!(vehicle.indicator_state, 0);

        // Check the nested ApiLamps struct
        assert_eq!(vehicle.all_lamps.light_main, 1.0);
        assert_eq!(vehicle.all_lamps.traveller_light, 0.5);
        assert_eq!(vehicle.all_lamps.front_door_light, 0.0);
        assert_eq!(vehicle.all_lamps.second_door_light, 1.0);
        assert_eq!(vehicle.all_lamps.led_stop_request, 0.0);
        assert_eq!(vehicle.all_lamps.light_stopbrake, 1.0);
    }

    #[test]
    fn test_api_json_parsing_error() {
        // Test the error handling for invalid JSON
        let invalid_json = json!({
            // Missing required fields
            "ActorName": "TestVehicle",
            // Other fields are missing
        });

        let result = serde_json::from_value::<ApiVehicleType>(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_api_json_parsing_success() {
        // Test successful JSON parsing
        let valid_json = json!({
            "ActorName": "TestVehicle",
            "VehicleModel": "TestVehicleModel",
            "IgnitionEnabled": "True",
            "EngineStarted": "True",
            "WarningLights": "False",
            "PassengerDoorsOpen": "False",
            "FixingBrake": "False",
            "Speed": 50.5,
            "AllowedSpeed": 60.0,
            "DisplayFuel": 75.5,
            "IndicatorState": 0,
            "AllLamps": {
                "LightHeadlight": 1.0,
                "LightTraveling": 0.5,
                "ButtonLight Door 1": 0.0,
                "ButtonLight Door 2": 1.0,
                "LED StopRequest": 0.0,
                "ButtonLight BusStopBrake": 1.0
            }
        });

        let result = serde_json::from_value::<ApiVehicleType>(valid_json);
        assert!(result.is_ok());

        let vehicle = result.unwrap();
        assert_eq!(vehicle.actor_name, "TestVehicle");
        assert_eq!(vehicle.speed, 50.5);
    }

    // Test for default values in ApiLamps
    #[test]
    fn test_api_lamps_default_values() {
        // Test that default values are used when fields are missing
        let json_data = json!({
            "LightHeadlight": 1.0,
            "LightTraveling": 0.5,
            "ButtonLight Door 1": 0.0
            // Missing: "ButtonLight Door 2", "LED StopRequest", "ButtonLight BusStopBrake"
        });

        let lamps: ApiLamps = serde_json::from_value(json_data).unwrap();

        // These fields have default values (0.0) when missing
        assert_eq!(lamps.second_door_light, 0.0);
        assert_eq!(lamps.led_stop_request, 0.0);
        assert_eq!(lamps.light_stopbrake, 0.0);
    }

    #[test]
    fn test_get_button_by_name_wiper() {
        use std::fs;
        let path = "tests/json/mb_ecitaro.json";
        let file = fs::read_to_string(path).expect("mb_ecitaro.json not found");
        let data: serde_json::Value = serde_json::from_str(&file).expect("invalid json");

        // existing
        let state = get_button_by_name(&data, "Wiper");
        assert_eq!(state.as_str(), "Off");

        // non-existing
        let none_state = get_button_by_name(&data, "__does_not_exist__");
        assert_eq!(none_state.as_str(), "");
    }
    #[test]
    fn test_get_button_by_name_gear_selector() {
        use std::fs;
        let path = "tests/json/mb_ecitaro.json";
        let file = fs::read_to_string(path).expect("mb_ecitaro.json not found");
        let data: serde_json::Value = serde_json::from_str(&file).expect("invalid json");

        // existing
        let state = get_button_by_name(&data, "Gear Selector");
        assert_eq!(state.as_str(), "Reverse");
    }

    #[test]
    fn test_get_button_by_name_gear_selector_solaris_urbino_18m_4d() {
        use std::fs;
        let path = "tests/json/BP_Solaris_Urbino_18m_4D_C.json";
        let file = fs::read_to_string(path).expect("BP_Solaris_Urbino_18m_4D_C.json not found");
        let data: serde_json::Value = serde_json::from_str(&file).expect("invalid json");

        // existing
        let state = get_button_by_name(&data, "Gear Selector");
        assert_eq!(state.as_str(), "Neutral");
    }

    #[test]
    fn test_get_button_by_name_gear_selector_solaris_urbino() {
        use std::fs;
        let path = "tests/json/Solaris_Urbino.txt";
        let file = fs::read_to_string(path).expect("Solaris_Urbino.txt not found");
        let data: serde_json::Value = serde_json::from_str(&file).expect("invalid json");

        // existing
        let state = get_button_by_name(&data, "Gear Selector");
        assert_eq!(state.as_str(), "Neutral");
    }
}

#[test]
fn test_api_vehicle_get_button_state_wiper() {
    use std::fs;
    use the_bus_telemetry::api::ApiVehicleType;
    let path = "tests/json/mb_ecitaro.json";
    let file = fs::read_to_string(path).expect("mb_ecitaro.json not found");
    let vehicle: ApiVehicleType = serde_json::from_str(&file).expect("invalid json");
    let state = vehicle.get_button_state("Wiper");
    assert_eq!(state.as_str(), "Off");
}

#[test]
fn test_api_vehicle_get_button_state_gear_selector() {
    use std::fs;
    use the_bus_telemetry::api::ApiVehicleType;

    // eCitaro
    let file = fs::read_to_string("tests/json/mb_ecitaro.json").expect("mb_ecitaro.json not found");
    let vehicle: ApiVehicleType = serde_json::from_str(&file).expect("invalid json");
    let state = vehicle.get_button_state("Gear Selector");
    assert_eq!(state.as_str(), "Reverse");

    // Solaris Urbino 18m 4D
    let file2 = fs::read_to_string("tests/json/BP_Solaris_Urbino_18m_4D_C.json")
        .expect("BP_Solaris_Urbino_18m_4D_C.json not found");
    let vehicle2: ApiVehicleType = serde_json::from_str(&file2).expect("invalid json");
    let state2 = vehicle2.get_button_state("Gear Selector");
    assert_eq!(state2.as_str(), "Neutral");

    // Non-existing button
    let none_state = vehicle.get_button_state("__does_not_exist__");
    assert_eq!(none_state.as_str(), "");
}

#[test]
fn test_api_vehicle_filtered_buttons_and_retain() {
    use std::fs;
    use the_bus_telemetry::api::ApiVehicleType;

    let file = fs::read_to_string("tests/json/mb_ecitaro.json").expect("mb_ecitaro.json not found");
    let mut vehicle: ApiVehicleType = serde_json::from_str(&file).expect("invalid json");

    // Non-mutating filter
    let wiper_buttons = vehicle.filtered_buttons("Wiper");
    assert!(wiper_buttons.iter().all(|b| b.name == "Wiper"));
    assert!(!wiper_buttons.is_empty());

    // In-place retain
    vehicle.retain_buttons_by_name("Wiper");
    assert!(vehicle.buttons.iter().all(|b| b.name == "Wiper"));
    assert_eq!(vehicle.buttons.len(), wiper_buttons.len());

    // Retain on name that does not exist -> becomes empty
    vehicle.retain_buttons_by_name("__does_not_exist__");
    assert!(vehicle.buttons.is_empty());
}

#[test]
fn test_api_vehicle_buttons_name_state() {
    use std::fs;
    use the_bus_telemetry::api::ApiVehicleType;

    let file = fs::read_to_string("tests/json/mb_ecitaro.json").expect("mb_ecitaro.json not found");
    let vehicle: ApiVehicleType = serde_json::from_str(&file).expect("invalid json");

    let pairs = vehicle.buttons_name_state();

    // The method should return as many entries as original buttons
    assert_eq!(pairs.len(), vehicle.buttons.len());

    // It should contain at least these known buttons with expected states
    assert!(pairs.contains(&("Wiper".to_string(), "Off".to_string())));
    assert!(pairs.contains(&("Gear Selector".to_string(), "Reverse".to_string())));
}

#[test]
fn test_api_vehicle_get_button_state_contains() {
    use std::fs;
    use the_bus_telemetry::api::ApiVehicleType;

    // eCitaro
    let file = fs::read_to_string("tests/json/mb_ecitaro.json").expect("mb_ecitaro.json not found");
    let vehicle: ApiVehicleType = serde_json::from_str(&file).expect("invalid json");

    // Partial matches
    assert_eq!(vehicle.get_button_state_contains("Wip"), "Off"); // matches "Wiper"
    assert_eq!(vehicle.get_button_state_contains("Gear"), "Reverse"); // matches "Gear Selector"

    // Not found
    assert_eq!(vehicle.get_button_state_contains("__nope__"), "");

    // Solaris Urbino 18m 4D
    let file2 = fs::read_to_string("tests/json/BP_Solaris_Urbino_18m_4D_C.json")
        .expect("BP_Solaris_Urbino_18m_4D_C.json not found");
    let vehicle2: ApiVehicleType = serde_json::from_str(&file2).expect("invalid json");
    assert_eq!(vehicle2.get_button_state_contains("Gear"), "Neutral");
}

#[test]
fn test_api_vehicle_get_button_returns_struct() {
    use std::fs;
    use the_bus_telemetry::api::ApiVehicleType;

    // eCitaro
    let file = fs::read_to_string("tests/json/mb_ecitaro.json").expect("mb_ecitaro.json not found");
    let vehicle: ApiVehicleType = serde_json::from_str(&file).expect("invalid json");

    // Existing: Wiper
    let wiper = vehicle.get_button("Wiper");
    assert!(wiper.is_some());
    let w = wiper.unwrap();
    assert_eq!(w.name, "Wiper");
    assert_eq!(w.state, "Off");

    // Existing: Gear Selector
    let gear = vehicle.get_button("Gear Selector");
    assert!(gear.is_some());
    assert_eq!(gear.as_ref().unwrap().state, "Reverse");

    // Not found
    assert!(vehicle.get_button("__does_not_exist__").is_none());

    // Solaris Urbino 18m 4D
    let file2 = fs::read_to_string("tests/json/BP_Solaris_Urbino_18m_4D_C.json")
        .expect("BP_Solaris_Urbino_18m_4D_C.json not found");
    let vehicle2: ApiVehicleType = serde_json::from_str(&file2).expect("invalid json");
    let gear2 = vehicle2.get_button("Gear Selector");
    assert!(gear2.is_some());
    assert_eq!(gear2.unwrap().state, "Neutral");
}

use std::fs;
use std::path::Path;
use the_bus_telemetry::api::{ApiVehicleType, ApiWorldType};
use serde_json;
use komsi::komsi::KomsiDateTime;


#[test]
fn test_json_deserialization_from_files() {
    test_vehicle_deserialization("tests/json/BP_Mercedes_eCitaro_12m_2Door_C.json", "BP_Mercedes_eCitaro_12m_2Door_C_2147297308");
    test_vehicle_deserialization("tests/json/BP_Solaris_Urbino_18m_4D_C.json", "BP_Solaris_Urbino_18m_4D_C_2147441325");

    test_vehicle_deserialization("tests/json/man_lionscity.json", "BP_MAN_LionsCityDD_Base_C_2147417439");
    test_vehicle_deserialization("tests/json/scania_citywide.json", "BP_Scania_Citywide_12M2D_C_2147248282");
    test_vehicle_deserialization("tests/json/solaris_urbino.json", "BP_Solaris_Urbino_12m_2D_C_2147468046");
    test_vehicle_deserialization("tests/json/Solaris_Urbino.txt", "BP_Solaris_Urbino_18m_3D_C_2146245266");
    test_vehicle_deserialization("tests/json/vdl_citea.json", "BP_VDL_Citea_LLE_120_2D_C_2147124848");
    
    test_world_deserialization("tests/json/world.json");
}

fn test_world_deserialization(file_path: &str) {
    let file = Path::new(file_path);
    let json = fs::read_to_string(file)
        .expect(&format!("Failed to read {}", file_path));

    let world: ApiWorldType = serde_json::from_str(&json)
        .expect(&format!("Failed to deserialize {}", file_path));

    assert_eq!(world.level_name, "Castrop");
    assert_eq!(world.date_time, "2026-01-01T09:43:48");
    assert!(world.time_factor > 0.0);

    let date_time = KomsiDateTime::from_iso(&world.date_time).unwrap();
    assert_eq!(date_time.year, 2026);
    assert_eq!(date_time.month, 1);
    assert_eq!(date_time.day, 1);
    assert_eq!(date_time.hour, 9);
    assert_eq!(date_time.min, 43);
    assert_eq!(date_time.sec, 48);

    println!("Successfully deserialized world: {}", world.level_name);
}

fn test_vehicle_deserialization(file_path: &str, actor_name: &str) {
    // Create path
    let file = Path::new(file_path);

    // Read and deserialize the JSON file
    let json = fs::read_to_string(file)
        .expect(&format!("Failed to read {}", file_path));

    let vehicle: ApiVehicleType = serde_json::from_str(&json)
        .expect(&format!("Failed to deserialize {}", file_path));

    // Basic validation
    assert!(!vehicle.actor_name.is_empty(), "Actor name should not be empty");
    assert_eq!(vehicle.actor_name, actor_name, "Actor name should match the expected value");

    // Print success message
    println!("Successfully deserialized \"{}\" model: {}", vehicle.vehicle_model, vehicle.actor_name);

    // Validate specific fields
    validate_vehicle(&vehicle, actor_name);
}

fn validate_vehicle(vehicle: &ApiVehicleType, name: &str) {
    // Validate common fields that should be present in any vehicle
    assert!(vehicle.speed >= 0.0, "{} speed should be non-negative", name);
    assert!(vehicle.allowed_speed >= 0.0, "{} allowed speed should be non-negative", name);
    assert!(vehicle.display_fuel >= 0.0 && vehicle.display_fuel <= 100.0, 
            "{} fuel should be between 0 and 100", name);

    // Validate indicator state
    assert!((-1..=2).contains(&vehicle.indicator_state), 
            "{} indicator state should be between -1 and 2", name);

    // Validate lamps
    let lamps = &vehicle.all_lamps;
    assert!((0.0..=1.0).contains(&lamps.light_main), 
            "{} headlight should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.traveller_light), 
            "{} traveling light should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.front_door_light), 
            "{} front door light should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.second_door_light), 
            "{} second door light should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.led_stop_request), 
            "{} LED stop request should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.light_stopbrake), 
            "{} stop brake light should be between 0 and 1", name);
}

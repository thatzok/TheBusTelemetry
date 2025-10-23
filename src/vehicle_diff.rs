use crate::vehicle::VehicleState;

// Helper function for handling u8 field changes
fn handle_u8_field_change(old_value: u8, new_value: u8, field_name: &str, force: bool) {
    if (old_value != new_value) || force {
        println!("{}: {} -> {} ", field_name, old_value, new_value);
    }
}

// Helper function for handling u32 field changes
fn handle_u32_field_change(old_value: u32, new_value: u32, field_name: &str, force: bool) {
    if (old_value != new_value) || force {
        println!("{}:  {} -> {} ", field_name, old_value, new_value);
    }
}

pub fn compare_vehicle_states(old: &VehicleState, new: &VehicleState, force: bool) {
    // Handle u8 fields
    handle_u8_field_change(old.ignition, new.ignition, "ignition", force);
    handle_u8_field_change(old.engine, new.engine, "engine", force);
    handle_u8_field_change(old.doors, new.doors, "doors", force);
    handle_u8_field_change(old.fixing_brake, new.fixing_brake, "fixing_brake", force);
    handle_u8_field_change(old.indicator, new.indicator, "indicator", force);
    handle_u8_field_change(
        old.lights_warning,
        new.lights_warning,
        "lights_warning",
        force,
    );
    handle_u8_field_change(old.lights_main, new.lights_main, "lights_main", force);
    handle_u8_field_change(
        old.lights_stop_request,
        new.lights_stop_request,
        "lights_stop_request",
        force,
    );
    handle_u8_field_change(
        old.lights_stop_brake,
        new.lights_stop_brake,
        "lights_stop_brake",
        force,
    );
    handle_u8_field_change(
        old.lights_front_door,
        new.lights_front_door,
        "lights_front_door",
        force,
    );
    handle_u8_field_change(
        old.lights_second_door,
        new.lights_second_door,
        "lights_second_door",
        force,
    );
    handle_u8_field_change(
        old.lights_high_beam,
        new.lights_high_beam,
        "lights_high_beam",
        force,
    );
    handle_u8_field_change(old.battery_light, new.battery_light, "batterylight", force);
    handle_u8_field_change(old.gear_selector, new.gear_selector, "gearselector", force);

    // Handle u32 fields
    handle_u32_field_change(old.fuel, new.fuel, "fuel", force);
    handle_u32_field_change(old.speed, new.speed, "speed", force);
    handle_u32_field_change(old.maxspeed, new.maxspeed, "maxspeed", force);
}

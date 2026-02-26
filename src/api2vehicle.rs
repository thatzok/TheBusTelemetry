use crate::api::ApiVehicleType;
use komsi::vehicle::VehicleState;

pub fn get_vehicle_state_from_api(av: ApiVehicleType) -> VehicleState {
    let mut s = VehicleState::default();

    match av.ignition_enabled.as_str() {
        "true" => s.ignition = true,
        _ => s.ignition = false,
    }

    match av.engine_started.as_str() {
        "true" => s.engine = true,
        _ => s.engine = false,
    }

    match av.warning_lights.as_str() {
        "true" => s.lights_warning = true,
        _ => s.lights_warning = false,
    }

    match av.passenger_doors_open.as_str() {
        "true" => s.doors = true,
        _ => s.doors = false,
    }

    match av.fixing_brake.as_str() {
        "true" => s.fixing_brake = true,
        _ => s.fixing_brake = false,
    }

    // we only check if set, not in which direction (in api: -1,0,1 for left,off,right)
    match av.indicator_state {
        0 => s.indicator = 0,  // off
        -1 => s.indicator = 1, // on left
        1 => s.indicator = 2,  // on right
        _ => s.indicator = 0,
    }

    let gear_selector = av.get_button_state("Gear Selector");
    match gear_selector.as_str() {
        "Drive" => s.gear_selector = 1,
        "Neutral" => s.gear_selector = 2,
        "Reverse" => s.gear_selector = 3,
        _ => s.gear_selector = 2,
    }

    s.speed = av.speed.abs().round() as u32;
    s.maxspeed = av.allowed_speed.abs().round() as u32;

    s.fuel = (av.display_fuel * 100.0).trunc() as u8;

    s.lights_main = av.all_lamps.light_main > 0.0;
    s.lights_high_beam = av.all_lamps.traveller_light > 0.0;
    s.lights_front_door = av.all_lamps.front_door_light > 0.0;
    s.lights_second_door = av.all_lamps.second_door_light > 0.0;
    s.lights_third_door = av.all_lamps.third_door_light > 0.0;
    s.lights_fourth_door = av.all_lamps.fourth_door_light > 0.0;
    s.lights_stop_request = av.all_lamps.led_stop_request > 0.0;
    s.lights_stop_brake = av.all_lamps.light_stopbrake > 0.0;

    return s;
}

#[derive(Debug)]
pub struct VehicleState {
    pub ignition: u8,
    pub engine: u8,
    pub doors: u8,
    pub speed: u32,
    pub maxspeed: u32,
    pub fuel: u32,
    pub indicator: u8,
    pub fixing_brake: u8,
    pub lights_warning: u8,
    pub lights_main: u8,
    pub lights_front_door: u8,
    pub lights_second_door: u8,
    pub lights_stop_request: u8,
    pub lights_stop_brake: u8,
    pub lights_high_beam: u8,
    pub battery_light: u8,
    pub gear_selector: u8,
}

pub fn print_vehicle_state(v: &VehicleState) {
    print!("ignition:{} ", v.ignition);
    print!("engine:{} ", v.engine);
    print!("indicator:{} ", v.indicator);
    print!("fuel:{} ", v.fuel);
    print!("warn:{} ", v.lights_warning);
    print!("lights:{} ", v.lights_main);
    print!("lights-highbeam:{} ", v.lights_high_beam);
    print!("stop:{} ", v.lights_stop_request);
    print!("fixingbrake:{} ", v.fixing_brake);
    print!("stopbrake:{} ", v.lights_stop_brake);
    print!("doors:{} ", v.doors);
    print!("door1:{} ", v.lights_front_door);
    print!("door2:{} ", v.lights_second_door);
    print!("speed:{} ", v.speed);
    print!("maxspeed:{} ", v.maxspeed);
    print!("batterylight:{} ", v.battery_light);
    print!("gear-selector:{} ", v.gear_selector);

    println!(" ");
}

pub fn init_vehicle_state() -> VehicleState {
    let s = VehicleState {
        ignition: 0,
        engine: 0,
        doors: 0,
        speed: 0,
        indicator: 0,
        fixing_brake: 0,
        lights_warning: 0,
        lights_main: 0,
        lights_front_door: 0,
        lights_second_door: 0,
        lights_stop_request: 0,
        maxspeed: 0,
        lights_high_beam: 0,
        fuel: 0,
        lights_stop_brake: 0,
        battery_light: 0,
        gear_selector: 2,
    };
    return s;
}

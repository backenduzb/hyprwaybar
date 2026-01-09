use std::fs;

pub fn get_baterry() -> Option<(u8, bool)>{
    let cap = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").ok()?;
    let status = fs::read_to_string("/sys/class/power_supply/BAT0/status").ok()?;
    
    let precent = cap.trim().parse::<u8>().ok()?;
    let charging = status.trim() == "Charging";
    Some((precent, charging))
}

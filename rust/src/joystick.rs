use gdnative::prelude::Input;

#[derive(Debug)]
pub enum Joytype {
    Keyboard,
    Xbox,
    Psx,
    Nintendo,
}

impl Joytype {
    pub fn get() -> Self {
        let input = Input::godot_singleton();
        let joyname = input.get_joy_name(0).to_string().to_lowercase();
        match joyname.as_ref() {
            "" => Joytype::Keyboard,
            name if is_psx(name) => Joytype::Psx,
            name if is_nintendo(name) => Joytype::Nintendo,
            // Fallback to Xbox layout
            _ => Joytype::Xbox,
        }
    }

    pub fn is_keyboard(&self) -> bool {
        match self {
            Joytype::Keyboard => true,
            _ => false,
        }
    }
}

fn is_psx(name: &str) -> bool {
    name == "ps4 controller"
        || name == "ps5_controller"
        || name.find("dualshock").is_some()
        || name.find("sony").is_some()
}

fn is_nintendo(name: &str) -> bool {
    name.find("nintendo").is_some() || name.find("pro controller").is_some()
}

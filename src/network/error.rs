use bevy::prelude::EventReader;
use bevy_renet::renet::RenetError;

// If any error is found we just panic
pub fn panic_on_error_system(mut renet_error: EventReader<RenetError>) {
    for e in renet_error.iter() {
        panic!("{}", e);
    }
}

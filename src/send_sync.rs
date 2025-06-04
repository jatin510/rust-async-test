use std::sync::Arc;
use std::thread;

trait Vehicle: Send + Sync {
    fn start(&self);
}

struct Car {}

impl Car {
    fn new() -> Car {
        Self {}
    }
}
struct Bike {}
impl Bike {
    fn new() -> Bike {
        Self {}
    }
}
impl Vehicle for Car {
    fn start(&self) {
        println!("starting car");
    }
}

impl Vehicle for Bike {
    fn start(&self) {
        println!("starting bike");
    }
}

pub fn run_send_sync() {
    let vehicles: Vec<Box<dyn Vehicle>> = vec![Box::new(Car::new()), Box::new(Bike::new())];

    vehicles.into_iter().for_each(|vehicle| {
        thread::spawn(move || vehicle.start())
            .join()
            .expect("Failed to start the vehicle")
    });

    let car: Arc<dyn Vehicle> = Arc::new(Car::new());

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let car_shared = Arc::clone(&car);

            thread::spawn(move || {
                car_shared.start();
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("failed");
    }
}

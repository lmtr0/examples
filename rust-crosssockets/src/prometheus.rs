use std::{time::Duration};

use prometheus::{Encoder, register_int_counter_vec, IntCounterVec};
use prometheus_static_metric::{make_auto_flush_static_metric, auto_flush_from};

use lazy_static::lazy_static;

make_auto_flush_static_metric! {
    pub label_enum ExampleCounter { app1, app2 }

    pub struct Lhrs: LocalIntCounter {
        "app" => ExampleCounter,
    }
}

lazy_static! {
    pub static ref COUNTER_VEC: IntCounterVec =
        register_int_counter_vec ! (
            "my_example_coutner",
            "Number of Shits I'm doing.",
            & ["app"]    // it doesn't matter for the label order
        ).unwrap();
}

lazy_static! {
    static ref EXAMPLE_COUNTER: Lhrs = auto_flush_from!(COUNTER_VEC, Lhrs, std::time::Duration::from_secs(1));
}

fn main() {
    
    for _ in 0..10 {
        EXAMPLE_COUNTER.app1.inc_by(10);
        EXAMPLE_COUNTER.app2.inc();
        EXAMPLE_COUNTER.flush();
    }
    
    print_metrics();
    EXAMPLE_COUNTER.get(ExampleCounter::app1).reset();
    
    
    for _ in 0..10 {
        EXAMPLE_COUNTER.app1.inc_by(20);
        EXAMPLE_COUNTER.app2.inc();
    }
    
    std::thread::sleep(Duration::from_secs(3));

    print_metrics();
}

fn print_metrics() {
    let mut buffer = Vec::<u8>::new();
    let encoder = prometheus::TextEncoder::new();
    encoder
        .encode(&prometheus::gather(), &mut buffer)
        .unwrap();
    println!("## Custom registry");
    println!("{}", String::from_utf8(buffer.clone()).unwrap());
}
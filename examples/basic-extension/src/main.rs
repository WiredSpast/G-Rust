use g_rust::extension::extension::Extension;
use g_rust::extension::parsers::incoming::{WiredFurniAction, WiredFurniCondition, WiredFurniSelector, WiredFurniTrigger};
use g_rust::extension::parsers::outgoing::{UpdateCondition, UpdateSelector};
use g_rust::protocol::hmessage::HMessage;

#[derive(Debug, Default)]
struct Test {}

fn main() {
    let mut ext: Extension<Test> = Extension::new();
    ext.info.name = String::from("G-Rust Test Extension");
    ext.intercept(on_selector_def);
    ext.intercept(on_condition_def);
    ext.intercept(on_action_def);
    ext.intercept(on_trigger_def);
    ext.intercept(on_add_on_def);
    ext.run();
}

fn on_selector_def(_ext: &mut Extension<Test>, _msg: &mut HMessage, selector: &mut WiredFurniSelector) {
    println!("{:?}", selector.def);
}

fn on_condition_def(_ext: &mut Extension<Test>, _msg: &mut HMessage, condition: &mut WiredFurniCondition) {
    println!("{:?}", condition.def);
}
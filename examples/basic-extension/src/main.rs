use std::thread;
use g_rust::extension::extension::Extension;
use g_rust::extension::parsers::incoming::{Objects, WiredFurniAction, WiredFurniAddon, WiredFurniCondition, WiredFurniSelector, WiredFurniTrigger};
use g_rust::extension::parsers::outgoing::{UpdateAction, UpdateAddon, UpdateCondition, UpdateSelector, UpdateTrigger};
use g_rust::protocol::hmessage::HMessage;
use g_rust::protocol::vars::legacy::LegacyId;

#[derive(Debug, Default)]
struct Test {}

fn main() {
    let mut ext: Extension<Test> = Extension::new();
    ext.info.name = String::from("G-Rust Test Extension");
    ext.intercept(on_objects);
    // ext.intercept(on_selector_def);
    // ext.intercept(on_condition_def);
    // ext.intercept(on_action_def);
    // ext.intercept(on_trigger_def);
    // ext.intercept(on_addon_def);
    // ext.intercept(on_save_selector);
    // ext.intercept(on_save_condition);
    // ext.intercept(on_save_action);
    // ext.intercept(on_save_trigger);
    // ext.intercept(on_save_addon);
    thread::spawn(move || {
        ext.run();
    });
}

fn on_objects(_ext: &mut Extension<Test>, _msg: &mut HMessage, objects: &mut Objects) {
    let mut ids: Vec<i64>= objects.clone().objects.into_iter().map(| f | *f.id).collect();
    ids.sort();
    println!("All ids:    ({}) {:?}", ids.len(), ids);
    ids.dedup();
    println!("Unique ids: ({}) {:?}", ids.len(), ids);
}

// fn on_selector_def(_ext: &mut Extension<Test>, _msg: &mut HMessage, selector: &mut WiredFurniSelector) {
//     println!("{:?}", selector.def);
// }
//
// fn on_condition_def(_ext: &mut Extension<Test>, _msg: &mut HMessage, condition: &mut WiredFurniCondition) {
//     println!("{:?}", condition.def);
// }
//
// fn on_action_def(_ext: &mut Extension<Test>, _msg: &mut HMessage, action: &mut WiredFurniAction) {
//     println!("{:?}", action.def);
// }
//
// fn on_trigger_def(_ext: &mut Extension<Test>, _msg: &mut HMessage, trigger: &mut WiredFurniTrigger) {
//     println!("{:?}", trigger.def);
// }
//
// fn on_addon_def(_ext: &mut Extension<Test>, _msg: &mut HMessage, addon: &mut WiredFurniAddon) {
//     println!("{:?}", addon.def);
// }
//
// fn on_save_selector(_ext: &mut Extension<Test>, _msg: &mut HMessage, selector: &mut UpdateSelector) {
//     println!("{:?}", selector);
// }
//
// fn on_save_condition(_ext: &mut Extension<Test>, _msg: &mut HMessage, condition: &mut UpdateCondition) {
//     println!("{:?}", condition);
// }
//
// fn on_save_action(_ext: &mut Extension<Test>, _msg: &mut HMessage, action: &mut UpdateAction) {
//     println!("{:?}", action);
// }
//
// fn on_save_trigger(_ext: &mut Extension<Test>, _msg: &mut HMessage, trigger: &mut UpdateTrigger) {
//     println!("{:?}", trigger);
// }
//
// fn on_save_addon(_ext: &mut Extension<Test>, _msg: &mut HMessage, addon: &mut UpdateAddon) {
//     println!("{:?}", addon);
// }
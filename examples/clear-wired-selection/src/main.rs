use g_rust::extension::extension::{Extension, ExtensionInfo};
use g_rust::extension::parsers::incoming::{CloseConnection, WiredFurniAction, WiredFurniAddon, WiredFurniCondition, WiredFurniSelector, WiredFurniTrigger, WiredSaveSuccess};
use g_rust::extension::parsers::outgoing::Chat;
use g_rust::extension::parsers::subparsers::SelectorDefinition;
use g_rust::protocol::hmessage::HMessage;

#[derive(Debug, Default)]
struct ClearOpenWired {
    last_opened_selector: Option<WiredFurniSelector>,
    last_opened_condition: Option<WiredFurniCondition>,
    last_opened_action: Option<WiredFurniAction>,
    last_opened_trigger: Option<WiredFurniTrigger>,
    last_opened_addon: Option<WiredFurniAddon>
}

impl ClearOpenWired {
    fn clear(&mut self) {
        self.last_opened_selector = None;
        self.last_opened_condition = None;
        self.last_opened_action = None;
        self.last_opened_trigger = None;
        self.last_opened_addon = None;
    }
}

fn main() {
    let mut ext: Extension<ClearOpenWired> = Extension::new();
    ext.info = ExtensionInfo {
        name: "Clear Wired Selection".to_string(),
        description: "Type :cw to clear the selected furni in the currently open wired".to_string(),
        author: "WiredSpast".to_string(),
        version: "0.1.0".to_string(),
    };
    ext.intercept(on_chat);
    ext.intercept(on_save_success);
    ext.intercept(on_selector_def);
    ext.intercept(on_condition_def);
    ext.intercept(on_action_def);
    ext.intercept(on_trigger_def);
    ext.intercept(on_addon_def);
    ext.run();
}

fn on_chat(ext: &mut Extension<ClearOpenWired>, _msg: &mut HMessage, chat: &mut Chat) {
    if chat.text == ":cw" {
        if ext.globals.last_opened_selector.is_some() {
            let mut selector = ext.globals.last_opened_selector.clone().unwrap();
            selector.def.stuff_ids = vec![];
            ext.send(selector);
        }
        if ext.globals.last_opened_condition.is_some() {
            let mut condition = ext.globals.last_opened_condition.clone().unwrap();
            condition.def.stuff_ids = vec![];
            ext.send(condition);
        }
        if ext.globals.last_opened_action.is_some() {
            let mut action = ext.globals.last_opened_action.clone().unwrap();
            action.def.stuff_ids = vec![];
            ext.send(action);
        }
        if ext.globals.last_opened_trigger.is_some() {
            let mut trigger = ext.globals.last_opened_trigger.clone().unwrap();
            trigger.def.stuff_ids = vec![];
            ext.send(trigger);
        }
        if ext.globals.last_opened_addon.is_some() {
            let mut addon = ext.globals.last_opened_addon.clone().unwrap();
            addon.def.stuff_ids = vec![];
            ext.send(addon);
        }
    }
}

fn on_save_success(ext: &mut Extension<ClearOpenWired>, _msg: &mut HMessage, _: &mut WiredSaveSuccess) {
    ext.globals.clear()
}

fn on_selector_def(ext: &mut Extension<ClearOpenWired>, _msg: &mut HMessage, selector: &mut WiredFurniSelector) {
    ext.globals.clear();
    ext.globals.last_opened_selector = Some(selector.clone());
}

fn on_condition_def(ext: &mut Extension<ClearOpenWired>, _msg: &mut HMessage, condition: &mut WiredFurniCondition) {
    ext.globals.clear();
    ext.globals.last_opened_condition = Some(condition.clone());
}

fn on_action_def(ext: &mut Extension<ClearOpenWired>, _msg: &mut HMessage, action: &mut WiredFurniAction) {
    ext.globals.clear();
    ext.globals.last_opened_action = Some(action.clone());
}

fn on_trigger_def(ext: &mut Extension<ClearOpenWired>, _msg: &mut HMessage, trigger: &mut WiredFurniTrigger) {
    ext.globals.clear();
    ext.globals.last_opened_trigger = Some(trigger.clone());
}

fn on_addon_def(ext: &mut Extension<ClearOpenWired>, _msg: &mut HMessage, addon: &mut WiredFurniAddon) {
    ext.globals.clear();
    ext.globals.last_opened_addon = Some(addon.clone());
}
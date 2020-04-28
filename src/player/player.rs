use js_sys::JsString;
use js_sys::JSON;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct Player {
    pub name: String,
}

impl Player {
    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }

    pub fn new() -> Player {
        Player {
            name: String::new(),
        }
    }
}

pub struct PlayerSotorage {
    local_storage: web_sys::Storage,
    name: String,
    data: Player,
}

impl PlayerSotorage {
    pub fn new(name: &str) -> Option<PlayerSotorage> {
        let window = web_sys::window()?;
        if let Ok(Some(local_storage)) = window.local_storage() {
            let mut store = PlayerSotorage {
                local_storage,
                data: Player::new(),
                name: String::from(name),
            };
            store.fetch_local_storage();
            Some(store)
        } else {
            None
        }
    }

    fn fetch_local_storage(&mut self) -> Option<()> {
        let mut player = Player {
            name: String::new(),
        };
        // If we have an existing cached value, return early.
        if let Ok(Some(value)) = self.local_storage.get_item(&self.name) {
            let data = JSON::parse(&value).ok()?;
            let iter = js_sys::try_iter(&data).ok()??;
            for item in iter {
                let item = item.ok()?;
                let item_array: &js_sys::Array = wasm_bindgen::JsCast::dyn_ref(&item)?;
                let name = item_array.shift().as_string()?;

                let temp_item = Player { name };
                player = temp_item;
            }
        }
        self.data = player;
        Some(())
    }

    /// Write the local ItemList to localStorage.
    fn sync_local_storage(&mut self) {
        let array = js_sys::Array::new();

        let child = js_sys::Array::new();
        child.push(&JsValue::from(&self.name));

        array.push(&JsValue::from(child));

        if let Ok(storage_string) = JSON::stringify(&JsValue::from(array)) {
            let storage_string: String = storage_string.into();
            self.local_storage
                .set_item(&self.name, &storage_string)
                .unwrap();
        }
    }

    pub fn insert(&mut self, item: Player) {
        self.data = item;
        self.sync_local_storage();
    }

    pub fn getPlayer(&self) -> Option<&Player> {
        let n = &self.data.name;
        match &n[..] {
            "" => None,
            _ => Some(&self.data),
        }
    }
}

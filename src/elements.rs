// all elements and their file extensions
pub const ELEMENT_TYPES: &[(&str, &str)] = &[
    ("function", ".mcfunction"),
    ("tag", ".json"),
    ("advancement", ".json"),
    ("banner_pattern", ".json"),
    ("chat_type", ".json"),
    ("damage_type", ".json"),
    ("enchantment", ".json"),
    ("enchantment_provider", ".json"),
    ("instrument", ".json"),
    ("item_modifier", ".json"),
    ("jukebox_song", ".json"),
    ("loot_table", ".json"),
    ("painting_variant", ".json"),
    ("predicate", ".json"),
    ("recipe", ".json"),
    ("trim_material", ".json"),
    ("trim_pattern", ".json"),
    ("walk_variant", ".json"),
];

// all elements and their template files
pub fn get_sample_content(element_type: &str) -> String {
    match element_type {
        "function" => String::new(),
        "advancement" => serde_json::to_string_pretty(&serde_json::json!({
          "criteria": {}
        }))
        .unwrap(),
        "banner_pattern" => serde_json::to_string_pretty(&serde_json::json!({
          "asset_id": "",
          "translation_key": ""
        }))
        .unwrap(),
        "chat_type" => serde_json::to_string_pretty(&serde_json::json!({
          "chat": {
            "translation_key": "",
            "parameters": []
          },
          "narration": {
            "translation_key": "",
            "parameters": []
          }
        }))
        .unwrap(),
        "damage_type" => serde_json::to_string_pretty(&serde_json::json!({
          "message_id": "",
          "exhaustion": 0,
          "scaling": "never"
        }))
        .unwrap(),
        "enchantment" => serde_json::to_string_pretty(&serde_json::json!({
          "description": "",
          "supported_items": "",
          "weight": 1,
          "max_level": 1,
          "min_cost": {
            "base": 0,
            "per_level_above_first": 0
          },
          "max_cost": {
            "base": 0,
            "per_level_above_first": 0
          },
          "anvil_cost": 0,
          "slots": []
        }))
        .unwrap(),
        "enchantment_provider" => serde_json::to_string_pretty(&serde_json::json!({
          "type": "minecraft:single",
          "enchantment": ""
        }))
        .unwrap(),
        "instrument" => serde_json::to_string_pretty(&serde_json::json!({
          "sound_event": "",
          "range": 1,
          "use_duration": 1,
          "description": ""
        }))
        .unwrap(),
        "item_modifier" => serde_json::to_string_pretty(&serde_json::json!({
          "function": ""
        }))
        .unwrap(),
        "jukebox_song" => serde_json::to_string_pretty(&serde_json::json!({
          "description": "",
          "comparator_output": 0,
          "length_in_seconds": 1,
          "sound_event": ""
        }))
        .unwrap(),
        "loot_table" => serde_json::to_string_pretty(&serde_json::json!({
          "type": ""
        }))
        .unwrap(),
        "painting_variant" => serde_json::to_string_pretty(&serde_json::json!({
          "asset_id": "",
          "width": 1,
          "height": 1,
          "title": "",
          "author": ""
        }))
        .unwrap(),
        "predicate" => serde_json::to_string_pretty(&serde_json::json!({
          "condition": ""
        }))
        .unwrap(),
        "recipe" => serde_json::to_string_pretty(&serde_json::json!({
          "type": ""
        }))
        .unwrap(),
        "tag" => serde_json::to_string_pretty(&serde_json::json!({
            "values": ["minecraft:stone"]
        }))
        .unwrap(),
        "trim_material" => serde_json::to_string_pretty(&serde_json::json!({
          "asset_name": "",
          "description": "",
          "ingredient": "",
          "item_model_index": 0
        }))
        .unwrap(),
        "trim_pattern" => serde_json::to_string_pretty(&serde_json::json!({
          "asset_id": "",
          "description": "",
          "template_item": ""
        }))
        .unwrap(),
        "wolf_variant" => serde_json::to_string_pretty(&serde_json::json!({
          "biomes": "",
          "wild_texture": "",
          "tame_texture": "",
          "angry_texture": ""
        }))
        .unwrap(),
        _ => serde_json::to_string_pretty(&serde_json::json!({})).unwrap(),
    }
}

// check if an element type is valid
pub fn is_valid_element_type(element_type: &str) -> bool {
    ELEMENT_TYPES.iter().any(|(name, _)| *name == element_type)
}

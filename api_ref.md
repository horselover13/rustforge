# Rust Public API Bindings

## Table of Contents

- **hf_api/actions.rs**
  - [function `heal`](#hf-apiactionsrs-heal)
  - [function `heal_limb`](#hf-apiactionsrs-heal_limb)
  - [function `injure`](#hf-apiactionsrs-injure)
  - [function `limb_state`](#hf-apiactionsrs-limb_state)
  - [function `apply_status`](#hf-apiactionsrs-apply_status)
  - [function `add_velocity`](#hf-apiactionsrs-add_velocity)
  - [function `resupply_from_pack`](#hf-apiactionsrs-resupply_from_pack)
  - [function `spawn_explosion`](#hf-apiactionsrs-spawn_explosion)
  - [function `spawn_projectile`](#hf-apiactionsrs-spawn_projectile)
  - [function `credit_to_player`](#hf-apiactionsrs-credit_to_player)
  - [function `freeze_mission_timer`](#hf-apiactionsrs-freeze_mission_timer)
  - [function `spawn_pelican`](#hf-apiactionsrs-spawn_pelican)
  - [function `modify_shots`](#hf-apiactionsrs-modify_shots)
  - [function `homing`](#hf-apiactionsrs-homing)
  - [function `stop_shots`](#hf-apiactionsrs-stop_shots)
- **hf_api/callback.rs**
  - [function `as_raw`](#hf-apicallbackrs-as_raw)
  - [function `from_raw`](#hf-apicallbackrs-from_raw)
  - [function `on`](#hf-apicallbackrs-on)
  - [function `on_game_loaded`](#hf-apicallbackrs-on_game_loaded)
  - [function `on_network_ready`](#hf-apicallbackrs-on_network_ready)
  - [function `on_entities_ready`](#hf-apicallbackrs-on_entities_ready)
  - [function `on_projectiles_ready`](#hf-apicallbackrs-on_projectiles_ready)
  - [function `on_mods_changed`](#hf-apicallbackrs-on_mods_changed)
  - [function `on_session_end`](#hf-apicallbackrs-on_session_end)
  - [function `on`](#hf-apicallbackrs-on)
- **hf_api/content.rs**
  - [function `component_record`](#hf-apicontentrs-component_record)
  - [function `add_weapon`](#hf-apicontentrs-add_weapon)
  - [function `weapon_entity`](#hf-apicontentrs-weapon_entity)
  - [function `add_projectile`](#hf-apicontentrs-add_projectile)
  - [function `projectile_row`](#hf-apicontentrs-projectile_row)
  - [function `add_explosion`](#hf-apicontentrs-add_explosion)
  - [function `explosion_row`](#hf-apicontentrs-explosion_row)
  - [function `add_beam`](#hf-apicontentrs-add_beam)
  - [function `beam_row`](#hf-apicontentrs-beam_row)
  - [function `add_arc`](#hf-apicontentrs-add_arc)
  - [function `arc_row`](#hf-apicontentrs-arc_row)
  - [function `add_damage`](#hf-apicontentrs-add_damage)
  - [function `damage_row`](#hf-apicontentrs-damage_row)
  - [function `resource_hash`](#hf-apicontentrs-resource_hash)
  - [function `add_stratagem`](#hf-apicontentrs-add_stratagem)
  - [function `stratagem_row`](#hf-apicontentrs-stratagem_row)
  - [function `add_entity`](#hf-apicontentrs-add_entity)
  - [function `add_behaviour`](#hf-apicontentrs-add_behaviour)
  - [function `add_difficulty`](#hf-apicontentrs-add_difficulty)
  - [function `difficulty_row`](#hf-apicontentrs-difficulty_row)
  - [function `add_ability`](#hf-apicontentrs-add_ability)
  - [function `add_stratagem_category`](#hf-apicontentrs-add_stratagem_category)
  - [function `find`](#hf-apicontentrs-find)
  - [function `content`](#hf-apicontentrs-content)
  - [function `copy_row`](#hf-apicontentrs-copy_row)
- **hf_api/entity.rs**
  - [function `register_entity`](#hf-apientityrs-register_entity)
  - [function `add_enemy`](#hf-apientityrs-add_enemy)
  - [function `add_enemy_swap`](#hf-apientityrs-add_enemy_swap)
  - [function `add_enemy_squad`](#hf-apientityrs-add_enemy_squad)
  - [function `copy_field`](#hf-apientityrs-copy_field)
  - [function `set_field`](#hf-apientityrs-set_field)
  - [function `set_tag`](#hf-apientityrs-set_tag)
  - [function `mount`](#hf-apientityrs-mount)
  - [function `record`](#hf-apientityrs-record)
  - [function `edit_entity`](#hf-apientityrs-edit_entity)
  - [function `set_live`](#hf-apientityrs-set_live)
  - [function `get_live`](#hf-apientityrs-get_live)
- **hf_api/hook.rs**
  - [function `hook`](#hf-apihookrs-hook)
  - [function `hook_ex`](#hf-apihookrs-hook_ex)
  - [function `unhook`](#hf-apihookrs-unhook)
  - [function `function`](#hf-apihookrs-function)
  - [function `hook_function`](#hf-apihookrs-hook_function)
  - [function `global`](#hf-apihookrs-global)
- **hf_api/listeners.rs**
  - [function `on_game_thread`](#hf-apilistenersrs-on_game_thread)
  - [function `subscribe`](#hf-apilistenersrs-subscribe)
- **hf_api/log.rs**
  - [function `log`](#hf-apilogrs-log)
  - [function `log_fmt`](#hf-apilogrs-log_fmt)
  - [function `get_build`](#hf-apilogrs-get_build)
  - [function `mod_id`](#hf-apilogrs-mod_id)
  - [function `mod_folder`](#hf-apilogrs-mod_folder)
  - [function `last_log`](#hf-apilogrs-last_log)
  - [function `logs_folder`](#hf-apilogrs-logs_folder)
- **hf_api/mem.rs**
  - [function `module_base`](#hf-apimemrs-module_base)
  - [function `module_base_with_size`](#hf-apimemrs-module_base_with_size)
  - [function `read`](#hf-apimemrs-read)
  - [function `write`](#hf-apimemrs-write)
  - [function `write_u32_if`](#hf-apimemrs-write_u32_if)
  - [function `read_u32`](#hf-apimemrs-read_u32)
  - [function `read_u64`](#hf-apimemrs-read_u64)
  - [function `find_pattern`](#hf-apimemrs-find_pattern)
  - [function `code_matches`](#hf-apimemrs-code_matches)
  - [function `read_code`](#hf-apimemrs-read_code)
- **hf_api/mod.rs**
  - [function `__register_entry`](#hf-apimodrs-__register_entry)
  - [struct `HfModPtr`](#hf-apimodrs-HfModPtr)
  - [function `new`](#hf-apimodrs-new)
  - [function `ptr`](#hf-apimodrs-ptr)
  - [struct `HfApiPtr`](#hf-apimodrs-HfApiPtr)
  - [function `new`](#hf-apimodrs-new)
  - [function `ptr`](#hf-apimodrs-ptr)
  - [struct `HellForgeApi`](#hf-apimodrs-HellForgeApi)
  - [function `version`](#hf-apimodrs-version)
  - [function `api_size`](#hf-apimodrs-api_size)
- **hf_api/multiplayer.rs**
  - [function `sync_state`](#hf-apimultiplayerrs-sync_state)
  - [function `sync_mod`](#hf-apimultiplayerrs-sync_mod)
  - [function `sync_answer`](#hf-apimultiplayerrs-sync_answer)
  - [function `browse_state`](#hf-apimultiplayerrs-browse_state)
  - [function `browse_lobby`](#hf-apimultiplayerrs-browse_lobby)
  - [function `browse_ask`](#hf-apimultiplayerrs-browse_ask)
- **hf_api/network.rs**
  - [function `network_field_type`](#hf-apinetworkrs-network_field_type)
  - [function `set_network_field_bits`](#hf-apinetworkrs-set_network_field_bits)
  - [function `is_authority`](#hf-apinetworkrs-is_authority)
  - [function `entity_of_network`](#hf-apinetworkrs-entity_of_network)
- **hf_api/session.rs**
  - [function `set_passives`](#hf-apisessionrs-set_passives)
  - [function `get_passives`](#hf-apisessionrs-get_passives)
  - [function `load_package`](#hf-apisessionrs-load_package)
  - [function `session`](#hf-apisessionrs-session)
  - [function `game_state`](#hf-apisessionrs-game_state)
  - [function `mods`](#hf-apisessionrs-mods)
  - [function `notice`](#hf-apisessionrs-notice)
  - [function `load_phase`](#hf-apisessionrs-load_phase)
  - [function `generation_capacity`](#hf-apisessionrs-generation_capacity)
- **hf_api/stats.rs**
  - [function `get_stat`](#hf-apistatsrs-get_stat)
  - [function `set_stat`](#hf-apistatsrs-set_stat)
  - [function `stats_source`](#hf-apistatsrs-stats_source)
- **hf_api/status.rs**
  - [function `set_status`](#hf-apistatusrs-set_status)
- **hf_api/util.rs**
  - [function `wwise_id`](#hf-apiutilrs-wwise_id)
- **hf_component.rs**
  - [function `component`](#hf-componentrs-component)
  - [function `component_mut`](#hf-componentrs-component_mut)
  - [function `shared_component`](#hf-componentrs-shared_component)
- **hf_customization.rs**
  - [function `add_attachment`](#hf-customizationrs-add_attachment)
  - [function `add_pattern`](#hf-customizationrs-add_pattern)
  - [struct `Attachment`](#hf-customizationrs-Attachment)
  - [function `new`](#hf-customizationrs-new)
  - [function `unit`](#hf-customizationrs-unit)
  - [function `name_text`](#hf-customizationrs-name_text)
  - [function `description`](#hf-customizationrs-description)
  - [function `register`](#hf-customizationrs-register)
  - [struct `Pattern`](#hf-customizationrs-Pattern)
  - [function `new`](#hf-customizationrs-new)
  - [function `name_text`](#hf-customizationrs-name_text)
  - [function `description`](#hf-customizationrs-description)
  - [function `pattern_lut`](#hf-customizationrs-pattern_lut)
  - [function `swap`](#hf-customizationrs-swap)
  - [function `swaps`](#hf-customizationrs-swaps)
  - [function `colour`](#hf-customizationrs-colour)
  - [function `colours`](#hf-customizationrs-colours)
  - [function `weapons`](#hf-customizationrs-weapons)
  - [function `register`](#hf-customizationrs-register)
- **hf_functions.rs**
  - [function `consume_damage_event`](#hf-functionsrs-consume_damage_event)
  - [function `kill_dispatch`](#hf-functionsrs-kill_dispatch)
  - [function `activate_stratagem_beacon`](#hf-functionsrs-activate_stratagem_beacon)
  - [function `spawn_entity`](#hf-functionsrs-spawn_entity)
  - [function `request_explosion`](#hf-functionsrs-request_explosion)
  - [function `add_health_fraction`](#hf-functionsrs-add_health_fraction)
  - [function `restore_zone`](#hf-functionsrs-restore_zone)
  - [function `queue_status_request`](#hf-functionsrs-queue_status_request)
  - [function `set_velocity`](#hf-functionsrs-set_velocity)
  - [function `fire_projectile`](#hf-functionsrs-fire_projectile)
  - [function `hook_consume_damage_event`](#hf-functionsrs-hook_consume_damage_event)
  - [function `hook_kill_dispatch`](#hf-functionsrs-hook_kill_dispatch)
  - [function `hook_activate_stratagem_beacon`](#hf-functionsrs-hook_activate_stratagem_beacon)
  - [function `hook_spawn_entity`](#hf-functionsrs-hook_spawn_entity)
  - [function `hook_request_explosion`](#hf-functionsrs-hook_request_explosion)
- **hf_globals.rs**
  - [function `as_ptr`](#hf-globalsrs-as_ptr)
  - [function `state`](#hf-globalsrs-state)
  - [function `wwise_world`](#hf-globalsrs-wwise_world)
  - [function `level`](#hf-globalsrs-level)
  - [function `mode`](#hf-globalsrs-mode)
  - [function `in_mission`](#hf-globalsrs-in_mission)
  - [function `mission`](#hf-globalsrs-mission)
  - [struct `Mission`](#hf-globalsrs-Mission)
  - [function `is_host`](#hf-globalsrs-is_host)
  - [function `entity`](#hf-globalsrs-entity)
  - [function `as_ptr`](#hf-globalsrs-as_ptr)
  - [function `count`](#hf-globalsrs-count)
  - [function `slot`](#hf-globalsrs-slot)
  - [function `players`](#hf-globalsrs-players)
  - [struct `Player`](#hf-globalsrs-Player)
  - [function `peer`](#hf-globalsrs-peer)
  - [function `lifecycle`](#hf-globalsrs-lifecycle)
  - [function `entity`](#hf-globalsrs-entity)
  - [function `network_id`](#hf-globalsrs-network_id)
  - [function `peer`](#hf-globalsrs-peer)
  - [function `host_peer`](#hf-globalsrs-host_peer)
  - [function `component_tables`](#hf-globalsrs-component_tables)
  - [function `entity_deltas`](#hf-globalsrs-entity_deltas)
  - [function `entity_map`](#hf-globalsrs-entity_map)
  - [function `short_ids`](#hf-globalsrs-short_ids)
  - [function `network_object`](#hf-globalsrs-network_object)
  - [struct `NetworkObject`](#hf-globalsrs-NetworkObject)
  - [function `entity`](#hf-globalsrs-entity)
  - [function `count`](#hf-globalsrs-count)
  - [function `descriptor`](#hf-globalsrs-descriptor)
  - [function `health`](#hf-globalsrs-health)
  - [function `life`](#hf-globalsrs-life)
  - [function `max_health`](#hf-globalsrs-max_health)
  - [struct `HealthDescriptor`](#hf-globalsrs-HealthDescriptor)
  - [function `resource`](#hf-globalsrs-resource)
  - [function `entity`](#hf-globalsrs-entity)
  - [function `unit`](#hf-globalsrs-unit)
  - [function `network_id`](#hf-globalsrs-network_id)
  - [function `owned_here`](#hf-globalsrs-owned_here)
  - [function `count`](#hf-globalsrs-count)
  - [function `positions`](#hf-globalsrs-positions)
  - [function `sources`](#hf-globalsrs-sources)
  - [function `entity`](#hf-globalsrs-entity)
  - [function `is_live`](#hf-globalsrs-is_live)
  - [function `velocity`](#hf-globalsrs-velocity)
  - [function `count`](#hf-globalsrs-count)
  - [function `is_running`](#hf-globalsrs-is_running)
  - [function `damage_count`](#hf-globalsrs-damage_count)
  - [function `status_count`](#hf-globalsrs-status_count)
  - [function `record`](#hf-globalsrs-record)
  - [struct `ScoreRecord`](#hf-globalsrs-ScoreRecord)
  - [function `sources`](#hf-globalsrs-sources)
  - [function `as_ptr`](#hf-globalsrs-as_ptr)
  - [struct `ScoreSources`](#hf-globalsrs-ScoreSources)
  - [function `get`](#hf-globalsrs-get)
  - [function `as_ptr`](#hf-globalsrs-as_ptr)
  - [struct `ScoreSource`](#hf-globalsrs-ScoreSource)
  - [function `as_ptr`](#hf-globalsrs-as_ptr)
  - [function `slots`](#hf-globalsrs-slots)
  - [function `hand`](#hf-globalsrs-hand)
  - [function `records`](#hf-globalsrs-records)
  - [function `record`](#hf-globalsrs-record)
  - [struct `InventoryRecord`](#hf-globalsrs-InventoryRecord)
  - [function `selection`](#hf-globalsrs-selection)
  - [function `index`](#hf-globalsrs-index)
  - [function `count`](#hf-globalsrs-count)
  - [function `descriptor`](#hf-globalsrs-descriptor)
  - [function `world_index`](#hf-globalsrs-world_index)
  - [function `slots`](#hf-globalsrs-slots)
  - [function `slot_count`](#hf-globalsrs-slot_count)
  - [function `empty_key`](#hf-globalsrs-empty_key)
  - [function `record`](#hf-globalsrs-record)
  - [struct `DestructionRecord`](#hf-globalsrs-DestructionRecord)
  - [function `unit`](#hf-globalsrs-unit)
  - [function `stage`](#hf-globalsrs-stage)
  - [function `registered`](#hf-globalsrs-registered)
  - [function `game`](#hf-globalsrs-game)
  - [function `game_mode`](#hf-globalsrs-game_mode)
  - [function `player_manager`](#hf-globalsrs-player_manager)
  - [function `local_user`](#hf-globalsrs-local_user)
  - [function `entity_manager`](#hf-globalsrs-entity_manager)
  - [function `health_manager`](#hf-globalsrs-health_manager)
  - [function `motion_manager`](#hf-globalsrs-motion_manager)
  - [function `explosion_queue`](#hf-globalsrs-explosion_queue)
  - [function `projectile_system`](#hf-globalsrs-projectile_system)
  - [function `combat_queues`](#hf-globalsrs-combat_queues)
  - [function `score_manager`](#hf-globalsrs-score_manager)
  - [function `wielder_manager`](#hf-globalsrs-wielder_manager)
  - [function `inventory_manager`](#hf-globalsrs-inventory_manager)
  - [function `destructibles`](#hf-globalsrs-destructibles)
  - [function `destruction_records`](#hf-globalsrs-destruction_records)
  - [function `amounts`](#hf-globalsrs-amounts)
  - [function `definitions`](#hf-globalsrs-definitions)
  - [function `simulated`](#hf-globalsrs-simulated)
  - [function `simulated`](#hf-globalsrs-simulated)
  - [function `wielders`](#hf-globalsrs-wielders)
  - [function `masks`](#hf-globalsrs-masks)
  - [function `handles`](#hf-globalsrs-handles)
  - [function `records`](#hf-globalsrs-records)
  - [function `tags`](#hf-globalsrs-tags)
  - [function `now`](#hf-globalsrs-now)
  - [function `application`](#hf-globalsrs-application)
  - [function `unit`](#hf-globalsrs-unit)
  - [function `actor_by_name`](#hf-globalsrs-actor_by_name)
  - [function `post`](#hf-globalsrs-post)
  - [function `unit_source`](#hf-globalsrs-unit_source)
  - [function `source_object`](#hf-globalsrs-source_object)
  - [function `descriptors`](#hf-globalsrs-descriptors)
  - [function `copy_count`](#hf-globalsrs-copy_count)
  - [function `copies`](#hf-globalsrs-copies)
  - [function `rows`](#hf-globalsrs-rows)
  - [function `instances`](#hf-globalsrs-instances)
  - [function `instances`](#hf-globalsrs-instances)
  - [function `on`](#hf-globalsrs-on)
  - [function `records`](#hf-globalsrs-records)
  - [function `records`](#hf-globalsrs-records)
  - [function `records`](#hf-globalsrs-records)
  - [function `records`](#hf-globalsrs-records)
  - [function `records`](#hf-globalsrs-records)
  - [function `hash`](#hf-globalsrs-hash)
  - [function `descriptors`](#hf-globalsrs-descriptors)
  - [function `live`](#hf-globalsrs-live)
  - [function `hash`](#hf-globalsrs-hash)
  - [function `descriptors`](#hf-globalsrs-descriptors)
  - [function `live`](#hf-globalsrs-live)
  - [function `records`](#hf-globalsrs-records)
  - [function `network`](#hf-globalsrs-network)
  - [function `map`](#hf-globalsrs-map)
  - [function `capacity`](#hf-globalsrs-capacity)
  - [function `empty`](#hf-globalsrs-empty)
  - [function `bindings`](#hf-globalsrs-bindings)
  - [function `rows`](#hf-globalsrs-rows)
  - [function `live`](#hf-globalsrs-live)
  - [function `kits`](#hf-globalsrs-kits)
  - [function `kit_count`](#hf-globalsrs-kit_count)
  - [function `passives`](#hf-globalsrs-passives)
  - [function `passive_count`](#hf-globalsrs-passive_count)
  - [function `record_count`](#hf-globalsrs-record_count)
  - [function `kits`](#hf-globalsrs-kits)
  - [function `kit_count`](#hf-globalsrs-kit_count)
  - [function `passives`](#hf-globalsrs-passives)
  - [function `passive_count`](#hf-globalsrs-passive_count)
  - [function `record_count`](#hf-globalsrs-record_count)
  - [function `blocks`](#hf-globalsrs-blocks)
  - [function `bought_count`](#hf-globalsrs-bought_count)
  - [function `login`](#hf-globalsrs-login)
  - [function `pending_buy`](#hf-globalsrs-pending_buy)
  - [function `pending_item`](#hf-globalsrs-pending_item)
  - [function `bought_count`](#hf-globalsrs-bought_count)
  - [function `login`](#hf-globalsrs-login)
  - [function `pending_buy`](#hf-globalsrs-pending_buy)
  - [function `pending_item`](#hf-globalsrs-pending_item)
  - [function `count`](#hf-globalsrs-count)
  - [function `left_count`](#hf-globalsrs-left_count)
  - [function `primary`](#hf-globalsrs-primary)
  - [function `secondary`](#hf-globalsrs-secondary)
  - [function `throwable`](#hf-globalsrs-throwable)
  - [function `presenter`](#hf-globalsrs-presenter)
  - [function `depth`](#hf-globalsrs-depth)
  - [function `presenter`](#hf-globalsrs-presenter)
  - [function `depth`](#hf-globalsrs-depth)
  - [function `top`](#hf-globalsrs-top)
  - [function `starts`](#hf-globalsrs-starts)
  - [function `counts`](#hf-globalsrs-counts)
  - [function `active_card`](#hf-globalsrs-active_card)
  - [function `phase`](#hf-globalsrs-phase)
  - [function `map`](#hf-globalsrs-map)
  - [function `capacity`](#hf-globalsrs-capacity)
  - [function `instance_count`](#hf-globalsrs-instance_count)
  - [function `level`](#hf-globalsrs-level)
  - [function `timer_count`](#hf-globalsrs-timer_count)
  - [function `director`](#hf-globalsrs-director)
  - [function `guard_compositions`](#hf-globalsrs-guard_compositions)
  - [function `reinforcement_compositions`](#hf-globalsrs-reinforcement_compositions)
  - [function `guard_budget`](#hf-globalsrs-guard_budget)
  - [function `countdown_left`](#hf-globalsrs-countdown_left)
  - [function `expired`](#hf-globalsrs-expired)
  - [function `paused`](#hf-globalsrs-paused)
  - [function `level`](#hf-globalsrs-level)
  - [function `timer_count`](#hf-globalsrs-timer_count)
  - [function `director`](#hf-globalsrs-director)
  - [function `guard_compositions`](#hf-globalsrs-guard_compositions)
  - [function `reinforcement_compositions`](#hf-globalsrs-reinforcement_compositions)
  - [function `guard_budget`](#hf-globalsrs-guard_budget)
  - [function `countdown_left`](#hf-globalsrs-countdown_left)
  - [function `expired`](#hf-globalsrs-expired)
  - [function `paused`](#hf-globalsrs-paused)
  - [function `count`](#hf-globalsrs-count)
  - [function `records`](#hf-globalsrs-records)
  - [function `objects`](#hf-globalsrs-objects)
  - [function `count`](#hf-globalsrs-count)
  - [function `generations`](#hf-globalsrs-generations)
  - [function `overrides`](#hf-globalsrs-overrides)
  - [function `resource_manager`](#hf-globalsrs-resource_manager)
  - [function `world_count`](#hf-globalsrs-world_count)
  - [function `worlds`](#hf-globalsrs-worlds)
  - [function `config`](#hf-globalsrs-config)
  - [function `transport`](#hf-globalsrs-transport)
  - [function `session`](#hf-globalsrs-session)
  - [function `kept_count`](#hf-globalsrs-kept_count)
  - [function `kept`](#hf-globalsrs-kept)
  - [function `config`](#hf-globalsrs-config)
  - [function `size`](#hf-globalsrs-size)
  - [function `generations`](#hf-globalsrs-generations)
  - [function `filter_count`](#hf-globalsrs-filter_count)
  - [function `filter_names`](#hf-globalsrs-filter_names)
  - [function `matrix`](#hf-globalsrs-matrix)
  - [function `matrix`](#hf-globalsrs-matrix)
  - [function `max`](#hf-globalsrs-max)
  - [function `count`](#hf-globalsrs-count)
  - [function `descriptor`](#hf-globalsrs-descriptor)
  - [function `kills`](#hf-globalsrs-kills)
  - [function `count`](#hf-globalsrs-count)
  - [function `descriptors`](#hf-globalsrs-descriptors)
  - [function `live`](#hf-globalsrs-live)
  - [function `count`](#hf-globalsrs-count)
  - [function `count`](#hf-globalsrs-count)
  - [function `descriptors`](#hf-globalsrs-descriptors)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `hover_input`](#hf-globalsrs-hover_input)
  - [function `flight`](#hf-globalsrs-flight)
  - [function `flags`](#hf-globalsrs-flags)
  - [function `map`](#hf-globalsrs-map)
  - [function `records`](#hf-globalsrs-records)
  - [function `flight`](#hf-globalsrs-flight)
  - [function `map`](#hf-globalsrs-map)
  - [function `count`](#hf-globalsrs-count)
  - [function `map`](#hf-globalsrs-map)
  - [function `descriptors`](#hf-globalsrs-descriptors)
  - [function `records`](#hf-globalsrs-records)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `map`](#hf-globalsrs-map)
  - [function `deposit_manager`](#hf-globalsrs-deposit_manager)
  - [function `avatar_manager`](#hf-globalsrs-avatar_manager)
  - [function `action_manager`](#hf-globalsrs-action_manager)
  - [function `wieldable_manager`](#hf-globalsrs-wieldable_manager)
  - [function `faction_manager`](#hf-globalsrs-faction_manager)
  - [function `behaviour_manager`](#hf-globalsrs-behaviour_manager)
  - [function `tag_manager`](#hf-globalsrs-tag_manager)
  - [function `clock`](#hf-globalsrs-clock)
  - [function `engine_api`](#hf-globalsrs-engine_api)
  - [function `engine_unit_api`](#hf-globalsrs-engine_unit_api)
  - [function `wwise_api`](#hf-globalsrs-wwise_api)
  - [function `ability_manager`](#hf-globalsrs-ability_manager)
  - [function `weapon_linker_manager`](#hf-globalsrs-weapon_linker_manager)
  - [function `seeking_missile_manager`](#hf-globalsrs-seeking_missile_manager)
  - [function `laser_designator_manager`](#hf-globalsrs-laser_designator_manager)
  - [function `guidance_targets`](#hf-globalsrs-guidance_targets)
  - [function `explosive_manager`](#hf-globalsrs-explosive_manager)
  - [function `equipment_manager`](#hf-globalsrs-equipment_manager)
  - [function `remote_owners`](#hf-globalsrs-remote_owners)
  - [function `hellpod_rack_manager`](#hf-globalsrs-hellpod_rack_manager)
  - [function `attachable_manager`](#hf-globalsrs-attachable_manager)
  - [function `mount_manager`](#hf-globalsrs-mount_manager)
  - [function `drop_anchors`](#hf-globalsrs-drop_anchors)
  - [function `perception_manager`](#hf-globalsrs-perception_manager)
  - [function `weapon_manager`](#hf-globalsrs-weapon_manager)
  - [function `magazine_manager`](#hf-globalsrs-magazine_manager)
  - [function `weapon_magazines`](#hf-globalsrs-weapon_magazines)
  - [function `magazine_refills`](#hf-globalsrs-magazine_refills)
  - [function `weapon_targeting`](#hf-globalsrs-weapon_targeting)
  - [function `shield_manager`](#hf-globalsrs-shield_manager)
  - [function `customization_manager`](#hf-globalsrs-customization_manager)
  - [function `kit_list`](#hf-globalsrs-kit_list)
  - [function `bombardment_manager`](#hf-globalsrs-bombardment_manager)
  - [function `progression`](#hf-globalsrs-progression)
  - [function `progression_manager`](#hf-globalsrs-progression_manager)
  - [function `player_history`](#hf-globalsrs-player_history)
  - [function `profile`](#hf-globalsrs-profile)
  - [function `menu_states`](#hf-globalsrs-menu_states)
  - [function `ui_manager`](#hf-globalsrs-ui_manager)
  - [function `menu_stack`](#hf-globalsrs-menu_stack)
  - [function `armory`](#hf-globalsrs-armory)
  - [function `thumbnail_manager`](#hf-globalsrs-thumbnail_manager)
  - [function `package_system`](#hf-globalsrs-package_system)
  - [function `game_config`](#hf-globalsrs-game_config)
  - [function `navigation_system`](#hf-globalsrs-navigation_system)
  - [function `physics_plugin`](#hf-globalsrs-physics_plugin)
  - [function `objective_manager`](#hf-globalsrs-objective_manager)
  - [function `hive_mind_manager`](#hf-globalsrs-hive_mind_manager)
  - [function `mission_timers`](#hf-globalsrs-mission_timers)
  - [function `telemetry`](#hf-globalsrs-telemetry)
  - [function `guard_forces`](#hf-globalsrs-guard_forces)
  - [function `unit_registry`](#hf-globalsrs-unit_registry)
  - [function `engine_core`](#hf-globalsrs-engine_core)
  - [function `network_manager`](#hf-globalsrs-network_manager)
  - [function `network_config_owner`](#hf-globalsrs-network_config_owner)
  - [function `engine_entity_index`](#hf-globalsrs-engine_entity_index)
  - [function `nav_worlds`](#hf-globalsrs-nav_worlds)
  - [function `physics_properties`](#hf-globalsrs-physics_properties)
  - [function `collision_filter_a`](#hf-globalsrs-collision_filter_a)
  - [function `collision_filter_b`](#hf-globalsrs-collision_filter_b)
  - [function `status_receivers`](#hf-globalsrs-status_receivers)
  - [function `component_world`](#hf-globalsrs-component_world)
  - [function `game_mode_mission`](#hf-globalsrs-game_mode_mission)
  - [function `career_stats`](#hf-globalsrs-career_stats)
  - [function `projectile_weapons`](#hf-globalsrs-projectile_weapons)
  - [function `weapon_datas`](#hf-globalsrs-weapon_datas)
  - [function `beam_weapons`](#hf-globalsrs-beam_weapons)
  - [function `linked_ammo`](#hf-globalsrs-linked_ammo)
  - [function `corpses`](#hf-globalsrs-corpses)
  - [function `weapon_wind_ups`](#hf-globalsrs-weapon_wind_ups)
  - [function `recharges`](#hf-globalsrs-recharges)
  - [function `weapon_reloads`](#hf-globalsrs-weapon_reloads)
  - [function `weapon_rounds_manager`](#hf-globalsrs-weapon_rounds_manager)
  - [function `weapon_charges`](#hf-globalsrs-weapon_charges)
  - [function `throwers`](#hf-globalsrs-throwers)
  - [function `throwables`](#hf-globalsrs-throwables)
  - [function `arc_weapons`](#hf-globalsrs-arc_weapons)
  - [function `weapon_heats`](#hf-globalsrs-weapon_heats)
  - [function `ability_weapons`](#hf-globalsrs-ability_weapons)
  - [function `melee_weapons`](#hf-globalsrs-melee_weapons)
  - [function `weapon_customizations`](#hf-globalsrs-weapon_customizations)
  - [function `ai_spawner_manager`](#hf-globalsrs-ai_spawner_manager)
  - [function `bughole_manager`](#hf-globalsrs-bughole_manager)
  - [function `vehicle_motion_manager`](#hf-globalsrs-vehicle_motion_manager)
  - [function `vehicle_manager`](#hf-globalsrs-vehicle_manager)
  - [function `squad_transport_manager`](#hf-globalsrs-squad_transport_manager)
  - [function `squad_formation_manager`](#hf-globalsrs-squad_formation_manager)
  - [function `gore_manager`](#hf-globalsrs-gore_manager)
  - [function `stratagem_blocker_manager`](#hf-globalsrs-stratagem_blocker_manager)
  - [function `orbital_ability_manager`](#hf-globalsrs-orbital_ability_manager)
  - [function `hellpod_payload_manager`](#hf-globalsrs-hellpod_payload_manager)
  - [function `unit_manager`](#hf-globalsrs-unit_manager)
  - [function `rotation_manager`](#hf-globalsrs-rotation_manager)
  - [function `displacement_manager`](#hf-globalsrs-displacement_manager)
  - [function `jumppack_manager`](#hf-globalsrs-jumppack_manager)
  - [function `eagle_manager`](#hf-globalsrs-eagle_manager)
  - [function `encounter_manager`](#hf-globalsrs-encounter_manager)
  - [function `ai_squad_manager`](#hf-globalsrs-ai_squad_manager)
  - [function `sensor_eye_manager`](#hf-globalsrs-sensor_eye_manager)
  - [function `turret_manager`](#hf-globalsrs-turret_manager)
  - [function `locomotion_manager`](#hf-globalsrs-locomotion_manager)
- **hf_net.rs**
  - [struct `MessageEvent`](#hf-netrs-MessageEvent)
  - [struct `ShareEvent`](#hf-netrs-ShareEvent)
  - [struct `RpcEvent`](#hf-netrs-RpcEvent)
  - [function `sender`](#hf-netrs-sender)
  - [function `rpc`](#hf-netrs-rpc)
  - [function `count`](#hf-netrs-count)
  - [function `args`](#hf-netrs-args)
  - [function `reader`](#hf-netrs-reader)
  - [struct `RpcReader`](#hf-netrs-RpcReader)
  - [function `bool`](#hf-netrs-bool)
  - [function `int`](#hf-netrs-int)
  - [function `float`](#hf-netrs-float)
  - [function `vector3`](#hf-netrs-vector3)
  - [function `quaternion`](#hf-netrs-quaternion)
  - [function `string`](#hf-netrs-string)
  - [function `u32`](#hf-netrs-u32)
  - [function `id64`](#hf-netrs-id64)
  - [function `u64`](#hf-netrs-u64)
  - [function `direction`](#hf-netrs-direction)
  - [function `vector2`](#hf-netrs-vector2)
  - [function `array`](#hf-netrs-array)
  - [function `finish`](#hf-netrs-finish)
  - [function `send`](#hf-netrs-send)
  - [function `share_set`](#hf-netrs-share_set)
  - [function `share_get`](#hf-netrs-share_get)
  - [function `share_row`](#hf-netrs-share_row)
  - [function `on_message`](#hf-netrs-on_message)
  - [function `on_share`](#hf-netrs-on_share)
  - [function `rpc_listen`](#hf-netrs-rpc_listen)
  - [function `rpc_send`](#hf-netrs-rpc_send)
  - [function `rpc_send_message`](#hf-netrs-rpc_send_message)
  - [function `rpc_listen_message`](#hf-netrs-rpc_listen_message)
- **hf_overlay.rs**
  - [struct `Color`](#hf-overlayrs-Color)
  - [function `rgb`](#hf-overlayrs-rgb)
  - [function `rgba`](#hf-overlayrs-rgba)
  - [function `as_raw`](#hf-overlayrs-as_raw)
  - [function `as_raw`](#hf-overlayrs-as_raw)
  - [struct `InputState`](#hf-overlayrs-InputState)
  - [function `from_ffi`](#hf-overlayrs-from_ffi)
  - [struct `DrawingContext`](#hf-overlayrs-DrawingContext)
  - [function `width`](#hf-overlayrs-width)
  - [function `height`](#hf-overlayrs-height)
  - [function `pixels`](#hf-overlayrs-pixels)
  - [function `rect`](#hf-overlayrs-rect)
  - [function `lines`](#hf-overlayrs-lines)
  - [function `polygon`](#hf-overlayrs-polygon)
  - [function `circle`](#hf-overlayrs-circle)
  - [function `text`](#hf-overlayrs-text)
  - [function `text_width`](#hf-overlayrs-text_width)
  - [function `image_create`](#hf-overlayrs-image_create)
  - [function `image`](#hf-overlayrs-image)
  - [function `clip_push`](#hf-overlayrs-clip_push)
  - [function `clip_pop`](#hf-overlayrs-clip_pop)
  - [function `backdrop`](#hf-overlayrs-backdrop)
  - [function `hide_pointer`](#hf-overlayrs-hide_pointer)
  - [struct `Image`](#hf-overlayrs-Image)
  - [function `update`](#hf-overlayrs-update)
  - [struct `NoOpened`](#hf-overlayrs-NoOpened)
  - [struct `NoCanOpen`](#hf-overlayrs-NoCanOpen)
  - [struct `ScreenSpec`](#hf-overlayrs-ScreenSpec)
  - [function `new`](#hf-overlayrs-new)
  - [function `on_opened`](#hf-overlayrs-on_opened)
  - [function `on_can_open`](#hf-overlayrs-on_can_open)
  - [function `screen_add`](#hf-overlayrs-screen_add)
  - [function `screen_show`](#hf-overlayrs-screen_show)
- **hf_ui.rs**
  - [function `as_raw`](#hf-uirs-as_raw)
  - [function `screen`](#hf-uirs-screen)
  - [function `ui_open`](#hf-uirs-ui_open)
  - [function `ui_set`](#hf-uirs-ui_set)
  - [function `ui_on`](#hf-uirs-ui_on)
  - [function `ui_close`](#hf-uirs-ui_close)

---

## File: `hf_api/actions.rs`

### FUNCTION: `heal`

**Documentation:**

```
Heals the local Helldiver by `amount`, up to its maximum.

Returns `HF_ACT_*`, or 0 when refused.
```

**Signature:**

```rust
pub fn heal(&self, amount: f32) -> u32
```

**Location:** `actions.rs:13`

---

### FUNCTION: `heal_limb`

**Documentation:**

```
Heals one of the local Helldiver's limbs in full.

Returns `HF_ACT_*`, or 0 when refused.
```

**Signature:**

```rust
pub fn heal_limb(&self, limb: u32) -> u32
```

**Location:** `actions.rs:28`

---

### FUNCTION: `injure`

**Documentation:**

```
Damages one of the local Helldiver's limbs.

Returns `HF_ACT_*`, or 0 when refused.
```

**Signature:**

```rust
pub fn injure(&self, limb: u32, damage: i32) -> u32
```

**Location:** `actions.rs:43`

---

### FUNCTION: `limb_state`

**Documentation:**

```
A Helldiver's limb now.

Returns `None` when the state cannot be read.
```

**Signature:**

```rust
pub fn limb_state(&self, entity: u32, limb: u32) -> Option<HfLimbState>
```

**Location:** `actions.rs:58`

---

### FUNCTION: `apply_status`

**Documentation:**

```
Applies a status (`HF_STATUS_*`) to an entity.

`buildup` is 0 .. 1000 (100 is a full hit). Returns `HF_ACT_*`, or 0
when refused.
```

**Signature:**

```rust
pub fn apply_status(&self, entity: u32, status: u32, buildup: f32) -> u32
```

**Location:** `actions.rs:73`

---

### FUNCTION: `add_velocity`

**Documentation:**

```
Adds to the local Helldiver's velocity (m/s, world space, Z up).

Each part is capped at 25 and the result at 50. Returns `HF_ACT_*`, or
0 when refused.
```

**Signature:**

```rust
pub fn add_velocity(&self, delta: [f32
```

**Location:** `actions.rs:89`

---

### FUNCTION: `resupply_from_pack`

**Documentation:**

```
Makes the local Helldiver use its B-1 Supply Pack on itself.

Returns `HF_ACT_*`, or 0 when refused.
```

**Signature:**

```rust
pub fn resupply_from_pack(&self) -> u32
```

**Location:** `actions.rs:104`

---

### FUNCTION: `spawn_explosion`

**Documentation:**

```
Spawns an explosion at `position`.

Returns `HF_ACT_*`, or 0 when refused.
```

**Signature:**

```rust
pub fn spawn_explosion(&self, explosion_type: u32, position: [f32
```

**Location:** `actions.rs:119`

---

### FUNCTION: `spawn_projectile`

**Documentation:**

```
Fires a projectile from `position` along `direction`.

Returns `HF_ACT_*`, or 0 when refused.
```

**Signature:**

```rust
pub fn spawn_projectile( &self, projectile_type: u32, position: [f32
```

**Location:** `actions.rs:134`

---

### FUNCTION: `credit_to_player`

**Documentation:**

```
Credits an entity's kills to its owner, this machine's player.

Returns `HF_ACT_*`, or 0 when refused.
```

**Signature:**

```rust
pub fn credit_to_player(&self, entity: u32) -> u32
```

**Location:** `actions.rs:161`

---

### FUNCTION: `freeze_mission_timer`

**Documentation:**

```
Pauses (`true`) or resumes (`false`) the mission timer.

Returns `HF_ACT_*`, or 0 when refused.
```

**Signature:**

```rust
pub fn freeze_mission_timer(&self, frozen: bool) -> u32
```

**Location:** `actions.rs:176`

---

### FUNCTION: `spawn_pelican`

**Documentation:**

```
Calls in a transport Pelican.

Returns its entity, or `None` when the call was refused or is still
waiting (try again on the next click).
```

**Signature:**

```rust
pub fn spawn_pelican(&self, call: &HfPelicanCall) -> Option<u32>
```

**Location:** `actions.rs:192`

---

### FUNCTION: `modify_shots`

**Documentation:**

```
Adds a shot rule for the local player's shots of one projectile type.

Returns the rule id (1 and up), or 0 when refused.
```

**Signature:**

```rust
pub fn modify_shots(&self, rule: &HfShotRule) -> u32
```

**Location:** `actions.rs:207`

---

### FUNCTION: `homing`

**Documentation:**

```
Makes the local player's shots of one projectile type home in.

Returns the rule id (1 and up), or 0 when refused.
```

**Signature:**

```rust
pub fn homing(&self, rule: &HfHoming) -> u32
```

**Location:** `actions.rs:222`

---

### FUNCTION: `stop_shots`

**Documentation:**

```
Ends a shot or homing rule.

Returns `true` when the rule was stopped, `false` when there was no such
rule.
```

**Signature:**

```rust
pub fn stop_shots(&self, id: u32) -> bool
```

**Location:** `actions.rs:238`

---

## File: `hf_api/callback.rs`

### FUNCTION: `as_raw`

**Documentation:**

```
The raw value the C API expects.
```

**Signature:**

```rust
pub const fn as_raw(self) -> HfEvent
```

**Location:** `callback.rs:44`

---

### FUNCTION: `from_raw`

**Documentation:**

```
Converts a raw event value into an [`Event`].
```

**Signature:**

```rust
pub const fn from_raw(raw: HfEvent) -> Self
```

**Location:** `callback.rs:57`

---

### FUNCTION: `on`

**Documentation:**

```
Registers `callback` to run when `event` occurs.

The callback runs on the loader's thread, so it must be `Send`. It may run
more than once for events that can repeat (`EntitiesReady`, `ModsChanged`).
The loader revokes callbacks when the mod is unloaded; the closure itself is
owned by the loader for the rest of the session, so one registered for an
event that never fires is not reclaimed until the DLL is unloaded.
```

**Signature:**

```rust
pub fn on<F>(&self, event: Event, callback: F) -> Result<(), CallbackError> where F: FnMut(&HellForgeApi, Event) + Send + 'static,
```

**Location:** `callback.rs:155`

---

### FUNCTION: `on_game_loaded`

**Documentation:**

```
Registers a callback for [`Event::GameLoaded`].
```

**Signature:**

```rust
pub fn on_game_loaded<F>(&self, mut callback: F) -> Result<(), CallbackError> where F: FnMut(&HellForgeApi) + Send + 'static,
```

**Location:** `callback.rs:183`

---

### FUNCTION: `on_network_ready`

**Documentation:**

```
Registers a callback for [`Event::NetworkReady`].
```

**Signature:**

```rust
pub fn on_network_ready<F>(&self, mut callback: F) -> Result<(), CallbackError> where F: FnMut(&HellForgeApi) + Send + 'static,
```

**Location:** `callback.rs:191`

---

### FUNCTION: `on_entities_ready`

**Documentation:**

```
Registers a callback for [`Event::EntitiesReady`].
```

**Signature:**

```rust
pub fn on_entities_ready<F>(&self, mut callback: F) -> Result<(), CallbackError> where F: FnMut(&HellForgeApi) + Send + 'static,
```

**Location:** `callback.rs:199`

---

### FUNCTION: `on_projectiles_ready`

**Documentation:**

```
Registers a callback for [`Event::ProjectilesReady`].
```

**Signature:**

```rust
pub fn on_projectiles_ready<F>(&self, mut callback: F) -> Result<(), CallbackError> where F: FnMut(&HellForgeApi) + Send + 'static,
```

**Location:** `callback.rs:207`

---

### FUNCTION: `on_mods_changed`

**Documentation:**

```
Registers a callback for [`Event::ModsChanged`].
```

**Signature:**

```rust
pub fn on_mods_changed<F>(&self, mut callback: F) -> Result<(), CallbackError> where F: FnMut(&HellForgeApi) + Send + 'static,
```

**Location:** `callback.rs:215`

---

### FUNCTION: `on_session_end`

**Documentation:**

```
Registers a callback for [`Event::SessionEnd`].
```

**Signature:**

```rust
pub fn on_session_end<F>(&self, mut callback: F) -> Result<(), CallbackError> where F: FnMut(&HellForgeApi) + Send + 'static,
```

**Location:** `callback.rs:223`

---

### FUNCTION: `on`

**Documentation:**

```
Registers a callback through the globally installed API.

This is a convenience wrapper around [`HellForgeApi::on`] for callers that do
not already hold the API instance.
```

**Signature:**

```rust
pub fn on<F>(event: Event, callback: F) -> Result<(), CallbackError> where F: FnMut(&HellForgeApi, Event) + Send + 'static,
```

**Location:** `callback.rs:235`

---

## File: `hf_api/content.rs`

### FUNCTION: `component_record`

**Documentation:**

```
The address of an entity's record of `component`, after
`HF_EVENT_ENTITIES_READY`.

Returns the record address and its stride, or `None` when the entity
owns no such record, owns several, or shares it with another entity.
```

**Signature:**

```rust
pub fn component_record(&self, component: &str, resource: u64) -> Option<(usize, u32)>
```

**Location:** `content.rs:18`

---

### FUNCTION: `add_weapon`

**Documentation:**

```
Adds a new arsenal weapon (call from `hellforge_mod_init`).

Returns the new loadout id, or 0 on failure.
```

**Signature:**

```rust
pub fn add_weapon(&self, weapon: &HfWeapon) -> u32
```

**Location:** `content.rs:31`

---

### FUNCTION: `weapon_entity`

**Documentation:**

```
The entity resource of a weapon any mod added, for
[`component_record`](Self::component_record); 0 if there is none.
```

**Signature:**

```rust
pub fn weapon_entity(&self, weapon: u32) -> u64
```

**Location:** `content.rs:45`

---

### FUNCTION: `add_projectile`

**Documentation:**

```
Adds a new projectile type (call from `hellforge_mod_init`).

Returns its `ProjectileType` (351 and up), or 0 on failure.
```

**Signature:**

```rust
pub fn add_projectile(&self, projectile: &HfProjectile) -> u32
```

**Location:** `content.rs:60`

---

### FUNCTION: `projectile_row`

**Documentation:**

```
The settings row of a projectile a mod added, after
`HF_EVENT_PROJECTILES_READY`; 0 if there is none.
```

**Signature:**

```rust
pub fn projectile_row(&self, projectile_type: u32) -> usize
```

**Location:** `content.rs:74`

---

### FUNCTION: `add_explosion`

**Documentation:**

```
Adds a new explosion type (call from `hellforge_mod_init`).

Returns its type (423 and up), or 0 on failure.
```

**Signature:**

```rust
pub fn add_explosion(&self, explosion: &HfExplosion) -> u32
```

**Location:** `content.rs:89`

---

### FUNCTION: `explosion_row`

**Documentation:**

```
The `ExplosionSettings` row of an explosion type a mod added, after
`HF_EVENT_PROJECTILES_READY`; 0 if there is none.

The row's first field is the type, and must stay.
```

**Signature:**

```rust
pub fn explosion_row(&self, explosion_type: u32) -> usize
```

**Location:** `content.rs:105`

---

### FUNCTION: `add_beam`

**Documentation:**

```
Adds a new beam type (call from `hellforge_mod_init`).

Returns its type (31 and up), or 0 on failure.
```

**Signature:**

```rust
pub fn add_beam(&self, beam: &HfBeam) -> u32
```

**Location:** `content.rs:120`

---

### FUNCTION: `beam_row`

**Documentation:**

```
The `BeamSettings` row of a beam type a mod added, after
`HF_EVENT_PROJECTILES_READY`; 0 if there is none.

The row's first field is the type, and must stay.
```

**Signature:**

```rust
pub fn beam_row(&self, beam_type: u32) -> usize
```

**Location:** `content.rs:136`

---

### FUNCTION: `add_arc`

**Documentation:**

```
Adds a new arc type (call from `hellforge_mod_init`).

Returns its type (17 and up), or 0 on failure.
```

**Signature:**

```rust
pub fn add_arc(&self, arc: &HfArc) -> u32
```

**Location:** `content.rs:151`

---

### FUNCTION: `arc_row`

**Documentation:**

```
The `ArcSettings` row of an arc type a mod added, after
`HF_EVENT_PROJECTILES_READY`; 0 if there is none.

The row's first field is the type, and must stay.
```

**Signature:**

```rust
pub fn arc_row(&self, arc_type: u32) -> usize
```

**Location:** `content.rs:167`

---

### FUNCTION: `add_damage`

**Documentation:**

```
Adds a new damage type (call from `hellforge_mod_init`, before the
projectiles that deal it).

Returns its id (650 and up), or 0 on failure.
```

**Signature:**

```rust
pub fn add_damage(&self, damage: &HfDamage) -> u32
```

**Location:** `content.rs:183`

---

### FUNCTION: `damage_row`

**Documentation:**

```
The `DamageInfo` row of a damage type a mod added, after
`HF_EVENT_PROJECTILES_READY`; 0 if there is none.
```

**Signature:**

```rust
pub fn damage_row(&self, id: u32) -> usize
```

**Location:** `content.rs:197`

---

### FUNCTION: `resource_hash`

**Documentation:**

```
The game's 64-bit resource id of a resource name.
```

**Signature:**

```rust
pub fn resource_hash(&self, name: &str) -> u64
```

**Location:** `content.rs:210`

---

### FUNCTION: `add_stratagem`

**Documentation:**

```
Adds a new stratagem (call from `hellforge_mod_init`).

Returns its `StratagemType` (150 and up), or 0 on failure.
```

**Signature:**

```rust
pub fn add_stratagem(&self, stratagem: &HfStratagem) -> u32
```

**Location:** `content.rs:228`

---

### FUNCTION: `stratagem_row`

**Documentation:**

```
The settings row of a stratagem a mod added, once the game has loaded
its stratagems; 0 if there is none.
```

**Signature:**

```rust
pub fn stratagem_row(&self, stratagem_type: u32) -> usize
```

**Location:** `content.rs:242`

---

### FUNCTION: `add_entity`

**Documentation:**

```
Adds a new entity (call from `hellforge_mod_init`).

Returns its resource id, or 0 on failure.
```

**Signature:**

```rust
pub fn add_entity(&self, entity: &HfEntity) -> u64
```

**Location:** `content.rs:257`

---

### FUNCTION: `add_behaviour`

**Documentation:**

```
Adds a new behaviour (call from `hellforge_mod_init`).

Returns its id (694 and up), or 0 on failure.
```

**Signature:**

```rust
pub fn add_behaviour(&self, behaviour: &HfBehaviour) -> u32
```

**Location:** `content.rs:272`

---

### FUNCTION: `add_difficulty`

**Documentation:**

```
Adds difficulty level 11 (call from `hellforge_mod_init`).

Returns 11, or 0 on failure.
```

**Signature:**

```rust
pub fn add_difficulty(&self, difficulty: &HfDifficulty) -> u32
```

**Location:** `content.rs:287`

---

### FUNCTION: `difficulty_row`

**Documentation:**

```
The settings row of a difficulty level once HellForge has moved the
table; 0 if there is none.
```

**Signature:**

```rust
pub fn difficulty_row(&self, level: u32) -> usize
```

**Location:** `content.rs:301`

---

### FUNCTION: `add_ability`

**Documentation:**

```
Adds a new ability (call from `hellforge_mod_init`).

Returns its id (2868 and up), or 0 on failure.
```

**Signature:**

```rust
pub fn add_ability(&self, ability: &HfAbility) -> u32
```

**Location:** `content.rs:316`

---

### FUNCTION: `add_stratagem_category`

**Documentation:**

```
Adds a stratagem selection-screen heading (call from
`hellforge_mod_init`, before its stratagems).

Returns the category handle, or 0 on failure.
```

**Signature:**

```rust
pub fn add_stratagem_category(&self, category: &HfStratagemCategory) -> u32
```

**Location:** `content.rs:332`

---

### FUNCTION: `find`

**Documentation:**

```
Finds content by kind and name.

Vanilla content is named by its catalogue name; content a mod added can
be named by its identity (for example
`"skyeshade.pelican-support:pelican_gatling"`). Returns 0 when nothing
matches.
```

**Signature:**

```rust
pub fn find(&self, kind: u32, name: &str) -> u64
```

**Location:** `content.rs:350`

---

### FUNCTION: `content`

**Documentation:**

```
The `index`-th piece of content mods added.

Returns `None` past the last piece, or when the loader is too old.
```

**Signature:**

```rust
pub fn content(&self, index: u32) -> Option<HfContent>
```

**Location:** `content.rs:368`

---

### FUNCTION: `copy_row`

**Documentation:**

```
Replaces a settings row's bytes with another row's, live.

Returns `HF_APPLIES_NOW`, or 0 on failure.
```

**Signature:**

```rust
pub fn copy_row(&self, table: &str, row: u64, from: u64, offset: u32, size: u32) -> u32
```

**Location:** `content.rs:383`

---

## File: `hf_api/entity.rs`

### FUNCTION: `register_entity`

**Documentation:**

```
Registers a new entity from its definition block (call from
`hellforge_mod_init`).

Registering the same block again returns the same entity. Returns the
entity, or 0 on failure.
```

**Signature:**

```rust
pub fn register_entity(&self, entity: &HfEntityDef) -> HfRef
```

**Location:** `entity.rs:18`

---

### FUNCTION: `add_enemy`

**Documentation:**

```
Adds an enemy to the spawn director's rosters (call from
`hellforge_mod_init`).

Its rows are made once the entity data loads. Returns `true` when
added, `false` when refused.
```

**Signature:**

```rust
pub fn add_enemy(&self, enemy: &HfEnemy) -> bool
```

**Location:** `entity.rs:35`

---

### FUNCTION: `add_enemy_swap`

**Documentation:**

```
Adds a replacement rule to the spawn director (call from
`hellforge_mod_init`).

Returns `true` when added, `false` when refused.
```

**Signature:**

```rust
pub fn add_enemy_swap(&self, swap: &HfEnemySwap) -> bool
```

**Location:** `entity.rs:51`

---

### FUNCTION: `add_enemy_squad`

**Documentation:**

```
Adds a squad to the spawn director (call from `hellforge_mod_init`).

Its rows are made with the enemies'. Returns `true` when added,
`false` when refused.
```

**Signature:**

```rust
pub fn add_enemy_squad(&self, squad: &HfEnemySquad) -> bool
```

**Location:** `entity.rs:67`

---

### FUNCTION: `copy_field`

**Documentation:**

```
Queues a deferred copy of `field` from `from` onto `entity`.

Returns `true` when queued, `false` when refused.
```

**Signature:**

```rust
pub fn copy_field(&self, entity: HfRef, from: HfRef, field: &HfField) -> bool
```

**Location:** `entity.rs:82`

---

### FUNCTION: `set_field`

**Documentation:**

```
Queues a deferred set of `field` on `entity`.

Returns `true` when queued, `false` when refused.
```

**Signature:**

```rust
pub fn set_field(&self, entity: HfRef, field: &HfField, value: &HfValue) -> bool
```

**Location:** `entity.rs:97`

---

### FUNCTION: `set_tag`

**Documentation:**

```
Queues a deferred change to `entity`'s tags.

Returns `true` when queued, `false` when refused.
```

**Signature:**

```rust
pub fn set_tag(&self, entity: HfRef, tag: u64, present: bool) -> bool
```

**Location:** `entity.rs:112`

---

### FUNCTION: `mount`

**Documentation:**

```
Queues a deferred mount of `carried` in `entity`'s `slot`.

Returns `true` when queued, `false` when refused.
```

**Signature:**

```rust
pub fn mount(&self, entity: HfRef, slot: u32, carried: HfRef) -> bool
```

**Location:** `entity.rs:127`

---

### FUNCTION: `record`

**Documentation:**

```
An entity's own record of `component`, checked against the layout the
mod was built with.

Returns `None` when the entity does not own the record, the layout is
unknown, or the loader is too old. Reading or writing the returned
pointer is the caller's responsibility.
```

**Signature:**

```rust
pub fn record( &self, entity: HfRef, component: &str, layout: u32, size: u32, ) -> Option<*mut c_void>
```

**Location:** `entity.rs:145`

---

### FUNCTION: `edit_entity`

**Documentation:**

```
Changes an existing entity from a definition block.

Returns `true` when queued, `false` when refused.
```

**Signature:**

```rust
pub fn edit_entity(&self, edit: &HfEditDef) -> bool
```

**Location:** `entity.rs:167`

---

### FUNCTION: `set_live`

**Documentation:**

```
Changes a field in the running game, kept for the rest of the session.

`value` is `None` to undo this mod's live edit (or, with
`HF_LIVE_BASE`, to restore the field's base value). Returns
`HF_APPLIES_*`, or 0 when refused.
```

**Signature:**

```rust
pub fn set_live( &self, target: HfRef, field: &HfField, value: Option<&HfValue>, flags: u32, ) -> u32
```

**Location:** `entity.rs:184`

---

### FUNCTION: `get_live`

**Documentation:**

```
Reads a field of the running game, found the way
[`set_live`](Self::set_live) finds it.

Returns the value now and its base, or `None` when it cannot be read.
```

**Signature:**

```rust
pub fn get_live(&self, target: HfRef, field: &HfField) -> Option<HfLiveValue>
```

**Location:** `entity.rs:208`

---

## File: `hf_api/hook.rs`

### FUNCTION: `hook`

**Documentation:**

```
Detours the function at `target` to `detour`.

`original` receives a pointer that calls the next hook in the chain (or
the function's own code). Hooks stay for the session. This is
[`hook_ex`](Self::hook_ex) with priority 0.

# Safety
`detour` must be a valid function pointer with the target's calling
convention and signature. `original`, when non-null, must point to
storage the loader may write a function pointer into. Hooking the wrong
address, or using a mismatched signature, is undefined behaviour.
```

**Signature:**

```rust
pub unsafe fn hook( &self, target: usize, detour: *mut c_void, original: *mut *mut c_void, ) -> bool
```

**Location:** `hook.rs:20`

---

### FUNCTION: `hook_ex`

**Documentation:**

```
Like [`hook`](Self::hook), with a chain priority.

Mods' hooks run first, highest priority first; HellForge's own hooks run
last, nearest the game's code.

# Safety
See [`hook`](Self::hook).
```

**Signature:**

```rust
pub unsafe fn hook_ex( &self, target: usize, detour: *mut c_void, original: *mut *mut c_void, priority: i32, ) -> bool
```

**Location:** `hook.rs:40`

---

### FUNCTION: `unhook`

**Documentation:**

```
Removes this mod's hook of `target` from its chain.

Returns `true` when a hook was removed, `false` when this mod had no hook
there (or the loader is too old).
```

**Signature:**

```rust
pub fn unhook(&self, target: usize) -> bool
```

**Location:** `hook.rs:62`

---

### FUNCTION: `function`

**Documentation:**

```
The address of a game function named in `mappings/functions.json`.

Returns 0 when the name is not mapped, the game build is not the
reviewed one, the code is not loaded yet, or the code differs.
```

**Signature:**

```rust
pub fn function(&self, name: &str) -> usize
```

**Location:** `hook.rs:78`

---

### FUNCTION: `hook_function`

**Documentation:**

```
Hooks a game function by its mapped name.

Returns `true` when the hook is in, or will be installed once the game's
code is decoded and verified.

# Safety
See [`hook`](Self::hook).
```

**Signature:**

```rust
pub unsafe fn hook_function( &self, name: &str, detour: *mut c_void, original: *mut *mut c_void, priority: i32, ) -> bool
```

**Location:** `hook.rs:100`

---

### FUNCTION: `global`

**Documentation:**

```
The address of a game global's cell, named in `mappings/globals.json`.

The cell usually holds a pointer to the game object. Returns 0 when the
name is not mapped, the build is not the reviewed one, the code is not
loaded yet, or its witness does not match.
```

**Signature:**

```rust
pub fn global(&self, name: &str) -> usize
```

**Location:** `hook.rs:126`

---

## File: `hf_api/listeners.rs`

### FUNCTION: `on_game_thread`

**Documentation:**

```
Calls `callback` every game update on the game thread.

The callback runs before the game's own update, every frame: keep it
short. It is `Send` because the game thread is not the registering
thread, and it stays registered for the session (the API has no
unregister call), so a closure that never runs is not reclaimed until
the DLL is unloaded.
```

**Signature:**

```rust
pub fn on_game_thread<F>(&self, callback: F) -> Result<(), CallbackError> where F: FnMut(&HellForgeApi, f32) + Send + 'static,
```

**Location:** `listeners.rs:45`

---

### FUNCTION: `subscribe`

**Documentation:**

```
Listens to one kind of game event (`HF_ON_*`).

`listener` receives an `HfEventInfo` whose real layout is the event
kind's struct (for example `HfStateEvent`); `user` is passed back
unchanged. Returns `true` when the listener was registered.

# Safety
`listener`, when called, must treat its event pointer as the layout of
`kind` and must not keep it after returning. `user` must remain valid
for the session, or be null.
```

**Signature:**

```rust
pub unsafe fn subscribe( &self, kind: u32, listener: HfListener, user: *mut c_void, priority: i32, ) -> bool
```

**Location:** `listeners.rs:80`

---

## File: `hf_api/log.rs`

### FUNCTION: `log`

**Documentation:** *No documentation comments found.*

**Signature:**

```rust
pub fn log(&self, msg: &str)
```

**Location:** `log.rs:11`

---

### FUNCTION: `log_fmt`

**Documentation:** *No documentation comments found.*

**Signature:**

```rust
pub fn log_fmt(&self, msg: &str)
```

**Location:** `log.rs:15`

---

### FUNCTION: `get_build`

**Documentation:** *No documentation comments found.*

**Signature:**

```rust
pub fn get_build(&self) -> String
```

**Location:** `log.rs:41`

---

### FUNCTION: `mod_id`

**Documentation:**

```
The mod's own id (UTF-8), or `""` when it is not known.
```

**Signature:**

```rust
pub fn mod_id(&self) -> String
```

**Location:** `log.rs:60`

---

### FUNCTION: `mod_folder`

**Documentation:**

```
The mod's own folder (UTF-8), or `""` when it is not known.
```

**Signature:**

```rust
pub fn mod_folder(&self) -> String
```

**Location:** `log.rs:69`

---

### FUNCTION: `last_log`

**Documentation:**

```
The mod's last log line, which often says why the call that just
returned 0 was refused.
```

**Signature:**

```rust
pub fn last_log(&self) -> String
```

**Location:** `log.rs:79`

---

### FUNCTION: `logs_folder`

**Documentation:**

```
The session's logs folder (UTF-8: `modloader.log` and every
`mod-<id>.log`), or `""` when it is not known.
```

**Signature:**

```rust
pub fn logs_folder(&self) -> String
```

**Location:** `log.rs:93`

---

## File: `hf_api/mem.rs`

### FUNCTION: `module_base`

**Documentation:**

```
A loaded module's base address, or 0 when it is not loaded.
```

**Signature:**

```rust
pub fn module_base(&self, module: &str) -> usize
```

**Location:** `mem.rs:8`

---

### FUNCTION: `module_base_with_size`

**Documentation:**

```
A loaded module's base address and image size, or `None`.
```

**Signature:**

```rust
pub fn module_base_with_size(&self, module: &str) -> Option<(usize, usize)>
```

**Location:** `mem.rs:14`

---

### FUNCTION: `read`

**Documentation:**

```
Reads `buffer.len()` bytes from `address`.

Returns `true` on success.
```

**Signature:**

```rust
pub fn read(&self, address: usize, buffer: &mut [u8]) -> bool
```

**Location:** `mem.rs:27`

---

### FUNCTION: `write`

**Documentation:**

```
# Safety

You need to know where are you writing and what you're doing
```

**Signature:**

```rust
pub unsafe fn write(&self, address: usize, data: &[u8]) -> bool
```

**Location:** `mem.rs:39`

---

### FUNCTION: `write_u32_if`

**Documentation:**

```
# Safety

You need to know where are you writing and what you're doing
```

**Signature:**

```rust
pub unsafe fn write_u32_if(&self, address: usize, expect: u32, value: u32) -> bool
```

**Location:** `mem.rs:52`

---

### FUNCTION: `read_u32`

**Documentation:** *No documentation comments found.*

**Signature:**

```rust
pub fn read_u32(&self, addr: usize) -> Option<u32>
```

**Location:** `mem.rs:60`

---

### FUNCTION: `read_u64`

**Documentation:**

```
Reads a u64 value from memory address.
```

**Signature:**

```rust
pub fn read_u64(&self, addr: usize) -> Option<u64>
```

**Location:** `mem.rs:69`

---

### FUNCTION: `find_pattern`

**Documentation:**

```
Finds the first match of an IDA-style byte pattern in a loaded module.

The pattern reads the code as it is now, so a function another mod
hooked starts with a jump. Use [`code_matches`](Self::code_matches) to
check a function before hooking it.
```

**Signature:**

```rust
pub fn find_pattern(&self, module: &str, pattern: &str) -> usize
```

**Location:** `mem.rs:82`

---

### FUNCTION: `code_matches`

**Documentation:**

```
Whether the bytes at `address` matched `pattern` before any hook
HellForge placed.

This is the check to make before hooking a function: another mod's hook
does not fail it, where `find_pattern` and `read` would see the jump.
```

**Signature:**

```rust
pub fn code_matches(&self, address: usize, pattern: &str) -> bool
```

**Location:** `mem.rs:101`

---

### FUNCTION: `read_code`

**Documentation:**

```
Reads memory the way [`code_matches`](Self::code_matches) sees it: with
any hook HellForge placed seen through.

Returns `true` on success.
```

**Signature:**

```rust
pub fn read_code(&self, address: usize, buffer: &mut [u8]) -> bool
```

**Location:** `mem.rs:120`

---

## File: `hf_api/mod.rs`

### FUNCTION: `__register_entry`

**Documentation:**

```
Stores the mod's entry point. `#[hf_entry]`-generated initializers call
this while the DLL is loading, before the loader calls
[`hellforge_mod_init`].
```

**Signature:**

```rust
pub fn __register_entry(entry: HfEntryFn)
```

**Location:** `mod.rs:36`

---

### STRUCT: `HfModPtr`

**Documentation:** *No documentation comments found.*

**Signature:**

```rust
pub struct HfModPtr(*mut HfMod)
```

**Location:** `mod.rs:68`

---

### FUNCTION: `new`

**Documentation:**

```
# Safety
That pointer probably should exist
```

**Signature:**

```rust
pub unsafe fn new(ptr: *mut HfMod) -> HfModPtr
```

**Location:** `mod.rs:77`

---

### FUNCTION: `ptr`

**Documentation:** *No documentation comments found.*

**Signature:**

```rust
pub fn ptr(&self) -> *const HfMod
```

**Location:** `mod.rs:81`

---

### STRUCT: `HfApiPtr`

**Documentation:** *No documentation comments found.*

**Signature:**

```rust
pub struct HfApiPtr(*const HfApi)
```

**Location:** `mod.rs:87`

---

### FUNCTION: `new`

**Documentation:**

```
# Safety
That pointer also, preferrably, should exist
```

**Signature:**

```rust
pub unsafe fn new(ptr: *const HfApi) -> HfApiPtr
```

**Location:** `mod.rs:96`

---

### FUNCTION: `ptr`

**Documentation:** *No documentation comments found.*

**Signature:**

```rust
pub fn ptr(&self) -> *const HfApi
```

**Location:** `mod.rs:100`

---

### STRUCT: `HellForgeApi`

**Documentation:** *No documentation comments found.*

**Signature:**

```rust
pub struct HellForgeApi
```

**Location:** `mod.rs:106`

---

### FUNCTION: `version`

**Documentation:**

```
The SDK version the loader was built against.
```

**Signature:**

```rust
pub fn version(&self) -> u32
```

**Location:** `mod.rs:127`

---

### FUNCTION: `api_size`

**Documentation:**

```
The loader's `sizeof(HfApi)`, which grows as functions are added.
```

**Signature:**

```rust
pub fn api_size(&self) -> u32
```

**Location:** `mod.rs:132`

---

## File: `hf_api/multiplayer.rs`

### FUNCTION: `sync_state`

**Documentation:**

```
The current mod-sync offer.

Returns `None` when there is no offer (solo, or no backend connected).
```

**Signature:**

```rust
pub fn sync_state(&self) -> Option<HfSyncState>
```

**Location:** `multiplayer.rs:15`

---

### FUNCTION: `sync_mod`

**Documentation:**

```
The `index`-th mod of the current sync offer.

Returns `None` past the last mod.
```

**Signature:**

```rust
pub fn sync_mod(&self, index: u32) -> Option<HfSyncMod>
```

**Location:** `multiplayer.rs:30`

---

### FUNCTION: `sync_answer`

**Documentation:**

```
Sends the player's answer to a sync offer.

`mod_id` names the mod for `HF_SYNC_BLOCK`, `HF_SYNC_TRUST` and
`HF_SYNC_OPEN`; pass `None` for the other answers.

Returns `true` when the answer was sent.
```

**Signature:**

```rust
pub fn sync_answer(&self, version: u32, answer: u32, mod_id: Option<&str>) -> bool
```

**Location:** `multiplayer.rs:48`

---

### FUNCTION: `browse_state`

**Documentation:**

```
The lobby browser's state.

Returns `None` only when the loader does not expose the browser. With
no backend connected the state is still filled (`HF_BROWSE_NONE`).
```

**Signature:**

```rust
pub fn browse_state(&self) -> Option<HfBrowseState>
```

**Location:** `multiplayer.rs:75`

---

### FUNCTION: `browse_lobby`

**Documentation:**

```
The `index`-th lobby of the browser's list.

Returns `None` past the last lobby.
```

**Signature:**

```rust
pub fn browse_lobby(&self, index: u32) -> Option<HfBrowseLobby>
```

**Location:** `multiplayer.rs:91`

---

### FUNCTION: `browse_ask`

**Documentation:**

```
Asks the backend for a new list, or to join a lobby.

`lobby_id` is used by `HF_BROWSE_JOIN` and is `None` for the other asks.

Returns `true` when the ask was sent.
```

**Signature:**

```rust
pub fn browse_ask(&self, ask: u32, lobby_id: Option<&str>) -> bool
```

**Location:** `multiplayer.rs:108`

---

## File: `hf_api/network.rs`

### FUNCTION: `network_field_type`

**Documentation:**

```
Looks up a network field's type by name (after `HF_EVENT_NETWORK_READY`).

Returns `None` when the field is unknown or the loader is too old.
```

**Signature:**

```rust
pub fn network_field_type(&self, name: &str) -> Option<HfFieldType>
```

**Location:** `network.rs:12`

---

### FUNCTION: `set_network_field_bits`

**Documentation:**

```
Widens an integer network field to `bits`, if it is exactly `expect`.

Returns `true` when the type was widened (or already was); `false` when
it was left alone or the field is unknown.
```

**Signature:**

```rust
pub fn set_network_field_bits(&self, name: &str, expect: HfFieldType, bits: i32) -> bool
```

**Location:** `network.rs:26`

---

### FUNCTION: `is_authority`

**Documentation:**

```
Whether this machine simulates `entity`.

`None` means there is no such entity; the loader is too old returns
`Some(false)`.
```

**Signature:**

```rust
pub fn is_authority(&self, entity: u32) -> Option<bool>
```

**Location:** `network.rs:41`

---

### FUNCTION: `entity_of_network`

**Documentation:**

```
The entity a network id names on this machine now.

Returns `None` when the id names no entity, or the loader is too old.
```

**Signature:**

```rust
pub fn entity_of_network(&self, network: u32) -> Option<u32>
```

**Location:** `network.rs:58`

---

## File: `hf_api/session.rs`

### FUNCTION: `set_passives`

**Documentation:**

```
Overrides the local player's armor and second passives (solo only).

Returns `true` when the override is held (or released), `false` for an
id the game has no passive for.
```

**Signature:**

```rust
pub fn set_passives(&self, armor: u32, second: u32) -> bool
```

**Location:** `session.rs:14`

---

### FUNCTION: `get_passives`

**Documentation:**

```
The local player's passives now.

Returns `None` when the record is not readable.
```

**Signature:**

```rust
pub fn get_passives(&self) -> Option<HfPassives>
```

**Location:** `session.rs:29`

---

### FUNCTION: `load_package`

**Documentation:**

```
Loads a package by its resource hash and holds it for the session.

Returns `HF_PACKAGE_RESIDENT` once the engine has all of it,
`HF_PACKAGE_LOADING` until then (call again to check), or 0 when refused
or failed.
```

**Signature:**

```rust
pub fn load_package(&self, package: u64) -> u32
```

**Location:** `session.rs:45`

---

### FUNCTION: `session`

**Documentation:**

```
The session's co-op role and host name.

Returns `None` only when the loader does not expose `session`.
```

**Signature:**

```rust
pub fn session(&self) -> Option<HfSession>
```

**Location:** `session.rs:60`

---

### FUNCTION: `game_state`

**Documentation:**

```
The game's state now (`HF_STATE_*`), or `HF_STATE_NONE`.
```

**Signature:**

```rust
pub fn game_state(&self) -> u32
```

**Location:** `session.rs:73`

---

### FUNCTION: `mods`

**Documentation:**

```
The `index`-th mod of the session, in load order.

Returns `None` past the last mod, or when the loader is too old.
```

**Signature:**

```rust
pub fn mods(&self, index: u32) -> Option<HfModInfo>
```

**Location:** `session.rs:88`

---

### FUNCTION: `notice`

**Documentation:**

```
HellForge's latest message for the player.

`version` is 0 before the first notice. Returns `None` only when the
loader does not expose `notice`.
```

**Signature:**

```rust
pub fn notice(&self) -> Option<HfNotice>
```

**Location:** `session.rs:104`

---

### FUNCTION: `load_phase`

**Documentation:**

```
How this mod was set up: `HF_LOAD_BOOT` or `HF_LOAD_LIVE`.

Older loaders report `HF_LOAD_BOOT`.
```

**Signature:**

```rust
pub fn load_phase(&self) -> u32
```

**Location:** `session.rs:119`

---

### FUNCTION: `generation_capacity`

**Documentation:**

```
Asks the map generator for room in its arrays, or reads the planned
capacities.

`want` asks for at least those capacities; `None` only reads. `have`
is filled with the capacities planned and where each region lies in
the block. Returns `true` on success, `false` when refused (too late,
or a field over its limit) or when the loader is too old.
```

**Signature:**

```rust
pub fn generation_capacity( &self, want: Option<&HfGenerationCapacity>, have: Option<&mut HfGenerationCapacity>, ) -> bool
```

**Location:** `session.rs:138`

---

## File: `hf_api/stats.rs`

### FUNCTION: `get_stat`

**Documentation:**

```
A stats field's value now and its base.

Covers the fields [`get_live`](Self::get_live) cannot (entity deltas,
for example). Returns `None` when the field is not in the snapshot or
has not been read yet.
```

**Signature:**

```rust
pub fn get_stat(&self, stat: &HfStat) -> Option<HfLiveValue>
```

**Location:** `stats.rs:14`

---

### FUNCTION: `set_stat`

**Documentation:**

```
Changes a stats field live, where a stats file would write it at boot.

`value` is `None` to restore the value from before this session's live
change (or, with `HF_LIVE_BASE`, the base value). Returns
`HF_APPLIES_RESPAWN`, or 0 when refused.
```

**Signature:**

```rust
pub fn set_stat(&self, stat: &HfStat, value: Option<&HfValue>, flags: u32) -> u32
```

**Location:** `stats.rs:30`

---

### FUNCTION: `stats_source`

**Documentation:**

```
Whose stats the lobby plays with: `HF_STATS_HOST` or `HF_STATS_OWN`.
```

**Signature:**

```rust
pub fn stats_source(&self) -> u32
```

**Location:** `stats.rs:45`

---

## File: `hf_api/status.rs`

### FUNCTION: `set_status`

**Documentation:**

```
Sets the mod's status for the player.

One line, at most 191 bytes; `None` or an empty string clears it.
Returns `false` when the loader is too old or the text contains an
interior NUL byte.
```

**Signature:**

```rust
pub fn set_status(&self, text: Option<&str>) -> bool
```

**Location:** `status.rs:13`

---

## File: `hf_api/util.rs`

### FUNCTION: `wwise_id`

**Documentation:**

```
A Wwise event's id from its name (FNV-1 of the lower-case name).

This is the C SDK's `hf_wwise_id`: an event of the mod's data spec
"sounds", for the record fields that name sound events.
```

**Signature:**

```rust
pub fn wwise_id(name: &str) -> u32
```

**Location:** `util.rs:20`

---

## File: `hf_component.rs`

### FUNCTION: `component`

**Documentation:**

```
An entity's own record of component `T`, after
`HF_EVENT_ENTITIES_READY`.

Returns `None` when the entity does not own the record, owns several,
the loader is older than SDK 6, or its layout is unknown.
```

**Signature:**

```rust
pub fn component<T: Component>(&self, entity: HfRef) -> Option<&T>
```

**Location:** `hf_component.rs:123`

---

### FUNCTION: `component_mut`

**Documentation:**

```
An entity's own record of component `T`, for writing.

# Safety
Only write the record of an entity a mod added: its records are
writable. A vanilla entity's record is the game's read-only data, and
writing through it crashes the game. The caller must also make sure no
other reference or game code touches the record at the same time, and
must call on the game thread.
```

**Signature:**

```rust
pub unsafe fn component_mut<T: Component>(&self, entity: HfRef) -> Option<&mut T>
```

**Location:** `hf_component.rs:137`

---

### FUNCTION: `shared_component`

**Documentation:**

```
The record of component `T` that `resource` owns, after
`HF_EVENT_ENTITIES_READY`.

This is [`component_record`](Self::component_record) with the type
applied: it returns the record and its stride, or `None` when the
entity owns no such record, owns several, or shares it with another
entity.

Unlike [`component`](Self::component), this lookup has no layout or
size check, so `T` must match the record the resource actually owns.
```

**Signature:**

```rust
pub fn shared_component<T: Component>(&self, resource: u64) -> Option<(&T, u32)>
```

**Location:** `hf_component.rs:153`

---

## File: `hf_customization.rs`

### FUNCTION: `add_attachment`

**Documentation:**

```
Adds a new weapon attachment (call from `hellforge_mod_init`, after the
weapons it names).

`attachment` is a zeroed [`HfAttachment`] whose `size` is set. Returns
the new item's id (1 and up), or 0 on failure.

Prefer [`Attachment`], which fills the struct and its strings for you.
```

**Signature:**

```rust
pub fn add_attachment(&self, attachment: &HfAttachment) -> u32
```

**Location:** `hf_customization.rs:33`

---

### FUNCTION: `add_pattern`

**Documentation:**

```
Adds a new pattern (call from `hellforge_mod_init`).

`pattern` is a zeroed [`HfPattern`] whose `size` is set. Returns the
pattern's id (1 and up), or 0 on failure.

Prefer [`Pattern`], which fills the struct and its arrays for you.
```

**Signature:**

```rust
pub fn add_pattern(&self, pattern: &HfPattern) -> u32
```

**Location:** `hf_customization.rs:51`

---

### STRUCT: `Attachment`

**Documentation:**

```
A new weapon attachment (SDK 17, `add_attachment`).

```ignore
Attachment::new(0x73CA1B5E, "yourname/your_mod/drum", [rifle_id])
.unit(api.resource_hash("yourname/your_mod/drum_magazine"))
.name_text("Drum")
.register(api);
```
```

**Signature:**

```rust
pub struct Attachment
```

**Location:** `hf_customization.rs:73`

---

### FUNCTION: `new`

**Documentation:**

```
A copy of vanilla item `base`, offered on `weapons` (1 to 16 ids
[`HellForgeApi::add_weapon`] returned).
```

**Signature:**

```rust
pub fn new(base: u32, name: impl Into<String>, weapons: impl IntoIterator<Item = u32>) -> Self
```

**Location:** `hf_customization.rs:85`

---

### FUNCTION: `unit`

**Documentation:**

```
Its model: a resource id ([`HellForgeApi::resource_hash`]) drawn at the
weapon's `attach_` joint; 0 keeps the base's.
```

**Signature:**

```rust
pub fn unit(mut self, resource: u64) -> Self
```

**Location:** `hf_customization.rs:98`

---

### FUNCTION: `name_text`

**Documentation:**

```
Its name in the armory; absent keeps the base's.
```

**Signature:**

```rust
pub fn name_text(mut self, text: impl Into<String>) -> Self
```

**Location:** `hf_customization.rs:104`

---

### FUNCTION: `description`

**Documentation:**

```
Its description in the armory; absent keeps the base's.
```

**Signature:**

```rust
pub fn description(mut self, text: impl Into<String>) -> Self
```

**Location:** `hf_customization.rs:110`

---

### FUNCTION: `register`

**Documentation:**

```
Adds it; returns its id (1 and up), or 0 on failure.

Fails when the loader is older than SDK 17, the weapon list is empty or
longer than 16, or a string has an interior NUL byte.
```

**Signature:**

```rust
pub fn register(&self, api: &HellForgeApi) -> u32
```

**Location:** `hf_customization.rs:119`

---

### STRUCT: `Pattern`

**Documentation:**

```
A new pattern (SDK 18, `add_pattern`).

```ignore
Pattern::new(0x8F739A74, "yourname/your_mod/tiger") // Forest Camo, a paint
.swap(0x6C93BF50B44EFBEE, api.resource_hash("yourname/your_mod/patterns/tiger"))
.colour([0.6, 0.35, 0.1])
.register(api);
```
```

**Signature:**

```rust
pub struct Pattern
```

**Location:** `hf_customization.rs:176`

---

### FUNCTION: `new`

**Documentation:**

```
A copy of vanilla pattern `base`: a weapon paint or a unit skin (not a
default).
```

**Signature:**

```rust
pub fn new(base: u32, name: impl Into<String>) -> Self
```

**Location:** `hf_customization.rs:190`

---

### FUNCTION: `name_text`

**Documentation:**

```
Its name in the armory or hangar; absent keeps the base's.
```

**Signature:**

```rust
pub fn name_text(mut self, text: impl Into<String>) -> Self
```

**Location:** `hf_customization.rs:204`

---

### FUNCTION: `description`

**Documentation:**

```
A weapon pattern's description; absent keeps the base's.
```

**Signature:**

```rust
pub fn description(mut self, text: impl Into<String>) -> Self
```

**Location:** `hf_customization.rs:210`

---

### FUNCTION: `pattern_lut`

**Documentation:**

```
A unit skin's pattern colour: a `pattern_lut` resource id
([`HellForgeApi::resource_hash`]); 0 keeps the base's.
```

**Signature:**

```rust
pub fn pattern_lut(mut self, resource: u64) -> Self
```

**Location:** `hf_customization.rs:217`

---

### FUNCTION: `swap`

**Documentation:**

```
Replaces one of the base's textures (`from`) with one of the mod's
(`to`), by resource id.
```

**Signature:**

```rust
pub fn swap(mut self, from: u64, to: u64) -> Self
```

**Location:** `hf_customization.rs:224`

---

### FUNCTION: `swaps`

**Documentation:**

```
Replaces several of the base's textures at once.
```

**Signature:**

```rust
pub fn swaps(mut self, swaps: impl IntoIterator<Item = (u64, u64)>) -> Self
```

**Location:** `hf_customization.rs:230`

---

### FUNCTION: `colour`

**Documentation:**

```
A swatch colour on its card, linear RGB.
```

**Signature:**

```rust
pub fn colour(mut self, colour: [f32
```

**Location:** `hf_customization.rs:236`

---

### FUNCTION: `colours`

**Documentation:**

```
The swatch colours on its card (up to 8); absent keeps the base's.
```

**Signature:**

```rust
pub fn colours(mut self, colours: impl IntoIterator<Item = [f32
```

**Location:** `hf_customization.rs:242`

---

### FUNCTION: `weapons`

**Documentation:**

```
A weapon pattern's weapons (up to 16 ids); empty means every weapon
offering the base. Not read for unit skins.
```

**Signature:**

```rust
pub fn weapons(mut self, weapons: impl IntoIterator<Item = u32>) -> Self
```

**Location:** `hf_customization.rs:249`

---

### FUNCTION: `register`

**Documentation:**

```
Adds it; returns its id (1 and up), or 0 on failure.

Fails when the loader is older than SDK 18, there are more than 16
texture swaps or 8 colours, more than 16 weapons, or a string has an
interior NUL byte.
```

**Signature:**

```rust
pub fn register(&self, api: &HellForgeApi) -> u32
```

**Location:** `hf_customization.rs:259`

---

## File: `hf_functions.rs`

### FUNCTION: `consume_damage_event`

**Documentation:**

```
The `ConsumeDamageEvent` function (`game.dll+0x12A7D40`), or `None`.

It takes one damage event from this machine's queue in the frame's
drain, before `ApplyDamage`'s armour and multipliers.
```

**Signature:**

```rust
pub fn consume_damage_event(&self) -> Option<ConsumeDamageEventFn>
```

**Location:** `hf_functions.rs:112`

---

### FUNCTION: `kill_dispatch`

**Documentation:**

```
The `KillDispatch` function (`game.dll+0xB96310`), or `None`.
```

**Signature:**

```rust
pub fn kill_dispatch(&self) -> Option<KillDispatchFn>
```

**Location:** `hf_functions.rs:117`

---

### FUNCTION: `activate_stratagem_beacon`

**Documentation:**

```
The `ActivateStratagemBeacon` function (`game.dll+0x6ABDB0`), or
`None`.
```

**Signature:**

```rust
pub fn activate_stratagem_beacon(&self) -> Option<ActivateStratagemBeaconFn>
```

**Location:** `hf_functions.rs:123`

---

### FUNCTION: `spawn_entity`

**Documentation:**

```
The `SpawnEntity` function (`game.dll+0xFDC140`), or `None`.
```

**Signature:**

```rust
pub fn spawn_entity(&self) -> Option<SpawnEntityFn>
```

**Location:** `hf_functions.rs:128`

---

### FUNCTION: `request_explosion`

**Documentation:**

```
The `RequestExplosion` function (`game.dll+0x13C0A80`), or `None`.
```

**Signature:**

```rust
pub fn request_explosion(&self) -> Option<RequestExplosionFn>
```

**Location:** `hf_functions.rs:133`

---

### FUNCTION: `add_health_fraction`

**Documentation:**

```
The `AddHealthFraction` function (`game.dll+0x91E920`), or `None`.
```

**Signature:**

```rust
pub fn add_health_fraction(&self) -> Option<AddHealthFractionFn>
```

**Location:** `hf_functions.rs:138`

---

### FUNCTION: `restore_zone`

**Documentation:**

```
The `RestoreZone` function (`game.dll+0x65B2D0`), or `None`.
```

**Signature:**

```rust
pub fn restore_zone(&self) -> Option<RestoreZoneFn>
```

**Location:** `hf_functions.rs:143`

---

### FUNCTION: `queue_status_request`

**Documentation:**

```
The `QueueStatusRequest` function (`game.dll+0x129F170`), or `None`.
```

**Signature:**

```rust
pub fn queue_status_request(&self) -> Option<QueueStatusRequestFn>
```

**Location:** `hf_functions.rs:148`

---

### FUNCTION: `set_velocity`

**Documentation:**

```
The `SetVelocity` function (`game.dll+0x4A7550`), or `None`.
```

**Signature:**

```rust
pub fn set_velocity(&self) -> Option<SetVelocityFn>
```

**Location:** `hf_functions.rs:153`

---

### FUNCTION: `fire_projectile`

**Documentation:**

```
The `FireProjectile` function (`game.dll+0x13A8F50`), or `None`.
```

**Signature:**

```rust
pub fn fire_projectile(&self) -> Option<FireProjectileFn>
```

**Location:** `hf_functions.rs:158`

---

### FUNCTION: `hook_consume_damage_event`

**Documentation:**

```
Hooks `ConsumeDamageEvent` with `detour`, at `priority`.

Returns `true` when the hook is in, or will be installed once the
game's code is decoded and verified.

# Safety
`detour` must have the exact signature of
[`ConsumeDamageEventFn`]. `original`, when non-null, must point to
storage the loader may write the original function pointer into.
```

**Signature:**

```rust
pub unsafe fn hook_consume_damage_event( &self, detour: ConsumeDamageEventFn, original: *mut ConsumeDamageEventFn, priority: i32, ) -> bool
```

**Location:** `hf_functions.rs:171`

---

### FUNCTION: `hook_kill_dispatch`

**Documentation:**

```
Hooks `KillDispatch` with `detour`, at `priority`.

# Safety
See [`hook_consume_damage_event`](Self::hook_consume_damage_event).
```

**Signature:**

```rust
pub unsafe fn hook_kill_dispatch( &self, detour: KillDispatchFn, original: *mut KillDispatchFn, priority: i32, ) -> bool
```

**Location:** `hf_functions.rs:191`

---

### FUNCTION: `hook_activate_stratagem_beacon`

**Documentation:**

```
Hooks `ActivateStratagemBeacon` with `detour`, at `priority`.

# Safety
See [`hook_consume_damage_event`](Self::hook_consume_damage_event).
```

**Signature:**

```rust
pub unsafe fn hook_activate_stratagem_beacon( &self, detour: ActivateStratagemBeaconFn, original: *mut ActivateStratagemBeaconFn, priority: i32, ) -> bool
```

**Location:** `hf_functions.rs:211`

---

### FUNCTION: `hook_spawn_entity`

**Documentation:**

```
Hooks `SpawnEntity` with `detour`, at `priority`.

# Safety
See [`hook_consume_damage_event`](Self::hook_consume_damage_event).
```

**Signature:**

```rust
pub unsafe fn hook_spawn_entity( &self, detour: SpawnEntityFn, original: *mut SpawnEntityFn, priority: i32, ) -> bool
```

**Location:** `hf_functions.rs:231`

---

### FUNCTION: `hook_request_explosion`

**Documentation:**

```
Hooks `RequestExplosion` with `detour`, at `priority`.

# Safety
See [`hook_consume_damage_event`](Self::hook_consume_damage_event).
```

**Signature:**

```rust
pub unsafe fn hook_request_explosion( &self, detour: RequestExplosionFn, original: *mut RequestExplosionFn, priority: i32, ) -> bool
```

**Location:** `hf_functions.rs:251`

---

## File: `hf_globals.rs`

### FUNCTION: `as_ptr`

**Documentation:**

```
The game-owned object this handle reads.

It is the loader's pointer, not a copy: it is live only on the
game thread, and only until the game remakes the object.
```

**Signature:**

```rust
pub fn as_ptr(&self) -> *mut c_void
```

**Location:** `hf_globals.rs:121`

---

### FUNCTION: `state`

**Documentation:**

```
The game's state now.
```

**Signature:**

```rust
pub fn state(&self) -> Option<GameState>
```

**Location:** `hf_globals.rs:165`

---

### FUNCTION: `wwise_world`

**Documentation:**

```
The game's Wwise world, or `None`.
```

**Signature:**

```rust
pub fn wwise_world(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:172`

---

### FUNCTION: `level`

**Documentation:**

```
The mission's level (the level generator), or `None` outside a mission.
```

**Signature:**

```rust
pub fn level(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:177`

---

### FUNCTION: `mode`

**Documentation:**

```
The mode: 1 in a mission.
```

**Signature:**

```rust
pub fn mode(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:189`

---

### FUNCTION: `in_mission`

**Documentation:**

```
Whether the game is in a mission.
```

**Signature:**

```rust
pub fn in_mission(&self) -> Option<bool>
```

**Location:** `hf_globals.rs:194`

---

### FUNCTION: `mission`

**Documentation:**

```
The mission's data, or `None` when there is no mission.
```

**Signature:**

```rust
pub fn mission(&self) -> Option<Mission<'a>>
```

**Location:** `hf_globals.rs:199`

---

### STRUCT: `Mission`

**Documentation:**

```
The mission's data, from [`GameMode::mission`].
```

**Signature:**

```rust
pub struct Mission<'a>(Memory<'a>)
```

**Location:** `hf_globals.rs:208`

---

### FUNCTION: `is_host`

**Documentation:**

```
Whether this machine hosts the mission.

Bit 0 of the mission's `+0x14` is set on the host.
```

**Signature:**

```rust
pub fn is_host(&self) -> Option<bool>
```

**Location:** `hf_globals.rs:214`

---

### FUNCTION: `entity`

**Documentation:**

```
The game mode's entity.
```

**Signature:**

```rust
pub fn entity(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:219`

---

### FUNCTION: `as_ptr`

**Documentation:**

```
The game-owned mission data this handle reads.
```

**Signature:**

```rust
pub fn as_ptr(&self) -> *mut c_void
```

**Location:** `hf_globals.rs:224`

---

### FUNCTION: `count`

**Documentation:**

```
The number of players in the session (at most 4).
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:256`

---

### FUNCTION: `slot`

**Documentation:**

```
The player in slot `index`, or `None` when the slot is empty.
```

**Signature:**

```rust
pub fn slot(&self, index: u32) -> Option<Player<'a>>
```

**Location:** `hf_globals.rs:263`

---

### FUNCTION: `players`

**Documentation:**

```
The players in the session, in slot order.
```

**Signature:**

```rust
pub fn players(&self) -> impl Iterator<Item = Player<'a>> + use<'a>
```

**Location:** `hf_globals.rs:271`

---

### STRUCT: `Player`

**Documentation:**

```
One slot of [`PlayerManager`].
```

**Signature:**

```rust
pub struct Player<'a>
```

**Location:** `hf_globals.rs:280`

---

### FUNCTION: `peer`

**Documentation:**

```
The player's peer id, the stable way to tell players apart.
```

**Signature:**

```rust
pub fn peer(&self) -> Option<u64>
```

**Location:** `hf_globals.rs:298`

---

### FUNCTION: `lifecycle`

**Documentation:**

```
The player's lifecycle.
```

**Signature:**

```rust
pub fn lifecycle(&self) -> Option<PlayerLifecycle>
```

**Location:** `hf_globals.rs:303`

---

### FUNCTION: `entity`

**Documentation:**

```
The player's player entity.
```

**Signature:**

```rust
pub fn entity(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:310`

---

### FUNCTION: `network_id`

**Documentation:**

```
The player's Helldiver's network id.
```

**Signature:**

```rust
pub fn network_id(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:319`

---

### FUNCTION: `peer`

**Documentation:**

```
This machine's player's peer id, as in [`Player::peer`].
```

**Signature:**

```rust
pub fn peer(&self) -> Option<u64>
```

**Location:** `hf_globals.rs:332`

---

### FUNCTION: `host_peer`

**Documentation:**

```
The game's host's peer id (it changes with a host migration).
```

**Signature:**

```rust
pub fn host_peer(&self) -> Option<u64>
```

**Location:** `hf_globals.rs:337`

---

### FUNCTION: `component_tables`

**Documentation:**

```
Component table 0; table `c`'s pointer is at `+8 c`.
```

**Signature:**

```rust
pub fn component_tables(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:358`

---

### FUNCTION: `entity_deltas`

**Documentation:**

```
The entity deltas a weapon's chosen attachments key into.
```

**Signature:**

```rust
pub fn entity_deltas(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:364`

---

### FUNCTION: `entity_map`

**Documentation:**

```
The entity map: 4096 rows, from entity % 4096.
```

**Signature:**

```rust
pub fn entity_map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:370`

---

### FUNCTION: `short_ids`

**Documentation:**

```
The short ids: sorted `{u32 short id, u32 map row}`.
```

**Signature:**

```rust
pub fn short_ids(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:375`

---

### FUNCTION: `network_object`

**Documentation:**

```
The network object in `slot`, whose `slot` comes from the manager's
network-id map.
```

**Signature:**

```rust
pub fn network_object(&self, slot: u32) -> NetworkObject<'a>
```

**Location:** `hf_globals.rs:381`

---

### STRUCT: `NetworkObject`

**Documentation:**

```
One entry of [`EntityManager`]'s network objects.
```

**Signature:**

```rust
pub struct NetworkObject<'a>(Memory<'a>)
```

**Location:** `hf_globals.rs:391`

---

### FUNCTION: `entity`

**Documentation:**

```
The entity this network object names.
```

**Signature:**

```rust
pub fn entity(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:395`

---

### FUNCTION: `count`

**Documentation:**

```
The number of indexes in use.
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:416`

---

### FUNCTION: `descriptor`

**Documentation:**

```
The descriptor of index `index`, which names its entity.
```

**Signature:**

```rust
pub fn descriptor(&self, index: u32) -> Option<HealthDescriptor<'a>>
```

**Location:** `hf_globals.rs:445`

---

### FUNCTION: `health`

**Documentation:**

```
The health of index `index`.
```

**Signature:**

```rust
pub fn health(&self, index: u32) -> Option<i32>
```

**Location:** `hf_globals.rs:455`

---

### FUNCTION: `life`

**Documentation:**

```
The life of index `index`.
```

**Signature:**

```rust
pub fn life(&self, index: u32) -> Option<HealthLife>
```

**Location:** `hf_globals.rs:463`

---

### FUNCTION: `max_health`

**Documentation:**

```
The maximum health of index `index`.
```

**Signature:**

```rust
pub fn max_health(&self, index: u32) -> Option<i32>
```

**Location:** `hf_globals.rs:472`

---

### STRUCT: `HealthDescriptor`

**Documentation:**

```
An entity named by a [`HealthManager`] descriptor.
```

**Signature:**

```rust
pub struct HealthDescriptor<'a>(Memory<'a>)
```

**Location:** `hf_globals.rs:482`

---

### FUNCTION: `resource`

**Documentation:**

```
The entity's resource id.
```

**Signature:**

```rust
pub fn resource(&self) -> Option<u64>
```

**Location:** `hf_globals.rs:486`

---

### FUNCTION: `entity`

**Documentation:**

```
The entity.
```

**Signature:**

```rust
pub fn entity(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:491`

---

### FUNCTION: `unit`

**Documentation:**

```
The entity's unit.
```

**Signature:**

```rust
pub fn unit(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:496`

---

### FUNCTION: `network_id`

**Documentation:**

```
The entity's network id.
```

**Signature:**

```rust
pub fn network_id(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:501`

---

### FUNCTION: `owned_here`

**Documentation:**

```
Whether this machine owns the entity (bit 0 of `+0x14`).
```

**Signature:**

```rust
pub fn owned_here(&self) -> Option<bool>
```

**Location:** `hf_globals.rs:506`

---

### FUNCTION: `count`

**Documentation:**

```
The number of indexes in use.
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:540`

---

### FUNCTION: `positions`

**Documentation:**

```
The positions `MoveObject` writes, 0xA4 bytes per index (three floats
at the start).
```

**Signature:**

```rust
pub fn positions(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:560`

---

### FUNCTION: `sources`

**Documentation:**

```
The positions motion starts from each frame, 0x2C bytes per index
(three floats).
```

**Signature:**

```rust
pub fn sources(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:566`

---

### FUNCTION: `entity`

**Documentation:**

```
The entity of index `index`.
```

**Signature:**

```rust
pub fn entity(&self, index: u32) -> Option<u32>
```

**Location:** `hf_globals.rs:580`

---

### FUNCTION: `is_live`

**Documentation:**

```
Whether index `index` is live (bit 0 of its descriptor's `+0x14`).
```

**Signature:**

```rust
pub fn is_live(&self, index: u32) -> Option<bool>
```

**Location:** `hf_globals.rs:585`

---

### FUNCTION: `velocity`

**Documentation:**

```
The velocity of index `index`, in metres per second.
```

**Signature:**

```rust
pub fn velocity(&self, index: u32) -> Option<[f32
```

**Location:** `hf_globals.rs:592`

---

### FUNCTION: `count`

**Documentation:**

```
The number of requests queued (256 at most: `RequestExplosion` refuses
more).
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:607`

---

### FUNCTION: `is_running`

**Documentation:**

```
Whether projectiles run now (in a mission).
```

**Signature:**

```rust
pub fn is_running(&self) -> Option<bool>
```

**Location:** `hf_globals.rs:620`

---

### FUNCTION: `damage_count`

**Documentation:**

```
The damage requests queued (4096 at most).
```

**Signature:**

```rust
pub fn damage_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:635`

---

### FUNCTION: `status_count`

**Documentation:**

```
The status requests queued (4096 at most).
```

**Signature:**

```rust
pub fn status_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:640`

---

### FUNCTION: `record`

**Documentation:**

```
The stat record of `index`, whose `index` comes from the manager's map
of player entities to indexes.
```

**Signature:**

```rust
pub fn record(&self, index: u32) -> ScoreRecord<'a>
```

**Location:** `hf_globals.rs:662`

---

### STRUCT: `ScoreRecord`

**Documentation:**

```
One player's stat record in [`ScoreManager`].
```

**Signature:**

```rust
pub struct ScoreRecord<'a>(Memory<'a>)
```

**Location:** `hf_globals.rs:672`

---

### FUNCTION: `sources`

**Documentation:**

```
The record's 64 source blocks, or `None` before the game fills them.
```

**Signature:**

```rust
pub fn sources(&self) -> Option<ScoreSources<'a>>
```

**Location:** `hf_globals.rs:679`

---

### FUNCTION: `as_ptr`

**Documentation:**

```
The record's data, as the game owns it.
```

**Signature:**

```rust
pub fn as_ptr(&self) -> *mut c_void
```

**Location:** `hf_globals.rs:686`

---

### STRUCT: `ScoreSources`

**Documentation:**

```
The source blocks of a [`ScoreRecord`].
```

**Signature:**

```rust
pub struct ScoreSources<'a>(Memory<'a>)
```

**Location:** `hf_globals.rs:693`

---

### FUNCTION: `get`

**Documentation:**

```
Source block `index` (fewer than [`COUNT`](Self::COUNT)).
```

**Signature:**

```rust
pub fn get(&self, index: usize) -> Option<ScoreSource<'a>>
```

**Location:** `hf_globals.rs:700`

---

### FUNCTION: `as_ptr`

**Documentation:**

```
The blocks' data, as the game owns it.
```

**Signature:**

```rust
pub fn as_ptr(&self) -> *mut c_void
```

**Location:** `hf_globals.rs:705`

---

### STRUCT: `ScoreSource`

**Documentation:**

```
One source block of a [`ScoreRecord`]: a resource and 16 stat entries
(`HF_STAT_*`).
```

**Signature:**

```rust
pub struct ScoreSource<'a>(Memory<'a>)
```

**Location:** `hf_globals.rs:713`

---

### FUNCTION: `as_ptr`

**Documentation:**

```
The block's data, as the game owns it.
```

**Signature:**

```rust
pub fn as_ptr(&self) -> *mut c_void
```

**Location:** `hf_globals.rs:717`

---

### FUNCTION: `slots`

**Documentation:**

```
The instances' slots, or `None` before the game makes them.
```

**Signature:**

```rust
pub fn slots(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:731`

---

### FUNCTION: `hand`

**Documentation:**

```
Slot 0 (the instance's hand) of `instance`.
```

**Signature:**

```rust
pub fn hand(&self, instance: u32) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:736`

---

### FUNCTION: `records`

**Documentation:**

```
The inventory records, or `None` before the game makes them.
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:777`

---

### FUNCTION: `record`

**Documentation:**

```
Record `index` of an entity, whose `index` comes from the manager's map
of entities to records.
```

**Signature:**

```rust
pub fn record(&self, index: u32) -> Option<InventoryRecord<'a>>
```

**Location:** `hf_globals.rs:783`

---

### STRUCT: `InventoryRecord`

**Documentation:**

```
One inventory record in [`InventoryManager`].
```

**Signature:**

```rust
pub struct InventoryRecord<'a>(Memory<'a>)
```

**Location:** `hf_globals.rs:795`

---

### FUNCTION: `selection`

**Documentation:**

```
The inventory selection.
```

**Signature:**

```rust
pub fn selection(&self) -> Option<InventorySelection>
```

**Location:** `hf_globals.rs:801`

---

### FUNCTION: `index`

**Documentation:**

```
The destructible index's holder, or `None`.
```

**Signature:**

```rust
pub fn index(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:818`

---

### FUNCTION: `count`

**Documentation:**

```
The number of destructibles.
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:823`

---

### FUNCTION: `descriptor`

**Documentation:**

```
The descriptor pointer of destructible `index`.
```

**Signature:**

```rust
pub fn descriptor(&self, index: u32) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:832`

---

### FUNCTION: `world_index`

**Documentation:**

```
The world index of destructible `index`.
```

**Signature:**

```rust
pub fn world_index(&self, index: u32) -> Option<u32>
```

**Location:** `hf_globals.rs:840`

---

### FUNCTION: `slots`

**Documentation:**

```
The record map's slots (0x8000 inline at `+0xC5F48`).
```

**Signature:**

```rust
pub fn slots(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:861`

---

### FUNCTION: `slot_count`

**Documentation:**

```
The record map's slot count (a power of two; mask it with count - 1).
```

**Signature:**

```rust
pub fn slot_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:866`

---

### FUNCTION: `empty_key`

**Documentation:**

```
The record map's empty key.
```

**Signature:**

```rust
pub fn empty_key(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:872`

---

### FUNCTION: `record`

**Documentation:**

```
The record of `world_index`, which must be one of
[`Destructibles::world_index`]'s values.
```

**Signature:**

```rust
pub fn record(&self, world_index: u32) -> DestructionRecord<'a>
```

**Location:** `hf_globals.rs:878`

---

### STRUCT: `DestructionRecord`

**Documentation:**

```
One destructible's state in [`DestructionRecords`].
```

**Signature:**

```rust
pub struct DestructionRecord<'a>(Memory<'a>)
```

**Location:** `hf_globals.rs:888`

---

### FUNCTION: `unit`

**Documentation:**

```
The destructible's unit.
```

**Signature:**

```rust
pub fn unit(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:896`

---

### FUNCTION: `stage`

**Documentation:**

```
The destructible's destruction stage.
```

**Signature:**

```rust
pub fn stage(&self) -> Option<u8>
```

**Location:** `hf_globals.rs:901`

---

### FUNCTION: `registered`

**Documentation:**

```
Whether the destructible is registered (bit 0 of `+0x8`).
```

**Signature:**

```rust
pub fn registered(&self) -> Option<bool>
```

**Location:** `hf_globals.rs:906`

---

### FUNCTION: `game`

**Documentation:**

```
The Game (`game.dll+0x3326340`), or `None` when the game has not made
it yet.
```

**Signature:**

```rust
pub fn game(&self) -> Option<Game<'_>>
```

**Location:** `hf_globals.rs:946`

---

### FUNCTION: `game_mode`

**Documentation:**

```
The game mode (`game.dll+0x33266A0`), or `None` when there is none.
```

**Signature:**

```rust
pub fn game_mode(&self) -> Option<GameMode<'_>>
```

**Location:** `hf_globals.rs:951`

---

### FUNCTION: `player_manager`

**Documentation:**

```
The players of the session (`game.dll+0x3326468`), or `None`.
```

**Signature:**

```rust
pub fn player_manager(&self) -> Option<PlayerManager<'_>>
```

**Location:** `hf_globals.rs:956`

---

### FUNCTION: `local_user`

**Documentation:**

```
This machine's user (`game.dll+0x347CEF0`), or `None`.
```

**Signature:**

```rust
pub fn local_user(&self) -> Option<LocalUser<'_>>
```

**Location:** `hf_globals.rs:964`

---

### FUNCTION: `entity_manager`

**Documentation:**

```
The entities of this machine's world (`game.dll+0x346BF98`), or `None`.
```

**Signature:**

```rust
pub fn entity_manager(&self) -> Option<EntityManager<'_>>
```

**Location:** `hf_globals.rs:969`

---

### FUNCTION: `health_manager`

**Documentation:**

```
Every entity's health (`game.dll+0x3326688`), or `None`.
```

**Signature:**

```rust
pub fn health_manager(&self) -> Option<HealthManager<'_>>
```

**Location:** `hf_globals.rs:977`

---

### FUNCTION: `motion_manager`

**Documentation:**

```
The entities that move (`game.dll+0x3326558`), or `None`.
```

**Signature:**

```rust
pub fn motion_manager(&self) -> Option<MotionManager<'_>>
```

**Location:** `hf_globals.rs:985`

---

### FUNCTION: `explosion_queue`

**Documentation:**

```
The explosion requests of this frame (`game.dll+0x346D558`), or `None`.
```

**Signature:**

```rust
pub fn explosion_queue(&self) -> Option<ExplosionQueue<'_>>
```

**Location:** `hf_globals.rs:993`

---

### FUNCTION: `projectile_system`

**Documentation:**

```
The projectiles in flight (`game.dll+0x347CEA8`), or `None`.
```

**Signature:**

```rust
pub fn projectile_system(&self) -> Option<ProjectileSystem<'_>>
```

**Location:** `hf_globals.rs:1001`

---

### FUNCTION: `combat_queues`

**Documentation:**

```
This frame's damage and status requests (`game.dll+0x347CF38`), or
`None`.
```

**Signature:**

```rust
pub fn combat_queues(&self) -> Option<CombatQueues<'_>>
```

**Location:** `hf_globals.rs:1010`

---

### FUNCTION: `score_manager`

**Documentation:**

```
The players' mission stats (`game.dll+0x3326AE0`), or `None`.
```

**Signature:**

```rust
pub fn score_manager(&self) -> Option<ScoreManager<'_>>
```

**Location:** `hf_globals.rs:1015`

---

### FUNCTION: `wielder_manager`

**Documentation:**

```
What entities hold (`game.dll+0x3326420`), or `None`.
```

**Signature:**

```rust
pub fn wielder_manager(&self) -> Option<WielderManager<'_>>
```

**Location:** `hf_globals.rs:1020`

---

### FUNCTION: `inventory_manager`

**Documentation:**

```
The Helldivers' inventories (`game.dll+0x3326738`), or `None`.
```

**Signature:**

```rust
pub fn inventory_manager(&self) -> Option<InventoryManager<'_>>
```

**Location:** `hf_globals.rs:1028`

---

### FUNCTION: `destructibles`

**Documentation:**

```
The world's destructible objects (`game.dll+0x3326550`), or `None`.
```

**Signature:**

```rust
pub fn destructibles(&self) -> Option<Destructibles<'_>>
```

**Location:** `hf_globals.rs:1036`

---

### FUNCTION: `destruction_records`

**Documentation:**

```
The destruction state of the world's destructibles
(`game.dll+0x346D528`), or `None`.
```

**Signature:**

```rust
pub fn destruction_records(&self) -> Option<DestructionRecords<'_>>
```

**Location:** `hf_globals.rs:1045`

---

### FUNCTION: `amounts`

**Documentation:**

```
`+0x50` 8 bytes per deposit: +0 the supplies left (u32)
```

**Signature:**

```rust
pub fn amounts(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1063`

---

### FUNCTION: `definitions`

**Documentation:**

```
`+0xA0` the overriding definitions, 0x98 bytes each (+0x24 the self-use ability: 2629 for
the B-1 Supply Pack)
```

**Signature:**

```rust
pub fn definitions(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1069`

---

### FUNCTION: `simulated`

**Documentation:**

```
`+0x70` avatars simulated on this machine: they take the first indexes, and AvatarUpdate
runs only those
```

**Signature:**

```rust
pub fn simulated(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1091`

---

### FUNCTION: `simulated`

**Documentation:**

```
`+0x70` avatars simulated on this machine: they take the first indexes, and AvatarUpdate
runs only those
```

**Signature:**

```rust
pub fn simulated(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1106`

---

### FUNCTION: `wielders`

**Documentation:**

```
`+0x38` the wielder of each index (u32 entity each)
```

**Signature:**

```rust
pub fn wielders(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1120`

---

### FUNCTION: `masks`

**Documentation:**

```
`+0x12078` the faction mask of each index (u32 each; bit 0 the Helldivers' side)
```

**Signature:**

```rust
pub fn masks(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1134`

---

### FUNCTION: `handles`

**Documentation:**

```
`+0x58` a pointer per index: +8 the entity (u32), +0x10 its network handle (0x7FFF: none)
```

**Signature:**

```rust
pub fn handles(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1148`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x60` the behaviour records, 0x1F8 bytes each; P (record + 8) is a behaviour's state, P+0
its stage
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1154`

---

### FUNCTION: `tags`

**Documentation:**

```
`+0x38` the tag word of each index (u64 each): 0x10000000 its kills credited to no one, bit
35 never targeted
```

**Signature:**

```rust
pub fn tags(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1169`

---

### FUNCTION: `now`

**Documentation:**

```
`+0x18` now, in microseconds
```

**Signature:**

```rust
pub fn now(&self) -> Option<u64>
```

**Location:** `hf_globals.rs:1183`

---

### FUNCTION: `application`

**Documentation:**

```
`+0x10` the Application functions (helldivers2.exe+0x27C9E60): +0x3E8 the text of an id,
const char* (u32 id), which HellForge points at its own to answer mod texts; +0x378 the
resource manager's lock (ResourceOverride)
```

**Signature:**

```rust
pub fn application(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1201`

---

### FUNCTION: `unit`

**Documentation:**

```
`+0x18` the Unit functions (helldivers2.exe+0x27C8F10): +0x88 a node's world position (unit,
node), +0x748 the unit object for a handle (jumps to UnitFromHandle)
```

**Signature:**

```rust
pub fn unit(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1207`

---

### FUNCTION: `actor_by_name`

**Documentation:**

```
`+0x10` UnitActorByName (helldivers2.exe 0x799DE0): (unit, actor name hash, NULL) -> the
actor, 0xFFFFFFFF none
```

**Signature:**

```rust
pub fn actor_by_name(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1222`

---

### FUNCTION: `post`

**Documentation:**

```
`+0x338` posts a sound: (world, event, game object, flags)
```

**Signature:**

```rust
pub fn post(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1238`

---

### FUNCTION: `unit_source`

**Documentation:**

```
`+0x590` the source on a unit's node: (world, unit, node)
```

**Signature:**

```rust
pub fn unit_source(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1243`

---

### FUNCTION: `source_object`

**Documentation:**

```
`+0x5A0` a source's game object: (world, source), -1 when the source is gone
```

**Signature:**

```rust
pub fn source_object(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1248`

---

### FUNCTION: `descriptors`

**Documentation:**

```
`+0x38` a descriptor pointer per index (LinkerRecord's argument)
```

**Signature:**

```rust
pub fn descriptors(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1272`

---

### FUNCTION: `copy_count`

**Documentation:**

```
`+0x90` record copies in use
```

**Signature:**

```rust
pub fn copy_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1278`

---

### FUNCTION: `copies`

**Documentation:**

```
`+0x98` the record copies, 12 bytes each
```

**Signature:**

```rust
pub fn copies(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1283`

---

### FUNCTION: `rows`

**Documentation:**

```
`+0x48` 0xC0 bytes per index: +0x48 the missile's position (3 floats)
```

**Signature:**

```rust
pub fn rows(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1298`

---

### FUNCTION: `instances`

**Documentation:**

```
`+0x50` 0x38 bytes per index: +8 its remote's network handle, +0xC its guidance target,
+0x35 engine byte, +0x36 launched byte
```

**Signature:**

```rust
pub fn instances(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1305`

---

### FUNCTION: `instances`

**Documentation:**

```
`+0x58` 0x38 bytes per index: +0x2C the marked point (3 floats)
```

**Signature:**

```rust
pub fn instances(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1320`

---

### FUNCTION: `on`

**Documentation:**

```
`+0x60` a byte per index: the laser on
```

**Signature:**

```rust
pub fn on(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1326`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x40` 48 bytes per index: +4 the owner entity
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1357`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x40` 48 bytes per index: +4 the owner entity
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1372`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x48` 6 u32 per index: the mounted children 0 to 4 first
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1401`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x60` 0x20 bytes per index: +0 the anchor
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1415`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x50` the perceiver records, 0x13F8 bytes each
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1430`

---

### FUNCTION: `hash`

**Documentation:**

```
`+0x20` the hash of the entities that have an instance (8-byte rows: entity, instance index)
```

**Signature:**

```rust
pub fn hash(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1462`

---

### FUNCTION: `descriptors`

**Documentation:**

```
`+0x38` each instance's entity descriptor (8 bytes each: +0 type, +8 entity)
```

**Signature:**

```rust
pub fn descriptors(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1467`

---

### FUNCTION: `live`

**Documentation:**

```
`+0x50` each instance's live row, 12 bytes: +0 spare magazines (network field 0xEC64918B),
+4 rounds in the magazine (MagazineRounds returns it)
```

**Signature:**

```rust
pub fn live(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1474`

---

### FUNCTION: `hash`

**Documentation:**

```
`+0x20` the hash of the entities that have an instance (8-byte rows: entity, instance index)
```

**Signature:**

```rust
pub fn hash(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1489`

---

### FUNCTION: `descriptors`

**Documentation:**

```
`+0x38` each instance's entity descriptor (8 bytes each: +0 type, +8 entity)
```

**Signature:**

```rust
pub fn descriptors(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1494`

---

### FUNCTION: `live`

**Documentation:**

```
`+0x50` each instance's live row, 12 bytes: +0 spare magazines (network field 0xEC64918B),
+4 rounds in the magazine (MagazineRounds returns it)
```

**Signature:**

```rust
pub fn live(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1501`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x178` the targeting records, 0xD0 bytes each
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1515`

---

### FUNCTION: `network`

**Documentation:**

```
`+0x180` their network blocks, 0x18 bytes each
```

**Signature:**

```rust
pub fn network(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1520`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x30` the map's entries, 8 bytes each
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1535`

---

### FUNCTION: `capacity`

**Documentation:**

```
`+0x38` the map's entries
```

**Signature:**

```rust
pub fn capacity(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1540`

---

### FUNCTION: `empty`

**Documentation:**

```
`+0x3C` the key of an empty entry
```

**Signature:**

```rust
pub fn empty(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1545`

---

### FUNCTION: `bindings`

**Documentation:**

```
`+0x48` a binding pointer per index (ShieldRecord's argument)
```

**Signature:**

```rust
pub fn bindings(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1550`

---

### FUNCTION: `rows`

**Documentation:**

```
`+0x50` 0x88 bytes per index: +0 the physics handle
```

**Signature:**

```rust
pub fn rows(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1555`

---

### FUNCTION: `live`

**Documentation:**

```
`+0x58` 0x14 bytes per index: +2 broken (u8), +4 health
```

**Signature:**

```rust
pub fn live(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1560`

---

### FUNCTION: `kits`

**Documentation:**

```
`+0x0` the armor kits (a kit row's +0x1C its passive)
```

**Signature:**

```rust
pub fn kits(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1577`

---

### FUNCTION: `kit_count`

**Documentation:**

```
`+0x8` the kits
```

**Signature:**

```rust
pub fn kit_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1582`

---

### FUNCTION: `passives`

**Documentation:**

```
`+0x20` pointers to the passive rows (0x34 bytes each), by slot
```

**Signature:**

```rust
pub fn passives(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1587`

---

### FUNCTION: `passive_count`

**Documentation:**

```
`+0x28` the passive rows (32)
```

**Signature:**

```rust
pub fn passive_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1593`

---

### FUNCTION: `record_count`

**Documentation:**

```
`+0x8E8` records in use (at most 4, one a player)
```

**Signature:**

```rust
pub fn record_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1599`

---

### FUNCTION: `kits`

**Documentation:**

```
`+0x0` the armor kits (a kit row's +0x1C its passive)
```

**Signature:**

```rust
pub fn kits(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1615`

---

### FUNCTION: `kit_count`

**Documentation:**

```
`+0x8` the kits
```

**Signature:**

```rust
pub fn kit_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1620`

---

### FUNCTION: `passives`

**Documentation:**

```
`+0x20` pointers to the passive rows (0x34 bytes each), by slot
```

**Signature:**

```rust
pub fn passives(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1625`

---

### FUNCTION: `passive_count`

**Documentation:**

```
`+0x28` the passive rows (32)
```

**Signature:**

```rust
pub fn passive_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1631`

---

### FUNCTION: `record_count`

**Documentation:**

```
`+0x8E8` records in use (at most 4, one a player)
```

**Signature:**

```rust
pub fn record_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1637`

---

### FUNCTION: `blocks`

**Documentation:**

```
`+0x60` 0x1C bytes per index: +0 shells a salvo, +4 salvos, +8 the target, +0x14 heading,
+0x18 seed
```

**Signature:**

```rust
pub fn blocks(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1654`

---

### FUNCTION: `bought_count`

**Documentation:**

```
`+0xD8A58` warbond items purchased (5280 at most), their ids following
```

**Signature:**

```rust
pub fn bought_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1674`

---

### FUNCTION: `login`

**Documentation:**

```
`+0xDDCF8` the login's state: 12 once logged in
```

**Signature:**

```rust
pub fn login(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1679`

---

### FUNCTION: `pending_buy`

**Documentation:**

```
`+0x17799D8` nonzero while a warbond buy waits for its reply
```

**Signature:**

```rust
pub fn pending_buy(&self) -> Option<u8>
```

**Location:** `hf_globals.rs:1684`

---

### FUNCTION: `pending_item`

**Documentation:**

```
`+0x17799DC` the pending buy's seasonPassItemId
```

**Signature:**

```rust
pub fn pending_item(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1689`

---

### FUNCTION: `bought_count`

**Documentation:**

```
`+0xD8A58` warbond items purchased (5280 at most), their ids following
```

**Signature:**

```rust
pub fn bought_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1704`

---

### FUNCTION: `login`

**Documentation:**

```
`+0xDDCF8` the login's state: 12 once logged in
```

**Signature:**

```rust
pub fn login(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1709`

---

### FUNCTION: `pending_buy`

**Documentation:**

```
`+0x17799D8` nonzero while a warbond buy waits for its reply
```

**Signature:**

```rust
pub fn pending_buy(&self) -> Option<u8>
```

**Location:** `hf_globals.rs:1714`

---

### FUNCTION: `pending_item`

**Documentation:**

```
`+0x17799DC` the pending buy's seasonPassItemId
```

**Signature:**

```rust
pub fn pending_item(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1719`

---

### FUNCTION: `count`

**Documentation:**

```
`+0x2D200` records in use
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1737`

---

### FUNCTION: `left_count`

**Documentation:**

```
`+0x2FD0C` loadouts kept for players who left
```

**Signature:**

```rust
pub fn left_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1742`

---

### FUNCTION: `primary`

**Documentation:**

```
`+0x40` the primary weapon's loadout id
```

**Signature:**

```rust
pub fn primary(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1757`

---

### FUNCTION: `secondary`

**Documentation:**

```
`+0x44` the secondary weapon's loadout id
```

**Signature:**

```rust
pub fn secondary(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1762`

---

### FUNCTION: `throwable`

**Documentation:**

```
`+0x48` the throwable's loadout id
```

**Signature:**

```rust
pub fn throwable(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1767`

---

### FUNCTION: `presenter`

**Documentation:**

```
`+0x4294` the menu presenter open (14 the loadout screen)
```

**Signature:**

```rust
pub fn presenter(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1783`

---

### FUNCTION: `depth`

**Documentation:**

```
`+0x42B0` states on the stack (1 to 5)
```

**Signature:**

```rust
pub fn depth(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1788`

---

### FUNCTION: `presenter`

**Documentation:**

```
`+0x4294` the menu presenter open (14 the loadout screen)
```

**Signature:**

```rust
pub fn presenter(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1802`

---

### FUNCTION: `depth`

**Documentation:**

```
`+0x42B0` states on the stack (1 to 5)
```

**Signature:**

```rust
pub fn depth(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1807`

---

### FUNCTION: `top`

**Documentation:**

```
`+0x0` the menu on top (11 the loadout screen)
```

**Signature:**

```rust
pub fn top(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1821`

---

### FUNCTION: `starts`

**Documentation:**

```
`+0x1B24` type 0's first entry; type t's at +4 t
```

**Signature:**

```rust
pub fn starts(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1838`

---

### FUNCTION: `counts`

**Documentation:**

```
`+0x1B40` type 0's entries; type t's at +4 t
```

**Signature:**

```rust
pub fn counts(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1843`

---

### FUNCTION: `active_card`

**Documentation:**

```
`+0x2B38` the card being generated (0xFFFFFFFF none)
```

**Signature:**

```rust
pub fn active_card(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1860`

---

### FUNCTION: `phase`

**Documentation:**

```
`+0x2B58` the active card's phase
```

**Signature:**

```rust
pub fn phase(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1865`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x0` the reference map's entries
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1880`

---

### FUNCTION: `capacity`

**Documentation:**

```
`+0x8` the map's entries
```

**Signature:**

```rust
pub fn capacity(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1885`

---

### FUNCTION: `instance_count`

**Documentation:**

```
`+0x24` objective instances in the mission
```

**Signature:**

```rust
pub fn instance_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1927`

---

### FUNCTION: `level`

**Documentation:**

```
`+0x650` the mission's level (lg): its difficulty level at +0xF9 (u8)
```

**Signature:**

```rust
pub fn level(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1944`

---

### FUNCTION: `timer_count`

**Documentation:**

```
`+0x67C` the mission's timers (hf->freeze_mission_timer needs 1)
```

**Signature:**

```rust
pub fn timer_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1949`

---

### FUNCTION: `director`

**Documentation:**

```
`+0x6B0` the timer's director: +8 the game mode's entity (u32)
```

**Signature:**

```rust
pub fn director(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:1954`

---

### FUNCTION: `guard_compositions`

**Documentation:**

```
`+0x51884` the compiled map guard (outpost) compositions, family 3; their rows inline from
+0x41138
```

**Signature:**

```rust
pub fn guard_compositions(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1960`

---

### FUNCTION: `reinforcement_compositions`

**Documentation:**

```
`+0x5188C` the compiled reinforcement compositions, family 4; their rows inline from
+0x432F8 (PickReinforcementComposition reads them)
```

**Signature:**

```rust
pub fn reinforcement_compositions(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:1967`

---

### FUNCTION: `guard_budget`

**Documentation:**

```
`+0x518B4` the outposts' guard budget, which the map guards spend
```

**Signature:**

```rust
pub fn guard_budget(&self) -> Option<f32>
```

**Location:** `hf_globals.rs:1973`

---

### FUNCTION: `countdown_left`

**Documentation:**

```
`+0x51948` the mission countdown left, s
```

**Signature:**

```rust
pub fn countdown_left(&self) -> Option<f32>
```

**Location:** `hf_globals.rs:1978`

---

### FUNCTION: `expired`

**Documentation:**

```
`+0x5194C` set once the timer has run out
```

**Signature:**

```rust
pub fn expired(&self) -> Option<u8>
```

**Location:** `hf_globals.rs:1983`

---

### FUNCTION: `paused`

**Documentation:**

```
`+0x5194E` 1 while the timer is paused
```

**Signature:**

```rust
pub fn paused(&self) -> Option<u8>
```

**Location:** `hf_globals.rs:1988`

---

### FUNCTION: `level`

**Documentation:**

```
`+0x650` the mission's level (lg): its difficulty level at +0xF9 (u8)
```

**Signature:**

```rust
pub fn level(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2003`

---

### FUNCTION: `timer_count`

**Documentation:**

```
`+0x67C` the mission's timers (hf->freeze_mission_timer needs 1)
```

**Signature:**

```rust
pub fn timer_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2008`

---

### FUNCTION: `director`

**Documentation:**

```
`+0x6B0` the timer's director: +8 the game mode's entity (u32)
```

**Signature:**

```rust
pub fn director(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2013`

---

### FUNCTION: `guard_compositions`

**Documentation:**

```
`+0x51884` the compiled map guard (outpost) compositions, family 3; their rows inline from
+0x41138
```

**Signature:**

```rust
pub fn guard_compositions(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2019`

---

### FUNCTION: `reinforcement_compositions`

**Documentation:**

```
`+0x5188C` the compiled reinforcement compositions, family 4; their rows inline from
+0x432F8 (PickReinforcementComposition reads them)
```

**Signature:**

```rust
pub fn reinforcement_compositions(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2026`

---

### FUNCTION: `guard_budget`

**Documentation:**

```
`+0x518B4` the outposts' guard budget, which the map guards spend
```

**Signature:**

```rust
pub fn guard_budget(&self) -> Option<f32>
```

**Location:** `hf_globals.rs:2032`

---

### FUNCTION: `countdown_left`

**Documentation:**

```
`+0x51948` the mission countdown left, s
```

**Signature:**

```rust
pub fn countdown_left(&self) -> Option<f32>
```

**Location:** `hf_globals.rs:2037`

---

### FUNCTION: `expired`

**Documentation:**

```
`+0x5194C` set once the timer has run out
```

**Signature:**

```rust
pub fn expired(&self) -> Option<u8>
```

**Location:** `hf_globals.rs:2042`

---

### FUNCTION: `paused`

**Documentation:**

```
`+0x5194E` 1 while the timer is paused
```

**Signature:**

```rust
pub fn paused(&self) -> Option<u8>
```

**Location:** `hf_globals.rs:2047`

---

### FUNCTION: `count`

**Documentation:**

```
`+0xC` outposts
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2070`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x48` the GuardForce records, 0x2B8 bytes each
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2075`

---

### FUNCTION: `objects`

**Documentation:**

```
`+0x88` the units, a pointer each by index
```

**Signature:**

```rust
pub fn objects(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2090`

---

### FUNCTION: `count`

**Documentation:**

```
`+0x98` how many indexes
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2095`

---

### FUNCTION: `generations`

**Documentation:**

```
`+0xA0` a byte per index: its generation (handle >> 22)
```

**Signature:**

```rust
pub fn generations(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2100`

---

### FUNCTION: `overrides`

**Documentation:**

```
`+0x3F8` the resource override store (SetResourceOverride)
```

**Signature:**

```rust
pub fn overrides(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2114`

---

### FUNCTION: `resource_manager`

**Documentation:**

```
`+0x400` the resource manager: +0x208 its package store (+0x80 count, +0x88 packages),
+0x218 its load queue (24-byte entries, head +0x321C, tail +0x3220)
```

**Signature:**

```rust
pub fn resource_manager(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2120`

---

### FUNCTION: `world_count`

**Documentation:**

```
`+0x590` how many worlds
```

**Signature:**

```rust
pub fn world_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2126`

---

### FUNCTION: `worlds`

**Documentation:**

```
`+0x598` the worlds, a pointer each
```

**Signature:**

```rust
pub fn worlds(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2131`

---

### FUNCTION: `config`

**Documentation:**

```
`+0x0` the network configuration: +0x0C the type width, +0x10 field type count, +0x18 field
types, +0x28/+0x40/+0x70 the entity type arrays
```

**Signature:**

```rust
pub fn config(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2146`

---

### FUNCTION: `transport`

**Documentation:**

```
`+0x98` the transport: +0xA0 its connection count, +0xA8 the connections
```

**Signature:**

```rust
pub fn transport(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2151`

---

### FUNCTION: `session`

**Documentation:**

```
`+0xA8` the game session (+0x18 its configuration, +0x4870 its copy of the type width)
```

**Signature:**

```rust
pub fn session(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2156`

---

### FUNCTION: `kept_count`

**Documentation:**

```
`+0xD0` how many peers the game asked to keep
```

**Signature:**

```rust
pub fn kept_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2161`

---

### FUNCTION: `kept`

**Documentation:**

```
`+0xD8` those peers' ids (u64)
```

**Signature:**

```rust
pub fn kept(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2166`

---

### FUNCTION: `config`

**Documentation:**

```
`+0x0` the network configuration (see NetworkManager's config)
```

**Signature:**

```rust
pub fn config(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2181`

---

### FUNCTION: `size`

**Documentation:**

```
`+0x80` how many indexes
```

**Signature:**

```rust
pub fn size(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2195`

---

### FUNCTION: `generations`

**Documentation:**

```
`+0x88` a byte per index: its generation (handle >> 22 & 0xFF)
```

**Signature:**

```rust
pub fn generations(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2200`

---

### FUNCTION: `filter_count`

**Documentation:**

```
`+0xF8` how many collision filters
```

**Signature:**

```rust
pub fn filter_count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2224`

---

### FUNCTION: `filter_names`

**Documentation:**

```
`+0x100` their name hashes (u32 each); a filter's row is its place here
```

**Signature:**

```rust
pub fn filter_names(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2229`

---

### FUNCTION: `matrix`

**Documentation:**

```
`+0x28` the collision matrix's first word: 16 bytes per filter row, a bit per other filter
```

**Signature:**

```rust
pub fn matrix(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2244`

---

### FUNCTION: `matrix`

**Documentation:**

```
`+0x28` the collision matrix's first word: 16 bytes per filter row, a bit per other filter
```

**Signature:**

```rust
pub fn matrix(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2258`

---

### FUNCTION: `max`

**Documentation:**

```
`+0x48` the most instances seen
```

**Signature:**

```rust
pub fn max(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2290`

---

### FUNCTION: `count`

**Documentation:**

```
`+0x50` instances (0 or 1)
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2295`

---

### FUNCTION: `descriptor`

**Documentation:**

```
`+0x80` the instance's entity descriptor
```

**Signature:**

```rust
pub fn descriptor(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2300`

---

### FUNCTION: `kills`

**Documentation:**

```
`+0x14` career kills, the larger of the local and server values
```

**Signature:**

```rust
pub fn kills(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2315`

---

### FUNCTION: `count`

**Documentation:**

```
`+0x18` instances (the world update's loop bound, 0x57201C)
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2396`

---

### FUNCTION: `descriptors`

**Documentation:**

```
`+0x40` each instance's entity descriptor (8 bytes each: +0 type, +8 entity)
```

**Signature:**

```rust
pub fn descriptors(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2401`

---

### FUNCTION: `live`

**Documentation:**

```
`+0x58` each instance's live row, 20 bytes: +0 its own spare rounds (network field
0x9A34C336)
```

**Signature:**

```rust
pub fn live(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2408`

---

### FUNCTION: `count`

**Documentation:**

```
`+0x38` instances (the world update's loop bound, 0x573737)
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2448`

---

### FUNCTION: `count`

**Documentation:**

```
`+0x18` instances (WeaponHeatUpdate's loop bound, 0x764988)
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2463`

---

### FUNCTION: `descriptors`

**Documentation:**

```
`+0x40` each instance's entity descriptor (8 bytes each: +8 entity, +0x14 bit 1 owned by
this machine; WeaponHeatUpdate steps only those)
```

**Signature:**

```rust
pub fn descriptors(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2469`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x28` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2514`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x28` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2531`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x40` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2548`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x38` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2566`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2583`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x18` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2600`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x156DA0` the entity map's entries, 8 bytes each (entity u32, index u32); the map's
capacity (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier
0x10 after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2617`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x18` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2634`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x58` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2653`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x28` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2671`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x40` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2689`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x38` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2707`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2725`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2743`

---

### FUNCTION: `hover_input`

**Documentation:**

```
`+0x40` 12 bytes per index: the replicated hover input vector
```

**Signature:**

```rust
pub fn hover_input(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2748`

---

### FUNCTION: `flight`

**Documentation:**

```
`+0x48` 76 bytes per index: +0 launch latch, +1 sustain latch, +4 launch elapsed (f32 s), +8
sustain elapsed (f32 s), +0xC launch window (f32 s)
```

**Signature:**

```rust
pub fn flight(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2755`

---

### FUNCTION: `flags`

**Documentation:**

```
`+0x50` 5 bytes per index, replicated: +0 active, +1 sustain, +2 hover launch, +3 mode
toggle, +4 thrusting
```

**Signature:**

```rust
pub fn flags(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2761`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x30` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2779`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x58` 0xFC bytes per index, the jet: +0xC its target index (9 none), +0x10 targets tracked
(<= 8), +0x14 8 target entities, +0x54 aim point (3 f32), +0x68 releases so far, +0x70
attacking (u8), +0x78 jet position (3 f32), +0xD4 velocity (3 f32), +0xE0 speed (f32)
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2786`

---

### FUNCTION: `flight`

**Documentation:**

```
`+0x60` 28 bytes per index: +0 the approach start position, +0xC the target point at the
call (3 f32 each)
```

**Signature:**

```rust
pub fn flight(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2792`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x948` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2810`

---

### FUNCTION: `count`

**Documentation:**

```
`+0xC` squads in use
```

**Signature:**

```rust
pub fn count(&self) -> Option<u32>
```

**Location:** `hf_globals.rs:2827`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2834`

---

### FUNCTION: `descriptors`

**Documentation:**

```
`+0x38` descriptor pointers by index (+8 the entity)
```

**Signature:**

```rust
pub fn descriptors(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2839`

---

### FUNCTION: `records`

**Documentation:**

```
`+0x40` the squad records, 0x2510 bytes each, by index
```

**Signature:**

```rust
pub fn records(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2845`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x30` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2863`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x28` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2881`

---

### FUNCTION: `map`

**Documentation:**

```
`+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
(u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
after it
```

**Signature:**

```rust
pub fn map(&self) -> Option<NonNull<c_void>>
```

**Location:** `hf_globals.rs:2898`

---

### FUNCTION: `deposit_manager`

**Documentation:**

```
The `DepositManager` global (`game.dll+0x33265E8`), or `None`.
```

**Signature:**

```rust
pub fn deposit_manager(&self) -> Option<DepositManager<'_>>
```

**Location:** `hf_globals.rs:2905`

---

### FUNCTION: `avatar_manager`

**Documentation:**

```
The `AvatarManager` global (`game.dll+0x3326D20`), or `None`.
```

**Signature:**

```rust
pub fn avatar_manager(&self) -> Option<AvatarManager<'_>>
```

**Location:** `hf_globals.rs:2913`

---

### FUNCTION: `action_manager`

**Documentation:**

```
The `ActionManager` global, or `None`.
```

**Signature:**

```rust
pub fn action_manager(&self) -> Option<ActionManager<'_>>
```

**Location:** `hf_globals.rs:2921`

---

### FUNCTION: `wieldable_manager`

**Documentation:**

```
The `WieldableManager` global (`game.dll+0x3326730`), or `None`.
```

**Signature:**

```rust
pub fn wieldable_manager(&self) -> Option<WieldableManager<'_>>
```

**Location:** `hf_globals.rs:2929`

---

### FUNCTION: `faction_manager`

**Documentation:**

```
The `FactionManager` global (`game.dll+0x3326CB8`), or `None`.
```

**Signature:**

```rust
pub fn faction_manager(&self) -> Option<FactionManager<'_>>
```

**Location:** `hf_globals.rs:2937`

---

### FUNCTION: `behaviour_manager`

**Documentation:**

```
The `BehaviourManager` global (`game.dll+0x3326740`), or `None`.
```

**Signature:**

```rust
pub fn behaviour_manager(&self) -> Option<BehaviourManager<'_>>
```

**Location:** `hf_globals.rs:2945`

---

### FUNCTION: `tag_manager`

**Documentation:**

```
The `TagManager` global (`game.dll+0x3326430`), or `None`.
```

**Signature:**

```rust
pub fn tag_manager(&self) -> Option<TagManager<'_>>
```

**Location:** `hf_globals.rs:2953`

---

### FUNCTION: `clock`

**Documentation:**

```
The `Clock` global (`game.dll+0x3326348`), or `None`.
```

**Signature:**

```rust
pub fn clock(&self) -> Option<Clock<'_>>
```

**Location:** `hf_globals.rs:2958`

---

### FUNCTION: `engine_api`

**Documentation:**

```
The `EngineApi` global (`game.dll+0x3326308`), or `None`.
```

**Signature:**

```rust
pub fn engine_api(&self) -> Option<EngineApi<'_>>
```

**Location:** `hf_globals.rs:2963`

---

### FUNCTION: `engine_unit_api`

**Documentation:**

```
The `EngineUnitApi` global (`game.dll+0x3326310`), or `None`.
```

**Signature:**

```rust
pub fn engine_unit_api(&self) -> Option<EngineUnitApi<'_>>
```

**Location:** `hf_globals.rs:2968`

---

### FUNCTION: `wwise_api`

**Documentation:**

```
The `WwiseApi` global (`game.dll+0x3326318`), or `None`.
```

**Signature:**

```rust
pub fn wwise_api(&self) -> Option<WwiseApi<'_>>
```

**Location:** `hf_globals.rs:2976`

---

### FUNCTION: `ability_manager`

**Documentation:**

```
The `AbilityManager` global (`game.dll+0x3326640`), or `None`.
```

**Signature:**

```rust
pub fn ability_manager(&self) -> Option<AbilityManager<'_>>
```

**Location:** `hf_globals.rs:2981`

---

### FUNCTION: `weapon_linker_manager`

**Documentation:**

```
The `WeaponLinkerManager` global (`game.dll+0x33267F8`), or `None`.
```

**Signature:**

```rust
pub fn weapon_linker_manager(&self) -> Option<WeaponLinkerManager<'_>>
```

**Location:** `hf_globals.rs:2989`

---

### FUNCTION: `seeking_missile_manager`

**Documentation:**

```
The `SeekingMissileManager` global (`game.dll+0x3326DF0`), or `None`.
```

**Signature:**

```rust
pub fn seeking_missile_manager(&self) -> Option<SeekingMissileManager<'_>>
```

**Location:** `hf_globals.rs:2997`

---

### FUNCTION: `laser_designator_manager`

**Documentation:**

```
The `LaserDesignatorManager` global (`game.dll+0x3326960`), or `None`.
```

**Signature:**

```rust
pub fn laser_designator_manager(&self) -> Option<LaserDesignatorManager<'_>>
```

**Location:** `hf_globals.rs:3005`

---

### FUNCTION: `guidance_targets`

**Documentation:**

```
The `GuidanceTargets` global (`game.dll+0x3326BD8`), or `None`.
```

**Signature:**

```rust
pub fn guidance_targets(&self) -> Option<GuidanceTargets<'_>>
```

**Location:** `hf_globals.rs:3013`

---

### FUNCTION: `explosive_manager`

**Documentation:**

```
The `ExplosiveManager` global (`game.dll+0x3326728`), or `None`.
```

**Signature:**

```rust
pub fn explosive_manager(&self) -> Option<ExplosiveManager<'_>>
```

**Location:** `hf_globals.rs:3021`

---

### FUNCTION: `equipment_manager`

**Documentation:**

```
The `EquipmentManager` global (`game.dll+0x3326DC0`), or `None`.
```

**Signature:**

```rust
pub fn equipment_manager(&self) -> Option<EquipmentManager<'_>>
```

**Location:** `hf_globals.rs:3029`

---

### FUNCTION: `remote_owners`

**Documentation:**

```
The `RemoteOwners` global, or `None`.
```

**Signature:**

```rust
pub fn remote_owners(&self) -> Option<RemoteOwners<'_>>
```

**Location:** `hf_globals.rs:3037`

---

### FUNCTION: `hellpod_rack_manager`

**Documentation:**

```
The `HellpodRackManager` global (`game.dll+0x3326A58`), or `None`.
```

**Signature:**

```rust
pub fn hellpod_rack_manager(&self) -> Option<HellpodRackManager<'_>>
```

**Location:** `hf_globals.rs:3045`

---

### FUNCTION: `attachable_manager`

**Documentation:**

```
The `AttachableManager` global (`game.dll+0x3326698`), or `None`.
```

**Signature:**

```rust
pub fn attachable_manager(&self) -> Option<AttachableManager<'_>>
```

**Location:** `hf_globals.rs:3053`

---

### FUNCTION: `mount_manager`

**Documentation:**

```
The `MountManager` global (`game.dll+0x3326438`), or `None`.
```

**Signature:**

```rust
pub fn mount_manager(&self) -> Option<MountManager<'_>>
```

**Location:** `hf_globals.rs:3061`

---

### FUNCTION: `drop_anchors`

**Documentation:**

```
The `DropAnchors` global (`game.dll+0x3326658`), or `None`.
```

**Signature:**

```rust
pub fn drop_anchors(&self) -> Option<DropAnchors<'_>>
```

**Location:** `hf_globals.rs:3066`

---

### FUNCTION: `perception_manager`

**Documentation:**

```
The `PerceptionManager` global (`game.dll+0x3326548`), or `None`.
```

**Signature:**

```rust
pub fn perception_manager(&self) -> Option<PerceptionManager<'_>>
```

**Location:** `hf_globals.rs:3071`

---

### FUNCTION: `weapon_manager`

**Documentation:**

```
The `WeaponManager` global (`game.dll+0x3326660`), or `None`.
```

**Signature:**

```rust
pub fn weapon_manager(&self) -> Option<WeaponManager<'_>>
```

**Location:** `hf_globals.rs:3079`

---

### FUNCTION: `magazine_manager`

**Documentation:**

```
The `MagazineManager` global, or `None`.
```

**Signature:**

```rust
pub fn magazine_manager(&self) -> Option<MagazineManager<'_>>
```

**Location:** `hf_globals.rs:3087`

---

### FUNCTION: `weapon_magazines`

**Documentation:**

```
The `WeaponMagazines` global (`game.dll+0x3326648`), or `None`.
```

**Signature:**

```rust
pub fn weapon_magazines(&self) -> Option<WeaponMagazines<'_>>
```

**Location:** `hf_globals.rs:3095`

---

### FUNCTION: `magazine_refills`

**Documentation:**

```
The `MagazineRefills` global, or `None`.
```

**Signature:**

```rust
pub fn magazine_refills(&self) -> Option<MagazineRefills<'_>>
```

**Location:** `hf_globals.rs:3103`

---

### FUNCTION: `weapon_targeting`

**Documentation:**

```
The `WeaponTargeting` global (`game.dll+0x3326D30`), or `None`.
```

**Signature:**

```rust
pub fn weapon_targeting(&self) -> Option<WeaponTargeting<'_>>
```

**Location:** `hf_globals.rs:3111`

---

### FUNCTION: `shield_manager`

**Documentation:**

```
The `ShieldManager` global (`game.dll+0x3326630`), or `None`.
```

**Signature:**

```rust
pub fn shield_manager(&self) -> Option<ShieldManager<'_>>
```

**Location:** `hf_globals.rs:3119`

---

### FUNCTION: `customization_manager`

**Documentation:**

```
The `CustomizationManager` global (`game.dll+0x33264F8`), or `None`.
```

**Signature:**

```rust
pub fn customization_manager(&self) -> Option<CustomizationManager<'_>>
```

**Location:** `hf_globals.rs:3127`

---

### FUNCTION: `kit_list`

**Documentation:**

```
The `KitList` global, or `None`.
```

**Signature:**

```rust
pub fn kit_list(&self) -> Option<KitList<'_>>
```

**Location:** `hf_globals.rs:3135`

---

### FUNCTION: `bombardment_manager`

**Documentation:**

```
The `BombardmentManager` global (`game.dll+0x3326CE8`), or `None`.
```

**Signature:**

```rust
pub fn bombardment_manager(&self) -> Option<BombardmentManager<'_>>
```

**Location:** `hf_globals.rs:3143`

---

### FUNCTION: `progression`

**Documentation:**

```
The `Progression` global (`game.dll+0x347CEF8`), or `None`.
```

**Signature:**

```rust
pub fn progression(&self) -> Option<Progression<'_>>
```

**Location:** `hf_globals.rs:3151`

---

### FUNCTION: `progression_manager`

**Documentation:**

```
The `ProgressionManager` global, or `None`.
```

**Signature:**

```rust
pub fn progression_manager(&self) -> Option<ProgressionManager<'_>>
```

**Location:** `hf_globals.rs:3156`

---

### FUNCTION: `player_history`

**Documentation:**

```
The `PlayerHistory` global (`game.dll+0x347CE50`), or `None`.
```

**Signature:**

```rust
pub fn player_history(&self) -> Option<PlayerHistory<'_>>
```

**Location:** `hf_globals.rs:3164`

---

### FUNCTION: `profile`

**Documentation:**

```
The `Profile` global (`game.dll+0x347CDD8`), or `None`.
```

**Signature:**

```rust
pub fn profile(&self) -> Option<Profile<'_>>
```

**Location:** `hf_globals.rs:3172`

---

### FUNCTION: `menu_states`

**Documentation:**

```
The `MenuStates` global (`game.dll+0x347CE28`), or `None`.
```

**Signature:**

```rust
pub fn menu_states(&self) -> Option<MenuStates<'_>>
```

**Location:** `hf_globals.rs:3177`

---

### FUNCTION: `ui_manager`

**Documentation:**

```
The `UiManager` global, or `None`.
```

**Signature:**

```rust
pub fn ui_manager(&self) -> Option<UiManager<'_>>
```

**Location:** `hf_globals.rs:3182`

---

### FUNCTION: `menu_stack`

**Documentation:**

```
The `MenuStack` global (`game.dll+0x347CE38`), or `None`.
```

**Signature:**

```rust
pub fn menu_stack(&self) -> Option<MenuStack<'_>>
```

**Location:** `hf_globals.rs:3187`

---

### FUNCTION: `armory`

**Documentation:**

```
The `Armory` global (`game.dll+0x33269E8`), or `None`.
```

**Signature:**

```rust
pub fn armory(&self) -> Option<Armory<'_>>
```

**Location:** `hf_globals.rs:3192`

---

### FUNCTION: `thumbnail_manager`

**Documentation:**

```
The `ThumbnailManager` global (`game.dll+0x347CD80`), or `None`.
```

**Signature:**

```rust
pub fn thumbnail_manager(&self) -> Option<ThumbnailManager<'_>>
```

**Location:** `hf_globals.rs:3197`

---

### FUNCTION: `package_system`

**Documentation:**

```
The `PackageSystem` global (`game.dll+0x347CEB0`), or `None`.
```

**Signature:**

```rust
pub fn package_system(&self) -> Option<PackageSystem<'_>>
```

**Location:** `hf_globals.rs:3205`

---

### FUNCTION: `game_config`

**Documentation:**

```
The `GameConfig` global (`game.dll+0x347CDF8`), or `None`.
```

**Signature:**

```rust
pub fn game_config(&self) -> Option<GameConfig<'_>>
```

**Location:** `hf_globals.rs:3213`

---

### FUNCTION: `navigation_system`

**Documentation:**

```
The `NavigationSystem` global (`game.dll+0x33263C8`), or `None`.
```

**Signature:**

```rust
pub fn navigation_system(&self) -> Option<NavigationSystem<'_>>
```

**Location:** `hf_globals.rs:3218`

---

### FUNCTION: `physics_plugin`

**Documentation:**

```
The `PhysicsPlugin` global (`game.dll+0x3326380`), or `None`.
```

**Signature:**

```rust
pub fn physics_plugin(&self) -> Option<PhysicsPlugin<'_>>
```

**Location:** `hf_globals.rs:3226`

---

### FUNCTION: `objective_manager`

**Documentation:**

```
The `ObjectiveManager` global (`game.dll+0x3326DA0`), or `None`.
```

**Signature:**

```rust
pub fn objective_manager(&self) -> Option<ObjectiveManager<'_>>
```

**Location:** `hf_globals.rs:3234`

---

### FUNCTION: `hive_mind_manager`

**Documentation:**

```
The `HiveMindManager` global (`game.dll+0x3326D10`), or `None`.
```

**Signature:**

```rust
pub fn hive_mind_manager(&self) -> Option<HiveMindManager<'_>>
```

**Location:** `hf_globals.rs:3242`

---

### FUNCTION: `mission_timers`

**Documentation:**

```
The `MissionTimers` global, or `None`.
```

**Signature:**

```rust
pub fn mission_timers(&self) -> Option<MissionTimers<'_>>
```

**Location:** `hf_globals.rs:3250`

---

### FUNCTION: `telemetry`

**Documentation:**

```
The `Telemetry` global (`game.dll+0x347CE18`), or `None`.
```

**Signature:**

```rust
pub fn telemetry(&self) -> Option<Telemetry<'_>>
```

**Location:** `hf_globals.rs:3258`

---

### FUNCTION: `guard_forces`

**Documentation:**

```
The `GuardForces` global (`game.dll+0x33265C0`), or `None`.
```

**Signature:**

```rust
pub fn guard_forces(&self) -> Option<GuardForces<'_>>
```

**Location:** `hf_globals.rs:3263`

---

### FUNCTION: `unit_registry`

**Documentation:**

```
The `UnitRegistry` global (`helldivers2.exe+0x1A100F0`), or `None`.
```

**Signature:**

```rust
pub fn unit_registry(&self) -> Option<UnitRegistry<'_>>
```

**Location:** `hf_globals.rs:3268`

---

### FUNCTION: `engine_core`

**Documentation:**

```
The `EngineCore` global (`helldivers2.exe+0x1A10208`), or `None`.
```

**Signature:**

```rust
pub fn engine_core(&self) -> Option<EngineCore<'_>>
```

**Location:** `hf_globals.rs:3273`

---

### FUNCTION: `network_manager`

**Documentation:**

```
The `NetworkManager` global (`helldivers2.exe+0x1A10278`), or `None`.
```

**Signature:**

```rust
pub fn network_manager(&self) -> Option<NetworkManager<'_>>
```

**Location:** `hf_globals.rs:3278`

---

### FUNCTION: `network_config_owner`

**Documentation:**

```
The `NetworkConfigOwner` global (`helldivers2.exe+0x1A10268`), or `None`.
```

**Signature:**

```rust
pub fn network_config_owner(&self) -> Option<NetworkConfigOwner<'_>>
```

**Location:** `hf_globals.rs:3286`

---

### FUNCTION: `engine_entity_index`

**Documentation:**

```
The `EngineEntityIndex` global (`helldivers2.exe+0x1B135B8`), or `None`.
```

**Signature:**

```rust
pub fn engine_entity_index(&self) -> Option<EngineEntityIndex<'_>>
```

**Location:** `hf_globals.rs:3294`

---

### FUNCTION: `nav_worlds`

**Documentation:**

```
The `NavWorlds` global (`helldivers2.exe+0x27C30B8`), or `None`.
```

**Signature:**

```rust
pub fn nav_worlds(&self) -> Option<NavWorlds<'_>>
```

**Location:** `hf_globals.rs:3302`

---

### FUNCTION: `physics_properties`

**Documentation:**

```
The `PhysicsProperties` global (`helldivers2.exe+0x27C5E48`), or `None`.
```

**Signature:**

```rust
pub fn physics_properties(&self) -> Option<PhysicsProperties<'_>>
```

**Location:** `hf_globals.rs:3307`

---

### FUNCTION: `collision_filter_a`

**Documentation:**

```
The `CollisionFilterA` global (`helldivers2.exe+0x27C5E40`), or `None`.
```

**Signature:**

```rust
pub fn collision_filter_a(&self) -> Option<CollisionFilterA<'_>>
```

**Location:** `hf_globals.rs:3315`

---

### FUNCTION: `collision_filter_b`

**Documentation:**

```
The `CollisionFilterB` global (`helldivers2.exe+0x27C5B90`), or `None`.
```

**Signature:**

```rust
pub fn collision_filter_b(&self) -> Option<CollisionFilterB<'_>>
```

**Location:** `hf_globals.rs:3323`

---

### FUNCTION: `status_receivers`

**Documentation:**

```
The `StatusReceivers` global (`game.dll+0x3326620`), or `None`.
```

**Signature:**

```rust
pub fn status_receivers(&self) -> Option<StatusReceivers<'_>>
```

**Location:** `hf_globals.rs:3331`

---

### FUNCTION: `component_world`

**Documentation:**

```
The `ComponentWorld` global (`game.dll+0x346BFC8`), or `None`.
```

**Signature:**

```rust
pub fn component_world(&self) -> Option<ComponentWorld<'_>>
```

**Location:** `hf_globals.rs:3339`

---

### FUNCTION: `game_mode_mission`

**Documentation:**

```
The `GameModeMission` global (`game.dll+0x3326D60`), or `None`.
```

**Signature:**

```rust
pub fn game_mode_mission(&self) -> Option<GameModeMission<'_>>
```

**Location:** `hf_globals.rs:3347`

---

### FUNCTION: `career_stats`

**Documentation:**

```
The `CareerStats` global (`game.dll+0x346D570`), or `None`.
```

**Signature:**

```rust
pub fn career_stats(&self) -> Option<CareerStats<'_>>
```

**Location:** `hf_globals.rs:3355`

---

### FUNCTION: `projectile_weapons`

**Documentation:**

```
The `ProjectileWeapons` global (`game.dll+0x33266D8`), or `None`.
```

**Signature:**

```rust
pub fn projectile_weapons(&self) -> Option<ProjectileWeapons<'_>>
```

**Location:** `hf_globals.rs:3360`

---

### FUNCTION: `weapon_datas`

**Documentation:**

```
The `WeaponDatas` global (`game.dll+0x3326CE0`), or `None`.
```

**Signature:**

```rust
pub fn weapon_datas(&self) -> Option<WeaponDatas<'_>>
```

**Location:** `hf_globals.rs:3368`

---

### FUNCTION: `beam_weapons`

**Documentation:**

```
The `BeamWeapons` global (`game.dll+0x3326A20`), or `None`.
```

**Signature:**

```rust
pub fn beam_weapons(&self) -> Option<BeamWeapons<'_>>
```

**Location:** `hf_globals.rs:3373`

---

### FUNCTION: `linked_ammo`

**Documentation:**

```
The `LinkedAmmo` global (`game.dll+0x3326AA8`), or `None`.
```

**Signature:**

```rust
pub fn linked_ammo(&self) -> Option<LinkedAmmo<'_>>
```

**Location:** `hf_globals.rs:3378`

---

### FUNCTION: `corpses`

**Documentation:**

```
The `Corpses` global (`game.dll+0x3326920`), or `None`.
```

**Signature:**

```rust
pub fn corpses(&self) -> Option<Corpses<'_>>
```

**Location:** `hf_globals.rs:3383`

---

### FUNCTION: `weapon_wind_ups`

**Documentation:**

```
The `WeaponWindUps` global (`game.dll+0x33267A0`), or `None`.
```

**Signature:**

```rust
pub fn weapon_wind_ups(&self) -> Option<WeaponWindUps<'_>>
```

**Location:** `hf_globals.rs:3388`

---

### FUNCTION: `recharges`

**Documentation:**

```
The `Recharges` global (`game.dll+0x3326768`), or `None`.
```

**Signature:**

```rust
pub fn recharges(&self) -> Option<Recharges<'_>>
```

**Location:** `hf_globals.rs:3396`

---

### FUNCTION: `weapon_reloads`

**Documentation:**

```
The `WeaponReloads` global (`game.dll+0x3326A70`), or `None`.
```

**Signature:**

```rust
pub fn weapon_reloads(&self) -> Option<WeaponReloads<'_>>
```

**Location:** `hf_globals.rs:3401`

---

### FUNCTION: `weapon_rounds_manager`

**Documentation:**

```
The `WeaponRoundsManager` global (`game.dll+0x3326CF0`), or `None`.
```

**Signature:**

```rust
pub fn weapon_rounds_manager(&self) -> Option<WeaponRoundsManager<'_>>
```

**Location:** `hf_globals.rs:3409`

---

### FUNCTION: `weapon_charges`

**Documentation:**

```
The `WeaponCharges` global (`game.dll+0x3326C20`), or `None`.
```

**Signature:**

```rust
pub fn weapon_charges(&self) -> Option<WeaponCharges<'_>>
```

**Location:** `hf_globals.rs:3417`

---

### FUNCTION: `throwers`

**Documentation:**

```
The `Throwers` global (`game.dll+0x3326880`), or `None`.
```

**Signature:**

```rust
pub fn throwers(&self) -> Option<Throwers<'_>>
```

**Location:** `hf_globals.rs:3425`

---

### FUNCTION: `throwables`

**Documentation:**

```
The `Throwables` global (`game.dll+0x33264C0`), or `None`.
```

**Signature:**

```rust
pub fn throwables(&self) -> Option<Throwables<'_>>
```

**Location:** `hf_globals.rs:3430`

---

### FUNCTION: `arc_weapons`

**Documentation:**

```
The `ArcWeapons` global (`game.dll+0x3326C10`), or `None`.
```

**Signature:**

```rust
pub fn arc_weapons(&self) -> Option<ArcWeapons<'_>>
```

**Location:** `hf_globals.rs:3435`

---

### FUNCTION: `weapon_heats`

**Documentation:**

```
The `WeaponHeats` global (`game.dll+0x3326D48`), or `None`.
```

**Signature:**

```rust
pub fn weapon_heats(&self) -> Option<WeaponHeats<'_>>
```

**Location:** `hf_globals.rs:3440`

---

### FUNCTION: `ability_weapons`

**Documentation:**

```
The `AbilityWeapons` global (`game.dll+0x3326A10`), or `None`.
```

**Signature:**

```rust
pub fn ability_weapons(&self) -> Option<AbilityWeapons<'_>>
```

**Location:** `hf_globals.rs:3445`

---

### FUNCTION: `melee_weapons`

**Documentation:**

```
The `MeleeWeapons` global (`game.dll+0x3326B78`), or `None`.
```

**Signature:**

```rust
pub fn melee_weapons(&self) -> Option<MeleeWeapons<'_>>
```

**Location:** `hf_globals.rs:3453`

---

### FUNCTION: `weapon_customizations`

**Documentation:**

```
The `WeaponCustomizations` global (`game.dll+0x3326A38`), or `None`.
```

**Signature:**

```rust
pub fn weapon_customizations(&self) -> Option<WeaponCustomizations<'_>>
```

**Location:** `hf_globals.rs:3458`

---

### FUNCTION: `ai_spawner_manager`

**Documentation:**

```
The `AiSpawnerManager` global (`game.dll+0x33265C8`), or `None`.
```

**Signature:**

```rust
pub fn ai_spawner_manager(&self) -> Option<AiSpawnerManager<'_>>
```

**Location:** `hf_globals.rs:3466`

---

### FUNCTION: `bughole_manager`

**Documentation:**

```
The `BugholeManager` global (`game.dll+0x33267B0`), or `None`.
```

**Signature:**

```rust
pub fn bughole_manager(&self) -> Option<BugholeManager<'_>>
```

**Location:** `hf_globals.rs:3474`

---

### FUNCTION: `vehicle_motion_manager`

**Documentation:**

```
The `VehicleMotionManager` global (`game.dll+0x3326458`), or `None`.
```

**Signature:**

```rust
pub fn vehicle_motion_manager(&self) -> Option<VehicleMotionManager<'_>>
```

**Location:** `hf_globals.rs:3482`

---

### FUNCTION: `vehicle_manager`

**Documentation:**

```
The `VehicleManager` global (`game.dll+0x3326668`), or `None`.
```

**Signature:**

```rust
pub fn vehicle_manager(&self) -> Option<VehicleManager<'_>>
```

**Location:** `hf_globals.rs:3490`

---

### FUNCTION: `squad_transport_manager`

**Documentation:**

```
The `SquadTransportManager` global (`game.dll+0x3326CC0`), or `None`.
```

**Signature:**

```rust
pub fn squad_transport_manager(&self) -> Option<SquadTransportManager<'_>>
```

**Location:** `hf_globals.rs:3498`

---

### FUNCTION: `squad_formation_manager`

**Documentation:**

```
The `SquadFormationManager` global (`game.dll+0x3326A88`), or `None`.
```

**Signature:**

```rust
pub fn squad_formation_manager(&self) -> Option<SquadFormationManager<'_>>
```

**Location:** `hf_globals.rs:3506`

---

### FUNCTION: `gore_manager`

**Documentation:**

```
The `GoreManager` global (`game.dll+0x33264D0`), or `None`.
```

**Signature:**

```rust
pub fn gore_manager(&self) -> Option<GoreManager<'_>>
```

**Location:** `hf_globals.rs:3514`

---

### FUNCTION: `stratagem_blocker_manager`

**Documentation:**

```
The `StratagemBlockerManager` global (`game.dll+0x3326568`), or `None`.
```

**Signature:**

```rust
pub fn stratagem_blocker_manager(&self) -> Option<StratagemBlockerManager<'_>>
```

**Location:** `hf_globals.rs:3519`

---

### FUNCTION: `orbital_ability_manager`

**Documentation:**

```
The `OrbitalAbilityManager` global (`game.dll+0x33267F0`), or `None`.
```

**Signature:**

```rust
pub fn orbital_ability_manager(&self) -> Option<OrbitalAbilityManager<'_>>
```

**Location:** `hf_globals.rs:3527`

---

### FUNCTION: `hellpod_payload_manager`

**Documentation:**

```
The `HellpodPayloadManager` global (`game.dll+0x3326560`), or `None`.
```

**Signature:**

```rust
pub fn hellpod_payload_manager(&self) -> Option<HellpodPayloadManager<'_>>
```

**Location:** `hf_globals.rs:3535`

---

### FUNCTION: `unit_manager`

**Documentation:**

```
The `UnitManager` global (`game.dll+0x3326508`), or `None`.
```

**Signature:**

```rust
pub fn unit_manager(&self) -> Option<UnitManager<'_>>
```

**Location:** `hf_globals.rs:3543`

---

### FUNCTION: `rotation_manager`

**Documentation:**

```
The `RotationManager` global (`game.dll+0x33264A0`), or `None`.
```

**Signature:**

```rust
pub fn rotation_manager(&self) -> Option<RotationManager<'_>>
```

**Location:** `hf_globals.rs:3548`

---

### FUNCTION: `displacement_manager`

**Documentation:**

```
The `DisplacementManager` global (`game.dll+0x3326AC8`), or `None`.
```

**Signature:**

```rust
pub fn displacement_manager(&self) -> Option<DisplacementManager<'_>>
```

**Location:** `hf_globals.rs:3556`

---

### FUNCTION: `jumppack_manager`

**Documentation:**

```
The `JumppackManager` global (`game.dll+0x3326BB8`), or `None`.
```

**Signature:**

```rust
pub fn jumppack_manager(&self) -> Option<JumppackManager<'_>>
```

**Location:** `hf_globals.rs:3564`

---

### FUNCTION: `eagle_manager`

**Documentation:**

```
The `EagleManager` global (`game.dll+0x3326650`), or `None`.
```

**Signature:**

```rust
pub fn eagle_manager(&self) -> Option<EagleManager<'_>>
```

**Location:** `hf_globals.rs:3572`

---

### FUNCTION: `encounter_manager`

**Documentation:**

```
The `EncounterManager` global (`game.dll+0x3326618`), or `None`.
```

**Signature:**

```rust
pub fn encounter_manager(&self) -> Option<EncounterManager<'_>>
```

**Location:** `hf_globals.rs:3577`

---

### FUNCTION: `ai_squad_manager`

**Documentation:**

```
The `AiSquadManager` global (`game.dll+0x33265A0`), or `None`.
```

**Signature:**

```rust
pub fn ai_squad_manager(&self) -> Option<AiSquadManager<'_>>
```

**Location:** `hf_globals.rs:3585`

---

### FUNCTION: `sensor_eye_manager`

**Documentation:**

```
The `SensorEyeManager` global (`game.dll+0x3326520`), or `None`.
```

**Signature:**

```rust
pub fn sensor_eye_manager(&self) -> Option<SensorEyeManager<'_>>
```

**Location:** `hf_globals.rs:3593`

---

### FUNCTION: `turret_manager`

**Documentation:**

```
The `TurretManager` global (`game.dll+0x3326D70`), or `None`.
```

**Signature:**

```rust
pub fn turret_manager(&self) -> Option<TurretManager<'_>>
```

**Location:** `hf_globals.rs:3601`

---

### FUNCTION: `locomotion_manager`

**Documentation:**

```
The `LocomotionManager` global (`game.dll+0x3326CF8`), or `None`.
```

**Signature:**

```rust
pub fn locomotion_manager(&self) -> Option<LocomotionManager<'_>>
```

**Location:** `hf_globals.rs:3609`

---

## File: `hf_net.rs`

### STRUCT: `MessageEvent`

**Documentation:**

```
A message from the same mod on another machine of the lobby.

It arrives through [`HellForgeApi::on_message`]; `data` is valid during the
callback only.
```

**Signature:**

```rust
pub struct MessageEvent<'a>
```

**Location:** `hf_net.rs:48`

---

### STRUCT: `ShareEvent`

**Documentation:**

```
One row of this mod's shared table, as it arrived, changed or went away.

It arrives through [`HellForgeApi::on_share`]; `value` is valid during the
callback only.
```

**Signature:**

```rust
pub struct ShareEvent<'a>
```

**Location:** `hf_net.rs:60`

---

### STRUCT: `RpcEvent`

**Documentation:**

```
One RPC this machine received (`HfRpcEvent`).

It is valid during the listener's call only; [`rpc_listen`](HellForgeApi::rpc_listen)
passes a reference to it. Read its arguments with [`reader`](Self::reader),
or decode a named RPC with [`RpcMessage::decode`].
```

**Signature:**

```rust
pub struct RpcEvent<'a>
```

**Location:** `hf_net.rs:258`

---

### FUNCTION: `sender`

**Documentation:**

```
The peer that sent it.
```

**Signature:**

```rust
pub const fn sender(&self) -> u64
```

**Location:** `hf_net.rs:266`

---

### FUNCTION: `rpc`

**Documentation:**

```
Its id (`HF_RPC_...`, or [`rpc::ALL`] in `rpc_listen`).
```

**Signature:**

```rust
pub const fn rpc(&self) -> u32
```

**Location:** `hf_net.rs:271`

---

### FUNCTION: `count`

**Documentation:**

```
How many argument entries it has, including each array's count entry.
```

**Signature:**

```rust
pub const fn count(&self) -> usize
```

**Location:** `hf_net.rs:276`

---

### FUNCTION: `args`

**Documentation:**

```
The raw arguments, for callers that walk them themselves.
```

**Signature:**

```rust
pub const fn args(&self) -> &'a [HfRpcArg]
```

**Location:** `hf_net.rs:281`

---

### FUNCTION: `reader`

**Documentation:**

```
A cursor over its arguments.
```

**Signature:**

```rust
pub fn reader(&self) -> RpcReader<'a>
```

**Location:** `hf_net.rs:286`

---

### STRUCT: `RpcReader`

**Documentation:**

```
Reads the arguments of an [`RpcEvent`] in their declared order.

Each method returns `None` when the next argument is missing, has another
kind, or its value does not fit. [`finish`](Self::finish) checks that no
argument is left over.
```

**Signature:**

```rust
pub struct RpcReader<'a>
```

**Location:** `hf_net.rs:315`

---

### FUNCTION: `bool`

**Documentation:**

```
The next argument as a `bool`.
```

**Signature:**

```rust
pub fn bool(&mut self) -> Option<bool>
```

**Location:** `hf_net.rs:341`

---

### FUNCTION: `int`

**Documentation:**

```
The next argument as an `int`.
```

**Signature:**

```rust
pub fn int(&mut self) -> Option<i32>
```

**Location:** `hf_net.rs:346`

---

### FUNCTION: `float`

**Documentation:**

```
The next argument as a `float`.
```

**Signature:**

```rust
pub fn float(&mut self) -> Option<f32>
```

**Location:** `hf_net.rs:351`

---

### FUNCTION: `vector3`

**Documentation:**

```
The next argument as a `vector3`.
```

**Signature:**

```rust
pub fn vector3(&mut self) -> Option<[f32
```

**Location:** `hf_net.rs:356`

---

### FUNCTION: `quaternion`

**Documentation:**

```
The next argument as a `quaternion`.
```

**Signature:**

```rust
pub fn quaternion(&mut self) -> Option<[f32
```

**Location:** `hf_net.rs:361`

---

### FUNCTION: `string`

**Documentation:**

```
The next argument as a string. The bytes are copied, so the result
outlives the event.
```

**Signature:**

```rust
pub fn string(&mut self) -> Option<String>
```

**Location:** `hf_net.rs:367`

---

### FUNCTION: `u32`

**Documentation:**

```
The next argument as a `u32`.
```

**Signature:**

```rust
pub fn u32(&mut self) -> Option<u32>
```

**Location:** `hf_net.rs:379`

---

### FUNCTION: `id64`

**Documentation:**

```
The next argument as an `id64` (a name hash).
```

**Signature:**

```rust
pub fn id64(&mut self) -> Option<u64>
```

**Location:** `hf_net.rs:384`

---

### FUNCTION: `u64`

**Documentation:**

```
The next argument as a `u64`.
```

**Signature:**

```rust
pub fn u64(&mut self) -> Option<u64>
```

**Location:** `hf_net.rs:389`

---

### FUNCTION: `direction`

**Documentation:**

```
The next argument as a `direction`.
```

**Signature:**

```rust
pub fn direction(&mut self) -> Option<[f32
```

**Location:** `hf_net.rs:394`

---

### FUNCTION: `vector2`

**Documentation:**

```
The next argument as a `vector2`.
```

**Signature:**

```rust
pub fn vector2(&mut self) -> Option<[f32
```

**Location:** `hf_net.rs:399`

---

### FUNCTION: `array`

**Documentation:**

```
The next argument as an array of at most `max` values, read by `read`.
```

**Signature:**

```rust
pub fn array<T>( &mut self, max: usize, mut read: impl FnMut(&mut Self) -> Option<T>, ) -> Option<Vec<T>>
```

**Location:** `hf_net.rs:404`

---

### FUNCTION: `finish`

**Documentation:**

```
Whether every argument was read.
```

**Signature:**

```rust
pub fn finish(&self) -> bool
```

**Location:** `hf_net.rs:424`

---

### FUNCTION: `send`

**Documentation:**

```
Sends a message to the same mod on other machines of the lobby.

`to` is [`TO_ALL`], [`TO_HOST`], or a peer id from a
[`MessageEvent::from`]. At most [`MESSAGE_MAX`] bytes. Returns `true`
when the message was queued.
```

**Signature:**

```rust
pub fn send(&self, to: u64, data: &[u8]) -> bool
```

**Location:** `hf_net.rs:558`

---

### FUNCTION: `share_set`

**Documentation:**

```
Sets a row of this mod's shared table (host only).

An empty `value` removes the row. Returns `true` when set.
```

**Signature:**

```rust
pub fn share_set(&self, key: u64, value: &[u8]) -> bool
```

**Location:** `hf_net.rs:576`

---

### FUNCTION: `share_get`

**Documentation:**

```
One row of this mod's shared table on this machine.

Returns `None` when the row is absent.
```

**Signature:**

```rust
pub fn share_get(&self, key: u64) -> Option<Vec<u8>>
```

**Location:** `hf_net.rs:601`

---

### FUNCTION: `share_row`

**Documentation:**

```
The `index`-th row of this mod's shared table on this machine.

Returns the key and value, or `None` past the last row.
```

**Signature:**

```rust
pub fn share_row(&self, index: u32) -> Option<(u64, Vec<u8>)>
```

**Location:** `hf_net.rs:623`

---

### FUNCTION: `on_message`

**Documentation:**

```
Calls `callback` when the same mod on another machine sends a message
(`HF_ON_MESSAGE`).

The callback runs on the game thread, so it must be `Send`. It stays
registered for the session (the API has no unregister call).
Returns `true` when the listener was registered.
```

**Signature:**

```rust
pub fn on_message<F>(&self, callback: F) -> bool where F: FnMut(&HellForgeApi, MessageEvent<'_>) + Send + 'static,
```

**Location:** `hf_net.rs:655`

---

### FUNCTION: `on_share`

**Documentation:**

```
Calls `callback` when a row of this mod's shared table arrives, changes
or goes (`HF_ON_SHARE`).

The callback runs on the game thread, so it must be `Send`. It stays
registered for the session (the API has no unregister call).
Returns `true` when the listener was registered.
```

**Signature:**

```rust
pub fn on_share<F>(&self, callback: F) -> bool where F: FnMut(&HellForgeApi, ShareEvent<'_>) + Send + 'static,
```

**Location:** `hf_net.rs:683`

---

### FUNCTION: `rpc_listen`

**Documentation:**

```
Listens to one of the game's RPCs as this machine receives it.

`rpc` is its `HF_RPC_...` id (see [`rpc`]), or [`rpc::ALL`] for every
one. Listeners run on the game thread before the game's own receiver,
highest priority first (equal priorities in the order they listened).
Call it from `#[hf_entry]`.

Returns `true` when listening; `false` when refused (an RPC this build
does not have, or a loader older than SDK 17).
```

**Signature:**

```rust
pub fn rpc_listen<F>(&self, rpc: u32, priority: i32, callback: F) -> bool where F: FnMut(&HellForgeApi, &RpcEvent<'_>) -> RpcAction + Send + 'static,
```

**Location:** `hf_net.rs:712`

---

### FUNCTION: `rpc_send`

**Documentation:**

```
Sends one of the game's RPCs.

`to` is [`TO_ALL`], [`TO_HOST`], or a peer from an
[`RpcEvent::sender`]. The game encodes each argument by its declared
type, so values outside a type's range are clamped or wrap. Call it on
the game thread. Returns `true` when sent.
```

**Signature:**

```rust
pub fn rpc_send(&self, rpc: u32, to: u64, args: &[RpcArg<'_>]) -> bool
```

**Location:** `hf_net.rs:751`

---

### FUNCTION: `rpc_send_message`

**Documentation:**

```
Sends one of the game's RPCs from its typed message
([`RpcMessage`]).

Returns `true` when sent; `false` when refused or when the message has
an array longer than its RPC allows.
```

**Signature:**

```rust
pub fn rpc_send_message<M: RpcMessage>(&self, to: u64, message: &M) -> bool
```

**Location:** `hf_net.rs:790`

---

### FUNCTION: `rpc_listen_message`

**Documentation:**

```
Listens to one of the game's RPCs and decodes it into its typed message
([`RpcMessage`]).

Like [`rpc_listen`](Self::rpc_listen), but the callback receives the
decoded message instead of the raw event. A message whose arguments do
not decode is logged and left for the game's own receiver.

Returns `true` when listening; `false` when refused.
```

**Signature:**

```rust
pub fn rpc_listen_message<M, F>(&self, priority: i32, callback: F) -> bool where M: RpcMessage + 'static, F: FnMut(&HellForgeApi, M) + Send + 'static,
```

**Location:** `hf_net.rs:805`

---

## File: `hf_overlay.rs`

### STRUCT: `Color`

**Documentation:**

```
A colour as the C SDK's `HfColour`: `HF_RGBA(r, g, b, a)`.
```

**Signature:**

```rust
pub struct Color
```

**Location:** `hf_overlay.rs:29`

---

### FUNCTION: `rgb`

**Documentation:**

```
An opaque colour.
```

**Signature:**

```rust
pub const fn rgb(r: u8, g: u8, b: u8) -> Self
```

**Location:** `hf_overlay.rs:41`

---

### FUNCTION: `rgba`

**Documentation:**

```
A colour with an alpha channel.
```

**Signature:**

```rust
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self
```

**Location:** `hf_overlay.rs:46`

---

### FUNCTION: `as_raw`

**Documentation:**

```
The raw `HF_FACE_*` value.
```

**Signature:**

```rust
pub const fn as_raw(self) -> u32
```

**Location:** `hf_overlay.rs:82`

---

### FUNCTION: `as_raw`

**Documentation:**

```
The raw `HF_ALIGN_*` value.
```

**Signature:**

```rust
pub const fn as_raw(self) -> u32
```

**Location:** `hf_overlay.rs:100`

---

### STRUCT: `InputState`

**Documentation:**

```
The mouse and keys for one frame, as the draw callback sees them.
```

**Signature:**

```rust
pub struct InputState
```

**Location:** `hf_overlay.rs:111`

---

### FUNCTION: `from_ffi`

**Documentation:**

```
Converts a host `HfInput` into a safe [`InputState`].

Only fields the host's `size` covers are read; the rest are defaults,
so a newer Rust binding can run on an older host.
```

**Signature:**

```rust
pub fn from_ffi(input: &HfInput) -> Self
```

**Location:** `hf_overlay.rs:137`

---

### STRUCT: `DrawingContext`

**Documentation:**

```
Drawing for one frame, from the host's `HfDraw`.

Only valid inside a screen's draw callback: the trampoline is the only thing
that constructs it, and it does not outlive the callback's borrow.
```

**Signature:**

```rust
pub struct DrawingContext
```

**Location:** `hf_overlay.rs:210`

---

### FUNCTION: `width`

**Documentation:**

```
The screen's width in units (1080 units high, width units wide).
```

**Signature:**

```rust
pub fn width(&self) -> f32
```

**Location:** `hf_overlay.rs:250`

---

### FUNCTION: `height`

**Documentation:**

```
The screen's height in units (always 1080).
```

**Signature:**

```rust
pub fn height(&self) -> f32
```

**Location:** `hf_overlay.rs:255`

---

### FUNCTION: `pixels`

**Documentation:**

```
Pixels per unit (the window's height / 1080).
```

**Signature:**

```rust
pub fn pixels(&self) -> f32
```

**Location:** `hf_overlay.rs:261`

---

### FUNCTION: `rect`

**Documentation:**

```
A rectangle, filled and optionally outlined.
```

**Signature:**

```rust
pub fn rect(&self, rect: Rect, fill: Color, outline: Option<Color>, thickness: f32)
```

**Location:** `hf_overlay.rs:267`

---

### FUNCTION: `lines`

**Documentation:**

```
An open (`closed = false`) or closed line through `points`.
```

**Signature:**

```rust
pub fn lines(&self, points: &[Point], colour: Color, thickness: f32, closed: bool)
```

**Location:** `hf_overlay.rs:288`

---

### FUNCTION: `polygon`

**Documentation:**

```
A filled polygon; the host triangulates concave ones.
```

**Signature:**

```rust
pub fn polygon(&self, points: &[Point], fill: Color)
```

**Location:** `hf_overlay.rs:306`

---

### FUNCTION: `circle`

**Documentation:**

```
A circle, filled and optionally outlined.
```

**Signature:**

```rust
pub fn circle( &self, centre: Point, radius: f32, fill: Color, outline: Option<Color>, thickness: f32, )
```

**Location:** `hf_overlay.rs:317`

---

### FUNCTION: `text`

**Documentation:**

```
One line of UTF-8 text with its cap height centred on `at.y`.

Returns its width in units, or 0 when the text contains an interior NUL
byte or the host is too old for the call.
```

**Signature:**

```rust
pub fn text( &self, at: Point, text: &str, size: f32, face: Face, align: Align, colour: Color, ) -> f32
```

**Location:** `hf_overlay.rs:338`

---

### FUNCTION: `text_width`

**Documentation:**

```
The width [`DrawingContext::text`] would use.
```

**Signature:**

```rust
pub fn text_width(&self, text: &str, size: f32, face: Face) -> f32
```

**Location:** `hf_overlay.rs:369`

---

### FUNCTION: `image_create`

**Documentation:**

```
Makes a picture from `width * height` RGBA8 pixels, row-major from the
top, or `None` when the host cannot make it or `rgba` is too short.

The picture lives until the returned [`Image`] is dropped.
```

**Signature:**

```rust
pub fn image_create(&self, width: u32, height: u32, rgba: &[u8]) -> Option<Image>
```

**Location:** `hf_overlay.rs:387`

---

### FUNCTION: `image`

**Documentation:**

```
Draws a picture into `box_`, bilinear, multiplied by `tint`.

`part` is the piece to draw, in 0..1 of its width and height; `None` is
all of it.
```

**Signature:**

```rust
pub fn image(&self, image: &Image, box_: Rect, part: Option<Rect>, tint: Color)
```

**Location:** `hf_overlay.rs:413`

---

### FUNCTION: `clip_push`

**Documentation:**

```
Cuts drawing off outside `rect` until [`DrawingContext::clip_pop`].
```

**Signature:**

```rust
pub fn clip_push(&self, rect: Rect)
```

**Location:** `hf_overlay.rs:425`

---

### FUNCTION: `clip_pop`

**Documentation:**

```
Ends the innermost [`DrawingContext::clip_push`].
```

**Signature:**

```rust
pub fn clip_pop(&self)
```

**Location:** `hf_overlay.rs:436`

---

### FUNCTION: `backdrop`

**Documentation:**

```
The game's frame as it was when the screen opened: dimmed (`0` none ..
`1` black), desaturated (`0` .. `1`) and blurred (`radius` in units).
```

**Signature:**

```rust
pub fn backdrop(&self, dim: f32, desaturate: f32, blur: f32)
```

**Location:** `hf_overlay.rs:448`

---

### FUNCTION: `hide_pointer`

**Documentation:**

```
Hides the mouse pointer while a modal screen draws its own cursor.

Call it every frame the cursor is over the screen's own cursor. Older
hosts end at `backdrop`, so this does nothing there.
```

**Signature:**

```rust
pub fn hide_pointer(&self)
```

**Location:** `hf_overlay.rs:463`

---

### STRUCT: `Image`

**Documentation:**

```
A picture made by [`DrawingContext::image_create`].

Drop it inside the draw callback (on the host's render thread): it frees the
host's picture. It is not `Send`, so it cannot move to another thread.
```

**Signature:**

```rust
pub struct Image
```

**Location:** `hf_overlay.rs:494`

---

### FUNCTION: `update`

**Documentation:**

```
Replaces all its pixels, or does nothing when `rgba` is too short.
```

**Signature:**

```rust
pub fn update(&mut self, rgba: &[u8])
```

**Location:** `hf_overlay.rs:504`

---

### STRUCT: `NoOpened`

**Documentation:**

```
The default [`ScreenOpened`]: does nothing.
```

**Signature:**

```rust
pub struct NoOpened
```

**Location:** `hf_overlay.rs:578`

---

### STRUCT: `NoCanOpen`

**Documentation:**

```
The default [`ScreenCanOpen`]: always allows.
```

**Signature:**

```rust
pub struct NoCanOpen
```

**Location:** `hf_overlay.rs:586`

---

### STRUCT: `ScreenSpec`

**Documentation:**

```
A screen to add: its name, key, flags and callbacks.
```

**Signature:**

```rust
pub struct ScreenSpec<D, O, C>
```

**Location:** `hf_overlay.rs:595`

---

### FUNCTION: `new`

**Documentation:**

```
A screen with no `opened` or `can_open` callback.

`key` is the virtual key that opens and closes it (`'M' as u32`), or 0
for none (open it with [`HellForgeApi::screen_show`]).
```

**Signature:**

```rust
pub fn new(name: impl Into<String>, key: u32, modal: bool, draw: D) -> Self
```

**Location:** `hf_overlay.rs:612`

---

### FUNCTION: `on_opened`

**Documentation:**

```
Called when the screen opens and after it closes.
```

**Signature:**

```rust
pub fn on_opened<F>(self, opened: F) -> ScreenSpec<D, F, C> where F: ScreenOpened,
```

**Location:** `hf_overlay.rs:626`

---

### FUNCTION: `on_can_open`

**Documentation:**

```
Called when its key is pressed, to decide whether it may open.
```

**Signature:**

```rust
pub fn on_can_open<F>(self, can_open: F) -> ScreenSpec<D, O, F> where F: ScreenCanOpen,
```

**Location:** `hf_overlay.rs:641`

---

### FUNCTION: `screen_add`

**Documentation:**

```
Adds a screen drawn over the game.

`key` is the virtual key that opens and closes it (`'M' as u32`), or 0
for none (open it with [`HellForgeApi::screen_show`]).

Returns the screen's id (1 and up), or `None` when the loader is older
than SDK 17, the name has an interior NUL byte, or the loader refuses it
(a key another screen already uses, say; the mod's log says why).
```

**Signature:**

```rust
pub fn screen_add<D, O, C>(&self, screen: ScreenSpec<D, O, C>) -> Option<i32> where D: ScreenDraw + 'static, O: ScreenOpened + 'static, C: ScreenCanOpen + 'static,
```

**Location:** `hf_overlay.rs:768`

---

### FUNCTION: `screen_show`

**Documentation:**

```
Opens (`true`) or closes (`false`) one of the mod's screens.

A close sets `input.closing`; draw is called until it returns `false`.
Returns `true` when queued, `false` when the loader is older than SDK 17.
```

**Signature:**

```rust
pub fn screen_show(&self, screen: i32, open: bool) -> bool
```

**Location:** `hf_overlay.rs:817`

---

## File: `hf_ui.rs`

### FUNCTION: `as_raw`

**Documentation:**

```
The raw `HF_UI_*` value.
```

**Signature:**

```rust
pub const fn as_raw(self) -> u32
```

**Location:** `hf_ui.rs:34`

---

### FUNCTION: `screen`

**Documentation:**

```
The screen the event came from.
```

**Signature:**

```rust
pub const fn screen(self) -> i32
```

**Location:** `hf_ui.rs:53`

---

### FUNCTION: `ui_open`

**Documentation:**

```
Opens a screen.

`xaml` is a file of the mod's folder (`"ui/shop.xaml"`) or XAML text.
The screen shows at the next game update; at most 64 mods' screens exist
across all mods.

Returns the screen's id, or `None` when refused (the mod's log says
why).
```

**Signature:**

```rust
pub fn ui_open(&self, xaml: &str, kind: ScreenKind) -> Option<i32>
```

**Location:** `hf_ui.rs:117`

---

### FUNCTION: `ui_set`

**Documentation:**

```
Sets a property of a named element of `screen` from text, as XAML
would.

`element` `None` (or empty) sets the screen's root. For example:
`"Text", "SUPPLY"`; `"Visibility", "Collapsed"`; `"Foreground",
"#FFE900"`.

Returns `true` when queued, `false` when refused or when a string
contains an interior NUL byte.
```

**Signature:**

```rust
pub fn ui_set(&self, screen: i32, element: Option<&str>, property: &str, value: &str) -> bool
```

**Location:** `hf_ui.rs:139`

---

### FUNCTION: `ui_on`

**Documentation:**

```
Calls `callback` when `element` of `screen` is clicked.

With `element` `None` (or empty), calls it when the screen closes
([`HellForgeApi::ui_close`], or Back on a menu), as
[`ScreenEvent::Closed`].

The callback runs on the game thread, so it must be `Send`: `ui_on`
may be called from any thread. The loader keeps it for the session
(the API has no unregister call), so it is not reclaimed until the DLL
is unloaded. Returns `true` when queued, `false` when refused.
```

**Signature:**

```rust
pub fn ui_on<F>(&self, screen: i32, element: Option<&str>, callback: F) -> bool where F: FnMut(&HellForgeApi, ScreenEvent<'_>) + Send + 'static,
```

**Location:** `hf_ui.rs:186`

---

### FUNCTION: `ui_close`

**Documentation:**

```
Closes one of the mod's screens.

A menu closes as the game closes its own. Returns `true` when queued,
`false` when refused.
```

**Signature:**

```rust
pub fn ui_close(&self, screen: i32) -> bool
```

**Location:** `hf_ui.rs:234`

---

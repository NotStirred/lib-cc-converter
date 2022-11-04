use std::collections::HashMap;
use crate::cursed::types::*;
use crate::cursed::types::type_names::*;
use quartz_nbt::Map;

use CompoundField::*;

fn v99_equipment() -> Type {
	compound(map![
		"Equipment": OptionalField(list(reference(ITEM_STACK)))
	])
}

fn v99_mob() -> Type {
	v99_equipment()
}

fn v99_throwable_projectile() -> Type {
	compound(map![
		"inTile": OptionalField(reference(BLOCK_NAME))
	])
}

fn v99_minecart() -> Type {
	compound(map![
		"DisplayTile": OptionalField(reference(BLOCK_NAME))
	])
}

fn v99_inventory() -> Type {
	compound(map![
		"Items": OptionalField(list(reference(ITEM_STACK)))
	])
}

fn v99_entities() -> HashMap<TypeName, Type> {
	map![
		"Item": compound(map!["Item": OptionalField(reference(ITEM_STACK))]),
		"XPOrb": rest(),
		"ThrownEgg": v99_throwable_projectile(),
		"LeashKnot": rest(),
		"Painting": rest(),
		"Arrow": v99_throwable_projectile(),
		"TippedArrow": v99_throwable_projectile(),
		"SpectralArrow": v99_throwable_projectile(),
		"Snowball": v99_throwable_projectile(),
		"Fireball": v99_throwable_projectile(),
		"SmallFireball": v99_throwable_projectile(),
		"ThrownEnderpearl": v99_throwable_projectile(),
		"EyeOfEnderSignal": rest(),
		"ThrownPotion": compound(map!["inTile": OptionalField(reference(BLOCK_NAME)), "Potion": OptionalField(reference(ITEM_STACK))]),
		"ThrownExpBottle": v99_throwable_projectile(),
		"ItemFrame": compound(map!["Item": OptionalField(reference(ITEM_STACK))]),
		"WitherSkull": v99_throwable_projectile(),
		"PrimedTnt": rest(),
		"FallingSand": compound(map!["Block": OptionalField(reference(BLOCK_NAME)), "TileEntityData": OptionalField(reference(BLOCK_ENTITY))]),
		"FireworksRocketEntity": compound(map!["FireworksItem": OptionalField(reference(ITEM_STACK))]),
		"Boat": rest(),
		"Minecart": compound(map!["DisplayTile": OptionalField(reference(BLOCK_NAME)), "Items": OptionalField(list(reference(ITEM_STACK)))]),
		"MinecartRideable": v99_minecart(),
		"MinecartChest": compound(map!["DisplayTile": OptionalField(reference(BLOCK_NAME)), "Items": OptionalField(list(reference(ITEM_STACK)))]),
		"MinecartFurnace": v99_minecart(),
		"MinecartTNT": v99_minecart(),
		"MinecartSpawner": and([compound(map!["DisplayTile": OptionalField(reference(BLOCK_NAME))]), reference(UNTAGGED_SPAWNER)]),
		"MinecartHopper": compound(map!["DisplayTile": OptionalField(reference(BLOCK_NAME)), "Items": OptionalField(list(reference(ITEM_STACK)))]),
		"MinecartCommandBlock": v99_minecart(),
		"ArmorStand": v99_mob(),
		"Creeper": v99_mob(),
		"Skeleton": v99_mob(),
		"Spider": v99_mob(),
		"Giant": v99_mob(),
		"Zombie": v99_mob(),
		"Slime": v99_mob(),
		"Ghast": v99_mob(),
		"PigZombie": v99_mob(),
		"Enderman": compound(map!["carried": OptionalField(reference(BLOCK_NAME)), "Equipment": OptionalField(list(reference(ITEM_STACK)))]),
		"CaveSpider": v99_mob(),
		"Silverfish": v99_mob(),
		"Blaze": v99_mob(),
		"LavaSlime": v99_mob(),
		"EnderDragon": v99_mob(),
		"WitherBoss": v99_mob(),
		"Bat": v99_mob(),
		"Witch": v99_mob(),
		"Endermite": v99_mob(),
		"Guardian": v99_mob(),
		"Pig": v99_mob(),
		"Sheep": v99_mob(),
		"Cow": v99_mob(),
		"Chicken": v99_mob(),
		"Squid": v99_mob(),
		"Wolf": v99_mob(),
		"MushroomCow": v99_mob(),
		"SnowMan": v99_mob(),
		"Ozelot": v99_mob(),
		"VillagerGolem": v99_mob(),
		"EntityHorse": compound(map![
			"Items": OptionalField(list(reference(ITEM_STACK))),
			"ArmorItem": OptionalField(reference(ITEM_STACK)),
			"SaddleItem": OptionalField(reference(ITEM_STACK)),
			"Equipment": OptionalField(list(reference(ITEM_STACK)))
		]),
		"Rabbit": v99_mob(),
		"Villager": compound(map![
			"Inventory": OptionalField(list(reference(ITEM_STACK))),
			"Offers": OptionalField(compound(map!["Recipes": OptionalField(list(compound(map![
				"buy": OptionalField(reference(ITEM_STACK)),
				"buyB": OptionalField(reference(ITEM_STACK)),
				"sell": OptionalField(reference(ITEM_STACK))
			])))])),
			"Equipment": OptionalField(list(reference(ITEM_STACK)))
		]),
		"EnderCrystal": rest(),
		"AreaEffectCloud": rest(),
		"ShulkerBullet": rest(),
		"Shulker": v99_mob()
	]
}

fn v99_block_entities() -> HashMap<TypeName, Type> {
	map![
		"Furnace": v99_inventory(),
		"Chest": v99_inventory(),
		"EnderChest": rest(),
		"RecordPlayer": compound(map!["RecordItem": OptionalField(reference(ITEM_STACK))]),
		"Trap": v99_inventory(),
		"Dropper": v99_inventory(),
		"Sign": rest(),
		"MobSpawner": reference(UNTAGGED_SPAWNER),
		"Music": rest(),
		"Piston": rest(),
		"Cauldron": v99_inventory(),
		"EnchantTable": rest(),
		"Airportal": rest(),
		"Control": rest(),
		"Beacon": rest(),
		"Skull": rest(),
		"DLDetector": rest(),
		"Hopper": v99_inventory(),
		"Comparator": rest(),
		"FlowerPot": compound(map!["Item": OptionalField(or([primitive(PrimitiveType::Int), reference(ITEM_NAME)]))]),
		"Banner": rest(),
		"Structure": rest(),
		"EndGateway": rest()
	]
}

fn v99_schema() -> Schema {
	let mut types: HashMap<TypeName, Type> = map![
		LEVEL: rest(),
		PLAYER: compound(map!["Inventory": OptionalField(list(reference(ITEM_STACK))), "EnderItems": OptionalField(list(reference(ITEM_STACK)))]),
		CHUNK: compound(map!["Level": RequiredField(compound(map![
			"Entities": OptionalField(list(reference(ENTITY_TREE))),
			"TileEntities": OptionalField(list(reference(BLOCK_ENTITY) /*or(reference(BLOCK_ENTITY), rest())*/)),
			"TileTicks": OptionalField(list(compound(map!["i": RequiredField(reference(BLOCK_NAME))])))
		]))]),
		BLOCK_ENTITY: tagged_choice_compound("id", PrimitiveType::String, v99_block_entities()),
		ENTITY_TREE: and([compound(map!["riding": OptionalField(reference(ENTITY_TREE))]), reference(ENTITY)]),
		// TODO do we want a separate type for namespaced string to match DFU?
		ENTITY_NAME: primitive(PrimitiveType::String),
		ENTITY: tagged_choice_compound("id", PrimitiveType::String, v99_entities()),
		// TODO DFU v99 has a Hook thing on ITEM_STACK; what does it do and how do we reimplement it?
		ITEM_STACK: compound(map![
			"id": OptionalField(or([primitive(PrimitiveType::Int), reference(ITEM_NAME)])),
			"tag": OptionalField(compound(map![
				"EntityTag": OptionalField(reference(ENTITY_TREE)),
				"BlockEntityTag": OptionalField(reference(BLOCK_ENTITY)),
				"CanDestroy": OptionalField(list(reference(BLOCK_NAME))),
				"CanPlaceOn": OptionalField(list(reference(BLOCK_NAME))),
				"Items": OptionalField(list(reference(ITEM_STACK)))
			]))
		]),
		OPTIONS: rest(),
		BLOCK_NAME: or([primitive(PrimitiveType::Int), primitive(PrimitiveType::String)]),
		ITEM_NAME: primitive(PrimitiveType::String),
		STATS: rest(),
		SAVED_DATA: compound(map!["data": OptionalField(compound(map![
			// TODO what is a CompoundList?
			// "Features": OptionalField(compound()),
			"Objectives": OptionalField(list(reference(OBJECTIVE))),
			"Teams": OptionalField(list(reference(TEAM)))
		]))]),
		STRUCTURE_FEATURE: rest(),
		OBJECTIVE: rest(),
		TEAM: rest(),
		UNTAGGED_SPAWNER: rest(),
		POI_CHUNK: rest(),
		WORLD_GEN_SETTINGS: rest(),
		ENTITY_CHUNK: compound(map!["Entities": OptionalField(list(reference(ENTITY_TREE)))])
	];

	Schema {
		types
	}
}

fn v100_equipment() -> Type {
	compound(map![
		"ArmorItems": OptionalField(list(reference(ITEM_STACK))),
		"HandItems": OptionalField(list(reference(ITEM_STACK)))
	])
}

fn v100_mob() -> Type {
	v100_equipment()
}

fn v100_entities() -> HashMap<TypeName, Type> {
	let mut entities = v99_entities();

	entities.extend(map![
		"ArmorStand": v100_mob(),
        "Creeper": v100_mob(),
        "Skeleton": v100_mob(),
        "Spider": v100_mob(),
        "Giant": v100_mob(),
        "Zombie": v100_mob(),
        "Slime": v100_mob(),
        "Ghast": v100_mob(),
        "PigZombie": v100_mob(),
		"Enderman": compound(map![
			"carried": OptionalField(reference(BLOCK_NAME)),
			"ArmorItems": OptionalField(list(reference(ITEM_STACK))),
			"HandItems": OptionalField(list(reference(ITEM_STACK)))
		]),
        "CaveSpider": v100_mob(),
        "Silverfish": v100_mob(),
        "Blaze": v100_mob(),
        "LavaSlime": v100_mob(),
        "EnderDragon": v100_mob(),
        "WitherBoss": v100_mob(),
        "Bat": v100_mob(),
        "Witch": v100_mob(),
        "Endermite": v100_mob(),
        "Guardian": v100_mob(),
        "Pig": v100_mob(),
        "Sheep": v100_mob(),
        "Cow": v100_mob(),
        "Chicken": v100_mob(),
        "Squid": v100_mob(),
        "Wolf": v100_mob(),
        "MushroomCow": v100_mob(),
        "SnowMan": v100_mob(),
        "Ozelot": v100_mob(),
        "VillagerGolem": v100_mob(),
        "EntityHorse": compound(map![
			"Items": OptionalField(list(reference(ITEM_STACK))),
			"ArmorItem": OptionalField(reference(ITEM_STACK)),
			"SaddleItem": OptionalField(reference(ITEM_STACK)),
			"ArmorItems": OptionalField(list(reference(ITEM_STACK))),
			"HandItems": OptionalField(list(reference(ITEM_STACK)))
		]),
		"Rabbit": v100_mob(),
		"Villager": compound(map![
			"Inventory": OptionalField(list(reference(ITEM_STACK))),
			"Offers": OptionalField(compound(map!["Recipes": OptionalField(list(compound(map![
				"buy": OptionalField(reference(ITEM_STACK)),
				"buyB": OptionalField(reference(ITEM_STACK)),
				"sell": OptionalField(reference(ITEM_STACK))
			])))])),
			"ArmorItems": OptionalField(list(reference(ITEM_STACK))),
			"HandItems": OptionalField(list(reference(ITEM_STACK)))
		]),
		"Shulker": v100_mob(),
        "AreaEffectCloud": rest(),
        "ShulkerBullet": rest()
	]);

	entities
}

fn v100_schema() -> Schema {
	let mut types = v99_schema().types;

	types.extend(map![
		ENTITY: tagged_choice_compound("id", PrimitiveType::String, v100_entities()),
		STRUCTURE: compound(map![
			"entities": OptionalField(list(compound(map!["nbt": OptionalField(reference(ENTITY_TREE))]))),
			"blocks": OptionalField(list(compound(map!["nbt": OptionalField(reference(BLOCK_ENTITY))]))),
			"palette": OptionalField(list(reference(BLOCK_STATE)))
		]),
		BLOCK_STATE: rest()
	]);

	Schema {
		types
	}
}

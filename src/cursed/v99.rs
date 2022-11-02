use std::collections::HashMap;
use crate::cursed::types::*;
use crate::cursed::types::type_names::*;
use quartz_nbt::Map;

use CompoundField::*;

fn equipment() -> Type {
	compound(map![
		"Equipment": OptionalField(list(reference(ITEM_STACK)))
	])
}

fn mob() -> Type {
	equipment()
}

fn throwable_projectile() -> Type {
	compound(map![
		"inTile": OptionalField(reference(BLOCK_NAME))
	])
}

fn minecart() -> Type {
	compound(map![
		"DisplayTile": OptionalField(reference(BLOCK_NAME))
	])
}

fn inventory() -> Type {
	compound(map![
		"Items": OptionalField(list(reference(ITEM_STACK)))
	])
}

fn register_entities() -> HashMap<TypeName, Type> {
	map![
		"Item": compound(map!["Item": OptionalField(reference(ITEM_STACK))]),
		"XPOrb": rest(),
		"ThrownEgg": throwable_projectile(),
		"LeashKnot": rest(),
		"Painting": rest(),
		"Arrow": throwable_projectile(),
		"TippedArrow": throwable_projectile(),
		"SpectralArrow": throwable_projectile(),
		"Snowball": throwable_projectile(),
		"Fireball": throwable_projectile(),
		"SmallFireball": throwable_projectile(),
		"ThrownEnderpearl": throwable_projectile(),
		"EyeOfEnderSignal": rest(),
		"ThrownPotion": compound(map!["inTile": OptionalField(reference(BLOCK_NAME)), "Potion": OptionalField(reference(ITEM_STACK))]),
		"ThrownExpBottle": throwable_projectile(),
		"ItemFrame": compound(map!["Item": OptionalField(reference(ITEM_STACK))]),
		"WitherSkull": throwable_projectile(),
		"PrimedTnt": rest(),
		"FallingSand": compound(map!["Block": OptionalField(reference(BLOCK_NAME)), "TileEntityData": OptionalField(reference(BLOCK_ENTITY))]),
		"FireworksRocketEntity": compound(map!["FireworksItem": OptionalField(reference(ITEM_STACK))]),
		"Boat": rest(),
		"Minecart": compound(map!["DisplayTile": OptionalField(reference(BLOCK_NAME)), "Items": OptionalField(list(reference(ITEM_STACK)))]),
		"MinecartRideable": minecart(),
		"MinecartChest": compound(map!["DisplayTile": OptionalField(reference(BLOCK_NAME)), "Items": OptionalField(list(reference(ITEM_STACK)))]),
		"MinecartFurnace": minecart(),
		"MinecartTNT": minecart(),
		"MinecartSpawner": and([compound(map!["DisplayTile": OptionalField(reference(BLOCK_NAME))]), reference(UNTAGGED_SPAWNER)]),
		"MinecartHopper": compound(map!["DisplayTile": OptionalField(reference(BLOCK_NAME)), "Items": OptionalField(list(reference(ITEM_STACK)))]),
		"MinecartCommandBlock": minecart(),
		"ArmorStand": mob(),
		"Creeper": mob(),
		"Skeleton": mob(),
		"Spider": mob(),
		"Giant": mob(),
		"Zombie": mob(),
		"Slime": mob(),
		"Ghast": mob(),
		"PigZombie": mob(),
		"Enderman": compound(map!["carried": OptionalField(reference(BLOCK_NAME)), "Equipment": OptionalField(list(reference(ITEM_STACK)))]),
		"CaveSpider": mob(),
		"Silverfish": mob(),
		"Blaze": mob(),
		"LavaSlime": mob(),
		"EnderDragon": mob(),
		"WitherBoss": mob(),
		"Bat": mob(),
		"Witch": mob(),
		"Endermite": mob(),
		"Guardian": mob(),
		"Pig": mob(),
		"Sheep": mob(),
		"Cow": mob(),
		"Chicken": mob(),
		"Squid": mob(),
		"Wolf": mob(),
		"MushroomCow": mob(),
		"SnowMan": mob(),
		"Ozelot": mob(),
		"VillagerGolem": mob(),
		"EntityHorse": compound(map![
			"Items": OptionalField(list(reference(ITEM_STACK))),
			"ArmorItem": OptionalField(reference(ITEM_STACK)),
			"SaddleItem": OptionalField(reference(ITEM_STACK)),
			"Equipment": OptionalField(list(reference(ITEM_STACK)))
		]),
		"Rabbit": mob(),
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
		"Shulker": mob()
	]
}

fn register_block_entities() -> HashMap<TypeName, Type> {
	map![
		"Furnace": inventory(),
		"Chest": inventory(),
		"EnderChest": rest(),
		"RecordPlayer": compound(map!["RecordItem": OptionalField(reference(ITEM_STACK))]),
		"Trap": inventory(),
		"Dropper": inventory(),
		"Sign": rest(),
		"MobSpawner": reference(UNTAGGED_SPAWNER),
		"Music": rest(),
		"Piston": rest(),
		"Cauldron": inventory(),
		"EnchantTable": rest(),
		"Airportal": rest(),
		"Control": rest(),
		"Beacon": rest(),
		"Skull": rest(),
		"DLDetector": rest(),
		"Hopper": inventory(),
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
		BLOCK_ENTITY: tagged_choice_compound("id", PrimitiveType::String, register_block_entities()),
		ENTITY_TREE: and([compound(map!["riding": OptionalField(reference(ENTITY_TREE))]), reference(ENTITY)]),
		// TODO do we want a separate type for namespaced string to match DFU?
		ENTITY_NAME: primitive(PrimitiveType::String),
		ENTITY: tagged_choice_compound("id", PrimitiveType::String, register_entities()),
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
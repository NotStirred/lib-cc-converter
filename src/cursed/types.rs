use quartz_nbt::{Map, NbtCompound, NbtTag};
use std::collections::{HashMap};
use std::string::ToString;

pub type TypeName = String;

#[derive(Clone)]
pub enum PrimitiveType {
	/// A signed, one-byte integer.
	Byte,
	/// A signed, two-byte integer.
	Short,
	/// A signed, four-byte integer.
	Int,
	/// A signed, eight-byte integer.
	Long,
	/// A 32-bit floating point value.
	Float,
	/// A 64-bit floating point value.
	Double,
	/// An array (vec) of one-byte integers. Minecraft treats this as an array of signed bytes.
	ByteArray,
	/// A UTF-8 string.
	String,
	/// An array (vec) of signed, four-byte integers.
	IntArray,
	/// An array (vec) of signed, eight-byte integers.
	LongArray,
}

#[derive(Clone)]
pub enum CompoundField {
	RequiredField(Type),
	OptionalField(Type)
}

impl CompoundField {
	pub fn get_type(&self) -> &Type {
		match self {
			CompoundField::RequiredField(t) => t,
			CompoundField::OptionalField(t) => t
		}
	}
}

#[derive(Clone)]
pub enum TypeData {
	Primitive(PrimitiveType),
	List(Box<Type>),
	Compound(Map<CompoundField>),
	/// Represents a compound where the type schema is determined by a tag of the compound (e.g. entity schemas are determined by the entity type)
	// TODO handle type keys other than String
	TaggedChoiceCompound {key_name: String, key_type: PrimitiveType, types: Map<Type>},
	/// Represents a sum type of multiple types
	Sum(Vec<Type>),
	/// Represents a product type (union) of multiple types. We currently handle this the same as sum types, but that might break in some cases?
	Product(Vec<Type>),
	/// Represents a reference to a type defined in the Schema
	Reference(TypeName),
	/// Used to represent data that is passed through unmodified
	Rest
}

#[derive(Clone)]
pub struct Type {
	type_data: TypeData
}

impl Type {
	fn new(type_data: TypeData) -> Self {
		Self {
			type_data
		}
	}
}

pub fn primitive(prim: PrimitiveType) -> Type {
	Type::new(TypeData::Primitive(prim))
}

pub fn list(kind: Type) -> Type {
	Type::new(TypeData::List(Box::new(kind)))
}

pub fn or(types: impl Into<Vec<Type>>) -> Type {
	Type::new(TypeData::Sum(types.into()))
}

pub fn and(types: impl Into<Vec<Type>>) -> Type {
	Type::new(TypeData::Product(types.into()))
}

pub fn reference(name: impl Into<String>) -> Type {
	Type::new(TypeData::Reference(name.into()))
}

pub fn rest() -> Type {
	Type::new(TypeData::Rest)
}

pub fn compound(children: Map<CompoundField>) -> Type {
	Type::new(TypeData::Compound(children))
}

pub fn tagged_choice_compound(key_name: impl Into<String>, key_type: PrimitiveType, types: Map<Type>) -> Type {
	Type::new(TypeData::TaggedChoiceCompound {
		key_name: key_name.into(),
		key_type,
		types
	})
}

pub struct Schema {
	pub types: HashMap<TypeName, Type>
}

macro_rules! map {
	( $( $name:literal : $t:expr ),* ) => { Map::from([ $(($name.to_string(), $t)),* ]) };
	( $( $name:ident : $t:expr ),* ) => { Map::from([ $(($name.to_string(), $t)),* ]) }
}

pub mod type_names {
	pub const LEVEL: &str = "level";
	pub const PLAYER: &str = "player";
	pub const CHUNK: &str = "chunk";
	pub const HOTBAR: &str = "hotbar";
	pub const OPTIONS: &str = "options";
	pub const STRUCTURE: &str = "structure";
	pub const STATS: &str = "stats";
	pub const SAVED_DATA: &str = "saved_data";
	pub const ADVANCEMENTS: &str = "advancements";
	pub const POI_CHUNK: &str = "poi_chunk";
	pub const ENTITY_CHUNK: &str = "entity_chunk";
	pub const BLOCK_ENTITY: &str = "block_entity";
	pub const ITEM_STACK: &str = "item_stack";
	pub const BLOCK_STATE: &str = "block_state";
	pub const ENTITY_NAME: &str = "entity_name";
	pub const ENTITY_TREE: &str = "entity_tree";
	pub const ENTITY: &str = "entity";
	pub const BLOCK_NAME: &str = "block_name";
	pub const ITEM_NAME: &str = "item_name";
	pub const UNTAGGED_SPAWNER: &str = "untagged_spawner";
	pub const STRUCTURE_FEATURE: &str = "structure_feature";
	pub const OBJECTIVE: &str = "objective";
	pub const TEAM: &str = "team";
	pub const RECIPE: &str = "recipe";
	pub const BIOME: &str = "biome";
	pub const WORLD_GEN_SETTINGS: &str = "world_gen_settings";
}

use type_names::*;

fn test() {
	use CompoundField::{RequiredField as Req, OptionalField as Opt};
	use PrimitiveType::*;
	// helper types for shorter definitions

	// Schema types
	let level = rest();
	let player = compound(map![
		"Inventory": Opt(list(reference(ITEM_STACK))),
		"EnderItems": Opt(list(reference(ITEM_STACK)))
	]);
	let chunk = compound(map![
		"Level": Req(compound(map![
			"Entities": Opt(list(reference(ENTITY_TREE))),
			"TileEntities": Opt(list(or([reference(BLOCK_ENTITY), rest()]))),
			"TileTicks": Opt(list(compound(map!["i": Req(reference(BLOCK_NAME))])))
		]))
	]);
	let block_entity = tagged_choice_compound("id", String, map![
		// TODO
	]);

	let item_stack = compound(map![
		"id": Opt(or([
			primitive(Int),
			reference("item_name")
		])),
		"tag": Opt(compound(map![
			"EntityTag": Opt(reference("entity_tree")),
			"BlockEntityTag": Opt(reference("block_entity")),
			"CanDestroy": Opt(reference("block_name")),
			"CanPlaceOn": Opt(reference("block_name")),
			"Items": Opt(reference(ITEM_STACK))
		]))
	]);
}

macro_rules! match_tag {
	( $name:ident: $pat:pat => $expr:expr ) => {
		match $name {
			$pat => $expr,
			_ => {
				// TODO should warn in places where failure isn't expected (i.e. not Sum types etc)
			}
		}
	}
}

pub struct SearchOptions<'a> {
	pub schema: &'a Schema,
	pub target_field_name: Option<&'a str>,
	pub target_type_name: &'a str,
	pub recurse_into_target: bool
}

pub type ModifyOp = fn(&mut NbtTag);

fn matches_type(_target_type: &Type, tag: &NbtTag) -> bool {
	// TODO are there any cases where we have to match things other than compound tags?
	match tag {
		NbtTag::Compound(_) => true,
		_ => false
	}
}

pub fn search(options: &SearchOptions, root: &Type, current_field_name: &str, tag: &mut NbtTag, transform: ModifyOp) -> () {
	// println!("search current_field_name: {}\n current tag: {:?}", current_field_name, tag);

	let matches = {
		let mut matches = false;
		if options.target_field_name.is_none() || current_field_name == options.target_field_name.unwrap() {
			if let TypeData::Reference(type_name) = &root.type_data {
				if type_name == options.target_type_name
						&& matches_type(options.schema.types.get(options.target_type_name).unwrap(), tag) {
					matches = true;
				}
			}
		}
		matches
	};

	if options.recurse_into_target || !matches {
		match &root.type_data {
			TypeData::List(inner) => {
				match_tag!(tag: NbtTag::List(list) => {
				list.iter_mut().for_each(|tag| search(options, inner, "", tag, transform))
			})
			}
			TypeData::Compound(fields) => {
				match_tag!(tag: NbtTag::Compound(compound) => {
				fields.iter().for_each(|(name, field)| {
					// TODO warn on missing required fields
					if let Some(inner_tag) = compound.get_mut::<_, &mut NbtTag>(name).ok() {
						search(options, field.get_type(), name, inner_tag, transform)
					}
				})
			})
			}
			TypeData::TaggedChoiceCompound { key_name, types, .. } => {
				match_tag!(tag: NbtTag::Compound(compound) => {
				// TODO don't unwrap
				let type_name: &str = compound.get(key_name).unwrap();
				let choice_type = types.get(type_name).unwrap();
				search(options, choice_type, current_field_name, tag, transform);
			})
			}
			// TODO is it safe to handle these both the same?
			TypeData::Sum(types) | TypeData::Product(types) => {
				types.iter().for_each(|t| search(options, t, current_field_name, tag, transform));
			}
			TypeData::Reference(type_name) => {
				// TODO don't unwrap
				let ref_type = options.schema.types.get(type_name).unwrap();
				search(options, ref_type, current_field_name, tag, transform);
			},
			TypeData::Primitive(_) => {
				// TODO these should just be ignored, right?
			},
			TypeData::Rest => {
				// TODO do we even need Rest, or is it just DFU jank?
			}
		}
	}

	// Apply outer transformation after inner transformations in case the data tree structure changes,
	// since we iterate based on the old structure not the new structure
	if matches {
		transform.call((tag, ));
	}
}

macro_rules! s {
    ($s:expr) => { $s.to_string() }
}

fn on_find_tag(tag: &mut NbtTag) {
	println!("Found tag:\n {:?}", tag);
}

#[test]
fn test_search() {
	use CompoundField::{RequiredField as Req, OptionalField as Opt};
	use PrimitiveType::*;

	let chunk = compound(map![
		"Level": Req(compound(map![
			"Entities": Opt(list(reference(ENTITY_TREE))),
			"TileEntities": Opt(list(or([reference(BLOCK_ENTITY), rest()]))),
			"TileTicks": Opt(list(compound(map!["i": Req(reference(BLOCK_NAME))])))
		]))
	]);

	let entities = rest();
	let block_name = rest();
	let tile_entities = compound(map![
		"name": Req(primitive(String)),
		"inner": Opt(or([reference(BLOCK_ENTITY), list(reference(BLOCK_ENTITY))]))
	]);

	let schema = Schema {
		types: HashMap::from([
			(s!(CHUNK), chunk),
			(s!(ENTITY_TREE), entities),
			(s!(BLOCK_NAME), block_name),
			(s!(BLOCK_ENTITY), tile_entities)
		])
	};

	let test_data = r#"{
	"Level": {
		"Entities": [],
		"TileEntities": [
			{"name": "simple"},
			{"name": "nested_outer", "inner": {"name": "nested_inner"}},
			{"name": "nested_array_outer", "inner": [{"name": "nested_array_inner0"}, {"name": "nested_array_inner1"}]}
		]
	}
}
"#;
	let test_tag = NbtCompound::from_snbt(test_data).unwrap();

	let search_options = SearchOptions {
		schema: &schema,
		target_field_name: None,
		target_type_name: BLOCK_ENTITY,
		recurse_into_target: true
	};

	search(&search_options, &schema.types.get(CHUNK).unwrap(), "", &mut NbtTag::Compound(test_tag), on_find_tag)
}



use super::builtins::exports::*;
use super::errors::exports::*;
use super::parser::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub mod documentation_model {
		
		pub use super::super::{
				
				Libraries,
				Library,
				Category,
				
				Export,
				
				Definition,
				DefinitionKind,
				
				ValueKind,
				ValueKindPredicate,
				
				ProcedureSignature,
				ProcedureSignatureVariant,
				ProcedureSignatureValues,
				ProcedureSignatureValue,
				
				SyntaxSignature,
				SyntaxSignatureVariant,
				SyntaxSignatureKeyword,
				SyntaxSignaturePattern,
				
				Entity,
				EntityLink,
				
				Entities,
				EntitiesOwned,
				EntitiesLinked,
				
				Appendix,
				
				Description,
				Links,
				Link,
				
				Features,
				
		};
		
	}
	
	#[ cfg ( feature = "vonuvoli_documentation_sources" ) ]
	pub use super::parse_library_specifications_for_builtins;
	
	pub use super::parse_library_specifications;
	
}




pub trait Entity {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier (&self) -> (&str) {
		return self.identifier_rc_ref () .deref () .deref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_clone (&self) -> (StdString) {
		return StdString::from (self.identifier ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_clone (&self) -> (StdRc<StdBox<str>>) {
		return StdRc::clone (self.identifier_rc_ref ());
	}
	
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>);
}




pub struct EntityLink <E : Entity> {
	identifier : StdRc<StdBox<str>>,
	entity : StdRefCell<Option<StdRc<E>>>,
}


impl <E : Entity> EntityLink<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_linked (identifier : StdRc<StdBox<str>>) -> (EntityLink<E>) {
		return EntityLink {
				identifier,
				entity : StdRefCell::new (None),
			};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_resolved (entity : StdRc<E>) -> (EntityLink<E>) {
		let identifier = entity.identifier_rc_clone ();
		return EntityLink {
				identifier,
				entity : StdRefCell::new (Some (entity)),
			};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entity_link (&self, entity : StdRc<E>) -> (Outcome<()>) {
		let mut reference = try_or_fail! (self.entity.try_borrow_mut (), 0x8b92f9db);
		if let Some (ref current) = *reference {
			if StdRc::ptr_eq (current, &entity) {
				succeed! (());
			} else {
				fail! (0xa540c122);
			}
		}
		{
			*reference = Some (entity);
			succeed! (());
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entity_link_from (&self, entities : &impl Entities<E>) -> (Outcome<()>) {
		let entity = try! (entities.entity_resolve_clone (self.identifier.deref () .deref ()));
		return self.entity_link (entity);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entity_resolve (&self) -> (Outcome<&E>) {
		if let Some (entity) = try! (self.try_entity_resolve ()) {
			succeed! (entity);
		} else {
			fail! (0x6bb3118b);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try_entity_resolve (&self) -> (Outcome<Option<&E>>) {
		let reference = try_or_fail! (self.entity.try_borrow (), 0xf5fd3c1f);
		if let Some (ref reference) = reference.deref () {
			let reference : &E = reference.deref ();
			let reference = unsafe { mem::transmute ( reference ) };
			succeed! (Some (reference));
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entity_resolve_clone (&self) -> (Outcome<StdRc<E>>) {
		if let Some (entity) = try! (self.try_entity_resolve_clone ()) {
			succeed! (entity);
		} else {
			fail! (0xdc496032);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try_entity_resolve_clone (&self) -> (Outcome<Option<StdRc<E>>>) {
		let reference = try_or_fail! (self.entity.try_borrow (), 0xc69a4caa);
		succeed! (reference.clone ());
	}
}


impl <E : Entity> Entity for EntityLink<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl <E : Entity> ops::Deref for EntityLink<E> {
	
	type Target = E;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn deref (&self) -> (&E) {
		return try_or_panic! (self.entity_resolve ());
	}
}




pub trait Entities <E : Entity> {
	
	// fn entities (&self) -> (impl iter::ExactSizeIterator<Item = &E>);
	fn entity_resolve (&self, identifier : &str) -> (Outcome<&E>);
	fn try_entity_resolve (&self, identifier : &str) -> (Outcome<Option<&E>>);
	fn entity_resolve_clone (&self, identifier : &str) -> (Outcome<StdRc<E>>);
	fn try_entity_resolve_clone (&self, identifier : &str) -> (Outcome<Option<StdRc<E>>>);
	fn has_entities (&self) -> (bool);
}




pub struct EntitiesOwned <E : Entity> (cell::UnsafeCell<EntitiesOwnedInternals<E>>);

struct EntitiesOwnedInternals <E : Entity> {
	entities : StdVec<StdRc<E>>,
	entities_index : StdMap<StdString, StdRc<E>>,
}


impl <E : Entity> Entities<E> for EntitiesOwned<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_resolve (&self, identifier : &str) -> (Outcome<&E>) {
		if let Some (entity) = try! (self.try_entity_resolve (identifier)) {
			succeed! (entity);
		} else {
			fail! (0xe8128887);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_entity_resolve (&self, identifier : &str) -> (Outcome<Option<&E>>) {
		let self_0 = self.internals_ref ();
		succeed! (self_0.entities_index.get (identifier) .map (StdRc::deref));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_resolve_clone (&self, identifier : &str) -> (Outcome<StdRc<E>>) {
		if let Some (entity) = try! (self.try_entity_resolve_clone (identifier)) {
			succeed! (entity);
		} else {
			eprintln! ("{}", identifier);
			fail! (0xd37f0a3b);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_entity_resolve_clone (&self, identifier : &str) -> (Outcome<Option<StdRc<E>>>) {
		let self_0 = self.internals_ref ();
		succeed! (self_0.entities_index.get (identifier) .map (StdRc::clone));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn has_entities (&self) -> (bool) {
		let self_0 = self.internals_ref ();
		return ! self_0.entities.is_empty ();
	}
}


impl <E : Entity> EntitiesOwned<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entities (&self) -> (impl iter::ExactSizeIterator<Item = &E>) {
		let self_0 = self.internals_ref ();
		return self_0.entities.iter () .map (StdRc::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_include (&self, entity : StdRc<E>) -> (Outcome<()>) {
		let self_0 = self.internals_ref_mut ();
		match self_0.entities_index.entry (entity.identifier_clone ()) {
			StdMapEntry::Occupied (current) =>
				if StdRc::ptr_eq (current.get (), &entity) {
					succeed! (());
				} else {
					fail! (0x95ea3b1d);
				},
			StdMapEntry::Vacant (new) => {
				new.insert (StdRc::clone (&entity));
			},
		}
		self_0.entities.push (entity);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (entities : impl iter::IntoIterator<Item = E>) -> (Outcome<EntitiesOwned<E>>) {
		let entities = entities.into_iter () .map (StdRc::new) .collect::<StdVec<_>> ();
		let mut entities_index = StdMap::with_capacity (entities.len ());
		for entity in &entities {
			let identifier = entity.identifier_clone ();
			if entities_index.insert (identifier, StdRc::clone (entity)) .is_some () {
				fail! (0x8a2e7ff9);
			}
		}
		let entities = EntitiesOwnedInternals {
				entities,
				entities_index,
			};
		let entities = EntitiesOwned (cell::UnsafeCell::new (entities));
		succeed! (entities);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_empty () -> (EntitiesOwned<E>) {
		let entities = EntitiesOwnedInternals {
				entities : StdVec::new (),
				entities_index : StdMap::new (),
			};
		let entities = EntitiesOwned (cell::UnsafeCell::new (entities));
		return entities;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn internals_ref (&self) -> (&EntitiesOwnedInternals<E>) {
		return unsafe { &* self.0.get () };
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (mut_from_ref) ) ]
	fn internals_ref_mut (&self) -> (&mut EntitiesOwnedInternals<E>) {
		return unsafe { &mut * self.0.get () };
	}
}




pub struct EntitiesLinked <E : Entity> (cell::UnsafeCell<EntitiesLinkedInternals<E>>);

struct EntitiesLinkedInternals <E : Entity> {
	entities : StdVec<StdRc<EntityLink<E>>>,
	entities_index : StdMap<StdString, StdRc<EntityLink<E>>>,
}


impl <E : Entity> Entities<E> for EntitiesLinked<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_resolve (&self, identifier : &str) -> (Outcome<&E>) {
		if let Some (entity) = try! (self.try_entity_resolve (identifier)) {
			succeed! (entity);
		} else {
			fail! (0xe8128887);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_entity_resolve (&self, identifier : &str) -> (Outcome<Option<&E>>) {
		let self_0 = self.internals_ref ();
		let entity = if let Some (entity) = self_0.entities_index.get (identifier) {
			try! (entity.try_entity_resolve ())
		} else {
			None
		};
		succeed! (entity);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_resolve_clone (&self, identifier : &str) -> (Outcome<StdRc<E>>) {
		if let Some (entity) = try! (self.try_entity_resolve_clone (identifier)) {
			succeed! (entity);
		} else {
			eprintln! ("{}", identifier);
			fail! (0x77988359);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_entity_resolve_clone (&self, identifier : &str) -> (Outcome<Option<StdRc<E>>>) {
		let self_0 = self.internals_ref ();
		let entity = if let Some (entity) = self_0.entities_index.get (identifier) {
			try! (entity.try_entity_resolve_clone ())
		} else {
			None
		};
		succeed! (entity);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn has_entities (&self) -> (bool) {
		let self_0 = self.internals_ref ();
		return ! self_0.entities.is_empty ();
	}
}


impl <E : Entity> EntitiesLinked<E> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entities (&self) -> (impl iter::ExactSizeIterator<Item = &E>) {
		let self_0 = self.internals_ref ();
		return self_0.entities.iter () .map (StdRc::deref) .map (EntityLink::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_include_resolved (&self, entity : StdRc<E>) -> (Outcome<()>) {
		let entity = StdRc::new (EntityLink::new_resolved (entity));
		return self.entity_include (entity);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_include_linked (&self, identifier : StdRc<StdBox<str>>) -> (Outcome<()>) {
		let entity = StdRc::new (EntityLink::new_linked (identifier));
		return self.entity_include (entity);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entities_include_linked (&self, entities : &EntitiesLinked<E>) -> (Outcome<()>) {
		let entities = entities.internals_ref ();
		for entity in &entities.entities {
			try! (self.entity_include (StdRc::clone (entity)));
		}
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn entity_include (&self, entity : StdRc<EntityLink<E>>) -> (Outcome<()>) {
		let self_0 = self.internals_ref_mut ();
		match self_0.entities_index.entry (entity.identifier_clone ()) {
			StdMapEntry::Occupied (current) => {
				let current = current.get ();
				let current = try_or_fail! (current.entity.try_borrow (), 0x0d7ace7b);
				let entity = try_or_fail! (entity.entity.try_borrow (), 0x3323ece9);
				match (current.deref (), entity.deref ()) {
					(Some (current), Some (entity)) =>
						if StdRc::ptr_eq (current, entity) {
							succeed! (());
						} else {
							fail! (0xcc708d06);
						},
					(Some (_current), None) =>
						succeed! (()),
					(None, Some (_entity)) =>
						fail! (0xc33aae1d),
					(None, None) =>
						succeed! (()),
				}
			},
			StdMapEntry::Vacant (new) => {
				new.insert (StdRc::clone (&entity));
			},
		}
		self_0.entities.push (entity);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (identifiers : impl iter::IntoIterator<Item = StdRc<StdBox<str>>>) -> (Outcome<EntitiesLinked<E>>) {
		let links = identifiers.into_iter () .map (|identifier| StdRc::new (EntityLink::new_linked (identifier))) .collect::<StdVec<StdRc<EntityLink<E>>>> ();
		let mut links_index = StdMap::with_capacity (links.len ());
		for link in &links {
			let identifier = link.identifier_clone ();
			if links_index.insert (identifier, StdRc::clone (link)) .is_some () {
				fail! (0xe6bdf0d7);
			}
		}
		let entities = EntitiesLinkedInternals {
				entities : links,
				entities_index : links_index,
			};
		let entities = EntitiesLinked (cell::UnsafeCell::new (entities));
		succeed! (entities);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_empty () -> (EntitiesLinked<E>) {
		let entities = EntitiesLinkedInternals {
				entities : StdVec::new (),
				entities_index : StdMap::new (),
			};
		let entities = EntitiesLinked (cell::UnsafeCell::new (entities));
		return entities;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn entities_link_from (&self, entities : &impl Entities<E>) -> (Outcome<()>) {
		let self_0 = self.internals_ref ();
		for entity in &self_0.entities {
			try! (entity.entity_link_from (entities));
		}
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn internals_ref (&self) -> (&EntitiesLinkedInternals<E>) {
		return unsafe { &* self.0.get () };
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (mut_from_ref) ) ]
	fn internals_ref_mut (&self) -> (&mut EntitiesLinkedInternals<E>) {
		return unsafe { &mut * self.0.get () };
	}
}




pub struct Libraries {
	libraries : EntitiesOwned<Library>,
}


impl Libraries {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn libraries (&self) -> (impl iter::ExactSizeIterator<Item = &Library>) {
		return self.libraries.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn library_resolve (&self, identifier : &str) -> (Option<&Library>) {
		return try_or_panic! (self.libraries.try_entity_resolve (identifier));
	}
}




pub struct Library {
	
	identifier : StdRc<StdBox<str>>,
	
	categories : EntitiesOwned<Category>,
	
	exports : EntitiesOwned<Export>,
	definitions : EntitiesOwned<Definition>,
	definitions_all : StdMap<StdString, StdRc<Definition>>,
	value_kinds : EntitiesOwned<ValueKind>,
	value_kinds_all : StdMap<StdString, StdRc<ValueKind>>,
	
	title : Option<StdRc<StdBox<str>>>,
	description : Option<Description>,
	links : Option<Links>,
	
	appendices : EntitiesOwned<Appendix>,
	
	features : Option<Features>,
	
}


impl Entity for Library {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl Library {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.categories.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn category_resolve (&self, identifier : &str) -> (Option<&Category>) {
		return try_or_panic! (self.categories.try_entity_resolve (identifier));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_categories (&self) -> (bool) {
		return self.categories.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn exports (&self) -> (impl iter::ExactSizeIterator<Item = &Export>) {
		return self.exports.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn export_resolve (&self, identifier : &str) -> (Option<&Export>) {
		return try_or_panic! (self.exports.try_entity_resolve (identifier));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_exports (&self) -> (bool) {
		return self.exports.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definition_resolve (&self, identifier : &str) -> (Option<&Definition>) {
		return try_or_panic! (self.definitions.try_entity_resolve (identifier));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions (&self) -> (bool) {
		return self.definitions.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_all (&self) -> (impl iter::ExactSizeIterator<Item = (&str, &Definition)>) {
		return self.definitions_all.iter () .map (|(alias, entity)| (alias.deref (), entity.deref ()));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_kinds (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.value_kinds.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_kind_resolve (&self, identifier : &str) -> (Option<&ValueKind>) {
		return try_or_panic! (self.value_kinds.try_entity_resolve (identifier));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_value_kinds (&self) -> (bool) {
		return self.value_kinds.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_kinds_all (&self) -> (impl iter::ExactSizeIterator<Item = (&str, &ValueKind)>) {
		return self.value_kinds_all.iter () .map (|(alias, entity)| (alias.deref (), entity.deref ()));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn title (&self) -> (Option<&str>) {
		return self.title.as_ref () .map (StdRc::deref) .map (StdBox::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn appendices (&self) -> (impl iter::ExactSizeIterator<Item = &Appendix>) {
		return self.appendices.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn appendix_resolve (&self, identifier : &str) -> (Option<&Appendix>) {
		return try_or_panic! (self.appendices.try_entity_resolve (identifier));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_appendices (&self) -> (bool) {
		return self.appendices.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn features (&self) -> (Option<&Features>) {
		return self.features.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
	fn link (self) -> (Outcome<Library>) {
		
		let mut library = self;
		
		for category in library.categories.entities () {
			try! (category.link (&library));
		}
		for export in library.exports.entities () {
			try! (export.link (&library));
		}
		for definition in library.definitions.entities () {
			try! (definition.link (&library));
		}
		for value_kind in library.value_kinds.entities () {
			try! (value_kind.link (&library));
		}
		
		let categories = mem::replace (&mut library.categories, EntitiesOwned::new_empty ());
		let exports = mem::replace (&mut library.exports, EntitiesOwned::new_empty ());
		let definitions = mem::replace (&mut library.definitions, EntitiesOwned::new_empty ());
		let mut definitions_all = mem::replace (&mut library.definitions_all, StdMap::new ());
		let value_kinds = mem::replace (&mut library.value_kinds, EntitiesOwned::new_empty ());
		let mut value_kinds_all = mem::replace (&mut library.value_kinds_all, StdMap::new ());
		
		for category in categories.entities () {
			let category_rc = try! (categories.entity_resolve_clone (category.identifier ()));
			let category = category_rc.deref ();
			for parent in category.parents.entities () {
				let parent_rc = try! (categories.entity_resolve_clone (parent.identifier ()));
				let parent = parent_rc.deref ();
				try! (parent.children.entity_include_resolved (StdRc::clone (&category_rc)));
			}
			{
				#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
				fn walk <'a> (category : &Category, category_rc : &StdRc<Category>, categories : &EntitiesOwned<Category>, parents : impl iter::Iterator<Item = &'a Category>) -> (Outcome<()>) {
					for parent in parents {
						let parent_rc = try! (categories.entity_resolve_clone (parent.identifier ()));
						let parent = parent_rc.deref ();
						try! (parent.children_all.entity_include_resolved (StdRc::clone (category_rc)));
						try! (category.parents_all.entity_include_resolved (StdRc::clone (&parent_rc)));
						try! (walk (category, category_rc, categories, parent.parents.entities ()));
					}
					succeed! (());
				};
				try! (walk (category, &category_rc, &categories, category.parents.entities ()));
			}
		}
		
		for export in exports.entities () {
			let export_rc = try! (exports.entity_resolve_clone (export.identifier ()));
			let export = export_rc.deref ();
			for parent in export.parents.entities () {
				let parent_rc = try! (exports.entity_resolve_clone (parent.identifier ()));
				let parent = parent_rc.deref ();
				try! (parent.children.entity_include_resolved (StdRc::clone (&export_rc)));
			}
			{
				#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
				fn walk <'a> (export : &Export, export_rc : &StdRc<Export>, exports : &EntitiesOwned<Export>, parents : impl iter::Iterator<Item = &'a Export>) -> (Outcome<()>) {
					for parent in parents {
						let parent_rc = try! (exports.entity_resolve_clone (parent.identifier ()));
						let parent = parent_rc.deref ();
						try! (parent.children_all.entity_include_resolved (StdRc::clone (export_rc)));
						try! (export.parents_all.entity_include_resolved (StdRc::clone (&parent_rc)));
						try! (walk (export, export_rc, exports, parent.parents.entities ()));
					}
					succeed! (());
				};
				try! (walk (export, &export_rc, &exports, export.parents.entities ()));
			}
			for category in export.categories.entities () {
				{
					let category_rc = try! (categories.entity_resolve_clone (category.identifier ()));
					let category = category_rc.deref ();
					try! (category.exports.entity_include_resolved (StdRc::clone (&export_rc)));
					try! (category.exports_all.entity_include_resolved (StdRc::clone (&export_rc)));
					try! (export.categories_all.entity_include_resolved (StdRc::clone (&category_rc)));
				}
				for category in category.parents_all.entities () {
					let category_rc = try! (categories.entity_resolve_clone (category.identifier ()));
					let category = category_rc.deref ();
					try! (category.exports_all.entity_include_resolved (StdRc::clone (&export_rc)));
					try! (export.categories_all.entity_include_resolved (StdRc::clone (&category_rc)));
				}
			}
		}
		
		for value_kind in value_kinds.entities () {
			let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
			let value_kind = value_kind_rc.deref ();
			// NOTE:  We already have child-parents relations.
			// NOTE:  Initialize direct parent-children relations.
			for parent in value_kind.parents.entities () {
				let parent_rc = try! (value_kinds.entity_resolve_clone (parent.identifier ()));
				let parent = parent_rc.deref ();
				try! (parent.children.entity_include_resolved (StdRc::clone (&value_kind_rc)));
			}
			// NOTE:  Copy covariant-for to direct covariants.
			for covariant in value_kind.covariants_for.entities () {
				let covariant_rc = try! (value_kinds.entity_resolve_clone (covariant.identifier ()));
				let covariant = covariant_rc.deref ();
				try! (covariant.covariants.entity_include_resolved (StdRc::clone (&value_kind_rc)));
			}
			// NOTE:  Copy contravariant-for to direct contravariants.
			for contravariant in value_kind.contravariants_for.entities () {
				let contravariant_rc = try! (value_kinds.entity_resolve_clone (contravariant.identifier ()));
				let contravariant = contravariant_rc.deref ();
				try! (contravariant.contravariants.entity_include_resolved (StdRc::clone (&value_kind_rc)));
			}
		}
		for value_kind in value_kinds.entities () {
			let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
			let value_kind = value_kind_rc.deref ();
			// NOTE:  Recurse over parents to establish parent-children and child-parents relations.
			{
				#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
				fn walk <'a> (value_kind : &ValueKind, value_kind_rc : &StdRc<ValueKind>, value_kinds : &EntitiesOwned<ValueKind>, parents : impl iter::Iterator<Item = &'a ValueKind>) -> (Outcome<()>) {
					for parent in parents {
						let parent_rc = try! (value_kinds.entity_resolve_clone (parent.identifier ()));
						let parent = parent_rc.deref ();
						try! (value_kind.parents_all.entity_include_resolved (StdRc::clone (&parent_rc)));
						try! (parent.children_all.entity_include_resolved (StdRc::clone (value_kind_rc)));
						try! (walk (value_kind, value_kind_rc, value_kinds, parent.parents.entities ()));
					}
					succeed! (());
				};
				try! (walk (value_kind, &value_kind_rc, &value_kinds, value_kind.parents.entities ()));
			}
		}
		for value_kind in value_kinds.entities () {
			let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
			let value_kind = value_kind_rc.deref ();
			// NOTE:  Initialize recursive covariants.
			for covariant in value_kind.covariants.entities () {
				let covariant_rc = try! (value_kinds.entity_resolve_clone (covariant.identifier ()));
				try! (value_kind.covariants_all.entity_include_resolved (StdRc::clone (&covariant_rc)));
			}
			// NOTE:  Initialize recursive contravariants.
			for contravariant in value_kind.contravariants.entities () {
				let contravariant_rc = try! (value_kinds.entity_resolve_clone (contravariant.identifier ()));
				try! (value_kind.contravariants_all.entity_include_resolved (StdRc::clone (&contravariant_rc)));
			}
		}
		for value_kind in value_kinds.entities () {
			let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
			let value_kind = value_kind_rc.deref ();
			// NOTE:  Augment recursive covariants and contravariants from parents (and their covariants and contravariants).
			for parent in value_kind.parents_all.entities () {
				let parent_rc = try! (value_kinds.entity_resolve_clone (parent.identifier ()));
				let parent = parent_rc.deref ();
				try! (value_kind.covariants_all.entity_include_resolved (StdRc::clone (&parent_rc)));
				try! (value_kind.covariants_all.entities_include_linked (&parent.covariants_all));
				try! (parent.contravariants_all.entity_include_resolved (StdRc::clone (&value_kind_rc)));
				try! (parent.contravariants_all.entities_include_linked (&value_kind.contravariants_all));
			}
			// NOTE:  Augment recursive covariants and contravariants from children (and their covariants and contravariants).
			for child in value_kind.children_all.entities () {
				let child_rc = try! (value_kinds.entity_resolve_clone (child.identifier ()));
				let child = child_rc.deref ();
				try! (value_kind.contravariants_all.entity_include_resolved (StdRc::clone (&child_rc)));
				try! (value_kind.contravariants_all.entities_include_linked (&child.contravariants_all));
				try! (child.covariants_all.entity_include_resolved (StdRc::clone (&value_kind_rc)));
				try! (child.covariants_all.entities_include_linked (&value_kind.covariants_all));
			}
		}
		for value_kind in value_kinds.entities () {
			let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
			let value_kind = value_kind_rc.deref ();
			// NOTE:  Recurse over covariant relations.
			{
				#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
				fn walk <'a> (value_kind : &ValueKind, value_kind_rc : &StdRc<ValueKind>, value_kinds : &EntitiesOwned<ValueKind>, covariants : impl iter::Iterator<Item = &'a ValueKind>) -> (Outcome<()>) {
					for covariant in covariants {
						let covariant_rc = try! (value_kinds.entity_resolve_clone (covariant.identifier ()));
						let covariant = covariant_rc.deref ();
						try! (value_kind.covariants_all.entity_include_resolved (StdRc::clone (&covariant_rc)));
						try! (walk (value_kind, value_kind_rc, value_kinds, covariant.covariants_all.entities ()));
					}
					succeed! (());
				};
				try! (walk (value_kind, &value_kind_rc, &value_kinds, value_kind.covariants_all.entities () .collect::<StdVec<_>> () .into_iter ()));
			}
			// NOTE:  Recurse over contravariant relations.
			{
				#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
				fn walk <'a> (value_kind : &ValueKind, value_kind_rc : &StdRc<ValueKind>, value_kinds : &EntitiesOwned<ValueKind>, contravariants : impl iter::Iterator<Item = &'a ValueKind>) -> (Outcome<()>) {
					for contravariant in contravariants {
						let contravariant_rc = try! (value_kinds.entity_resolve_clone (contravariant.identifier ()));
						let contravariant = contravariant_rc.deref ();
						try! (value_kind.contravariants_all.entity_include_resolved (StdRc::clone (&contravariant_rc)));
						try! (walk (value_kind, value_kind_rc, value_kinds, contravariant.contravariants_all.entities ()));
					}
					succeed! (());
				};
				try! (walk (value_kind, &value_kind_rc, &value_kinds, value_kind.contravariants_all.entities () .collect::<StdVec<_>> () .into_iter ()));
			}
		}
		for value_kind in value_kinds.entities () {
			let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
			let value_kind = value_kind_rc.deref ();
			if value_kinds_all.insert (value_kind.identifier_clone (), StdRc::clone (&value_kind_rc)) .is_some () {
				fail! (0xde87379f);
			}
			for alias in &value_kind.aliases {
				if value_kinds_all.insert (StdString::from (alias.deref () .deref ()), StdRc::clone (&value_kind_rc)) .is_some () {
					fail! (0x42f7f808);
				}
			}
			for category in value_kind.categories.entities () {
				{
					let category_rc = try! (categories.entity_resolve_clone (category.identifier ()));
					let category = category_rc.deref ();
					try! (category.value_kinds.entity_include_resolved (StdRc::clone (&value_kind_rc)));
					try! (category.value_kinds_all.entity_include_resolved (StdRc::clone (&value_kind_rc)));
					try! (value_kind.categories_all.entity_include_resolved (StdRc::clone (&category_rc)));
				}
				for category in category.parents_all.entities () {
					let category_rc = try! (categories.entity_resolve_clone (category.identifier ()));
					let category = category_rc.deref ();
					try! (category.value_kinds_all.entity_include_resolved (StdRc::clone (&value_kind_rc)));
					try! (value_kind.categories_all.entity_include_resolved (StdRc::clone (&category_rc)));
				}
			}
		}
		
		for definition in definitions.entities () {
			let definition_rc = try! (definitions.entity_resolve_clone (definition.identifier ()));
			let definition = definition_rc.deref ();
			if definitions_all.insert (definition.identifier_clone (), StdRc::clone (&definition_rc)) .is_some () {
				fail! (0x38d906bc);
			}
			for alias in &definition.aliases {
				if definitions_all.insert (StdString::from (alias.deref () .deref ()), StdRc::clone (&definition_rc)) .is_some () {
					fail! (0xd60c3f11);
				}
			}
			for category in definition.categories.entities () {
				{
					let category_rc = try! (categories.entity_resolve_clone (category.identifier ()));
					let category = category_rc.deref ();
					try! (category.definitions.entity_include_resolved (StdRc::clone (&definition_rc)));
					try! (category.definitions_all.entity_include_resolved (StdRc::clone (&definition_rc)));
					try! (definition.categories_all.entity_include_resolved (StdRc::clone (&category_rc)));
				}
				for category in category.parents_all.entities () {
					let category_rc = try! (categories.entity_resolve_clone (category.identifier ()));
					let category = category_rc.deref ();
					try! (category.definitions_all.entity_include_resolved (StdRc::clone (&definition_rc)));
					try! (definition.categories_all.entity_include_resolved (StdRc::clone (&category_rc)));
				}
			}
			for export in definition.exports.entities () {
				{
					let export_rc = try! (exports.entity_resolve_clone (export.identifier ()));
					let export = export_rc.deref ();
					try! (export.definitions.entity_include_resolved (StdRc::clone (&definition_rc)));
					try! (export.definitions_all.entity_include_resolved (StdRc::clone (&definition_rc)));
					try! (definition.exports_all.entity_include_resolved (StdRc::clone (&export_rc)));
				}
				for export in export.parents_all.entities () {
					let export_rc = try! (exports.entity_resolve_clone (export.identifier ()));
					let export = export_rc.deref ();
					try! (export.definitions_all.entity_include_resolved (StdRc::clone (&definition_rc)));
					try! (definition.exports_all.entity_include_resolved (StdRc::clone (&export_rc)));
				}
			}
			if let Some (procedure_signature) = &definition.procedure_signature {
				for procedure_signature_variant in procedure_signature.variants.iter () {
					for procedure_signature_value in procedure_signature_variant.inputs.values.iter () {
						let value_kind = try! (procedure_signature_value.kind.entity_resolve_clone ());
						{
							let value_kind = value_kind.deref ();
							try! (value_kind.definitions_input.entity_include_resolved (StdRc::clone (&definition_rc)));
							try! (value_kind.definitions_input_all.entity_include_resolved (StdRc::clone (&definition_rc)));
							try! (value_kind.definitions_input_all_2.entity_include_resolved (StdRc::clone (&definition_rc)));
						}
						{
							try! (definition.referenced_value_kinds.entity_include_resolved (value_kind));
						}
					}
					for procedure_signature_value in procedure_signature_variant.outputs.values.iter () {
						let value_kind = try! (procedure_signature_value.kind.entity_resolve_clone ());
						{
							let value_kind = value_kind.deref ();
							try! (value_kind.definitions_output.entity_include_resolved (StdRc::clone (&definition_rc)));
							try! (value_kind.definitions_output_all.entity_include_resolved (StdRc::clone (&definition_rc)));
							try! (value_kind.definitions_output_all_2.entity_include_resolved (StdRc::clone (&definition_rc)));
						}
						{
							try! (definition.referenced_value_kinds.entity_include_resolved (value_kind));
						}
					}
				}
			}
			if let Some (syntax_signature) = &definition.syntax_signature {
				for syntax_signature_keyword in syntax_signature.keywords.iter () {
					match syntax_signature_keyword.deref () {
						SyntaxSignatureKeyword::Value { kind : Some (value_kind), .. } => {
							let value_kind = try! (value_kind.entity_resolve_clone ());
							{
								let value_kind = value_kind.deref ();
								try! (value_kind.definitions_input.entity_include_resolved (StdRc::clone (&definition_rc)));
								try! (value_kind.definitions_input_all.entity_include_resolved (StdRc::clone (&definition_rc)));
								try! (value_kind.definitions_input_all_2.entity_include_resolved (StdRc::clone (&definition_rc)));
							}
							{
								try! (definition.referenced_value_kinds.entity_include_resolved (value_kind));
							}
						}
						_ =>
							(),
					}
				}
			}
		}
		
		for value_kind in value_kinds.entities () {
			let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
			let value_kind = value_kind_rc.deref ();
			for definition in value_kind.definitions_input.entities () {
				let definition_rc = try! (definitions.entity_resolve_clone (definition.identifier ()));
				for value_kind in value_kind.children_all.entities () {
					let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
					let value_kind = value_kind_rc.deref ();
					try! (value_kind.definitions_input_all.entity_include_resolved (StdRc::clone (&definition_rc)));
				}
				for value_kind in value_kind.contravariants_all.entities () {
					let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
					let value_kind = value_kind_rc.deref ();
					try! (value_kind.definitions_input_all_2.entity_include_resolved (StdRc::clone (&definition_rc)));
				}
			}
			for definition in value_kind.definitions_output.entities () {
				let definition_rc = try! (definitions.entity_resolve_clone (definition.identifier ()));
				for value_kind in value_kind.parents_all.entities () {
					let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
					let value_kind = value_kind_rc.deref ();
					try! (value_kind.definitions_output_all.entity_include_resolved (StdRc::clone (&definition_rc)));
				}
				for value_kind in value_kind.covariants_all.entities () {
					let value_kind_rc = try! (value_kinds.entity_resolve_clone (value_kind.identifier ()));
					let value_kind = value_kind_rc.deref ();
					try! (value_kind.definitions_output_all_2.entity_include_resolved (StdRc::clone (&definition_rc)));
				}
			}
		}
		
		library.categories = categories;
		library.exports = exports;
		library.definitions = definitions;
		library.definitions_all = definitions_all;
		library.value_kinds = value_kinds;
		library.value_kinds_all = value_kinds_all;
		
		for category in library.categories.entities () {
			try! (category.link (&library));
		}
		for exports in library.exports.entities () {
			try! (exports.link (&library));
		}
		for definition in library.definitions.entities () {
			try! (definition.link (&library));
		}
		for value_kind in library.value_kinds.entities () {
			try! (value_kind.link (&library));
		}
		
		succeed! (library);
	}
}




pub struct Category {
	
	identifier : StdRc<StdBox<str>>,
	
	parents : EntitiesLinked<Category>,
	parents_all : EntitiesLinked<Category>,
	children : EntitiesLinked<Category>,
	children_all : EntitiesLinked<Category>,
	
	exports : EntitiesLinked<Export>,
	exports_all : EntitiesLinked<Export>,
	definitions : EntitiesLinked<Definition>,
	definitions_all : EntitiesLinked<Definition>,
	value_kinds : EntitiesLinked<ValueKind>,
	value_kinds_all : EntitiesLinked<ValueKind>,
	
	description : Option<Description>,
	links : Option<Links>,
	
}


impl Entity for Category {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl Category {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.parents.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.parents_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parents (&self) -> (bool) {
		return self.parents.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.children.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.children_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_children (&self) -> (bool) {
		return self.children.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn exports (&self) -> (impl iter::ExactSizeIterator<Item = &Export>) {
		return self.exports.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn exports_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Export>) {
		return self.exports_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_exports (&self) -> (bool) {
		return self.exports.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions (&self) -> (bool) {
		return self.definitions.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_kinds (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.value_kinds.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_kinds_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.value_kinds_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_value_kinds (&self) -> (bool) {
		return self.value_kinds.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		try! (self.parents.entities_link_from (&library.categories));
		try! (self.parents_all.entities_link_from (&library.categories));
		try! (self.children.entities_link_from (&library.categories));
		try! (self.children_all.entities_link_from (&library.categories));
		try! (self.exports.entities_link_from (&library.exports));
		try! (self.exports_all.entities_link_from (&library.exports));
		try! (self.definitions.entities_link_from (&library.definitions));
		try! (self.definitions_all.entities_link_from (&library.definitions));
		try! (self.value_kinds.entities_link_from (&library.value_kinds));
		try! (self.value_kinds_all.entities_link_from (&library.value_kinds));
		succeed! (());
	}
}




pub struct Export {
	
	identifier : StdRc<StdBox<str>>,
	
	parents : EntitiesLinked<Export>,
	parents_all : EntitiesLinked<Export>,
	children : EntitiesLinked<Export>,
	children_all : EntitiesLinked<Export>,
	
	categories : EntitiesLinked<Category>,
	categories_all : EntitiesLinked<Category>,
	
	description : Option<Description>,
	links : Option<Links>,
	
	descriptor : Value,
	
	definitions : EntitiesLinked<Definition>,
	definitions_all : EntitiesLinked<Definition>,
	
	features : Option<Features>,
	
}


impl Entity for Export {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl Export {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents (&self) -> (impl iter::ExactSizeIterator<Item = &Export>) {
		return self.parents.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Export>) {
		return self.parents_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parents (&self) -> (bool) {
		return self.parents.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children (&self) -> (impl iter::ExactSizeIterator<Item = &Export>) {
		return self.children.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Export>) {
		return self.children_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_children (&self) -> (bool) {
		return self.children.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.categories.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.categories_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_categories (&self) -> (bool) {
		return self.categories.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn descriptor_format (&self) -> (Value) {
		return self.descriptor.clone ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions (&self) -> (bool) {
		return self.definitions.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn features (&self) -> (Option<&Features>) {
		return self.features.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		try! (self.parents.entities_link_from (&library.exports));
		try! (self.parents_all.entities_link_from (&library.exports));
		try! (self.children.entities_link_from (&library.exports));
		try! (self.children_all.entities_link_from (&library.exports));
		try! (self.categories.entities_link_from (&library.categories));
		try! (self.categories_all.entities_link_from (&library.categories));
		try! (self.definitions.entities_link_from (&library.definitions));
		try! (self.definitions_all.entities_link_from (&library.definitions));
		succeed! (());
	}
}




pub struct Definition {
	
	identifier : StdRc<StdBox<str>>,
	
	kind : DefinitionKind,
	categories : EntitiesLinked<Category>,
	categories_all : EntitiesLinked<Category>,
	
	aliases : StdVec<StdRc<StdBox<str>>>,
	
	description : Option<Description>,
	links : Option<Links>,
	
	procedure_signature : Option<ProcedureSignature>,
	syntax_signature : Option<SyntaxSignature>,
	
	referenced_value_kinds : EntitiesLinked<ValueKind>,
	
	exports : EntitiesLinked<Export>,
	exports_all : EntitiesLinked<Export>,
	
	features : Option<Features>,
	
}


impl Entity for Definition {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl Definition {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn kind (&self) -> (&DefinitionKind) {
		return &self.kind;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.categories.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.categories_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_categories (&self) -> (bool) {
		return self.categories.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn exports (&self) -> (impl iter::ExactSizeIterator<Item = &Export>) {
		return self.exports.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn exports_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Export>) {
		return self.exports_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_exports (&self) -> (bool) {
		return self.exports.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn aliases (&self) -> (impl iter::ExactSizeIterator<Item = &str>) {
		return self.aliases.iter () .map (StdRc::deref) .map (StdBox::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_aliases (&self) -> (bool) {
		return ! self.aliases.is_empty ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn procedure_signature (&self) -> (Option<&ProcedureSignature>) {
		return self.procedure_signature.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn syntax_signature (&self) -> (Option<&SyntaxSignature>) {
		return self.syntax_signature.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn referenced_value_kinds (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.referenced_value_kinds.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_referenced_value_kinds (&self) -> (bool) {
		return self.referenced_value_kinds.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn features (&self) -> (Option<&Features>) {
		return self.features.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		try! (self.categories.entities_link_from (&library.categories));
		if let Some (ref procedure_signature) = self.procedure_signature {
			try! (procedure_signature.link (&library.value_kinds));
		}
		if let Some (ref syntax_signature) = self.syntax_signature {
			try! (syntax_signature.link (&library.value_kinds));
		}
		try! (self.referenced_value_kinds.entities_link_from (&library.value_kinds));
		try! (self.exports.entities_link_from (&library.exports));
		try! (self.exports_all.entities_link_from (&library.exports));
		for alias in &self.aliases {
			if try! (library.definitions.try_entity_resolve (alias)) .is_some () {
				fail! (0x73f2e1e7);
			}
		}
		succeed! (());
	}
}




#[ derive ( Copy, Clone ) ] // OK
pub enum DefinitionKind {
	
	Syntax,
	SyntaxAuxiliary,
	
	Procedure,
	ProcedureWithMutation,
	
	Predicate,
	TypePredicate,
	
	Comparator,
	
	ValueConstructor,
	ValueConverter,
	ValueAccessor,
	ValueMutator,
	ValueConstant,
	
	Parameter,
	
	Functor,
	
}


impl DefinitionKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn identifier (self) -> (&'static str) {
		return match self {
			
			DefinitionKind::Syntax => "syntax",
			DefinitionKind::SyntaxAuxiliary => "auxiliary-syntax",
			
			DefinitionKind::Procedure => "procedure",
			DefinitionKind::ProcedureWithMutation => "procedure!",
			
			DefinitionKind::Predicate => "predicate",
			DefinitionKind::TypePredicate => "type-predicate",
			
			DefinitionKind::Comparator => "comparator",
			
			DefinitionKind::ValueConstructor => "constructor",
			DefinitionKind::ValueConverter => "converter",
			DefinitionKind::ValueAccessor => "accessor",
			DefinitionKind::ValueMutator => "mutator!",
			DefinitionKind::ValueConstant => "constant",
			
			DefinitionKind::Parameter => "parameter",
			
			DefinitionKind::Functor => "functor",
			
		};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parent (self) -> (Option<DefinitionKind>) {
		return match self {
			
			DefinitionKind::Syntax => None,
			DefinitionKind::SyntaxAuxiliary => None,
			
			DefinitionKind::Procedure => None,
			DefinitionKind::ProcedureWithMutation => Some (DefinitionKind::Procedure),
			
			DefinitionKind::Predicate => Some (DefinitionKind::Procedure),
			DefinitionKind::TypePredicate => Some (DefinitionKind::Predicate),
			
			DefinitionKind::Comparator => Some (DefinitionKind::Predicate),
			
			DefinitionKind::ValueConstructor => Some (DefinitionKind::Procedure),
			DefinitionKind::ValueConverter => Some (DefinitionKind::Procedure),
			DefinitionKind::ValueAccessor => Some (DefinitionKind::Procedure),
			DefinitionKind::ValueMutator => Some (DefinitionKind::ProcedureWithMutation),
			DefinitionKind::ValueConstant => None,
			
			DefinitionKind::Parameter => Some (DefinitionKind::ValueConstant),
			
			DefinitionKind::Functor => Some (DefinitionKind::Procedure),
			
		};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_procedure (self) -> (bool) {
		return match self {
			
			DefinitionKind::Syntax => false,
			DefinitionKind::SyntaxAuxiliary => false,
			
			DefinitionKind::Procedure => true,
			DefinitionKind::ProcedureWithMutation => true,
			
			DefinitionKind::Predicate => true,
			DefinitionKind::TypePredicate => true,
			
			DefinitionKind::Comparator => true,
			
			DefinitionKind::ValueConstructor => true,
			DefinitionKind::ValueConverter => true,
			DefinitionKind::ValueAccessor => true,
			DefinitionKind::ValueMutator => true,
			DefinitionKind::ValueConstant => true,
			
			DefinitionKind::Parameter => true,
			
			DefinitionKind::Functor => true,
			
		};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_syntax (self) -> (bool) {
		return match self {
			
			DefinitionKind::Syntax => true,
			DefinitionKind::SyntaxAuxiliary => false,
			
			DefinitionKind::Procedure => false,
			DefinitionKind::ProcedureWithMutation => false,
			
			DefinitionKind::Predicate => false,
			DefinitionKind::TypePredicate => false,
			
			DefinitionKind::Comparator => false,
			
			DefinitionKind::ValueConstructor => false,
			DefinitionKind::ValueConverter => false,
			DefinitionKind::ValueAccessor => false,
			DefinitionKind::ValueMutator => false,
			DefinitionKind::ValueConstant => false,
			
			DefinitionKind::Parameter => false,
			
			DefinitionKind::Functor => false,
			
		};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents_recursive (self) -> (impl iter::Iterator<Item = DefinitionKind>) {
		struct Parents (Option<DefinitionKind>);
		impl iter::Iterator for Parents {
			type Item = DefinitionKind;
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			fn next (&mut self) -> (Option<DefinitionKind>) {
				if let Some (current) = self.0 {
					let parent = current.parent ();
					self.0 = parent;
				}
				return self.0;
			}
		}
		return Parents (Some (self));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parent (self) -> (bool) {
		return self.parent () .is_some ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve (identifier : &str) -> (Outcome<DefinitionKind>) {
		succeed! (match identifier {
			
			"syntax" =>
				DefinitionKind::Syntax,
			"auxiliary-syntax" =>
				DefinitionKind::SyntaxAuxiliary,
			
			"procedure" =>
				DefinitionKind::Procedure,
			"procedure!" =>
				DefinitionKind::ProcedureWithMutation,
			
			"predicate" =>
				DefinitionKind::Predicate,
			"type-predicate" =>
				DefinitionKind::TypePredicate,
			
			"comparator" |
			"comparator=" |
			"comparator<" |
			"comparator>" |
			"comparator<=" |
			"comparator>=" =>
				DefinitionKind::Comparator,
			
			"constructor" =>
				DefinitionKind::ValueConstructor,
			"converter" =>
				DefinitionKind::ValueConverter,
			"accessor" =>
				DefinitionKind::ValueAccessor,
			"mutator!" =>
				DefinitionKind::ValueMutator,
			"constant" =>
				DefinitionKind::ValueConstant,
			
			"map" |
			"for-each" |
			"fold" =>
				DefinitionKind::Functor,
			
			"parameter" =>
				DefinitionKind::Parameter,
			
			_ =>
				fail! (0x417c9e8a),
			
		});
	}
}




pub struct ValueKind {
	
	identifier : StdRc<StdBox<str>>,
	
	parents : EntitiesLinked<ValueKind>,
	parents_all : EntitiesLinked<ValueKind>,
	children : EntitiesLinked<ValueKind>,
	children_all : EntitiesLinked<ValueKind>,
	
	categories : EntitiesLinked<Category>,
	categories_all : EntitiesLinked<Category>,
	
	aliases : StdVec<StdRc<StdBox<str>>>,
	
	description : Option<Description>,
	links : Option<Links>,
	
	predicate : Option<ValueKindPredicate>,
	
	features : Option<Features>,
	
	covariants : EntitiesLinked<ValueKind>,
	covariants_for : EntitiesLinked<ValueKind>,
	covariants_all : EntitiesLinked<ValueKind>,
	contravariants : EntitiesLinked<ValueKind>,
	contravariants_for : EntitiesLinked<ValueKind>,
	contravariants_all : EntitiesLinked<ValueKind>,
	
	definitions_input : EntitiesLinked<Definition>,
	definitions_input_all : EntitiesLinked<Definition>,
	definitions_input_all_2 : EntitiesLinked<Definition>,
	definitions_output : EntitiesLinked<Definition>,
	definitions_output_all : EntitiesLinked<Definition>,
	definitions_output_all_2 : EntitiesLinked<Definition>,
	
}


impl Entity for ValueKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl ValueKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.parents.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parents_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.parents_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_parents (&self) -> (bool) {
		return self.parents.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.children.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn children_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.children_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_children (&self) -> (bool) {
		return self.children.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.categories.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn categories_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Category>) {
		return self.categories_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_categories (&self) -> (bool) {
		return self.categories.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn aliases (&self) -> (impl iter::ExactSizeIterator<Item = &str>) {
		return self.aliases.iter () .map (StdRc::deref) .map (StdBox::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_aliases (&self) -> (bool) {
		return ! self.aliases.is_empty ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn predicate (&self) -> (Option<&ValueKindPredicate>) {
		return self.predicate.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn features (&self) -> (Option<&Features>) {
		return self.features.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn covariants (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.covariants.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn covariants_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.covariants_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_covariants (&self) -> (bool) {
		return self.covariants.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn contravariants (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.contravariants.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn contravariants_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &ValueKind>) {
		return self.contravariants_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_contravariants (&self) -> (bool) {
		return self.contravariants.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_input (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions_input.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_input_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions_input_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_input_contravariant_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions_input_all_2.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions_input (&self) -> (bool) {
		return self.definitions_input.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_output (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions_output.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_output_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions_output_all.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn definitions_output_covariant_recursive (&self) -> (impl iter::ExactSizeIterator<Item = &Definition>) {
		return self.definitions_output_all_2.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_definitions_output (&self) -> (bool) {
		return self.definitions_output.has_entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, library : &Library) -> (Outcome<()>) {
		try! (self.parents.entities_link_from (&library.value_kinds));
		try! (self.parents_all.entities_link_from (&library.value_kinds));
		try! (self.children.entities_link_from (&library.value_kinds));
		try! (self.children_all.entities_link_from (&library.value_kinds));
		try! (self.categories.entities_link_from (&library.categories));
		try! (self.categories_all.entities_link_from (&library.categories));
		try! (self.covariants.entities_link_from (&library.value_kinds));
		try! (self.covariants_for.entities_link_from (&library.value_kinds));
		try! (self.covariants_all.entities_link_from (&library.value_kinds));
		try! (self.contravariants.entities_link_from (&library.value_kinds));
		try! (self.contravariants_for.entities_link_from (&library.value_kinds));
		try! (self.contravariants_all.entities_link_from (&library.value_kinds));
		try! (self.definitions_input.entities_link_from (&library.definitions));
		try! (self.definitions_input_all.entities_link_from (&library.definitions));
		try! (self.definitions_input_all_2.entities_link_from (&library.definitions));
		try! (self.definitions_output.entities_link_from (&library.definitions));
		try! (self.definitions_output_all.entities_link_from (&library.definitions));
		try! (self.definitions_output_all_2.entities_link_from (&library.definitions));
		for alias in &self.aliases {
			if try! (library.value_kinds.try_entity_resolve (alias)) .is_some () {
				fail! (0x12252744);
			}
		}
		succeed! (());
	}
}




pub enum ValueKindPredicate {
	None,
	Inherit,
	Fixme,
	Expression (Value),
}


impl ValueKindPredicate {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		match self {
			ValueKindPredicate::None =>
				symbol_clone_str ("none") .into (),
			ValueKindPredicate::Inherit =>
				symbol_clone_str ("inherit") .into (),
			ValueKindPredicate::Fixme =>
				symbol_clone_str ("fixme!") .into (),
			ValueKindPredicate::Expression (value) =>
				value.clone (),
		}
	}
}




pub struct ProcedureSignature {
	pub variants : StdBox<[ProcedureSignatureVariant]>,
}

pub struct ProcedureSignatureVariant {
	pub inputs : ProcedureSignatureValues,
	pub outputs : ProcedureSignatureValues,
	pub features : Option<Features>,
}

pub struct ProcedureSignatureValues {
	pub values : StdBox<[ProcedureSignatureValue]>,
	pub variadic : bool,
}

pub struct ProcedureSignatureValue {
	pub identifier : Option<StdRc<StdBox<str>>>,
	pub kind : EntityLink<ValueKind>,
}


impl ProcedureSignature {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		for variant in self.variants.iter () {
			try! (variant.link (value_kinds));
		}
		succeed! (());
	}
}


impl ProcedureSignatureVariant {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		try! (self.inputs.link (value_kinds));
		try! (self.outputs.link (value_kinds));
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		return list_build_3 (
				& self.inputs.format (),
				& symbol_clone_str ("->") .into (),
				& self.outputs.format (),
				None,
				Some (true));
	}
}


impl ProcedureSignatureValues {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		for value in self.values.iter () {
			try! (value.link (value_kinds));
		}
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		let mut tokens = StdVec::with_capacity (self.values.len ());
		for value in self.values.iter () {
			tokens.push (value.format ());
		}
		if self.variadic {
			tokens.push (symbol_clone_str ("...") .into ());
		}
		let tokens = list_collect (tokens, Some (true));
		return tokens;
	}
}


impl ProcedureSignatureValue {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		try! (self.kind.entity_link_from (value_kinds));
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		let kind_token = symbol_from_rc (self.kind.identifier_rc_clone ());
		if let Some (identifier) = &self.identifier {
			return pair_new (
					symbol_from_rc (StdRc::clone (identifier)) .into (),
					kind_token.into (),
					Some (true));
		} else {
			return kind_token.into ();
		}
	}
}




pub struct SyntaxSignature {
	pub keywords : StdBox<[StdRc<SyntaxSignatureKeyword>]>,
	pub keywords_map : StdMap<StdString, StdRc<SyntaxSignatureKeyword>>,
	pub variants : StdBox<[SyntaxSignatureVariant]>,
}

pub enum SyntaxSignatureKeyword {
	
	Literal (StdRc<StdBox<str>>),
	Identifier (StdRc<StdBox<str>>),
	Expression (StdRc<StdBox<str>>),
	
	Constant {
		identifier : StdRc<StdBox<str>>,
		value : Value,
	},
	Value {
		identifier : StdRc<StdBox<str>>,
		kind : Option<EntityLink<ValueKind>>,
	},
	
	Pattern {
		identifier : StdRc<StdBox<str>>,
		patterns : StdBox<[SyntaxSignaturePattern]>,
	},
	
}

pub struct SyntaxSignatureVariant {
	pub pattern : SyntaxSignaturePattern,
}

pub enum SyntaxSignaturePattern {
	List (StdBox<[SyntaxSignaturePattern]>, Option<StdBox<SyntaxSignaturePattern>>),
	Keyword (StdRc<SyntaxSignatureKeyword>),
	Variadic (StdBox<SyntaxSignaturePattern>),
	SyntaxIdentifier,
	SyntaxRules,
	SyntaxTransformer,
}


impl SyntaxSignature {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		for keyword in self.keywords.iter () {
			try! (keyword.link (value_kinds));
		}
		succeed! (());
	}
}


impl SyntaxSignatureKeyword {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn link (&self, value_kinds : &impl Entities<ValueKind>) -> (Outcome<()>) {
		match self {
			
			SyntaxSignatureKeyword::Literal (_) =>
				succeed! (()),
			SyntaxSignatureKeyword::Identifier (_) =>
				succeed! (()),
			SyntaxSignatureKeyword::Expression (_) =>
				succeed! (()),
			
			SyntaxSignatureKeyword::Constant { .. } =>
				succeed! (()),
			SyntaxSignatureKeyword::Value { kind, .. } => {
				if let Some (kind) = kind {
					try! (kind.entity_link_from (value_kinds));
				}
				succeed! (());
			},
			
			SyntaxSignatureKeyword::Pattern { .. } =>
				succeed! (()),
		}
	}
}


impl Entity for SyntaxSignatureKeyword {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		match self {
			
			SyntaxSignatureKeyword::Literal (identifier) =>
				identifier,
			SyntaxSignatureKeyword::Identifier (identifier) =>
				identifier,
			SyntaxSignatureKeyword::Expression (identifier) =>
				identifier,
			
			SyntaxSignatureKeyword::Constant { identifier, .. } =>
				identifier,
			SyntaxSignatureKeyword::Value { identifier, .. } =>
				identifier,
			
			SyntaxSignatureKeyword::Pattern { identifier, .. } =>
				identifier,
			
		}
	}
}


impl SyntaxSignaturePattern {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		match self {
			SyntaxSignaturePattern::List (patterns, pattern_dotted) => {
				let mut tokens = StdVec::with_capacity (patterns.len ());
				for pattern in patterns.iter () {
					match pattern {
						SyntaxSignaturePattern::Variadic (pattern) => {
							tokens.push (pattern.format ());
							tokens.push (symbol_clone_str ("...") .into ());
						},
						_ =>
							tokens.push (pattern.format ()),
					}
				}
				let token_dotted = if let Some (pattern_dotted) = pattern_dotted {
					let token_dotted = pattern_dotted.format ();
					Some (token_dotted)
				} else {
					None
				};
				let tokens = list_collect_dotted (tokens, token_dotted, Some (true));
				return tokens;
			},
			SyntaxSignaturePattern::Variadic (pattern) => {
				//  NOTE:  This case shouldn't happen!
				let tokens = pair_new (
						symbol_clone_str ("...") .into (),
						pattern.format (),
						Some (true));
				return tokens;
			},
			SyntaxSignaturePattern::Keyword (keyword) =>
				symbol_from_rc (keyword.identifier_rc_clone ()) .into (),
			SyntaxSignaturePattern::SyntaxIdentifier =>
				symbol_clone_str ("_") .into (),
			SyntaxSignaturePattern::SyntaxRules =>
				symbol_clone_str ("@syntax-rules") .into (),
			SyntaxSignaturePattern::SyntaxTransformer =>
				symbol_clone_str ("@syntax-transformer") .into (),
		}
	}
}




pub struct Description {
	lines : StdVec<StdRc<StdBox<str>>>,
}


impl Description {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (lines : StdVec<StdRc<StdBox<str>>>) -> (Description) {
		return Description {
				lines : lines,
			};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn lines (&self) -> (impl iter::ExactSizeIterator<Item = &str>) {
		return self.lines.iter () .map (StdRc::deref) .map (StdBox::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn lines_clone (&self) -> (StdVec<StdString>) {
		return vec_map! (self.lines.iter (), line, StdString::from (line.deref () .deref ()));
	}
}




pub struct Links {
	links : EntitiesOwned<Link>,
}


impl Links {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (impl iter::ExactSizeIterator<Item = &Link>) {
		return self.links.entities ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn link_resolve (&self, identifier : &str) -> (Option<&Link>) {
		return try_or_panic! (self.links.try_entity_resolve (identifier));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn has_links (&self) -> (bool) {
		return self.links.has_entities ();
	}
}




pub struct Link {
	// FIXME: ...
	identifier : StdRc<StdBox<str>>,
}

impl Entity for Link {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}




pub struct Features {
	// FIXME: ...
	pub condition : Value,
}


impl Features {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn format (&self) -> (Value) {
		return self.condition.clone ();
	}
}




pub struct Appendix {
	
	identifier : StdRc<StdBox<str>>,
	
	title : Option<StdRc<StdBox<str>>>,
	description : Option<Description>,
	links : Option<Links>,
	
}


impl Entity for Appendix {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn identifier_rc_ref (&self) -> (&StdRc<StdBox<str>>) {
		return &self.identifier;
	}
}


impl Appendix {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn title (&self) -> (Option<&str>) {
		return self.title.as_ref () .map (StdRc::deref) .map (StdBox::deref);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn description (&self) -> (Option<&Description>) {
		return self.description.as_ref ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn links (&self) -> (Option<&Links>) {
		return self.links.as_ref ();
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parse_library_specifications (input : &str) -> (Outcome<Libraries>) {
	
	let inputs = try! (parse_values (input, None));
	
	let libraries = try_vec_map_into! (inputs, input, parse_library (input));
	let libraries = try! (EntitiesOwned::new (libraries));
	
	let libraries = Libraries {
			libraries,
		};
	
	succeed! (libraries);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_library (input : Value) -> (Outcome<Library>) {
	
	let (_, attributes) = try! (parse_object_with_attributes (input, Some ("library"), false));
	
	let mut identifier = None;
	let mut categories_input = None;
	let mut exports_input = None;
	let mut definitions_input = None;
	let mut value_kinds_input = None;
	let mut appendices_input = None;
	let mut title = None;
	let mut description = None;
	let mut links = None;
	let mut features = None;
	
	for (attribute, tokens) in attributes {
		match attribute.deref () .as_ref () {
			
			"identifier" => {
				let token = try! (vec_explode_1 (tokens));
				let token = try_into_symbol! (token);
				identifier = Some (token.string_rc_clone ());
			},
			
			"categories" => {
				categories_input = Some (tokens);
			},
			"exports" => {
				exports_input = Some (tokens);
			},
			"definitions" => {
				definitions_input = Some (tokens);
			},
			"types" => {
				value_kinds_input = Some (tokens);
			},
			"appendices" => {
				appendices_input = Some (tokens);
			},
			
			"title" => {
				let token = try! (vec_explode_1 (tokens));
				let token = try_into_string_immutable! (token);
				title = Some (token.string_rc_clone ());
			},
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			
			"features" => {
				features = Some (try! (parse_features (tokens)));
			}
			
			_ =>
				fail! (0x9c7a1941),
			
		}
	}
	
	let identifier = try_some! (identifier, 0x70cdea2b);
	
	let categories = if let Some (inputs) = categories_input {
		try! (parse_list_of (inputs, parse_category))
	} else {
		StdVec::new ()
	};
	let categories = try! (EntitiesOwned::new (categories));
	
	let exports = if let Some (inputs) = exports_input {
		try! (parse_list_of (inputs, parse_export))
	} else {
		StdVec::new ()
	};
	let exports = try! (EntitiesOwned::new (exports));
	
	let definitions = if let Some (inputs) = definitions_input {
		try! (parse_list_of (inputs, parse_definition))
	} else {
		StdVec::new ()
	};
	let definitions = try! (EntitiesOwned::new (definitions));
	
	let value_kinds = if let Some (inputs) = value_kinds_input {
		try! (parse_list_of (inputs, parse_value_kind))
	} else {
		StdVec::new ()
	};
	let value_kinds = try! (EntitiesOwned::new (value_kinds));
	
	let appendices = if let Some (inputs) = appendices_input {
		try! (parse_list_of (inputs, parse_appendix))
	} else {
		StdVec::new ()
	};
	let appendices = try! (EntitiesOwned::new (appendices));
	
	let library = Library {
			identifier,
			categories,
			exports,
			definitions,
			definitions_all : StdMap::new (),
			value_kinds,
			value_kinds_all : StdMap::new (),
			title,
			description,
			links,
			appendices,
			features,
		};
	
	let library = try! (library.link ());
	
	succeed! (library);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_category (input : Value) -> (Outcome<Category>) {
	
	let (identifier, attributes) = try! (parse_object_with_attributes (input, None, true));
	
	let identifier = try_some_or_panic! (identifier, 0xb2b59df4);
	
	let mut parents = None;
	let mut description = None;
	let mut links = None;
	
	for (attribute, tokens) in attributes {
		match attribute.deref () .as_ref () {
			
			"parent" | "parents" => {
				parents = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			
			_ =>
				fail! (0x06977abb),
			
		}
	}
	
	let parents = try_option_map! (parents, EntitiesLinked::new (parents)) .unwrap_or_else (EntitiesLinked::new_empty);
	
	let category = Category {
			identifier,
			parents,
			parents_all : EntitiesLinked::new_empty (),
			children : EntitiesLinked::new_empty (),
			children_all : EntitiesLinked::new_empty (),
			exports : EntitiesLinked::new_empty (),
			exports_all : EntitiesLinked::new_empty (),
			definitions : EntitiesLinked::new_empty (),
			definitions_all : EntitiesLinked::new_empty (),
			value_kinds : EntitiesLinked::new_empty (),
			value_kinds_all : EntitiesLinked::new_empty (),
			description,
			links,
		};
	
	succeed! (category);
}



#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_export (input : Value) -> (Outcome<Export>) {
	
	let (identifier, attributes) = try! (parse_object_with_attributes (input, None, true));
	
	let identifier = try_some_or_panic! (identifier, 0xdb401fa3);
	
	let mut parents = None;
	let mut categories = None;
	let mut description = None;
	let mut links = None;
	let mut descriptor = None;
	let mut features = None;
	
	for (attribute, tokens) in attributes {
		match attribute.deref () .as_ref () {
			
			"parent" | "parents" => {
				parents = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"category" | "categories" => {
				categories = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			
			"descriptor" => {
				let token = try! (vec_explode_1 (tokens));
				descriptor = Some (token);
			},
			"features" => {
				features = Some (try! (parse_features (tokens)));
			},
			
			_ =>
				fail! (0x85d8ab85),
			
		}
	}
	
	let parents = try_option_map! (parents, EntitiesLinked::new (parents)) .unwrap_or_else (EntitiesLinked::new_empty);
	
	let categories = try_option_map! (categories, EntitiesLinked::new (categories)) .unwrap_or_else (EntitiesLinked::new_empty);
	
	let descriptor = try_some! (descriptor, 0x0ff120c3);
	
	let export = Export {
			identifier,
			parents,
			parents_all : EntitiesLinked::new_empty (),
			children : EntitiesLinked::new_empty (),
			children_all : EntitiesLinked::new_empty (),
			categories,
			categories_all : EntitiesLinked::new_empty (),
			description,
			links,
			descriptor : descriptor,
			definitions : EntitiesLinked::new_empty (),
			definitions_all : EntitiesLinked::new_empty (),
			features,
		};
	
	succeed! (export);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_definition (input : Value) -> (Outcome<Definition>) {
	
	let (identifier, attributes) = try! (parse_object_with_attributes (input, None, true));
	
	let identifier = try_some_or_panic! (identifier, 0x5181cc5e);
	
	let mut kind = None;
	let mut categories = None;
	let mut exports = None;
	let mut aliases = None;
	let mut procedure_signature = None;
	let mut syntax_signature = None;
	let mut description = None;
	let mut links = None;
	let mut features = None;
	
	for (attribute, tokens) in attributes {
		match attribute.deref () .as_ref () {
			
			"type" => {
				let token = try! (vec_explode_1 (tokens));
				let token = try_into_symbol! (token);
				kind = Some (try! (DefinitionKind::resolve (token.string_as_str ())));
			},
			
			"category" | "categories" => {
				categories = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			"export" | "exports" => {
				exports = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"alias" | "aliases" => {
				aliases = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"signature" => {
				procedure_signature = Some (try! (parse_procedure_signature (tokens)));
			},
			"syntax-rules" => {
				syntax_signature = Some (try! (parse_syntax_signature (tokens)));
			},
			
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			
			"features" => {
				features = Some (try! (parse_features (tokens)));
			}
			
			_ =>
				fail! (0x24ac563a),
			
		}
	}
	
	let kind = try_some! (kind, 0x74b6b39e);
	
	if procedure_signature.is_some () && ! kind.is_procedure () {
		fail! (0xf67ee3aa);
	}
	if syntax_signature.is_some () && ! kind.is_syntax () {
		fail! (0xb0210771);
	}
	
	let categories = try_option_map! (categories, EntitiesLinked::new (categories)) .unwrap_or_else (EntitiesLinked::new_empty);
	let exports = try_option_map! (exports, EntitiesLinked::new (exports)) .unwrap_or_else (EntitiesLinked::new_empty);
	
	let aliases = aliases.unwrap_or_else (StdVec::new);
	
	let definition = Definition {
			identifier,
			kind,
			categories,
			categories_all : EntitiesLinked::new_empty (),
			exports,
			exports_all : EntitiesLinked::new_empty (),
			aliases,
			description,
			links,
			procedure_signature,
			syntax_signature,
			referenced_value_kinds : EntitiesLinked::new_empty (),
			features,
		};
	
	succeed! (definition);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn parse_value_kind (input : Value) -> (Outcome<ValueKind>) {
	
	let (identifier, attributes) = try! (parse_object_with_attributes (input, None, true));
	
	let identifier = try_some_or_panic! (identifier, 0x6ad37e55);
	
	let mut parents = None;
	let mut covariants = None;
	let mut covariants_for = None;
	let mut contravariants = None;
	let mut contravariants_for = None;
	let mut union = None;
	let mut intersection = None;
	let mut accepts = None;
	let mut accepts_for = None;
	let mut categories = None;
	let mut aliases = None;
	let mut description = None;
	let mut links = None;
	let mut predicate = None;
	let mut features = None;
	
	for (attribute, tokens) in attributes {
		match attribute.deref () .as_ref () {
			
			"parent" | "parents" => {
				parents = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"covariant" | "covariants" => {
				covariants = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			"covariant-for" | "covariants-for" => {
				covariants_for = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			"contravariant" | "contravariants" => {
				contravariants = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			"contravariant-for" | "contravariants-for" => {
				contravariants_for = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			"union" => {
				union = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			"intersection" => {
				intersection = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			"accepts" => {
				accepts = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			"accepted-by" | "accepts-for" => {
				accepts_for = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			}
			
			"category" | "categories" => {
				categories = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"alias" | "aliases" => {
				aliases = Some (try! (parse_list_of (tokens, |token| succeed! (try_into_symbol! (token) .string_rc_clone ()))));
			},
			
			"predicate" => {
				let token = try! (vec_explode_1 (tokens));
				predicate = Some (try! (parse_value_kind_predicate (token)));
			},
			
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			
			"features" => {
				features = Some (try! (parse_features (tokens)));
			}
			
			_ =>
				fail! (0x239f24d1),
			
		}
	}
	
	let parents = try_option_map! (parents, EntitiesLinked::new (parents)) .unwrap_or_else (EntitiesLinked::new_empty);
	
	let covariants = try_option_map! (covariants, EntitiesLinked::new (covariants)) .unwrap_or_else (EntitiesLinked::new_empty);
	let covariants_for = try_option_map! (covariants_for, EntitiesLinked::new (covariants_for)) .unwrap_or_else (EntitiesLinked::new_empty);
	let contravariants = try_option_map! (contravariants, EntitiesLinked::new (contravariants)) .unwrap_or_else (EntitiesLinked::new_empty);
	let contravariants_for = try_option_map! (contravariants_for, EntitiesLinked::new (contravariants_for)) .unwrap_or_else (EntitiesLinked::new_empty);
	
	let union = try_option_map! (union, EntitiesLinked::new (union));
	let intersection = try_option_map! (intersection, EntitiesLinked::new (intersection));
	let accepts = try_option_map! (accepts, EntitiesLinked::new (accepts));
	let accepts_for = try_option_map! (accepts_for, EntitiesLinked::new (accepts_for));
	
	if let Some (union) = union {
		try! (covariants_for.entities_include_linked (&union));
		try! (contravariants.entities_include_linked (&union));
	}
	if let Some (intersection) = intersection {
		try! (covariants_for.entities_include_linked (&intersection));
	}
	
	if let Some (accepts) = accepts {
		try! (covariants_for.entities_include_linked (&accepts));
		try! (contravariants.entities_include_linked (&accepts));
	}
	if let Some (accepts_for) = accepts_for {
		try! (covariants.entities_include_linked (&accepts_for));
		try! (contravariants_for.entities_include_linked (&accepts_for));
	}
	
	let categories = try_option_map! (categories, EntitiesLinked::new (categories)) .unwrap_or_else (EntitiesLinked::new_empty);
	
	let aliases = aliases.unwrap_or_else (StdVec::new);
	
	let value_kind = ValueKind {
			identifier,
			parents,
			parents_all : EntitiesLinked::new_empty (),
			children : EntitiesLinked::new_empty (),
			children_all : EntitiesLinked::new_empty (),
			categories,
			categories_all : EntitiesLinked::new_empty (),
			aliases,
			description,
			links,
			predicate,
			features,
			covariants,
			covariants_for,
			covariants_all : EntitiesLinked::new_empty (),
			contravariants,
			contravariants_for,
			contravariants_all : EntitiesLinked::new_empty (),
			definitions_input : EntitiesLinked::new_empty (),
			definitions_input_all : EntitiesLinked::new_empty (),
			definitions_input_all_2 : EntitiesLinked::new_empty (),
			definitions_output : EntitiesLinked::new_empty (),
			definitions_output_all : EntitiesLinked::new_empty (),
			definitions_output_all_2 : EntitiesLinked::new_empty (),
		};
	
	succeed! (value_kind);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_value_kind_predicate (token : Value) -> (Outcome<ValueKindPredicate>) {
	match token.class_match_into () {
		ValueClassMatchInto::Symbol (value) =>
			match value.string_as_str () {
				"none" =>
					succeed! (ValueKindPredicate::None),
				"inherit" =>
					succeed! (ValueKindPredicate::Inherit),
				"fixme!" =>
					succeed! (ValueKindPredicate::Fixme),
				_ =>
					succeed! (ValueKindPredicate::Expression (value.into ())),
			},
		ValueClassMatchInto::Pair (value) =>
			succeed! (ValueKindPredicate::Expression (value.value ())),
		_ =>
			fail! (0xd99c307d),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_procedure_signature (input : StdVec<Value>) -> (Outcome<ProcedureSignature>) {
	
	let variants = try! (parse_list_of (input, parse_procedure_signature_variant)) .into_boxed_slice ();
	
	if variants.is_empty () {
		fail! (0x2281d2dd);
	}
	
	let signature = ProcedureSignature {
			variants,
		};
	
	succeed! (signature);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
fn parse_procedure_signature_variant (input : Value) -> (Outcome<ProcedureSignatureVariant>) {
	
	let tokens = try! (vec_list_clone (&input));
	let (inputs, becomes, outputs, tokens) = try! (vec_explode_3n (tokens));
	{
		let becomes = try_into_symbol! (becomes);
		if becomes.string_as_str () != "->" {
			fail! (0x9aa6e666);
		}
	}
	
	let inputs = try! (parse_procedure_signature_values (inputs));
	let outputs = try! (parse_procedure_signature_values (outputs));
	
	let mut features = None;
	
	if ! tokens.is_empty () {
		
		let (_, attributes) = try! (parse_object_with_attributes_0 (tokens, Some ("::"), false));
		
		for (attribute, tokens) in attributes {
			match attribute.deref () .as_ref () {
				
				"features" => {
					features = Some (try! (parse_features (tokens)));
				}
				
				_ =>
					fail! (0xe911c007),
				
			}
		}
	}
	
	let variant = ProcedureSignatureVariant {
			inputs,
			outputs,
			features,
		};
	
	succeed! (variant);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_procedure_signature_values (token : Value) -> (Outcome<ProcedureSignatureValues>) {
	match token.class () {
		ValueClass::Symbol => {
			let value = try! (parse_procedure_signature_value (token));
			let values = ProcedureSignatureValues {
					values : StdBox::new ([value]),
					variadic : false,
				};
			succeed! (values);
		},
		ValueClass::Pair => {
			let tokens = try! (vec_list_clone (&token));
			let variadic = if let Some (last) = tokens.last () {
				match last.class_match_as_ref () {
					ValueClassMatchAsRef::Symbol (last) =>
						last.string_eq ("..."),
					_ =>
						false,
				}
			} else {
				false
			};
			let tokens = if variadic {
				let mut tokens = tokens;
				try_some_or_panic! (tokens.pop (), 0xcca15f6f);
				tokens
			} else {
				tokens
			};
			let values = try_vec_map_into! (tokens, token, parse_procedure_signature_value (token));
			let values = ProcedureSignatureValues {
					values : values.into_boxed_slice (),
					variadic : variadic,
				};
			succeed! (values);
		},
		ValueClass::Null => {
			let values = ProcedureSignatureValues {
					values : StdBox::new ([]),
					variadic : false,
				};
			succeed! (values);
		},
		_ =>
			fail! (0xa00d30be),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_procedure_signature_value (token : Value) -> (Outcome<ProcedureSignatureValue>) {
	match token.class_match_into () {
		ValueClassMatchInto::Symbol (kind) => {
			if kind.string_eq ("...") {
				fail! (0x0bbd4e95);
			}
			let kind = EntityLink::new_linked (kind.string_rc_clone ());
			let value = ProcedureSignatureValue {
					identifier : None,
					kind : kind,
				};
			succeed! (value);
		}
		ValueClassMatchInto::Pair (tokens) => {
			let tokens = try! (vec_list_clone (& tokens.value ()));
			let (identifier, kind) = try! (vec_explode_2 (tokens));
			let identifier = try_into_symbol! (identifier);
			let kind = try_into_symbol! (kind);
			if identifier.string_eq ("...") || kind.string_eq ("...") {
				fail! (0xd3afa44f);
			}
			let identifier = if ! identifier.string_eq ("_") {
				Some (identifier.string_rc_clone ())
			} else {
				None
			};
			let kind = EntityLink::new_linked (kind.string_rc_clone ());
			let value = ProcedureSignatureValue {
					identifier : identifier,
					kind : kind,
				};
			succeed! (value);
		},
		_ =>
			fail! (0x4045ae98),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_syntax_signature (input : StdVec<Value>) -> (Outcome<SyntaxSignature>) {
	
	let (keywords, variants) = try! (vec_explode_1n (input));
	
	let keywords = try! (vec_list_clone (&keywords));
	let (keywords, keywords_map) = try! (parse_syntax_signature_keywords (keywords));
	
	let variants = try_vec_map_into! (variants, variant, parse_syntax_signature_variant (variant, &keywords_map));
	
	let signature = SyntaxSignature {
			keywords : keywords.into_boxed_slice (),
			keywords_map : keywords_map,
			variants : variants.into_boxed_slice (),
		};
	
	succeed! (signature);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (type_complexity) ) ]
fn parse_syntax_signature_keywords (tokens : StdVec<Value>) -> (Outcome<(StdVec<StdRc<SyntaxSignatureKeyword>>, StdMap<StdString, StdRc<SyntaxSignatureKeyword>>)>) {
	
	let mut keywords = StdVec::with_capacity (tokens.len ());
	let mut keywords_map = StdMap::with_capacity (tokens.len ());
	
	for token in tokens {
		let keyword = try! (parse_syntax_signature_keyword (token, &keywords_map));
		let keyword = StdRc::new (keyword);
		keywords.push (StdRc::clone (&keyword));
		if keywords_map.insert (keyword.identifier_clone (), StdRc::clone (&keyword)) .is_some () {
			fail! (0xc4cf1b8f);
		}
	}
	
	succeed! ((keywords, keywords_map));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_syntax_signature_keyword (token : Value, keywords : &StdMap<StdString, StdRc<SyntaxSignatureKeyword>>) -> (Outcome<SyntaxSignatureKeyword>) {
	match token.class_match_into () {
		ValueClassMatchInto::Symbol (literal) => {
			let keyword = SyntaxSignatureKeyword::Literal (literal.string_rc_clone ());
			succeed! (keyword);
		},
		ValueClassMatchInto::Pair (tokens) => {
			let tokens = try! (vec_list_clone (& tokens.value ()));
			let (identifier, kind, tokens) = try! (vec_explode_2n (tokens));
			let identifier = try_into_symbol! (identifier);
			let kind = try_into_symbol! (kind);
			match kind.string_as_str () {
				"literal" => {
					if ! tokens.is_empty () {
						fail! (0x76b1463b);
					}
					let keyword = SyntaxSignatureKeyword::Literal (
							identifier.string_rc_clone ()
						);
					succeed! (keyword);
				},
				"identifier" => {
					if ! tokens.is_empty () {
						fail! (0x5df8e23a);
					}
					let keyword = SyntaxSignatureKeyword::Identifier (
							identifier.string_rc_clone ()
						);
					succeed! (keyword);
				},
				"expression" => {
					if ! tokens.is_empty () {
						fail! (0x1ec8b264);
					}
					let keyword = SyntaxSignatureKeyword::Expression (
							identifier.string_rc_clone ()
						);
					succeed! (keyword);
				},
				"constant" => {
					let value = try! (vec_explode_1 (tokens));
					let keyword = SyntaxSignatureKeyword::Constant {
							identifier : identifier.string_rc_clone (),
							value : value,
						};
					succeed! (keyword);
				},
				"value" => {
					let kind = try! (vec_explode_1 (tokens));
					let kind = try_into_symbol! (kind);
					let kind = EntityLink::new_linked (kind.string_rc_clone ());
					let keyword = SyntaxSignatureKeyword::Value {
							identifier : identifier.string_rc_clone (),
							kind : Some (kind),
						};
					succeed! (keyword);
				},
				"pattern" => {
					let patterns = try_vec_map_into! (tokens, token, parse_syntax_signature_pattern (token, keywords));
					let keyword = SyntaxSignatureKeyword::Pattern {
							identifier : identifier.string_rc_clone (),
							patterns : patterns.into_boxed_slice (),
						};
					succeed! (keyword);
				},
				_ =>
					fail! (0x7e5640f4),
			}
		}
		_ =>
			fail! (0x5b273bdf),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
fn parse_syntax_signature_variant (token : Value, keywords : &StdMap<StdString, StdRc<SyntaxSignatureKeyword>>) -> (Outcome<SyntaxSignatureVariant>) {
	let (tokens, token_dotted) = try! (vec_list_clone_dotted (&token));
	{
		let head = try_some! (tokens.first (), 0x6cbf707b);
		let head = try_as_symbol_ref! (head);
		if ! head.string_eq ("_") {
			fail! (0x867a2057);
		}
	}
	let pattern = try! (parse_syntax_signature_patterns (tokens, token_dotted, keywords));
	let variant = SyntaxSignatureVariant {
			pattern,
		};
	succeed! (variant);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_syntax_signature_patterns (tokens : StdVec<Value>, token_dotted : Option<Value>, keywords : &StdMap<StdString, StdRc<SyntaxSignatureKeyword>>) -> (Outcome<SyntaxSignaturePattern>) {
	let mut patterns = StdVec::with_capacity (tokens.len ());
	let mut end_expected = false;
	for token in tokens {
		if end_expected {
			fail! (0xfbe4c0da);
		}
		match token.class_match_into () {
			ValueClassMatchInto::Symbol (token) => {
				if token.string_eq ("...") {
					if let Some (last) = patterns.pop () {
						let pattern = SyntaxSignaturePattern::Variadic (StdBox::new (last));
						patterns.push (pattern);
						end_expected = true;
					} else {
						fail! (0x6ef5ca55);
					}
				} else {
					let pattern = try! (parse_syntax_signature_pattern (token.into (), keywords));
					patterns.push (pattern);
				}
			},
			ValueClassMatchInto::Pair (list) => {
				let pattern = try! (parse_syntax_signature_pattern (list.value (), keywords));
				patterns.push (pattern);
			},
			_ =>
				fail! (0x60d8e7c6),
		}
	}
	let pattern_dotted = if let Some (token_dotted) = token_dotted {
		let pattern_dotted = try! (parse_syntax_signature_pattern (token_dotted, keywords));
		Some (StdBox::new (pattern_dotted))
	} else {
		None
	};
	let pattern = SyntaxSignaturePattern::List (patterns.into_boxed_slice (), pattern_dotted);
	succeed! (pattern);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_syntax_signature_pattern (token : Value, keywords : &StdMap<StdString, StdRc<SyntaxSignatureKeyword>>) -> (Outcome<SyntaxSignaturePattern>) {
	match token.class_match_into () {
		ValueClassMatchInto::Symbol (keyword) => {
			if keyword.string_eq ("...") {
				fail! (0xaaefecfb);
			} else if keyword.string_eq ("_") {
				succeed! (SyntaxSignaturePattern::SyntaxIdentifier);
			} else if keyword.string_eq ("@syntax-rules") {
				succeed! (SyntaxSignaturePattern::SyntaxRules);
			} else if keyword.string_eq ("@syntax-transformer") {
				succeed! (SyntaxSignaturePattern::SyntaxTransformer);
			} else {
				let keyword = try_some! (keywords.get (keyword.string_as_str ()), 0x97ac4521);
				let keyword = StdRc::clone (keyword);
				let pattern = SyntaxSignaturePattern::Keyword (keyword);
				succeed! (pattern);
			}
		},
		ValueClassMatchInto::Pair (list) => {
			let (tokens, token_dotted) = try! (vec_list_clone_dotted (& list.value ()));
			return parse_syntax_signature_patterns (tokens, token_dotted, keywords);
		},
		ValueClassMatchInto::Null =>
			succeed! (SyntaxSignaturePattern::List (StdBox::new ([]), None)),
		_ =>
			fail! (0x2274e06a),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_appendix (input : Value) -> (Outcome<Appendix>) {
	
	let (identifier, attributes) = try! (parse_object_with_attributes (input, None, true));
	
	let identifier = try_some_or_panic! (identifier, 0xb9669009);
	
	let mut title = None;
	let mut description = None;
	let mut links = None;
	
	for (attribute, tokens) in attributes {
		match attribute.deref () .as_ref () {
			
			"title" => {
				let token = try! (vec_explode_1 (tokens));
				let token = try_into_string_immutable! (token);
				title = Some (token.string_rc_clone ());
			},
			"description" => {
				description = Some (try! (parse_description (tokens)));
			},
			"links" => {
				links = Some (try! (parse_links (tokens)));
			},
			
			_ =>
				fail! (0x9e7c02e8),
			
		}
	}
	
	let appendix = Appendix {
			identifier,
			title,
			description,
			links,
		};
	
	succeed! (appendix);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value, type_complexity) ) ]
fn parse_object_with_attributes (input : Value, keyword : Option<&str>, identifier_expected : bool) -> (Outcome<(Option<StdRc<StdBox<str>>>, StdVec<(StdRc<StdBox<str>>, StdVec<Value>)>)>) {
	
	let tokens = try! (vec_list_clone (&input));
	
	return parse_object_with_attributes_0 (tokens, keyword, identifier_expected);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (type_complexity) ) ]
fn parse_object_with_attributes_0 (tokens : StdVec<Value>, keyword : Option<&str>, identifier_expected : bool) -> (Outcome<(Option<StdRc<StdBox<str>>>, StdVec<(StdRc<StdBox<str>>, StdVec<Value>)>)>) {
	
	let tokens = if let Some (keyword) = keyword {
		let (head, rest) = try! (vec_explode_1n (tokens));
		let head = try_into_symbol! (head);
		if ! head.string_eq (keyword) {
			fail! (0x3ec7c223);
		}
		rest
	} else {
		tokens
	};
	
	let (identifier, tokens) = if identifier_expected {
		let (head, rest) = try! (vec_explode_1n (tokens));
		let identifier = try_into_symbol! (head);
		let identifier = identifier.string_rc_clone ();
		(Some (identifier), rest)
	} else {
		(None, tokens)
	};
	
	let mut attributes = StdMap::with_capacity (tokens.len ());
	for tokens in tokens {
		let tokens = try! (vec_list_clone (&tokens));
		let (head, rest) = try! (vec_explode_1n (tokens));
		let identifier = try_into_symbol! (head);
		let identifier = identifier.string_rc_clone ();
		if attributes.insert (identifier, rest) .is_some () {
			fail! (0x9a98dec4);
		}
	}
	
	let attributes = attributes.into_iter () .collect ();
	
	succeed! ((identifier, attributes));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_description (input : StdVec<Value>) -> (Outcome<Description>) {
	
	let input = try! (vec_explode_1 (input));
	
	let mut lines = match input.class_match_as_ref () {
		ValueClassMatchAsRef::Symbol (value) =>
			match value.string_as_str () {
				"fixme!" =>
					vec! [ StdRc::new (StdString::from ("FIXME!") .into_boxed_str ()) ],
				_ =>
					fail! (0x41a13440),
			},
		ValueClassMatchAsRef::String (value) =>
			vec_map! (try! (value.string_ref ()) .string_as_str () .lines (), line, StdRc::new (StdString::from (line.trim_right ()) .into_boxed_str ())),
		_ =>
			fail! (0x5ca05f5a),
	};
	
	for _ in 0..2 {
		#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (while_let_loop) ) ]
		loop {
			let pop = if let Some (line) = lines.last () {
				line.trim_left () .is_empty ()
			} else {
				break;
			};
			if pop {
				lines.pop ();
			} else {
				break;
			}
		}
		lines.reverse ();
	}
	
	let description = Description {
			lines,
		};
	
	succeed! (description);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
fn parse_links (_input : StdVec<Value>) -> (Outcome<Links>) {
	fail_unimplemented! (0xd3359173);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_features (input : StdVec<Value>) -> (Outcome<Features>) {
	
	let input = try! (vec_explode_1 (input));
	
	let features = Features {
			condition : input,
		};
	
	succeed! (features);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_list_of <T> (input : StdVec<Value>, parser : impl Fn (Value) -> (Outcome<T>)) -> (Outcome<StdVec<T>>) {
	let output = try! (input.into_iter () .map (parser) .collect ());
	succeed! (output);
}




#[ cfg ( feature = "vonuvoli_documentation_sources" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parse_library_specifications_for_builtins () -> (Outcome<Libraries>) {
	return parse_library_specifications (LIBRARY_SPECIFICATIONS_SOURCES);
}

#[ cfg ( feature = "vonuvoli_documentation_sources" ) ]
static LIBRARY_SPECIFICATIONS_SOURCES : &'static str = include_str! ("../documentation/libraries.ss");


use bevy::prelude::*;

use crate::util::collisions::{Contact, Manifold};

#[derive(Resource, Default)]
pub struct Contacts(pub Vec<Contact>);

#[derive(Resource, Default)]
pub struct Manifolds(pub Vec<Manifold>);

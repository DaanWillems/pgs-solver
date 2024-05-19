use bevy::prelude::*;

use crate::util::collisions::Contact;

#[derive(Resource, Default)]
pub struct Contacts(pub Vec<Contact>);
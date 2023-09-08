use crate::attributes::RelationshipType;

#[derive(Clone)]
pub struct Relationship {
    relationship_type: RelationshipType,
    person: usize,
}

impl Relationship {
    pub fn get_person_id(&self) -> usize { self.person }
    pub fn get_relationship_type(&self) -> RelationshipType { self.relationship_type }
    pub fn new(relationship_type: RelationshipType, person: usize) -> Relationship { Relationship { relationship_type, person } }
}

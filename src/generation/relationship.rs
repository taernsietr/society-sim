use crate::generation::{
    people::attributes::RelationshipType,
    population::Population
};

#[derive(Clone, Debug)]
pub struct Relationship {
    relationship_type: RelationshipType,
    person_1: usize,
    person_2: usize
}

impl Relationship {
    pub fn new(
        relationship_type: RelationshipType,
        person_1: usize,
        person_2: usize
    ) -> Relationship { Relationship { relationship_type, person_1, person_2 } }

    pub fn contains_id(&self, person_id: usize) -> bool { self.person_1 == person_id || self.person_2 == person_id }
    pub fn get_relationship_type(&self) -> RelationshipType { self.relationship_type }
    pub fn get_person_id(&self, person: usize) -> usize {
        match person {
            0 => self.person_1,
            1 => self.person_2,
            _ => panic!("Invalid relationship index!")
        }
    }
}

impl Population {
    pub fn create_relationship(&mut self, relationship: Relationship) { self.get_relationships_mut().push(relationship); }

    pub fn has_spouses(&self, person: usize) -> bool {
        self.get_relationships()
            .iter()
            .filter(
                |relationship|
                relationship.contains_id(person) &&
                matches!(relationship.get_relationship_type(), RelationshipType::Spouse)
            )
            .count() > 0
    }
}


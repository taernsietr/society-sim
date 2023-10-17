impl Population {
    //pub fn children(&mut self) {
    //    // this means children will have the same birthday
    //    if !self.elapsed_time % CHILDBIRTH_PERIOD == 0 { return }

    //    let mut rng = rand::thread_rng();
    //    let mut children: Vec<Human> = Vec::new();
    //    
    //    let mut candidates: Vec<usize> = self.alive_pop
    //        .iter()
    //        .filter(|person| person.1.spouse.is_some())
    //        .filter(|person| person.1.age <= FERTILE_AGE)
    //        .map(|person| person.1.id)
    //        .collect::<Vec<usize>>();
    //    candidates.sort();
    //    candidates.dedup();

    //    for couple in candidates {
    //        let parent_1 = self.alive_pop.get(&couple.0).unwrap().clone();
    //        let parent_2 = self.alive_pop.get(&couple.1).unwrap().clone();
    //        let family_name = parent_1.family;

    //        let childbirth_threshold: usize = 100;
    //        let roll = rng.gen_range(0..=100);

    //        if roll <= childbirth_threshold {
    //            let child = Human::new(self.new_id(), Some(request_word()), Some(family_name), None, None, Some(0), None, None);
    //            println!(
    //                "[BIRTH]: {}, {} has been born. [{} | {}]",
    //                child.family,
    //                child.name,
    //                roll,
    //                childbirth_threshold
    //            );
    //            self.create_relationship((parent_1.id, child.id), RelationshipType::Parent);
    //            self.create_relationship((parent_2.id, child.id), RelationshipType::Parent);
    //            children.push(child);
    //        }
    //    }
    //}
}


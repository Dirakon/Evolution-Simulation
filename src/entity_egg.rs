use crate::genetic_code::GeneticCode;
use crate::entity::Entity;
pub struct EntityEgg{
    parental_genetic_code:GeneticCode,
    pub pos_x:i32,
    pub pos_y:i32
}
impl EntityEgg{
    pub fn new(parental_genetic_code:GeneticCode,pos_x:i32,pos_y:i32)->EntityEgg{
        EntityEgg{
            parental_genetic_code,
            pos_x,
            pos_y
        }
    }
    pub fn evolve_into_entity(self,entity_id:u32)->Entity{
        Entity::new_from_genes_with_mutation_chance(self.pos_x, self.pos_y, self.parental_genetic_code, entity_id)
    }
}
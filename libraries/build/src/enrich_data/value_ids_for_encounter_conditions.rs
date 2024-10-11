use pokedata_api_entities::api::encounter_condition::EncounterCondition;
use pokedata_api_entities::api::encounter_condition_value::EncounterConditionValue;
use std::collections::HashMap;

pub fn enrich_encounter_conditions_with_value_ids(
    encounter_conditions: &mut HashMap<i32, EncounterCondition>,
    encounter_condition_values: &HashMap<i32, EncounterConditionValue>,
)
{
    for value in encounter_condition_values.values() {
        let Some(condition) = encounter_conditions.get_mut(&value.encounter_condition_id) else { continue };
        condition.condition_value_ids.push(value.id);
    }
}
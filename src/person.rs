// src/person.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Person {
    pub id: usize,
    pub age: u8,
    pub health_status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Interaction {
    pub frequency: u8,
    pub strength: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_person() {
        let person = Person {
            id: 1,
            age: 30,
            health_status: "Healthy".to_string(),
        };
        assert_eq!(person.id, 1);
        assert_eq!(person.age, 30);
        assert_eq!(person.health_status, "Healthy");
    }

    #[test]
    fn test_create_interaction() {
        let interaction = Interaction {
            frequency: 5,
            strength: 0.8,
        };
        assert_eq!(interaction.frequency, 5);
        assert!(interaction.strength - 0.8 < f32::EPSILON);
    }
}

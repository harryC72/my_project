pub fn test_option_type() -> Option<u8> {
    let mut opt1 = None;

    opt1 = Some(10);
    opt1
}

pub fn chose_chartype(character: CharacterType) -> Option<CharacterType> {
    let mut chartype = None;
    chartype = Some(character);
    chartype
}

pub enum CharacterType {
    Archer,
    Warrior,
    Mage,
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer".to_string(),
            CharacterType::Warrior => "Warrior".to_string(),
            CharacterType::Mage => "Mage".to_string(),
        }
    }
}

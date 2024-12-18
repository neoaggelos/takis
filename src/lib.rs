pub fn format_to_title(a: &str) -> String {
    fn next_chars(ch: char, some_prev: Option<char>) -> String {
        let this = match ch {
            'ά' => 'α',
            'έ' => 'ε',
            'ή' => 'η',
            'ί' => 'ι',
            'ό' => 'ο',
            'ύ' => 'υ',
            'ώ' => 'ω',
            'Ά' => 'Α',
            'Έ' => 'Ε',
            'Ή' => 'Η',
            'Ί' => 'Ι',
            'Ό' => 'Ο',
            'Ύ' => 'Υ',
            'Ώ' => 'Ω',
            _ => ch,
        };

        return match some_prev.map(|p| p.is_alphanumeric() || p == '\'') {
            Some(true) => this.to_lowercase().collect(),
            _ => this.to_uppercase().collect(),
        };
    }

    let mut b = String::with_capacity(a.len());
    a.chars().for_each(|ch| {
        b.push_str(next_chars(ch, b.chars().last()).as_str());
    });

    b
}

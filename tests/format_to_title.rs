use takis::format_to_title;

#[test]
fn greek() {
    assert_eq!(format_to_title("η Βαθρα του Φονια"), "Η Βαθρα Του Φονια");
    assert_eq!(format_to_title("η Βάθρα του Φονιά"), "Η Βαθρα Του Φονια");
}

#[test]
fn capitalization() {
    assert_eq!(
        format_to_title("ArchItect (feat. currents) (re-Recorded)"),
        "Architect (Feat. Currents) (Re-Recorded)"
    );

    assert_eq!(format_to_title("the end of everything"), "The End Of Everything");
}

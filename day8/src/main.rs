enum States{
    TamilNadu,
    AndhraPradesh,
    Karnataka,
    Telangana,
    Kerala,
}

fn match_language(state:States) -> &'static str{
    match state{
        States::TamilNadu => "Tamil",
        States::AndhraPradesh => "Telugu",
        States::Kerala => "Malayalam",
        States::Telangana => "Telugu",
        States::Karnataka => "Kannada",
        _ => "Some Other language",
    };
}

fn main() {
    let state:States = States::TamilNadu;

    let language = match_language(state);

    // println!("IN {:?} the official language is {:?}",state,language);

}

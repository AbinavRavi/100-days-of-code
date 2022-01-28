enum States{
    TamilNadu,
    AndhraPradesh,
    Karnataka,
    Telangana,
    Kerala,
}

fn match_language(state:States){
    match state{
        States::TamilNadu => println!("Tamil"),
        States::AndhraPradesh => println!("Telugu"),
        States::Kerala => println!("Malayalam"),
        States::Telangana => println!("Telugu"),
        States::Karnataka => println!("Kannada"),
        _ => println!("Some Other language"),
    };
}

fn main() {
    let state:States = States::TamilNadu;
    match_language(state);

    // println!("IN {:?} the official language is {:?}",state,language);

}

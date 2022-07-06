use characters_lib::character::*;

fn main() {
    hello_mu();
    how_old_is_mu(2);
    println!("Your Walgreens points are ready!");
    // above is for reference

    let mut some_number: u8 = 15;
    some_number = 7;
    println!("my favorite number is: {}", some_number + 1);
    println!("number of tacos: {}", some_number - 5);

    // make our characters (instances)
    let mut mu_dog = Character {
        name: "Mu".to_string(),
        max_hp: 200,
        current_hp: 200,
        level: 2,
        damage_amount: 110,
        damage_type: "smelly".to_string(),
        armor: 50,
    };
    let quark_dog = Character {
        name: "Quark".to_string(),
        max_hp: 1200,
        current_hp: 1200,
        level: 14,
        damage_amount: 9000,
        damage_type: "kissy".to_string(),
        armor: 99,
    };


    // do stuff with our characters
    mu_dog.whats_my_name(); // this one just prints stuff
    let some_variable = mu_dog.get_name(); // this is the one that returns
    println!("{}", some_variable);
    println!("hi {}", some_variable);
    println!("hello {}", some_variable);
    println!("night night {}", some_variable);

    println!("_________________________________________________");


    // deal some damage to Mu character
    println!("Mu has {} hit points.", mu_dog.get_current_hp());
    mu_dog.receive_damage(1);
    println!("Mu has {} hit points.", mu_dog.get_current_hp());
    mu_dog.receive_damage(100);
    println!("Mu has {} hit points.", mu_dog.get_current_hp());

    // Mu drinks a health potion
    mu_dog.heal(50);
    println!("Mu has {} hit points.", mu_dog.get_current_hp());

    // Mu fights a bear (Quark)
    mu_dog.receive_damage(1000);

}

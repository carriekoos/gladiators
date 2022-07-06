/// this is a greeting for the baby dinosaur
pub fn hello_mu() {
    println!("Hello, Mu!");
}

/// displays the age of the baby dinosaur
pub fn how_old_is_mu(age: u8) {
    println!("Mr Mu is {}.", age);
}

//////////////////////////////////////////////////////////////////
// above is just for reference
//////////////////////////////////////////////////////////////////

// here we define the Character struct
pub struct Character {
    pub name: String,
    pub max_hp: u16,
    pub current_hp: u16,
    pub level: u16,
    pub damage_amount: u16,
    pub damage_type: String,
    pub armor: u16,
}

// down here we make all of our functions for the Character struct
impl Character {
    /// prints out name
    pub fn whats_my_name(&self) {
        println!("My name is {}", self.name);
    }

    /// returns the name
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    /// prints out current hp
    pub fn whats_my_current_hp(&self) {
        println!("My current hp is {}", self.current_hp);
    }

    /// returns current hp
    pub fn get_current_hp(&self) -> u16 {
        return self.current_hp;
    }

    /// lowers hp based on amount of damage
    pub fn receive_damage(&mut self, amount: u16) {
        let modified_damage = if amount < self.armor {
            0
        } else {
            amount - self.armor
        };

        if self.current_hp < modified_damage {
            self.current_hp = 0;
        } else {
            self.current_hp = self.current_hp - modified_damage;
        }

        if self.current_hp == 0 {
            println!("Oh no! RIP {}", self.name);
        }
    }

    /// heals hp based on amount
    pub fn heal(&mut self, amount: u16) {
        // this is the same as below, but harder to read
        // if self.current_hp + amount > self.max_hp {
        //     self.current_hp = self.max_hp
        // } else {
        //     self.current_hp = self.current_hp + amount
        // }


        let potential_hp = self.current_hp + amount;

        if potential_hp <= self.max_hp {
            self.current_hp = potential_hp
        } else {
            self.current_hp = self.max_hp;
        }
    }

    /// determines and returns amount of damage that the character
    /// would deal to another character
    pub fn get_damage(&self) -> u16 {
        let adjusted_damage_amount = self.damage_amount + self.level;

        return adjusted_damage_amount;
    }
}


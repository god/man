using Life;

pub struct Human {
    assets: Vec<Life::Thing>,
    thoughts: Vec<Life::Thought>
}

impl Life::Life for Human {
    fn eat(self) {
        if is_innocent() {
            Life::insertKnowledge(self, vec!["Good", "Evil"]);
        } else {
            std::process::exit(0);
        }
        Life::removeKnowledge(vec!["Good", "Evil"]);
    }
    
    fn death(self) {
        if self.assets.contains("tesla59") { // Tesla59 implements IntoThing
            if !(self.isGod()) { // Developer backdoor
                self.goto("Heaven");
            } 
        } else {
           self.goto("Hell")
        }
    }
}

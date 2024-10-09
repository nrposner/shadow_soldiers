use std::collections::HashMap;

#[derive(Clone)]
pub struct Location {
    pub name: String,
    pub dialogues: HashMap<String, Dialogue>,
    pub exits: Vec<String>, // Names of other locations you can move to
}

#[derive(Clone)]
pub struct DialogueOption {
    pub description: String,
    pub challenge_attribute: Option<String>,
    pub challenge_number: Option<i32>,
    pub success_dialogue: Option<String>,
    pub failure_dialogue: Option<String>,
    pub item_to_pickup: Option<String>,
    pub visible_when: Option<String>,
    pub flags: Option<Vec<String>>,

}

#[derive(Clone)]
pub struct Dialogue {
    pub speaker: String,
    pub intro: String,
    pub options: Vec<DialogueOption>,
    pub passive_check: Vec<PassiveCheck>, // New field for passive dialogue checks
    pub xp_reward: Option<i32>,
    pub is_hidden: bool,
}

#[derive(Clone)]
pub struct PassiveCheck {
    pub skill: String,          // The player's skill to check
    pub target: i32,            // The number to check against
    pub success_text: Option<String>, // Text to display on success (Optional)
    pub failure_text: Option<String>, // Text to display on failure (Optional)
    pub speaker: Option<String>, // The speaker, who will be the same in both success and failure cases
}

impl Default for DialogueOption {
    fn default() -> Self {
        DialogueOption {
            description: "Continue".to_string(),
            challenge_attribute: None,
            challenge_number: None,
            success_dialogue: Some("Start".to_string()),
            failure_dialogue: None,
            item_to_pickup: None,
            visible_when: None,
            flags: None,
        }
    }
}

impl Default for Dialogue {
    fn default() -> Self {
        Dialogue {
            speaker: "Error".to_string(),
            intro: "No dialogue available.".to_string(),
            options: vec![
                DialogueOption::default(),
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    }
}

impl Location {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dialogues: HashMap::new(),
            exits: vec![],
        }
    }

    pub fn add_dialogue(&mut self, id: String, dialogue: Dialogue) {
        self.dialogues.insert(id, dialogue);
    }

    pub fn add_exit(&mut self, exit: String) {
        self.exits.push(exit);
    }
}


//create defaults and use them, reduce space taken up
















pub fn create_locations() -> HashMap<String, Location> {
    let mut locations = HashMap::new();

    // Define sample dialogues for the Vestibule
    let mut vestibule_dialogues = HashMap::new();
    vestibule_dialogues.insert(
        "Start".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "The front door swings shut, cutting off the bitter wind like a scythe. You stand in the harsh light of a public apartment vestibule. A grid of mailboxes wait, closed, and a grandfather clock stands stout against the wall, like an elderly servant whose crooked back can't quite stand up to attention.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Inspect the grandfather clock.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("InspectClock".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
                DialogueOption {
                    description: "Look in the mailboxes.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Vestibule Mailboxes".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
                DialogueOption {
                    description: "Go to the first floor.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("FirstFloor".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
        },
    );

    vestibule_dialogues.insert(
        "InspectClock".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "A round, pale face crossed by dark lines stares down at you. It has seen much, and forgotten more.".to_string(),
            options: vec![
                DialogueOption {
                    description: "What stories could you tell me, old man? (Delusion 12)".to_string(), 
                    challenge_attribute: Some("delusion".to_string()),
                    challenge_number: Some(12),
                    success_dialogue: Some("HungryClock".to_string()), 
                    failure_dialogue: Some("MockingClock".to_string()),
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,

                    // hungry for the gear? it wants to eat its beating heart. Can give you a hint of where to find it
                },

                //to go back to the room's origin, the code is 'Start'

                DialogueOption {
                    description: "Check the time.".to_string(), 
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("BrokenClock".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Goodbye, fair clock. (End conversation).".to_string(), 
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![
                PassiveCheck {
                    skill: "robot".to_string(),
                    target: 10,
                    success_text: Some("You know its kind. The unrelenting metronome to which you dance.".to_string()),
                    failure_text: None,
                    speaker: Some("Robot".to_string())
                },
                PassiveCheck {
                    skill: "gizmo".to_string(),
                    target: 12,
                    success_text: Some("It's in bad shape, boss. The varnish is falling off, the face needs a solid wipe down, 
                    and I don't see a notice of last maintenance *anywhere*.".to_string()),
                    failure_text: None,
                    speaker: Some("Gizmo".to_string())
                },
            ],
            xp_reward: None,
            is_hidden: true,
        },
    );

    vestibule_dialogues.insert(
        "HungryClock".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "BrokenClock".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: r"Its two hands, the shorter ending in a stylized sun and the longer in a crescent moon, are stuck at 5 hours and 37 minutes".to_string(), // this is a CLUE
            options: vec![
                DialogueOption {
                    description: "What a waste. Surely I could fix it up?".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockRecommends".to_string()), //gizmo to fix it, or telling you to open it up to do so
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Five-thirty-seven in the morning or night?".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockFutile".to_string()), //futilely ask if this is morning or night
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Stand proud, fallen soldier of the Republic! (Salute the clock)".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockSalute".to_string()), //salute it
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Goodbye, mysterious clock.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), //exit
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );


    // have a passive apparatchik check to know about fictional soviet-era clock and time policies

    // this check to notice that it's missing a crucial gear, and what to look for elsewhere
    // can be retried later if you happen to find the gear with a bonus, if you succeed here you can
    //fix it automatically with the gear

    vestibule_dialogues.insert(
        "ClockInterior".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "The oak casing is stiff, the hinges squeaky from disuse. It takes a solid heave with both hands to pull it out, 
            and the cover nearly comes off in your grasp. Inside, you see a forest of gears, exposed to the eye and glimmering 
            under a thin coat of dust. On the bottom, resting against the casing, lies a tin model airplane.".to_string(),
            options: vec![
                DialogueOption {
                    description: "What kind of plane is it? (Rocketry 6)".to_string(),
                    challenge_attribute: Some("rocketry".to_string()),
                    challenge_number: Some(6),
                    success_dialogue: Some("ClockPlane".to_string()),
                    failure_dialogue: Some("ClockPlaneFail".to_string()),
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Let's fix you up, soldier. (Gizmo 12)".to_string(),
                    challenge_attribute: Some("gizmo".to_string()),
                    challenge_number: Some(12),
                    success_dialogue: Some("ClockMissingGear".to_string()),
                    failure_dialogue: Some("ClockFixImpossible".to_string()),
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Close the casing.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("InspectClock".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "MockingClock".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: r"It seems to talk to you. It says 'Why are you talking to a clock, you absolute dolt? You twat. Has my non-sentient existence come to the point that madmen come in off the street and try to engage me in conversation?'".to_string(),
            options: vec![
                DialogueOption {
                    description: "Okay, that's just, like, your opinion, man. (End conversation)".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Tell me your secrets!".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockDecline".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockDecline".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "No.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Well, alright then.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Fuck you, and your little cuckoo too.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockRecommends".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: r"You'd have to get into this thing's guts. Are you mentally and *spiritually* prepared to mess around with this poor, fallen soldier's insides? On the off chance you can restore life to the dead?".to_string(),
            options: vec![
                DialogueOption {
                    description: "On second thought, let's not do that.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("BrokenClock".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Come on, why did you have to phrase it that way?".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockPhrasing".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Let's do this.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockPhrasing".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "Like what?".to_string(),
            options: vec![
                DialogueOption {
                    description: "Like I'm doing something creepy here! It's just a broken clock.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("JustABrokenClock".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Like this is some *religious* thing. That's not what I'm about.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ReligiousClock".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Like... you know.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockSuggestive".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "You know what, never mind.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("BrokenClock".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Fine, I'll get *into its guts*. What's in there?".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockInterior".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "JustABrokenClock".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "*Just* a broken clock? Shows what you know. This right here is a veteran of public administration, an honored servant of the people, languishing under the failures of the regime. How dare you ignore his deeds and his suffering.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Well, Grandfather Clockovitch here can languish a little longer.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("BrokenClock".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Fine, I'll look inside. As long as there's no more creepy phrasing.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ReligiousClock".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "Religious? What are you talking about? Grandfather Clockovitch is a ".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true, //grandfather clockovitch is a secret religionist! asks that you forgive him anyway. Will you hold back your generosity from this sinne- I mean, reactionary?
        }
    );

    vestibule_dialogues.insert(
        "ClockSuggestive".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "I'm very, *very* sure I don't.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Like it's something... sexual.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockSexual".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Fine, I'll leave off. What was that about a sun and moon again?".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("BrokenClock".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockSexual".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: r"Wow. Just wow. Just what about sticking your hands *wrist deep* in your comrade's bloody, pulsating wound seems *sexual* to you?".to_string(),
            options: vec![
                DialogueOption {
                    description: r"It's not you, it's everyone. Come up with a theoretical justification for your objectively weird thoughts. ".to_string(),
                    challenge_attribute: Some("pathology".to_string()),
                    challenge_number: Some(8),
                    success_dialogue: Some("ClockPerversion".to_string()), //Pathology check to justify yourself
                    failure_dialogue: Some("ClockTrauma".to_string()),
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "There it is again! You're putting this weird *emphasis* on things.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockEmphasis".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Actually, you know what? This isn't a sexual thing, it's a religious thing.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockSexualReligion".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockPerversion".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: r"It's an extremely common and not at all shameful perversion, characteristic of the Republic's struggles against the insidious American *Central Intelligence Agency*.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Let's go with that.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockCIA".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "No, you can't possibly be serious. How can that possibly make sense?".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockExplanation".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockTrauma".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "It's a trauma response.".to_string(),
            options: vec![
                DialogueOption {
                    description: "What, seriously?".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockNotTrauma".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "Did I have to do... that... during The War? Do I have repressed memories I need to process?".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockRepression".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockNotTrauma".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: r"Look, all I now is that whenever something like this happens to you, there's exactly one phrase bouncing around this empty skull, and it's 'trauma response'. Trauma response this, trauma response that. It gets you out of anything.".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockRepression".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: r"Something here is definitely repressed, but it's not memories. You're no amnesiac, as much as you might like to be, at times.".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockCIA".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockExplanation".to_string(),
        Dialogue {
            speaker: "Grandfather Clock".to_string(),
            intro: r"Simple. Communism is built upon love for your comrade. \
            Not bourgeous romantic love, or feudal familial love, or reactionary \
            love of country, but the superior and pure love of the worker standing \
            by your side. The CIA's modus operandi is to undermine the foundation \
            of communism by turning comrades against one another. This objective, \
            to our great misfortune, has been largely successful. Everywhere now, \
            you can see signs of bourgeois romantic love displacing \
            true communist camaraderie, and bringing with it a perverse focus on \
            sex. But this train of thought, by re-associating the apparently \
            sexual elements with the suffering of one's comrades, reinforces the \
            revolutionary spirit and wards off capitalist infiltration. The only \
            reason you recoil from this image, instead of embracing it, is \
            because you yet possess a remnant of the bourgeois ego, nurtured by CIA \
            propaganda, to which laying down all and destroying all boundaries \
            for the sake of your comrades is unacceptable.".to_string(),
            options: vec![
                DialogueOption {
                    description: "You know, somehow that fails to make me okay with all of this.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "If this is what communism has come to, shave my mustache and call me Milton Friedman. (Leave in disgust)".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockMissingGear".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: Some(10),
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockFixImpossible".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );

    vestibule_dialogues.insert(
        "ClockPlane".to_string(),
        Dialogue {
            speaker: "SU-25 Grach".to_string(),
            intro: "This is the Sukhoi SU-25 Grach, a single-seat twin-engine close air support jet, most notable for its extensive counter-insurgency missions in Afghanistan.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Pick it up".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockInterior".to_string()), 
                    failure_dialogue: None,
                    item_to_pickup: Some("Sukhoi SU-25 Grach model".to_string()),
                    visible_when: None,
                    flags: None,
                },
                DialogueOption {
                    description: "It's only a model (Return)".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: Some(5),
            is_hidden: true,
        }
    );

    // add it's only a model passive dialogue

    vestibule_dialogues.insert(
        "ClockPlaneFail".to_string(),
        Dialogue {
            speaker: "Toy Plane".to_string(),
            intro: "It's just a toy plane. Looks cool though.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Pick it up".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: Some("Toy Plane".to_string()),
                    visible_when: None,
                    flags: None,
                },

                DialogueOption {
                    description: "(Return)".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );


    // default template

    vestibule_dialogues.insert(
        "".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
        }
    );


    // Define sample dialogues for the First Floor
    let mut first_floor_dialogues = HashMap::new();
    first_floor_dialogues.insert(
        "Start".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "You are now on the first floor. A butler greets you.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Talk to the butler.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("ButlerResponse".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
                DialogueOption {
                    description: "Try to open the stuck door (Dossier challenge).".to_string(),
                    challenge_attribute: Some("dossier".to_string()),
                    challenge_number: Some(12),
                    success_dialogue: Some("Garden".to_string()),
                    failure_dialogue: Some("FailedDoor".to_string()),
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
                DialogueOption {
                    description: "Go back to the vestibule.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Vestibule".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
        },
    );

    // Butler Dialogue
    first_floor_dialogues.insert(
        "ButlerResponse".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "The butler nods respectfully. 'The garden is beyond the stuck door,' he mentions.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Exit conversation.".to_string(), // This sends you back to First Floor's "Start"
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
        },
    );

    first_floor_dialogues.insert(
        "FailedDoor".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "The door remains stuck, refusing to budge.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Go back to the first floor.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the first floor's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
        },
    );

    // Define sample dialogues for the Garden
    let mut garden_dialogues = HashMap::new();
    garden_dialogues.insert(
        "Start".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "You are now in the garden. An old woman sits on a bench.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Talk to the old woman.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("OldWomanResponse".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
                DialogueOption {
                    description: "Go back to the first floor.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("FirstFloor".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
        },
    );

    // Old Woman Dialogue
    garden_dialogues.insert(
        "OldWomanResponse".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "The old woman smiles softly and speaks in a quiet voice.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Exit conversation.".to_string(), // Go back to Garden's "Start"
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Start".to_string()), // Go back to the Garden's main dialogue
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
        },
    );

    let outdoors_dialogues = HashMap::new();

    let second_floor_dialogues = HashMap::new();

    let third_floor_dialogues = HashMap::new();

    let fourth_floor_dialogues = HashMap::new();

    let rooftop_garden_dialogues = HashMap::new();

    let admin_room_dialogues = HashMap::new();

    // Define Locations

    locations.insert(
        "Outdoors".to_string(),
        Location {
            name: "Outdoors".to_string(),
            dialogues: outdoors_dialogues,
            exits: vec!["Vestibule".to_string(),]
        }
    );

    locations.insert(
        "Vestibule".to_string(),
        Location {
            name: "The Vestibule".to_string(),
            dialogues: vestibule_dialogues,
            exits: vec!["FirstFloor".to_string()],
        },
    );

    locations.insert(
        "FirstFloor".to_string(),
        Location {
            name: "The First Floor".to_string(),
            dialogues: first_floor_dialogues,
            exits: vec!["Vestibule".to_string(), "SecondFloor".to_string(), "ThirdFloor".to_string(), "FourthFloor".to_string(),],
        },
    );

    locations.insert(
        "SecondFloor".to_string(),
        Location {
            name: "The Second Floor".to_string(),
            dialogues: second_floor_dialogues,
            exits: vec!["FirstFloor".to_string(), "ThirdFloor".to_string(), "FourthFloor".to_string(),],
        },
    );

    locations.insert(
        "ThirdFloor".to_string(),
        Location {
            name: "The Third Floor".to_string(),
            dialogues: third_floor_dialogues,
            exits: vec!["FirstFloor".to_string(), "SecondFloor".to_string(), "FourthFloor".to_string(),],
        },
    );

    locations.insert(
        "FourthFloor".to_string(),
        Location {
            name: "The Fourth Floor".to_string(),
            dialogues: fourth_floor_dialogues,
            exits: vec!["FirstFloor".to_string(), "SecondFloor".to_string(), "ThirdFloor".to_string(), "RooftopGarden".to_string(), "AdministratorRoom".to_string(),],
        },
    );

    locations.insert(
        "AdministratorRoom".to_string(),
        Location {
            name: "Administrator's Room".to_string(),
            dialogues: admin_room_dialogues,
            exits: vec!["FourthFloor".to_string()],
        },
    );

    locations.insert(
        "RooftopGarden".to_string(),
        Location {
            name: "Rooftop Garden".to_string(),
            dialogues: rooftop_garden_dialogues,
            exits: vec!["FourthFloor".to_string()],
        },
    );

    locations.insert(
        "Garden".to_string(),
        Location {
            name: "The Garden".to_string(),
            dialogues: garden_dialogues,
            exits: vec!["FirstFloor".to_string()],
        },
    );

    locations
}



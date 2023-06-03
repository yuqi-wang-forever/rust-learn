mod stargate_shows {
    pub struct Stargate {
        pub name: String,
        pub sex: String,
        pub age: u8,
    }
    impl Stargate {
        pub fn character(age: u8) -> Stargate{
            Stargate{
                name: String::from("Jack"),
                sex:String::from("male"),
                age, //相当于 age:age, Json也这样写
            }           
        }
    }
    pub mod watch_shows {
        fn open_tv(){
            println!("Trun on the Tv");
        }
        pub fn turn_up_voice(){
            open_tv();
            println!("turn up voice");
            let character: super::Stargate = super::Stargate::character(52);
            start_watching(character);
        }
        fn start_watching(character: super::Stargate){
            println!("you can start watching stargate,{}'s age is {},and is {}",character.name,character.age,character.sex);
        }

    }
}

pub fn watch_stargate(){
    crate::tvshows::stargate_shows::watch_shows::turn_up_voice();
}
// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

// I AM !NOT DONE

mod delicious_snacks {

    // TODO: Fix these use statements
    pub use self::fruits::PEAR as pear;
    pub use self::veggies::CUCUMBER as cucumber;

    mod fruits {
        pub const PEAR: &'static str = "Pear"; // notice const is pub but mod is private
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::pear,
        delicious_snacks::cucumber,
    );
}

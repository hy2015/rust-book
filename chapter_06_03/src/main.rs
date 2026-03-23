#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        // For simplicity, let's assume all states existed in 2024
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::Arizona => year >= 1912,
            UsState::Arkansas => year >= 1836,
            UsState::California => year >= 1850,
            UsState::Colorado => year >= 1876,
            UsState::Connecticut => year >= 1788,
            UsState::Delaware => year >= 1787,
            UsState::Florida => year >= 1845,
            UsState::Georgia => year >= 1788,
            UsState::Hawaii => year >= 1959,
            UsState::Idaho => year >= 1890,
            UsState::Illinois => year >= 1818,
            UsState::Indiana => year >= 1816,
            UsState::Iowa => year >= 1846,
            UsState::Kansas => year >= 1861,
            UsState::Kentucky => year >= 1792,
            UsState::Louisiana => year >= 1812,
            UsState::Maine => year >= 1820,
            UsState::Maryland => year >= 1788,
            UsState::Massachusetts => year >= 1788,
            UsState::Michigan => year >= 1837,
            UsState::Minnesota => year >= 1858,
            UsState::Mississippi => year >= 1817,
            UsState::Missouri => year >= 1821,
            UsState::Montana => year >= 1889,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new, for America!"))
        }
    } else {
        None
    }
}

fn describe_state_quarter2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new, for America!"))
    }
}

fn describe_state_quarter3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new, for America!"))
    }
}


fn pretty_print_state_quarter(coin: Coin){
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    // borrow `coin` so the match doesn't take ownership
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;
    // borrow `coin` again to avoid moving it
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn main() {
    let result1 = describe_state_quarter(Coin::Quarter(UsState::Alaska));
    let result2 = describe_state_quarter2(Coin::Quarter(UsState::Alaska));
    let result3 = describe_state_quarter3(Coin::Quarter(UsState::Alaska));

    println!("{result1:?}");
    println!("{result2:?}");
    println!("{result3:?}");
}

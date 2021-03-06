fn if_statement() {
    let temp = 35;

    if temp > 30 {
        println!("Quite hot out there !");
    } else if temp < 10 {
        println!("Quite chilly out there !");
    } else {
        println!("Temperature is nice today.")
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("Today is {}", day);

    println!("It is {}", if temp > 20 { "nice" } else { "gloomy" });

    println!(
        "It is {}",
        if temp > 20 {
            if temp > 30 {
                "Very Hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "Cold"
        } else {
            "Okay"
        }
    );
}

fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        } else if x == 512 {
            break;
        }
        println!("x = {}", x)
    }

    let mut y = 1;
    loop
    // while true
    {
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10 {
            break;
        }
    }
}

fn for_loop() {
    // break and continue work in here also
    for x in 1..11 { // .. is an exclusive range
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "UNKNOWN", // ... & ..= are inclusive ranges (exclusive range not currently allowed in match statement)
        _ => "INVALID"
    };

    println!(
        "The country with country code {} is {}",
        country_code, country
    );
}

fn main() {
    if_statement();
    while_and_loop();
    for_loop();
    match_statement();
}

const SURE: &str = "Sure.";
const WHATEVER: &str = "Whatever.";
const CHILL_OUT: &str = "Whoa, chill out!";
const FINE: &str = "Fine. Be that way!";
const CALM_DOWN: &str = "Calm down, I know what I'm doing!";

pub fn reply(message: &str) -> &str {
    if message
        .chars()
        .filter(|x| x.is_whitespace())
        .fold(0, |acc, _| acc + 1)
        == message.len()
    {
        return FINE;
    }

    let question = message.chars().filter(|x| !x.is_whitespace()).last() == Some('?');

    let shout = message
        .chars()
        .filter(|x| x.is_alphabetic())
        .fold(0, |acc, _| acc + 1)
        > 0
        && message
            .chars()
            .filter(|x| x.is_lowercase())
            .fold(0, |acc, _| acc + 1)
            == 0;

    if shout && question {
        return CALM_DOWN;
    }

    if question {
        return SURE;
    }

    if shout {
        return CHILL_OUT;
    }

    match message {
        // m
        // ,
        _ => WHATEVER,
    }
}

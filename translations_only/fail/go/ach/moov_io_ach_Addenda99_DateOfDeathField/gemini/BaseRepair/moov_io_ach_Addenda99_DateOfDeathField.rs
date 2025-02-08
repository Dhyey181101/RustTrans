
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99 {
    pub date_of_death: Option<String>,
}

impl MoovIoAchAddenda99 {
    pub fn date_of_death_field(&self) -> String {
        match &self.date_of_death {
            Some(date) => date.clone(),
            None => String::from(" ").repeat(6),
        }
    }
}

impl FromStr for MoovIoAchAddenda99 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut addenda99 = MoovIoAchAddenda99 {
            date_of_death: None,
        };

        for line in s.lines() {
            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();

            match key {
                "DateOfDeath" => addenda99.date_of_death = Some(value.to_string()),
                _ => {}
            }
        }

        Ok(addenda99)
    }
}

impl fmt::Display for MoovIoAchAddenda99 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DateOfDeath: {}", self.date_of_death_field())
    }
}

fn main() {
    let addenda99 = MoovIoAchAddenda99::from_str(
        "DateOfDeath: 20230308",
    )
    .unwrap();

    println!("{}", addenda99);
}

use figlet_rs::FIGfont;
use std::env;
use std::fmt;
use std::str;
use std::time::Duration;
use tokio::time::{self, Instant};

#[derive(Debug, Clone)]
pub struct ParserError;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing arguments")
    }
}

pub struct Config {
    pub seconds: usize,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let seconds = match args.next() {
            Some(arg) => Config::parse_time_string(&arg).expect("Error parsing time string"),
            None => return Err("Didn't get a countdown"),
        };

        Ok(Config { seconds })
    }

    /// Parses the time argument into hours, minutes and seconds
    ///
    /// h -> hours
    /// m -> minutes
    /// s -> seconds
    ///
    /// Examples:
    /// 1h, 2m, 3s (integral)
    /// 1h2m, 10m10s (combined)
    ///
    /// The`parse_time_string` function will throw `ParserError`
    fn parse_time_string(time_string: &str) -> Result<usize, ParserError> {
        let mut hours: usize = 0;
        let mut minutes: usize = 0;
        let mut seconds: usize = 0;

        let mut s = String::new();
        // slice string at h, m and s
        for char in time_string.chars() {
            match char {
                'h' => {
                    hours = s.parse::<usize>().unwrap();
                    s = String::new();
                }
                'm' => {
                    minutes = s.parse::<usize>().unwrap();
                    s = String::new();
                }
                's' => {
                    seconds = s.parse::<usize>().unwrap();
                    s = String::new();
                }
                _ => s.push(char),
            };
        }

        let seconds = hours * 60 * 60 + minutes * 60 + seconds;

        Ok(seconds)
    }
}

pub async fn run(config: Config) {
    let start_time = Instant::now();    
    for time in 0..config.seconds {
        // clear terminal
        print!("\x1B[2J\x1B[1;1H");

        let font = FIGfont::standand().unwrap();
        time::sleep_until(start_time + Duration::from_secs(time as u64)).await;

        let time_left = config.seconds - time;
        let figure = font.convert(time_left.to_string().as_str());
        println!("{}", figure.unwrap());

        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_string() {
        // checking with integral timestamp
        assert_eq!(Config::parse_time_string("1h").unwrap(), 60 * 60);
        assert_eq!(Config::parse_time_string("1m").unwrap(), 60);
        assert_eq!(Config::parse_time_string("1s").unwrap(), 1);

        // checking with combined timestamp
        assert_eq!(Config::parse_time_string("1h2m").unwrap(), 60 * 60 + 2 * 60);
        assert_eq!(Config::parse_time_string("10m10s").unwrap(), 10 * 60 + 10);
    }
}

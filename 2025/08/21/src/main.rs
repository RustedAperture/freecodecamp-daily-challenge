//! Authors: RustedAperture
//! Date: 2025/11/07

const DESCRIPTION: &str = r#"
Mile Pace

Given a number of miles ran, and a time in "MM:SS" (minutes:seconds) it took to run those miles, return a string for the average time it took to run each mile in the format "MM:SS".

- Add leading zeros when needed.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-21
"#;

fn mile_pace(distance_miles: f64, time_minutes: &str) -> String {
    let time_parts: Vec<&str> = time_minutes.split(':').collect();
    let seconds: f64 =
        (time_parts[0].parse::<f64>().unwrap() * 60.0) + time_parts[1].parse::<f64>().unwrap();
    let mps = (seconds / distance_miles).round() as i64;
    let mins = mps / 60;
    let secs = (mps % 60).abs();
    return format!("{:02}:{:02}", mins, secs);
}

fn main() {
    println!("{}", DESCRIPTION);

    println!(
        "Example: mile_pace(3.0, \"24:00\") = {:?}",
        mile_pace(3.0, "24:00")
    );
}

#[cfg(test)]
mod tests {
    use super::mile_pace;

    #[test]
    fn test_mile_pace() {
        assert_eq!(mile_pace(3.0, "24:00"), "08:00");
        assert_eq!(mile_pace(1.0, "06:45"), "06:45");
        assert_eq!(mile_pace(2.0, "07:00"), "03:30");
        assert_eq!(mile_pace(26.2, "120:35"), "04:36");
    }
}

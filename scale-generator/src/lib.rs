// A, A#, B, C, C#, D, D#, E, F, F#, G, G#
// A, Bb, B, C, Db, D, Eb, E, F, Gb, G, Ab

#[derive(Debug)]
pub enum Error {
    BadInterval,
    BadTonic,
}

pub struct Scale(Vec<String>);

const SCALE_SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const SCALE_FLATS: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        // find chromatic
        dbg!(&tonic);
        let chromatic_scale: Vec<&str> = match tonic {
            "C" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#" | "a" => SCALE_SHARPS.to_vec(),
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => SCALE_FLATS.to_vec(),
            _ => return Err(Error::BadTonic),
        };

        // find index of starting note
        let mut index = chromatic_scale.iter().position(|c| *c.to_uppercase()==tonic.to_uppercase()).unwrap_or_default();
        
        let mut _scale: Vec<String> = vec![chromatic_scale[index].to_string()];

        // iterate through intervals and add notes to new scale
        for step in intervals.chars() {
            index += match step {
                'm' => 1,
                'M' => 2,
                'A' => 3,
                _ => return Err(Error::BadInterval),
            };
            _scale.push(chromatic_scale[index % chromatic_scale.len()].to_string());
        }
        Ok(Scale(_scale))
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        // A new chromatic is a new scale with all half steps
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }
}

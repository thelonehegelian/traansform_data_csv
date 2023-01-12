use csv::{Reader, Writer};
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

fn main() {
    // create a row struct
    #[derive(Debug)]
    struct HexColor {
        red: u8,
        green: u8,
        blue: u8,
    }

    #[derive(Debug, Deserialize)]
    struct Row {
        color_name: String,
        color: HexColor,
    }

    impl FromStr for HexColor {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // trim the # from the string
            let trimmed = s.trim_start_matches('#');
            if trimmed.len() != 6 {
                return Err("Invalid length");
            } else {
                Ok(HexColor {
                    red: u8::from_str_radix(&trimmed[0..2], 16).unwrap(),
                    green: u8::from_str_radix(&trimmed[2..4], 16).unwrap(),
                    blue: u8::from_str_radix(&trimmed[4..6], 16).unwrap(),
                })
            }
        }
    }

    impl<'de> Deserialize<'de> for HexColor {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer).unwrap();
            FromStr::from_str(&s).map_err(de::Error::custom)
        }
    }

    let data = "color_name,color
    red,#ff0000
    green,#00ff00
    blue,#0000FF
    periwinkle,#ccccff
    magenta,#ff00ff"
        .to_owned();

    let mut out = Writer::from_writer(vec![]);
    let mut reader = Reader::from_reader(data.as_bytes());
    for result in reader.deserialize::<Row>() {
        let res = result.unwrap();
        out.serialize((
            res.color_name,
            res.color.red,
            res.color.green,
            res.color.blue,
        ))
        .unwrap();
    }

    let written = String::from_utf8(out.into_inner().unwrap()).unwrap();
    println!("{}", written);
}

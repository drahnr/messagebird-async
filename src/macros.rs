#[cfg(test)]
use regex;

#[cfg(test)]
lazy_static! {
    pub static ref RE: regex::Regex = regex::Regex::new(r"[\n\s]+").unwrap();
}

#[cfg(test)]
macro_rules! serde_roundtrip {
    ($testfn:tt, $ty:ty, $x:expr) => {
        #[test]
        fn $testfn() {
            use serde_json;
            let obj: $ty = $x;
            println!("obj original {:?}", obj);
            let obj_json = serde_json::to_string(&obj).unwrap();
            println!("obj {}", obj_json);
            let obj_recovered: $ty = serde_json::from_str(obj_json.as_str()).unwrap();
            println!("obj recovered {:?}", obj_recovered);
            assert_eq!(obj, obj_recovered);
        }
    };
}

#[cfg(test)]
macro_rules! deser_roundtrip {
    ($testfn:tt, $ty:ty, $obj_json:expr) => {
        #[test]
        fn $testfn() {
            use regex;
            use serde_json;
            let obj_json: &str = $obj_json;
            let obj_json = obj_json.clone();
            // remove all whitespace and linebreaks or the resulting
            // json from serialization will not be equal due to the missing
            // spaces/linebreaks
            let obj_json = ::macros::RE.replace_all(obj_json, regex::NoExpand(""));

            println!("obj {}", obj_json);
            let obj: $ty = serde_json::from_str(&obj_json).unwrap();
            println!("obj {:?}", obj);
            let obj_json_recovered = serde_json::to_string(&obj).unwrap();
            println!("obj original {}", obj_json_recovered);
            assert_eq!(obj_json, obj_json_recovered);
        }
    };
}

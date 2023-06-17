use std::collections::HashMap;

// json-trait/lib
pub trait Json {
    fn to_json(&self) -> String;
}

impl Json for String {
    fn to_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

macro_rules! implement_int {
    ($($int:ty),+) => {
        $(
            impl Json for $int {
                fn to_json(&self) -> String {
                    self.to_string()
                }
            }
        )+
    };
}

implement_int!(bool, usize, isize, i8, i16, i32 /* add other types */);

impl<T: Json> Json for Vec<T> {
    fn to_json(&self) -> String {
        let mut inner = self
            .iter()
            .map(|val| format!("{}, ", val.to_json()))
            .collect::<String>();
        inner.pop(); //remove space 
        inner.pop(); //remove comma
        format!("[ {} ]", inner)
    }
}

impl<T: Json> Json for Option<T> {
    fn to_json(&self) -> String {
        self.map(|val| val.to_json()).unwrap_or_else(|| "null".to_string())
    }
}

impl<K: Json, V: Json> Json for HashMap<K, V> {
    fn to_json(&self) -> String {
        todo!()
    }
}
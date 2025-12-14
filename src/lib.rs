use std::fmt::Debug;

pub mod template;

pub enum Endpoint {
    Inclusive(u64),
    Exclusive(u64),
}

pub struct Range {
    start: Endpoint,
    end: Endpoint,
}

impl Range {
    pub fn new(start: Endpoint, end: Endpoint) -> Self {
        Self { start, end }
    }
    pub fn includes_value(&self, value: u64) -> bool {
        match self.start {
            Endpoint::Inclusive(v) => {
                if value < v {
                    return false;
                }
            }
            Endpoint::Exclusive(v) => {
                if value <= v {
                    return false;
                }
            }
        };

        match self.end {
            Endpoint::Inclusive(v) => {
                if value > v {
                    return false;
                }
            }
            Endpoint::Exclusive(v) => {
                if value >= v {
                    return false;
                }
            }
        };

        return true;
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let open_bracket;
        let open_value;

        match self.start {
            Endpoint::Inclusive(v) => {
                open_bracket = "[";
                open_value = v;
            }
            Endpoint::Exclusive(v) => {
                open_bracket = "(";
                open_value = v;
            }
        }

        let close_bracket;
        let close_value;

        match self.end {
            Endpoint::Inclusive(v) => {
                close_bracket = "]";
                close_value = v;
            }
            Endpoint::Exclusive(v) => {
                close_bracket = ")";
                close_value = v;
            }
        }
        write!(
            f,
            "Range<{}{}, {}{} >",
            open_bracket, open_value, close_value, close_bracket
        )
    }
}
// Use this file to add helper functions and additional modules.

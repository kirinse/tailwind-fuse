use super::*;

#[derive(Debug, Clone)]
pub struct TailwindTextOverflow {
    kind: TextOverflow,
}

#[derive(Debug, Clone)]
enum TextOverflow {
    Truncate,
    Standard(String),
    Arbitrary,
}

impl TailwindInstance for TailwindTextOverflow {
    fn collision_id(&self) -> &'static str {
        "text-overflow"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindTextOverflow {
    /// `truncate`
    pub const Truncate: Self = Self {
        kind: TextOverflow::Truncate,
    };
    /// https://tailwindcss.com/docs/text-overflow
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => TextOverflow::Arbitrary,
            _ => {
                let input = pattern.join("-");
                debug_assert!(Self::check_valid(&input));
                TextOverflow::Standard(input)
            }
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/text-overflow#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "clip", "ellipsis", "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}

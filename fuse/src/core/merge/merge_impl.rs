use std::collections::HashSet;

use crate::ast::AstStyle;

use super::{CollisionIdFn, GetCollisionsFn, MergeOptions};
use crate::core::merge::get_collisions::get_collisions;

/// Merges all the Tailwind classes, resolving conflicts.
/// Can supply custom options, collision_id_fn and collisions_fn.
pub fn tw_merge_with_override(
    class: &[&str],
    options: MergeOptions,
    collision_id_fn: impl CollisionIdFn,
    collisions_fn: impl GetCollisionsFn,
) -> String {
    let styles: Vec<Result<AstStyle, &str>> = crate::ast::parse_tailwind(class, options.into());

    let mut valid_styles: Vec<Result<AstStyle, &str>> = vec![];
    let mut collision_styles: HashSet<Collision> = HashSet::new();

    for style in styles.into_iter().rev() {
        let style = match style {
            Ok(style) => style,
            Err(s) => {
                valid_styles.push(Err(s));
                continue;
            }
        };

        let elements = style.elements.as_slice();
        let result = collision_id_fn
            .apply(elements, style.arbitrary)
            .map(Ok)
            .unwrap_or_else(|| {
                let arbitrary = style.arbitrary.unwrap_or_default();
                super::get_collision_id::get_collision_id(elements, arbitrary)
            });

        match result {
            Err(error) => match style.arbitrary.and_then(Collision::from_arbitrary) {
                Some(collision) => {
                    if collision_styles.contains(&collision) {
                        continue;
                    }
                    collision_styles.insert(collision);
                }
                None => {
                    #[cfg(debug)]
                    println!("No Instance found: {style:?} {error:?}");
                    let _ = error;
                }
            },
            Ok(collision_id) => {
                // hover:md:focus
                let all_variants: Vec<&str> = style.variants.clone();

                let collision = Collision::Default {
                    important: style.important,
                    variants: all_variants.clone(),
                    collision_id,
                };

                if collision_styles.contains(&collision) {
                    continue;
                }

                // Add the current collision_id.
                collision_styles.insert(collision);

                let collisions = collisions_fn
                    .apply(collision_id)
                    .or_else(|| get_collisions(collision_id));

                if let Some(collisions) = collisions {
                    collisions.into_iter().for_each(|collision_id| {
                        let collision = Collision::Default {
                            important: style.important,
                            variants: all_variants.clone(),
                            collision_id,
                        };

                        collision_styles.insert(collision);
                    });
                }
            }
        }
        valid_styles.push(Ok(style));
    }

    valid_styles.reverse();

    valid_styles
        .into_iter()
        .map(|s| match s {
            Ok(style) => style.source,
            Err(s) => s,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Collision<'a> {
    Default {
        important: bool,
        variants: Vec<&'a str>,
        collision_id: &'a str,
    },
    // For [color:blue] => label = "color"
    Arbitrary {
        label: &'a str,
    },
}

impl<'a> Collision<'a> {
    fn from_arbitrary(arbitrary: &'a str) -> Option<Self> {
        if arbitrary.contains(':') {
            let mut parts = arbitrary.split(':');
            let label = parts.next()?;
            Some(Self::Arbitrary { label })
        } else {
            None
        }
    }
}

#[test]
fn from_arb() {
    assert_eq!(
        Collision::from_arbitrary("color:blue"),
        Some(Collision::Arbitrary { label: "color" })
    );
    assert_eq!(Collision::from_arbitrary("blue"), None);
}

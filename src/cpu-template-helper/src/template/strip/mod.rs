// Copyright 2023 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crate::utils::{ModifierMapKey, ModifierMapValue};

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::strip;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::strip;

pub fn strip_common<K, V>(maps: &mut [HashMap<K, V>])
where
    K: ModifierMapKey,
    V: ModifierMapValue,
{
    // Get common items shared by all the sets.
    let mut common = maps[0].clone();
    common.retain(|key, value| maps[1..].iter().all(|map| map.get(key) == Some(value)));

    // Remove the common items from all the sets.
    for key in common.keys() {
        for map in maps.iter_mut() {
            map.remove(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::{mock_modifier, MockModifierMapKey, MockModifierMapValue};

    #[test]
    fn test_strip_common() {
        let mut input = vec![
            HashMap::from([
                mock_modifier!(0x0, (0b1111_1111, 0b0000_0000)),
                mock_modifier!(0x1, (0b1111_0000, 0b1111_1111)),
                mock_modifier!(0x2, (0b1111_1111, 0b1111_1111)),
            ]),
            HashMap::from([
                mock_modifier!(0x0, (0b1111_1111, 0b0000_0000)),
                mock_modifier!(0x1, (0b0000_1111, 0b1111_1111)),
                mock_modifier!(0x2, (0b1111_1111, 0b1111_1111)),
            ]),
            HashMap::from([
                mock_modifier!(0x0, (0b1111_1111, 0b0000_0000)),
                mock_modifier!(0x1, (0b1111_1111, 0b1111_1111)),
                mock_modifier!(0x3, (0b1111_1111, 0b1111_1111)),
            ]),
        ];
        let expected = vec![
            HashMap::from([
                mock_modifier!(0x1, (0b1111_0000, 0b1111_1111)),
                mock_modifier!(0x2, (0b1111_1111, 0b1111_1111)),
            ]),
            HashMap::from([
                mock_modifier!(0x1, (0b0000_1111, 0b1111_1111)),
                mock_modifier!(0x2, (0b1111_1111, 0b1111_1111)),
            ]),
            HashMap::from([
                mock_modifier!(0x1, (0b1111_1111, 0b1111_1111)),
                mock_modifier!(0x3, (0b1111_1111, 0b1111_1111)),
            ]),
        ];

        strip_common(&mut input);
        assert_eq!(input, expected);
    }
}

use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub object_id: Uuid,
    pub t: f64,
}

pub fn hit(ints: Vec<Intersection>) -> Option<Intersection> {
    let mut ints_filtered: Vec<Intersection> = ints.iter().cloned().filter(|&i| i.t >= 0.).collect();

    ints_filtered.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    ints_filtered.first().copied()
}

#[cfg(test)]
mod tests {
    use crate::intersections::{hit, Intersection};
    use crate::utils::float_compare;
    use uuid::Uuid;

    #[test]
    fn test_hit_filter() {
        let uid = Uuid::new_v4();
        let before = vec![
            Intersection { object_id: uid, t: 5.0 },
            Intersection { object_id: uid, t: 2.0 },
            Intersection {
                object_id: uid,
                t: -1.0,
            },
        ];

        let r = hit(before).unwrap();
        assert!(float_compare(r.t, 2.0));
    }
}

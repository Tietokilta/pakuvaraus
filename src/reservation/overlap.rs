use std::collections::HashMap;

use crate::reservation::overlap;

use super::types::{Reservation, SortedReservations, Overlap};

pub fn find_overlaps(res: &SortedReservations) -> Vec<(usize, usize)> {
    let mut overlaps = Vec::new();

    for i in 0..res.len() {
        let a = &res[i];

        for j in i+1..res.len() {
            let b = &res[j];

            if b.start_time >= a.end_time {
                break;
            }

            if a.start_time < b.end_time {
                overlaps.push((i, j));
            }
        }
    }

    overlaps
}

pub fn distribute_overlaps(res: &[Reservation], overlaps: Vec<(usize, usize)>) -> Vec<Vec<Overlap>> {
    let mut result = vec![Vec::new(); res.len()];
    for (a, b) in overlaps {        
        let duration = res[a].end_time.min(res[b].end_time) - res[a].start_time.max(res[b].start_time);
        result[a].push(Overlap{
            other_id: res[b].id, 
            duration: duration
        });
        result[b].push(Overlap{
            other_id: res[a].id, 
            duration: duration
        });
    }

    result
}
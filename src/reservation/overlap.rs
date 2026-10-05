use super::types::{Reservation, SortedReservations, Overlap};

pub fn find_overlaps(res: &SortedReservations) -> Vec<(usize, usize)> {
    let mut overlaps = Vec::new();

    for (i, res_a) in res.iter().enumerate() {
        for (j, res_b) in res.iter().skip(i).enumerate() {
            if res_b.start_time >= res_a.end_time {
                break;
            }

            if res_a.start_time < res_b.end_time {
                overlaps.push((i, j));
            }
        }
    }

    overlaps
}

#[allow(clippy::indexing_slicing)]
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

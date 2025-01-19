use std::cmp::Ordering;

/// Sort a sequence of numbers generically, in either ascending or descending order
///
/// # Arguments
/// - `seq`: A vector of items that implement the `Ord` trait.
/// - `descending`: A boolean flag; if `true` sorts in descending order. Otherwise, ascending
/// order.
///
/// # Returns
/// - A new sorted vector.
pub fn sort_seq<T: PartialOrd>(seq: &[T], descending: bool) -> Vec<T>
where
    T: Clone,
{
    let mut sorted_seq = seq.to_vec();
    sorted_seq.sort_by(|a, b| {
        if descending {
            b.partial_cmp(a).unwrap_or(Ordering::Equal) // sort descending
        } else {
            a.partial_cmp(b).unwrap_or(Ordering::Equal) // sort ascending
        }
    });
    sorted_seq
}

// fn main() {}

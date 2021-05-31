pub fn sum(data: &[u32]) -> Option<u32> {
    data.iter().fold(Some(0u32), |acc, curr| {
        if let Some(acc_v) = acc { acc_v.checked_add(*curr) } else {None}
    })
}
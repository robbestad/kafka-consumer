pub fn calc(v: &Vec<(i64, i64)>) -> i64 {
    let mut prev = -1;
    let mut prevdt = -1;
    let mut diffs: Vec<i64> = Vec::with_capacity(10);
    let mut sum = 0;
    for r in v {
        let (tx, tm) = r;
        if prev != -1 {
            let diff = tx - prev;
            let dvdr = (tm - prevdt) / 1000;
            diffs.push(diff / dvdr);
            sum = sum + diff / dvdr;
        }
        prev = *tx;
        prevdt = *tm;
    }
    sum / diffs.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tps() {
        let v: Vec<(i64, i64)> = vec![
            (60339333859, 1646061615237),
            (60339336950, 1646061618398),
            (60339344784, 1646061621443),
            (60339345883, 1646061624516),
            (60339347089, 1646061627568),
            (60339349564, 1646061630620),
            (60339350972, 1646061633827),
            (60339356380, 1646061636978),
            (60339362158, 1646061640154),
            (60339368786, 1646061643221),
        ];
        let sum = calc(&v);

        assert_eq!(sum, 1293);
    }
}

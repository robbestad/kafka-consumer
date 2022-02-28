fn main(){
let v:Vec<(i64,i64)> = vec![
        (
            60339333859,
            1646061615237,
        ),
        (
            60339336950,
            1646061618398,
        ),
        (
            60339344784,
            1646061621443,
        ),
        (
            60339345883,
            1646061624516,
        ),
        (
            60339347089,
            1646061627568,
        ),
        (
            60339349564,
            1646061630620,
        ),
        (
            60339350972,
            1646061633827,
        ),
        (
            60339356380,
            1646061636978,
        ),
        (
            60339362158,
            1646061640154,
        ),
        (
            60339368786,
            1646061643221,
        ),
    ];


let mut prev=0;
let mut prevdt=0;
let mut diffs:Vec<i64>=Vec::with_capacity(10); 
let mut sum=0;
for r in &v{
	let (tx,tm) = r;
	println!("{} {}",tx,tm);
  if prev != -1 {
	      let diff = tx - prev;
	println!("tx:{} prev:{} diff:{}",tx,prev,diff);
	      let dvdr= (tm-prevdt)/1000;
	println!("tm:{} prevdt:{} dvdr:{}",tm,prevdt,dvdr);
	      diffs.push(diff/dvdr);
	println!("diff:{} dvdr:{} res:{}",diff,dvdr,diff/dvdr);

	      sum = sum + diff/dvdr;
  }
  prev = *tx;
  prevdt = *tm;
}
println!("Avg TPS... {}, {:?}",sum,sum/diffs.len() as i64)
}



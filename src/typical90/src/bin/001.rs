use proconio::input;

fn main() {
    input! {
        n: usize,  // N 個の切れ目
        l: i64,  // L [cm] のようかん
        k: usize,  // K 個の切れ目を選択
        mut a: [i64; n],  // i 番目の切れ目の位置
    };
    let k = k + 1;
    a.insert(0, 0);
    a.push(l);

    let mut ok = 0;
    let mut ng = l;
    while (ok - ng).abs() > 1 {
        let mid = (ng + ok) / 2;

        let mut count = 0;
        let mut temp = 0;
        for i in 0..(n + 1) {
            temp += a[i + 1] - a[i];
            if temp >= mid {
                count += 1;
                temp = 0;
            }
        }
        if count >= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

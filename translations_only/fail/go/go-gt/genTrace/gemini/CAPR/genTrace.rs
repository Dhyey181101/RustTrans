
use rand::Rng;

fn gen_trace(p: &mut Vec<i64>, trace: &Vec<Vec<i64>>) {
    let n = p.len() as usize;
    let mut nexti = vec![0; n];
    let mut nextj = vec![0; n];
    let mut sum_trace = vec![0; n];

    perm(&mut nexti);
    perm(&mut nextj);
    for i in 0..n {
        for j in 0..n {
            sum_trace[i] += trace[i][j];
        }
    }

    for i in 0..n {
        let target = rand::thread_rng().gen_range(0..sum_trace[nexti[i]] - 1);
        let mut j = i;
        let mut sum = trace[nexti[i]][nextj[j]];
        while sum < target {
            j += 1;
            sum += trace[nexti[i]][nextj[j]];
        }
        p[nexti[i]] = nextj[j] as i64;
        for k in i..n {
            sum_trace[nexti[k]] -= trace[nexti[k]][nextj[j]];
        }
        nextj.swap(j, i);
    }
}

fn perm(p: &mut Vec<usize>) {
    let n = p.len();
    for i in 0..n {
        p[i] = i;
    }
    for i in 0..n {
        p.swap(i, (i + rand::thread_rng().gen_range(0..n - i)) as usize);
    }
}

fn unif(low: i64, high: i64) -> i64 {
    low + rand::thread_rng().gen_range(0..high - low + 1)
}

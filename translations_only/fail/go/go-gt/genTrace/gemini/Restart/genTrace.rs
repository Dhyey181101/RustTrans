
use rand::Rng;

fn gen_trace(p: &mut Vec<i64>, trace: &Vec<Vec<i64>>) {
    let n = p.len() as i64;
    let mut nexti = vec![0; n as usize];
    let mut nextj = vec![0; n as usize];
    let mut sum_trace = vec![0; n as usize];

    perm(&mut nexti);
    perm(&mut nextj);
    for i in 0..n {
        for j in 0..n {
            sum_trace[i as usize] += trace[i as usize][j as usize];
        }
    }

    for i in 0..n {
        let target = rand::thread_rng().gen_range(0..sum_trace[nexti[i as usize] as usize]);
        let mut j = i;
        let mut sum = trace[nexti[i as usize] as usize][nextj[j as usize] as usize];
        while sum < target {
            j += 1;
            sum += trace[nexti[i as usize] as usize][nextj[j as usize] as usize];
        }
        p[nexti[i as usize] as usize] = nextj[j as usize];
        for k in i..n {
            sum_trace[nexti[k as usize] as usize] -= trace[nexti[k as usize] as usize][nextj[j as usize] as usize];
        }
        nextj.swap(j as usize, i as usize);
    }
}

fn perm(p: &mut Vec<i64>) {
    let n = p.len() as i64;
    for i in 0..n {
        p[i as usize] = i;
    }
    for i in 0..n {
        p.swap(i as usize, (i + rand::thread_rng().gen_range(0..n - i)) as usize);
    }
}

fn get(m: &Vec<Vec<i64>>, i: i64, j: i64) -> i64 {
    m[i as usize][j as usize]
}

fn unif(low: i64, high: i64) -> i64 {
    low + rand::thread_rng().gen_range(0..high - low + 1)
}

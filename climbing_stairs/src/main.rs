fn climb(n: u64, target: u64) -> u64 {
    if n>target {
        return 0;
    }
    if n==target {
        return 1;
    }
    return climb(n+1,target) + climb(n+2,target);
}

fn climb_memo(n:u64, memo: &mut[i32]) ->i32 {
    if memo[n as usize]==-1 {
        memo[n as usize] = climb_memo(n+1,memo) + climb_memo(n+2,memo);
    }
    memo[n as usize]
}

fn climb_dp(n:i64) -> i32 {
    let mut onestep = 1;
    let mut twosteps = 1;

    for _i in 0..n-1 {
        (onestep,twosteps) = (onestep+twosteps, onestep);
    }
    onestep
}

fn main() {
    let mut memo = [-1i32;7];
    memo[6]=0;
    memo[5]=1;
    println!("{}",climb(0,5));
    println!("{}",climb_memo(0, &mut memo));
    println!("{:?}",memo);
    println!("{}",climb_dp(5));

    let mut memo2 = [-1i32;20];
    memo2[19]=0;
    memo2[18]=1;
    println!("{}",climb(0,18));
    println!("{}",climb_memo(0, &mut memo2));
    println!("{}",climb_dp(18));


}

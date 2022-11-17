use rand::prelude::SliceRandom;
use rand::thread_rng;


fn merge_count(tomerge: &mut [i32], l: usize, m:usize, r:usize) -> u32{
        let mut count=0u32;
        let mut temp  = Vec::new();
        let mut i = l;
        let mut j = m+1;
        while i<=m && j<=r {
            if tomerge[i]<tomerge[j] {
                temp.push(tomerge[i]);
                i+=1;
            } else {
                temp.push(tomerge[j]);
                j+=1;
                count+= (m-i+1) as u32;
            }
        }
        while i<=m {
            temp.push(tomerge[i]);
            i+=1;
        }
        while j<=r {
            temp.push(tomerge[j]);
            j+=1;
        }
        i=0;
        for c in l..r+1 {
            tomerge[c] = temp[i];
            i+=1;
        }
    count
}

fn merge_sort(tosort: &mut [i32], l:usize, r:usize) -> u32 {
    if l<r {
        let m = (l+r) / 2;
        let c1 = merge_sort(tosort,l,m);
        let c2 = merge_sort(tosort,m+1,r);
        let c3 = merge_count(tosort,l,m,r);
        return c1 + c2 + c3;
    }
    return 0;
}

fn naive(v: &[i32], l: usize, r:usize) -> u32 {
    let mut c = 0;
    for i in l..r {
        for j in i+1..r+1 {
            if v[i]>v[j] {
                c+=1;
            }
        }
    }
    c
}



#[test]
fn test_merge_count() {
    let mut v: Vec<i32> = vec![1,3,6,8,2,4,5,7];
    let c =  merge_count(v.as_mut_slice(),0,3,7);
    assert!(c==8,"{} {:?}",c,v)

}

#[test]
fn test_count_stress() {
    let mut s: Vec<i32> = (0..1000).collect();

    for _ in 1..10 {
        s.shuffle(&mut thread_rng());
        let n = naive(s.as_slice(),0,999);
        let ms = merge_sort(s.as_mut_slice(),0,999);
        assert!(n==ms,"Error: {} {}",n,ms);
    }
}


fn main() {
    //let mut v: Vec<i32> = vec![12,1,4,6,7,2,5,8,9];
    let mut v: Vec<i32> = vec![12,1,2];
    println!("{}",naive(v.as_slice(),0,2));
    println!("{}",merge_sort(v.as_mut_slice(),0,2));

}

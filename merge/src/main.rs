use rand::prelude::SliceRandom;
use rand::thread_rng;


fn merge(tomerge: &mut [i32], l: usize, m:usize, r:usize) {
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
}

#[test]
fn test_merge_1() {
    let mut t1: Vec<i32> = vec![1,4,6,7,2,5,8,9];
    assert!(is_sorted(t1.as_mut_slice(),0,3),"Testset fail");
    assert!(is_sorted(t1.as_mut_slice(),4,7),"Testset fail");
    merge(t1.as_mut_slice(),0,3,7);
    assert!(is_sorted(t1.as_mut_slice(),0,7),"Merge 1 fail");
}


#[test]
fn test_merge_2() {
    let mut t2: Vec<i32> = vec![1,5,9,2,3,4,5,7];
    assert!(is_sorted(t2.as_mut_slice(),0,2),"Testset fail");
    assert!(is_sorted(t2.as_mut_slice(),3,7),"Testset fail");
    merge(t2.as_mut_slice(),0,2,7);
    assert!(is_sorted(t2.as_mut_slice(),0,7),"Merge t2 fail");
}

#[test]
fn test_merge_3() {
    let mut t3: Vec<i32> = vec![8,9,6,7,4,5,1,2,3];
    merge(t3.as_mut_slice(),0,1,3);
    assert!(is_sorted(t3.as_mut_slice(),0,3),"Testset fail");
    merge(t3.as_mut_slice(),4,5,8);
    assert!(is_sorted(t3.as_mut_slice(),4,8),"Testset fail");
    merge(t3.as_mut_slice(),0,3,8);
    assert!(is_sorted(t3.as_mut_slice(),0,8),"Merge t3 fail");
}

#[test]
fn test_sort_1() {
    let mut s: Vec<i32> = vec![3,6,2,1,4,8,9,0,6,7];
    merge_sort(s.as_mut_slice(),0,9);
    assert!(is_sorted(s.as_mut_slice(),0,9));
}

#[test]
fn test_sort_stress() {
    let mut s: Vec<i32> = (0..1000).collect();

    for _ in 1..10 {
        s.shuffle(&mut thread_rng());
        merge_sort(s.as_mut_slice(),0,999);
        for i in  0..1000 {
            assert!(s[i]==i.try_into().unwrap(),"{:?}",s);
        }
    }
}


fn merge_sort(tosort: &mut [i32], l:usize, r:usize) {
    if l<r {
        let m = (l+r) / 2;
        merge_sort(tosort,l,m);
        assert!(is_sorted(tosort,l,m),"left vector {} {}",l,m);
        merge_sort(tosort,m+1,r);
        assert!(is_sorted(tosort,m+1,r),"right vector {} {}",m+1,r);
        merge(tosort,l,m,r);
        assert!(is_sorted(tosort,m+1,r),"merge {} {} {}",l,m,r);
    }
}

fn is_sorted(v: &[i32], l:usize, r:usize) -> bool {
    assert!(l<=r);
    if l==r {
        return true;
    }
    for i in l..r-1 {
        if v[i]>v[i+1] {
            return false;
        }
    }
    return true;
}


fn main() {
    let mut t2: Vec<i32> = vec![3,6,2,1,4,8,9,0,6,7];
    merge_sort(t2.as_mut_slice(),0,9);
    println!("{:?}",t2);
}

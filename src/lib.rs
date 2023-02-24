pub fn partition_range_vec<T0, T1, T2, T3, T4>(vec: &Vec<(T0, T1, T2, T3, T4)>, lower_needle: &(&T0, &T1, &T2, &T3, &T4), higher_needle: &(&T0, &T1, &T2, &T3, &T4)) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    let lower = vec[..].partition_point(|(ref e0, ref e1, ref e2, ref e3, ref e4)| {
        (e0, e1, e2, e3, e4).cmp(lower_needle).is_lt()
    });
    let higher = vec[lower..].partition_point(|(ref e0, ref e1, ref e2, ref e3, ref e4)| {
        (e0, e1, e2, e3, e4).cmp(higher_needle).is_lt()
    });
    (lower, higher)
}

pub fn partition_range_vec_segmented<T0, T1, T2, T3, T4>(vec: &Vec<(T0, T1, T2, T3, T4)>, lower_needle: &(&T0, &T1, &T2, &T3, &T4), higher_needle: &(&T0, &T1, &T2, &T3, &T4)) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    let lower = vec[..].partition_point(|(ref e0, _, _, _, _)| {
        e0.cmp(lower_needle.0).is_lt()
    });

    let higher = vec[lower..].partition_point(|(ref e0, _, _, _, _)| {
        e0.cmp(higher_needle.0).is_lt()
    });


    let lower = vec[lower..higher].partition_point(|(_, ref e1, _, _, _)| {
        e1.cmp(lower_needle.1).is_lt()
    });

    let higher = vec[lower..higher].partition_point(|(_, ref e1, _, _, _)| {
        e1.cmp(higher_needle.1).is_lt()
    });

    let lower = vec[lower..higher].partition_point(|(_, _, ref e2, _, _)| {
        e2.cmp(lower_needle.2).is_lt()
    });

    let higher = vec[lower..higher].partition_point(|(_, _, ref e2, _, _)| {
        e2.cmp(higher_needle.2).is_lt()
    });

    let lower = vec[lower..higher].partition_point(|(_, _, _, ref e3, _)| {
        e3.cmp(lower_needle.3).is_lt()
    });

    let higher = vec[lower..higher].partition_point(|(_, _, _, ref e3, _)| {
        e3.cmp(higher_needle.3).is_lt()
    });

    let lower = vec[lower..higher].partition_point(|(_, _, _, _, ref e4)| {
        e4.cmp(lower_needle.4).is_lt()
    });

    let higher = vec[lower..higher].partition_point(|(_, _, _, _, ref e4)| {
        e4.cmp(higher_needle.4).is_lt()
    });

    (lower, higher)
}

pub fn partition_range_soa<T0, T1, T2, T3, T4>(soa: &(Vec<T0>, Vec<T1>, Vec<T2>, Vec<T3>, Vec<T4>), lower_needle: &(&T0, &T1, &T2, &T3, &T4), higher_needle: &(&T0, &T1, &T2, &T3, &T4)) -> (usize, usize)
where
    T0: Ord,
    T1: Ord,
    T2: Ord,
    T3: Ord,
    T4: Ord,
{
    let lower = soa.0[..].partition_point(|e0| {
        e0.cmp(lower_needle.0).is_lt()
    });

    let higher = soa.0[lower..].partition_point(|e0| {
        e0.cmp(higher_needle.0).is_lt()
    });


    let lower = soa.1[lower..higher].partition_point(|e1| {
        e1.cmp(lower_needle.1).is_lt()
    });

    let higher = soa.1[lower..higher].partition_point(|e1| {
        e1.cmp(higher_needle.1).is_lt()
    });

    let lower = soa.2[lower..higher].partition_point(|e2| {
        e2.cmp(lower_needle.2).is_lt()
    });

    let higher = soa.2[lower..higher].partition_point(|e2| {
        e2.cmp(higher_needle.2).is_lt()
    });

    let lower = soa.3[lower..higher].partition_point(|e3| {
        e3.cmp(lower_needle.3).is_lt()
    });

    let higher = soa.3[lower..higher].partition_point(|e3| {
        e3.cmp(higher_needle.3).is_lt()
    });

    let lower = soa.4[lower..higher].partition_point(|e4| {
        e4.cmp(lower_needle.4).is_lt()
    });

    let higher = soa.4[lower..higher].partition_point(|e4| {
        e4.cmp(higher_needle.4).is_lt()
    });

    (lower, higher)
}

pub fn aos_to_soa<T0, T1, T2, T3, T4>(aos: Vec<(T0, T1, T2, T3, T4)>) -> (Vec<T0>, Vec<T1>, Vec<T2>, Vec<T3>, Vec<T4>) {
    let len = aos.len();
    let mut soa = (Vec::with_capacity(len), Vec::with_capacity(len), Vec::with_capacity(len), Vec::with_capacity(len), Vec::with_capacity(len));

    for (e0, e1, e2, e3, e4) in aos {
        soa.0.push(e0);
        soa.1.push(e1);
        soa.2.push(e2);
        soa.3.push(e3);
        soa.4.push(e4);
    }
    soa
}


fn main() {
    println!("Hello, world!");
}







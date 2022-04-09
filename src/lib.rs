pub fn find<T>(array: &[T], key: T) -> Option<usize> where T: AsRef<[T]>{

    let mut p=0;
    let l= array.len();

    //array vuoto
    if l==0 {return None;}

    let mut r =l-1;
    //Key non appartiene all'array
    if key<array[p] || key>array[r] {  return None; }

    while p<=r {

        let q=(p+r)/2;

        if array[q]==key { return Some(q);}
        if array[q]>key{
            r=q-1;
        }else {
            p=q+1; }
    }

    return None;
}


/*function binarySearchIterativo(array A, int p, int r, int v)
    if v < A[p] or v > A[r]
      return -1   -- vuol dire che v non è presente in A
    while p <= r
      q=(p+r)/2
      if A[q] == v
        return q
      if A[q] > v
        r = q-1
      else
        p = q+1
     return -1*/
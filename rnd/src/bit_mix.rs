#[inline]
pub const fn stafford_mix13(mut v: i64) -> i64 {
    v = (v ^ (v as u64 >> 30) as i64).wrapping_mul(0xbf58476d1ce4e5b9u64 as i64);
    v = (v ^ (v as u64 >> 27) as i64).wrapping_mul(0x94d049bb133111ebu64 as i64);
    v ^ (v as u64 >> 31) as i64
}

#[inline]
pub const fn rrxmrrxmsx(mut v: i64) -> i64 {
    v ^= ((v as u64 >> 25) as i64 | (v << 39)) ^ ((v as u64 >> 50) as i64 | (v << 14));
    v = v.wrapping_mul(0xa24baed4963ee407u64 as i64);
    v ^= ((v as u64 >> 24) as i64 | (v << 40)) ^ ((v as u64 >> 49) as i64 | (v << 15));
    v = v.wrapping_mul(0x9fb21c651e98df25u64 as i64);
    v ^ (v as u64 >> 28) as i64
}

#[cfg(test)]
mod bit_mix_tests {
    use super::*;

    #[test]
    fn test_stafford_mix13() {
        let l1 = -3222165538581252362i64;
        let l2 = -6575083715474529190i64;
        let l3 = 5852023251876651789i64;
        let l4 = -2105427755758183442i64;
        let l5 = 8768027636157682880i64;
        let l1 = stafford_mix13(l1);
        let l2 = stafford_mix13(l2);
        let l3 = stafford_mix13(l3);
        let l4 = stafford_mix13(l4);
        let l5 = stafford_mix13(l5);
        println!("l1: {}", l1);
        println!("l2: {}", l2);
        println!("l3: {}", l3);
        println!("l4: {}", l4);
        println!("l5: {}", l5);
        assert_eq!(l1, -364480833079461142i64);
        assert_eq!(l2, -2195553744329346567i64);
        assert_eq!(l3, 8519880896858805593i64);
        assert_eq!(l4, 6042095800684428140i64);
        assert_eq!(l5, 7558165513295548834i64);
    }

    #[test]
    fn test_rrxmrrxmsx() {
        let l1 = 8437077494049660495i64;
        let l2 = 9148403451769538754i64;
        let l3 = -5204784484909980772i64;
        let l4 = 3756336564354234498i64;
        let l5 = -4784683118053631234i64;
        let l1 = rrxmrrxmsx(l1);
        let l2 = rrxmrrxmsx(l2);
        let l3 = rrxmrrxmsx(l3);
        let l4 = rrxmrrxmsx(l4);
        let l5 = rrxmrrxmsx(l5);
        println!("l1: {}", l1);
        println!("l2: {}", l2);
        println!("l3: {}", l3);
        println!("l4: {}", l4);
        println!("l5: {}", l5);
        assert_eq!(l1, -1074278017023486638i64);
        assert_eq!(l2, 791467320547438801i64);
        assert_eq!(l3, -6237844429596446841i64);
        assert_eq!(l4, 967439720212533408i64);
        assert_eq!(l5, 4511633004553689581i64);
    }
}

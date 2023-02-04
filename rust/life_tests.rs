#[cfg(test)]
mod life_tests {
    use crate::life::life::*;
    #[test]
    fn test_life_init() {
        let l: Life = Life::new();
        assert_eq!(format!("{l}"), "(Life x:lists=3 *LEN[0]=0 len[1]=0 len[2]=0 +)");
    }
}

extern crate parcoursdb;

#[cfg(test)]
mod test {
    use parcoursdb::seasons::repository::all;

    #[test]
    fn test_all() {
       let mut expected:Vec<i32> = (1896 .. 1915).collect();
       for yr in 1917 .. 2019 + 1 {
           expected.push(yr);
       }
       assert_eq!(all(), expected);
    }
}

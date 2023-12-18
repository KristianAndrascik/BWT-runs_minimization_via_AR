#[cfg(test)]
mod tests {
    use crate::{bwt::bwt, count_runs::count_runs, min_runs::min_runs};
   
    #[test]
    fn basic1() {

        let input = String::from("aabaaabac");

       
        let bwt = bwt(&input);
        assert_eq!(
            bwt,"bcaaabaaa",
            "basic BWT creation is not correct",
        );


        let bwt_number_of_runs = count_runs(&*bwt);
        assert_eq!(
            bwt_number_of_runs,5,
            "BWT number of runs calculation is not correct",
        );


        let min_number_of_runs = min_runs(&input);
        assert_eq!(
            min_number_of_runs,3,
            "minimal number of runs calculation is not correct",
        );
        
        
    }


    // Problem with big alphabet size; this is biggest as we can go with current implementation :()
    #[test]
    fn bigger_alphabet_no_match() {

        let input = String::from("Lorem ");

        
        let bwt = bwt(&input);
        assert_eq!(
            bwt,"m reLo",
            "basic BWT creation is not correct",
        );

       
        let bwt_number_of_runs = count_runs(&*bwt);
        assert_eq!(
            bwt_number_of_runs,6,
            "BWT number of runs calculation is not correct",
        );

       
        let min_number_of_runs = min_runs(&input);
        assert_eq!(
            min_number_of_runs,6,
            "minimal number of runs calculation is not correct",
        );
        
        
    }

    #[test]
    fn basic2() {

        let input = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbccccccccccccccccccccccccccccccccccccccc");

        let bwt = bwt(&input);
        assert_eq!(
            bwt,"caaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbccccccccccccccccccccccccccccccccccccccb",
            "basic BWT creation is not correct",
        );


        let bwt_number_of_runs = count_runs(&*bwt);
        assert_eq!(
            bwt_number_of_runs,5,
            "BWT number of runs calculation is not correct",
        );


        let min_number_of_runs = min_runs(&input);
        assert_eq!(
            min_number_of_runs,3,
            "minimal number of runs calculation is not correct",
        );
        
        
    }


    #[test]
    fn dna_sequence() {

        let input = String::from("TGCGCAGGCT$");

        let bwt = bwt(&input);
        assert_eq!(
            bwt,"TCGGGCTGAC$",
            "basic BWT creation is not correct",
        );


        let bwt_number_of_runs = count_runs(&*bwt);
        assert_eq!(
            bwt_number_of_runs,9,
            "BWT number of runs calculation is not correct",
        );


        let min_number_of_runs = min_runs(&input);
        assert_eq!(
            min_number_of_runs,7,
            "minimal number of runs calculation is not correct",
        );
        
        
    }
  

}

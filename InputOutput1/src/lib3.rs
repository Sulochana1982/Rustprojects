fn search<'a>(query: &str, contents:& 'a str) -> Vec<& 'a str>
{
    let mut results = Vec::new();
   for line in contents.lines()
   {
    if line.contains(&query)
    {
        let line= line.trim();
       results.push(line);
    }
   }
   results
}

fn search_case_insensitivity<'a>(query: &str, contents:& 'a str) -> Vec<& 'a str>
{    
    let query= query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines()
    {
     if line.to_lowercase().contains(&query)
     {
         let line= line.trim();
        results.push(line);
     }
    }
    results

}

#[cfg(test)]
mod test{
    use super::*;

    // #[test]
    // fn one_result(){
    //     //let query = "dust";
    //     let query = "safe";
    //     let contents = 
    //     "\
    //     Rust:
    //     safe, fast, productive
    //     Pick Three
    //     trust me";

    //     assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    // }
    #[test]
    fn one_result(){
        //let query = "dust";
        let query = "rUsT";
        let contents = 
        "\
        Rust
        safe, fast, productive
        Pick Three.
        Trust me";

        assert_eq!(vec!["Rust", "Trust me"], search_case_insensitivity(query, contents));
    }

    
}
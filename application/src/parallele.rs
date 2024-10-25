use crate::HashMap;

pub fn test()
{
    let mut map: Option<HashMap<String, usize>> = None; 
    let mut other = None; 
    rayon::scope(|s| 
    {
        s.spawn(|_s| 
        {
            let iter = (0..10000).enumerate().map(|(a, b)| (format!("index {}", a), b)); 
            map = Some(HashMap::from_iter(iter)); 
        });

        s.spawn(|_s| 
        {
            other = Some("value") 
        }) 
    }); 

    // println!("PARALLELE : map {:?}", map.unwrap()); 
    println!("PARALLELE : other {:?}", other); 
}

use std::{collections::HashMap, fmt::{write, Display}, hash::Hash};

struct Graph<T>{
vertex : T,
neighbors : Box<Vec<Graph<T>>>
}
fn main()
{
let two = Graph{
    vertex : String::from("Azeddine"),
    neighbors : Box::new(vec![])
};

let four = Graph{

    vertex : String::from("Hamza") , 
    neighbors : Box::new(vec![])
};
let three = Graph{
vertex : String::from("laila"),
neighbors : Box::new(vec![])
};
let one  = Graph{
    vertex : String::from("Aicha"),
    neighbors : Box::new(vec![two,three,four])
};
let mut hm :HashMap<String,bool> = HashMap::new(); 
DFS(one, &mut hm);

}
fn DFS<T:Display+Clone>(root : Graph<T> ,Visited : &mut HashMap<T,bool>)
where T : Eq , T : Hash
{
let state = Visited.get(&root.vertex);
if state.is_some() {return;}
Visited.insert(root.vertex.clone(), true);
println!("{}",root.vertex);
if root.neighbors.len()==0{
return;
}
for neighbor in *root.neighbors{
DFS(neighbor,Visited);
}

}



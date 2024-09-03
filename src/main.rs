use std::{collections::HashMap, fmt::{write, Display}, hash::Hash};
use queues::*;
struct Graph<T>{
vertex : T,
neighbors : Box<Vec<Graph<T>>>
}


impl<T : Display+Clone> Graph<T>{
 
    
fn DFS(&self,Visited : &mut HashMap<T,bool>)
where T : Eq , T : Hash
{
    let root = self;
    let state = Visited.get(&root.vertex);
    if state.is_some() {return;}
    Visited.insert(root.vertex.clone(), true);
    println!("{}",root.vertex);
    if root.neighbors.len()==0{
    return;
}
    for neighbor in &*root.neighbors{
    neighbor.DFS(Visited);
}}

fn BFS(&self,Visited : &mut HashMap<T,bool>)
where T:Eq,T:Hash
{   
    let mut q : Vec<&Graph<T>> = Vec::new();
    q.push(self);
    while q.len()!=0{
    let current  = q.remove(0);
    println!("{}",current.vertex);
    for neighbor in &*current.neighbors{
        let state = Visited.get(&neighbor.vertex);
        if state.is_none(){
            Visited.insert(neighbor.vertex.clone(),true);
            q.push(neighbor);
        }
    }
    }
}
}
fn main()
{
let four = Graph{

    vertex : String::from("child3") , 
    neighbors : Box::new(vec![])
};
let two = Graph{
    vertex : String::from("child1"),
    neighbors : Box::new(vec![four])
};


let three = Graph{
vertex : String::from("child2"),
neighbors : Box::new(vec![])
};
let one  = Graph{
    vertex : String::from("mother"),
    neighbors : Box::new(vec![two,three])
};
let mut hm :HashMap<String,bool> = HashMap::new(); 
one.DFS( &mut hm);
println!("Now BFS");
hm.clear();
one.BFS(&mut hm);

}


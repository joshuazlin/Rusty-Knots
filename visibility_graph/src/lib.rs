/*
This hopefully defines what's needed for a visibility graph.
This is defined by a number of vertices in the plane, and a number of edges connecting vertices.
The graph has an edge connecting every pair of vertices that can see each other. 
So really, it's just a list of edges. 
It probably needs a little something to tell it how precise to be too. 
*/

extern crate rand;

use std::ops;
use std::fmt;
use rand::Rng;



pub struct Point { pub x : f64, pub y : f64,}

impl Point {
    fn distance(&self, v : &Point) -> f64{
        ((self.x - v.x).powf(2.0) + (self.y - v.y).powf(2.0)).powf(0.5)
    }
}

impl std::ops::Sub for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}


pub struct Edge<'a> (&'a Point,&'a Point);

impl Edge<'_> {
    fn length(&self) -> f64{
        (self.0).distance(&self.1)
    }

    fn orientation(&self, v : &Point, eps : f64) -> Result<bool, String>{
        //Finds the orientation of v with respect to this edge. 
        //essentially, is v on the right of e.0 -> e.1?

        let edge_vec = (self.1) - (self.0);
        let point_vec = v - self.0;

        let det = (edge_vec.x*point_vec.y)-(edge_vec.y*point_vec.x);
        if det.abs() > eps{
            if det > 0.0{return Ok(true)}
            else{return Ok(false)}
        }

        Err("Everything in a line!".to_string())
    }

    fn intersect(&self, e : &Edge, eps : f64) -> Result<bool, String>{
        //checks if this edge intersects with the edge "f".
        //Essentially, are the vertices of e on either side of self, and
        //are the vertices of self on either side of e?

        let or1_1 = self.orientation(e.0, eps);
        let or1_2 = self.orientation(e.1, eps);

        let or2_1 = e.orientation(self.0, eps);
        let or2_2 = e.orientation(self.1, eps);

        match (or1_1,or1_2){
            (Ok(a),Ok(b)) => if a ^ b == false{return Ok(false)}
                             else {match (or2_1,or2_2){
                                        (Ok(c),Ok(d)) => if c ^ d == false{return Ok(false)}
                                                         else {return Ok(true)},
                                        _ => {},
                                   };},
            _ => match (or2_1,or2_2){
                (Ok(a),Ok(b)) => if a^b == false{return Ok(false)},
                _ => {},
            },
        };

        if (self.0.x.max(self.1.x) < e.0.x.min(e.1.x)-eps) | (self.0.x.min(self.1.x) > e.0.x.max(e.1.x)+eps){
            return Ok(false)
        }
        if (self.0.y.max(self.1.y) < e.0.y.min(e.1.y)-eps) | (self.0.y.min(self.1.y) > e.0.y.max(e.1.y)+eps){
            return Ok(false)
        }

        Err("Precision too low to determine intersection of segments".to_string())
    }
}


#[cfg(test)]
mod test_point {
    use super::*;

    #[test]
    fn init_point(){
        let x = Point{ 
            x : 2.0,
            y : 3.0,
        };
    }

    #[test]
    fn check_distance(){
        let x = Point{x : 2.0, y : 3.0};
        let y = Point{x : 3.0, y : 2.0};
        if (x.distance(&y) - (2.0_f64).powf(0.5)) >= 0.01{
            panic!("uh oh!")
        }
    }

    #[test]
    fn check_intersect(){
        let v1 = Point{x : 0.0,y:0.0,};
        let v2 = Point{x : 0.0,y:1.0,};
        let v3 = Point{x : 1.0,y:0.0,};
        let v4 = Point{x : 1.0,y:1.0,};

        assert!(!Edge(&v1,&v2).intersect(&Edge(&v3,&v4),0.1).unwrap());
        assert!(!Edge(&v2,&v1).intersect(&Edge(&v3,&v4),0.1).unwrap());
        assert!(Edge(&v1,&v4).intersect(&Edge(&v2,&v3),0.1).unwrap());
        assert!(Edge(&v4,&v1).intersect(&Edge(&v3,&v2),0.1).unwrap());

        let w0 = Point{x : 0.0,y:0.0,};
        let w1 = Point{x : 1.0,y:1.0,};
        let w2 = Point{x : 2.0,y:1.0,};
        let w4 = Point{x : 2.0,y:2.0,};

        assert!(!Edge(&w0,&w1).intersect(&Edge(&w2,&w4),0.01).unwrap());

    }
}


pub struct VisibilityGraph {
    eps : f64,
    //vertices : &'a Vec<&'a Point>,
    pub vertices : Vec<Point>,
    pub physical_edges : Vec<(usize,usize)>, //indexes elements in vertices. 
    pub visibility_edges : Vec<(usize,usize)>,
}

impl VisibilityGraph {

    fn new(eps : f64) -> VisibilityGraph{
        VisibilityGraph{eps:eps, vertices : vec![], physical_edges : vec![], visibility_edges : vec![]}
    }

    fn add_point(&mut self, v : Point){
        //Adds a point to the visibility graph 

        for (i,w) in self.vertices.iter().enumerate(){
            let temp_edge = Edge(&v,w);
            if self.physical_edges.iter().all(|e| !temp_edge.intersect(&Edge(&self.vertices[e.0],&self.vertices[e.1]),self.eps).unwrap_or(true)){
                self.visibility_edges.push((self.vertices.len(),i));
            }
        }
        self.vertices.push(v);
    }

    fn edge_ok(&self, e : (usize,usize),f : &(usize,usize)) -> bool{
        
        if (e.0 == f.0) | (e.1 == f.0) | (e.0 == f.1) | (e.1 == f.1){
            return true
        }

        let temp_e = Edge(&self.vertices[e.0],&self.vertices[e.1]);
        let temp_f = Edge(&self.vertices[f.0],&self.vertices[f.1]);
        
        !temp_e.intersect(&temp_f, self.eps).unwrap_or(true)

    }


    fn add_edge(&mut self, e : (usize,usize)){
        //Adds a physical_edge between two vertices
        //assert!(i < j);
        let temp_e = Edge(&self.vertices[e.0],&self.vertices[e.1]);

        self.visibility_edges = self.visibility_edges
                                    .iter()
                                    .filter(|f| self.edge_ok(e,*f))
                                    .map(|f| *f)
                                    .collect();

        self.physical_edges.push(e);
    }

    pub fn random(width: f64, height:f64, num_verts: u32, num_edges:u32, eps:f64) -> VisibilityGraph{
        //Everything is constrained to live within [0,width]*[0,height]
        //num_verts is the number of vertices 
        //this algorithm is big dumb, and will try to have physical edges between 0->1, 1->2, bla bla 
        //until we saturate num_edges
        //So, we might have less than num_edges number of edges

        let mut rng = rand::thread_rng();

        //let points : [f64;num_verts] = [rng.gen();num_verts];
        let mut g = VisibilityGraph::new(eps);
        for i in 1..num_verts{
            let xf : f64 = rng.gen();
            let yf : f64 = rng.gen();
            g.add_point(Point{x:width*xf,y:height*yf});
        }

        let mut num_added_edges = 0;
        for i in 0..num_verts-2{
            //let new_edge = (i,i+1);
            if !g.physical_edges.iter().any(
                |e| Edge(&g.vertices[e.0],&g.vertices[e.1]).intersect(
                        &Edge(&g.vertices[i as usize],&g.vertices[(i+1) as usize]),eps).unwrap_or(true)){
                //g.physical_edges.push((i as usize, (i+1) as usize));
                g.add_edge((i as usize, (i+1) as usize));
                num_added_edges = num_added_edges + 1;
                if num_added_edges >= num_edges{
                    break;
                }
            }
        }
        g
    }


}

impl fmt::Debug for VisibilityGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Visibility Graph")
         .field("eps", &self.eps)
         .field("vertices", &self.vertices)
         .field("physical_edges", &self.physical_edges)
         .field("visibility_edges", &self.visibility_edges)
         .finish()
    }
}


#[cfg(test)]
mod test_visibility_graph {
    use super::*;

    #[test]
    fn test_graph(){
        
        let mut g = VisibilityGraph{
            eps: 0.01,
            vertices : vec![],
            physical_edges : vec![],
            visibility_edges : vec![],
        };

        g.add_point(Point{x:0.0,y:0.0,});
        g.add_point(Point{x:1.0,y:1.0,});
        g.add_point(Point{x:2.0,y:1.0,});
        g.add_point(Point{x:1.0,y:2.0,});
        g.add_point(Point{x:2.0,y:2.0,});
        g.add_point(Point{x:3.0,y:3.0,});

        g.add_edge((1,2));
        println!("{:?}",g.visibility_edges);
        g.add_edge((1,3));
        println!("{:?}",g.visibility_edges);
        g.add_edge((2,4));
        println!("{:?}",g.visibility_edges);
        g.add_edge((3,4));
        println!("{:?}",g.visibility_edges);

    }

    #[test]
    fn test_random_graph(){
        let g = VisibilityGraph::random(500.0,500.0,100,20,0.01);
    }

}


#[cfg(test)]
mod test_random{
    use super::*;

    #[test]
    fn test_see(){
        
    }
}

//fn main(){}


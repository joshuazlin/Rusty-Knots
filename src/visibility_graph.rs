/*
This hopefully defines what's needed for a visibility graph.
This is defined by a number of vertices in the plane, and a number of edges connecting vertices.
The graph has an edge connecting every pair of vertices that can see each other. 
So really, it's just a list of edges. 
It probably needs a little something to tell it how precise to be too. 
*/

use std::ops;
use std::fmt;

pub struct Point {
    x : f64,
    y : f64,
}

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
        
        //let orient1 = det(&self)
        //(self.1.x-self.0.x)*(v.y) - (self.1.y-self.0.y)*(v.x)

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
        Ok((self.orientation(e.0, eps).unwrap() ^ self.orientation(e.1,eps).unwrap()) & 
            (e.orientation(self.0,eps).unwrap() ^ e.orientation(self.1,eps).unwrap()))
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
    }
}


pub struct VisibilityGraph {
    eps : f64,
    //vertices : &'a Vec<&'a Point>,
    vertices : Vec<Point>,
    physical_edges : Vec<(usize,usize)>, //indexes elements in vertices. 
    visibility_edges : Vec<(usize,usize)>,
}

impl VisibilityGraph {
    fn add_point(&mut self, v : Point){
        //Adds a point to the visibility graph 

        for (i,w) in self.vertices.iter().enumerate(){
            let temp_edge = Edge(&v,w);
            if self.physical_edges.iter().all(|e| !temp_edge.intersect(&Edge(&self.vertices[e.0],&self.vertices[e.1]),self.eps).unwrap()){
                self.visibility_edges.push((self.vertices.len(),i));
            }
        }
        self.vertices.push(v);
    }

    fn add_edge(&mut self, i : usize, j : usize){
        //Adds a physical_edge between two vertices
        //assert!(i < j);
        let temp_edge = Edge(&self.vertices[i],&self.vertices[j]);

        //self.visibility_edges = self.visibility_edges
        //                            .iter()
        //                            .filter(|e| temp_edge.intersect(&Edge(&self.vertices[e.0],&self.vertices[e.1]),self.eps).unwrap());
        
        self.visibility_edges = self.visibility_edges
                                    .iter()
                                    .filter(|e| !((**e == (i,j)) | (**e == (j,i)))
                                              & temp_edge.intersect(&Edge(&self.vertices[e.0],&self.vertices[e.1]),self.eps).unwrap()  
                                )
                                    .map(|e| *e)
                                    .collect();//::<Vec<(usize,usize)>>()

        self.physical_edges.push((i,j));
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
            eps: 0.1,
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

        g.add_edge(1,2);
        g.add_edge(1,3);
        g.add_edge(2,4);
        g.add_edge(3,4);

        println!("{:?}",g);

    }

}




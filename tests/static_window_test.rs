
//extern crate static_window;
//mod src/static_window;
use test_1;


fn main(){

    let mut w = StaticWindow::new();
    w.items.push(GraphicsElement::LineElement{x1 : 1.0,
                                                y1 : 2.0,
                                                x2 : 3.0,
                                                y2 : 4.0,
                                                radius:0.1,
                                                dashed:false,
                                                c:CommonColors::Blue});
    w.draw();

}
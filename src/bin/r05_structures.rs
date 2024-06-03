struct ShippingBox{
    depth: i32,
    width: i32,
    height: i32
}

impl ShippingBox {
    fn new(depth:i32, width:i32, height:i32)->Self{
        Self{
            depth,
            width,
            height
        }
    }
}

fn main(){
    let shipping_box = ShippingBox::new( 40, 50, 60);
    println!( 
        "depth: {:?},\nwidth: {:?},\nheight: {:?}",
        shipping_box.depth, 
        shipping_box.height,
        shipping_box.width
    )

}
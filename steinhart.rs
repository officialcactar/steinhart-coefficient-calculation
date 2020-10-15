fn main(){

    //input three points from thermistor datasheet
    let r:[f64;3] = [239830.0,10000.0,237.98];
    let t:[f64;3] = [-40.0,25.0,150.0];

    let mut l:[f64;3] = [0.0; 3];
    let mut y:[f64;3] = [0.0; 3];

    for i in 0..3 {
        y[i] = 1.0 / (t[i] + 273.15);
        l[i] = r[i].ln();
    }

    let g:[f64;2] = [(y[1]-y[0])/(l[1]-l[0]),(y[2]-y[0])/(l[2]-l[0])];

    let c:f64 = ((g[1]-g[0])/(l[2]-l[1]))*(l[0]+l[1]+l[2]).powf(-1.0);
    let b:f64 = g[0] - c * ((l[0].powf(2.0)+(l[0]*l[1])+l[1].powf(2.0)));
    let a:f64 = y[0]-((b+(l[0].powf(2.0)*c))*l[0]);

    println!("A:{}\nB:{}\nC:{}", a, b, c);
}

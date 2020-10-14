fn main(){

    //input info from data sheet
    let r1:f64 = 239830.0;
    let mut t1:f64 = -40.0;
    let r2:f64 = 10000.0;
    let mut t2:f64 = 25.0;
    let r3:f64 = 237.98;
    let mut t3:f64 = 150.0;

    t1 = t1 + 273.15;
    t2 = t2 + 273.15;
    t3 = t3 + 273.15;

    let l1:f64 = r1.ln();
    let l2:f64 = r2.ln();
    let l3:f64 = r3.ln();

    let y1:f64 = 1.0 / t1;
    let y2:f64 = 1.0 / t2;
    let y3:f64 = 1.0 / t3;

    let gamma2:f64 = (y2 - y1) / (l2 - l1);
    let gamma3:f64 = (y3 - y1) / (l3 - l1);

    let c:f64 = (( gamma3 - gamma2) / (l3 - l2)) * (l1 + l2 + l3).powf(-1.0);
    let b:f64 = gamma2 - c * ((l1.powf(2.0) + ( l1 * l2 ) + l2.powf(2.0)));
    let a:f64 = y1 - (( b + ( l1.powf(2.0) * c) ) * l1);

    let mut i:f64 = 254.15;

    while i < 373.15 {
        i += 1.0;
        let x:f64 = (1.0 / (2.0 * c)) * (a - (1.0 / i));
        let y:f64 = (( b / (3.0 * c)).powf(3.0) + x.powf(2.0)).sqrt();
        let rr:f64 = (( y - x ).cbrt() - ( y + x).cbrt()).exp();
        let _cel:f64 = i - 273.15;

        println!("T:{:.0}C R:{:.10}", _cel, rr);
    }

    println!("\nA: {}\nB: {}\nC: {}", a, b, c);
}

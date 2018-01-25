use std::io;
use std::f32;

#[derive(Clone, Copy)]
struct Ray {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    A1: f32,
    B1: f32,
    C1: f32,
}
impl Ray {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Ray {
        Ray{
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            A1: y1-y2,
            B1: x2-x1,
            C1: x1*y2-x2*y1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    A2: f32,
    B2: f32,
    C2: f32,
}
impl Line {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line{
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            A2: y1-y2,
            B2: x2-x1,
            C2: x1*y2-x2*y1,
        }
    }
}

fn det(a1: f32, b1: f32, a2: f32, b2: f32) -> f32{
  return a1*b2-a2*b1;
}

fn intersection(mut ray: Ray, line: Line) -> (bool, f32){
    let temp = det(ray.A1, ray.B1, line.A2, line.B2);
    if temp == 0.0 {
      return (false, 0.0); // прямые паралельны или совпадают
    }
    else {
    let pointX = -( det(ray.C1, ray.B1, line.C2, line.B2) / temp); // точка пересечения
    let pointY = -( det(ray.A1, ray.C1, line.A2, line.C2) / temp);
    
    if (pointX >= line.x1 && pointX <= line.x2) || (pointX <= line.x1 && pointX >= line.x2){  // принадлежит ли точка пересечения отрезку
      if (pointY >= line.y1 && pointY <= line.y2) || (pointY <= line.y1 && pointY >= line.y2) {
        if ( (pointX > ray.x1) == (ray.x2 > ray.x1) ) && ( (pointY > ray.y1) == (ray.y2 > ray.y1) ) { // точка лежит на стороне луча
          let distance = ((pointX - ray.x1).powf(2.0) + (pointY - ray.y1).powf(2.0)).sqrt();
          return(true, distance);
        }
        else {
          return (false, 0.0); // точка пересечения вне луча
        }
      }
      else {
        return (false, 0.0); // точка пересечения прямых вне отрезка
      }
    }
    else {
      return (false, 0.0); // точка пересечения прямых вне отрезка
    }
    }
}    

fn main() {
    println!("enter coordinates of ray x1,y1 x2,y2");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Не удалось прочитать строку");
    let mut v: Vec<_> = input.split(|c| c == ',' || c == ' ' || c == '\n').collect();
    let mut coordinates: Vec<f32> = vec![];
    for i in 0..4 {
        let num: f32 = v[i].trim().parse()
            .expect("Пожалуйста, введите число!");
        coordinates.push(num);
    }
    let mut ray = Ray::new(coordinates[0],coordinates[1], coordinates[2],coordinates[3]);
    
    let mut min = f32::MAX;
    let mut close_line = Line::new(0.0,0.0,0.0,0.0);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Не удалось прочитать строку");
        let mut v: Vec<_> = input.split(|c| c == ',' || c == ' ' || c == '\n').collect();
        if v[0] == "S" {
            break;
        }
        for i in 0..4 {
            let num: f32 = v[i].trim().parse()
                .expect("Пожалуйста, введите число!");
            coordinates[i] = num;
        }
        let line = Line::new(coordinates[0],coordinates[1], coordinates[2],coordinates[3]);
        let res = intersection(ray, line);
        if res.0 {
            if res.1 < min {
                close_line = line;
                min = res.1;
            }
        }
    }
    if min == f32::MAX {
        println!("\n");
    } else {
        println!("x1:{}, y1:{}, x2:{}, y2:{}", close_line.x1, close_line.y1, close_line.x2, close_line.y2);
    }
}
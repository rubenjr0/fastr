type Coords = (u32, u32);

fn bresenham_circle(cx: u32, cy: u32) -> Vec<Coords> {
    vec![
        (cx, cy + 3),
        (cx + 1, cy + 3),
        (cx + 2, cy + 2),
        (cx + 3, cy + 1),
        (cx + 3, cy),
        (cx + 3, cy - 1),
        (cx + 2, cy - 2),
        (cx + 1, cy - 3),
        (cx, cy - 3),
        (cx - 1, cy - 3),
        (cx - 2, cy - 2),
        (cx - 3, cy - 1),
        (cx - 3, cy),
        (cx - 3, cy + 1),
        (cx - 2, cy + 2),
        (cx - 1, cy + 3),
    ]
}

pub fn fast(image: &image::GrayImage) -> Vec<Coords> {
    let threshold = 30;
    let n = 12;
    let mut key_points: Vec<Coords> = Vec::new();
    for y in 3..image.height() - 3 {
        for x in 3..image.width() - 3 {
            let ip = image.get_pixel(x, y).0[0] as i16;
            let surrounding = bresenham_circle(x, y);
            let interest: Vec<_> = [
                surrounding[0],
                surrounding[4],
                surrounding[8],
                surrounding[12],
            ]
            .iter()
            .map(|(x, y)| image.get_pixel(*x, *y).0[0] as i16)
            .collect();
            if n >= 12
                && interest[0] > ip - threshold
                && interest[0] < ip + threshold
                && interest[2] > ip - threshold
                && interest[2] < ip + threshold
                || interest[1] > ip - threshold
                    && interest[1] < ip + threshold
                    && interest[3] > ip - threshold
                    && interest[3] < ip + threshold
            {
                continue;
            }
            let surrounding: Vec<_> = surrounding
                .iter()
                .map(|(x, y)| image.get_pixel(*x, *y).0[0] as i16)
                .collect();
            let mut brighter = 0;
            let mut darker = 0;
            for &vs in &surrounding {
                if vs > ip + threshold {
                    brighter += 1;
                    darker = 0;
                } else if vs < ip - threshold {
                    darker += 1;
                    brighter = 0;
                } else {
                    brighter = 0;
                    darker = 0;
                }
                if brighter == n || darker == n {
                    // let v: u16 = surrounding
                    //     .iter()
                    //     .map(|&v| (ip as i16 - v as i16).abs() as u16)
                    //     .sum();
                    key_points.push((x, y));
                }
            }
        }
    }
    key_points
}

#[cfg(test)]
mod tests {
    use crate::bresenham_circle;

    #[test]
    fn bresenham() {
        assert_eq!(bresenham_circle(16, 16).len(), 16)
    }
}

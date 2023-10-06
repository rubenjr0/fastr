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
    let threshold = 20;
    let n = 12;
    image
        .enumerate_pixels()
        .flat_map(|(x, y, p)| {
            if x < 3 || x >= image.width() - 3 || y < 3 || y >= image.height() - 3 {
                return None;
            }
            let p = p.0[0] as i16;
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
                && interest[0] > p - threshold
                && interest[0] < p + threshold
                && interest[2] > p - threshold
                && interest[2] < p + threshold
                || interest[1] > p - threshold
                    && interest[1] < p + threshold
                    && interest[3] > p - threshold
                    && interest[3] < p + threshold
            {
                return None;
            }
            let surrounding: Vec<_> = surrounding
                .iter()
                .map(|(x, y)| image.get_pixel(*x, *y).0[0] as i16)
                .collect();
            let mut brighter = 0;
            let mut darker = 0;
            for &vs in &surrounding {
                if vs > p + threshold {
                    brighter += 1;
                    darker = 0;
                } else if vs < p - threshold {
                    darker += 1;
                    brighter = 0;
                } else {
                    brighter = 0;
                    darker = 0;
                }
                if brighter == n || darker == n {
                    return Some((x, y));
                }
            }
            return None;
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::bresenham_circle;

    #[test]
    fn bresenham() {
        assert_eq!(bresenham_circle(16, 16).len(), 16)
    }
}

mod dragonhelper;
mod renderer;

struct Point {
    x: i32,
    y: i32
}

struct Agent {
    position: Point, // Current position of the agent
    direction: i8, // 0 north, 1 east, 2 south, 3 west
    min: Point, // Minimum position reached
    max: Point // Maximum position reached
}

fn main() {
    const ITERATIONS: i32 = 26;

    println!("Processing turns to {ITERATIONS} iterations...");

    let mut turns: Vec<bool> = vec![true]; // true is a right turn, false is a left turn
    for _ in 0..ITERATIONS {
        dragonhelper::iterate_curve(&mut turns);
    }

    println!("Turns processed. ({} turns)", turns.len());

    // Agent which moves to calculate the dimensions of the image.
    let mut agent: Agent = Agent { 
        position: Point { x: 0, y: 0 }, 
        direction: 0, 
        min: Point { x: 0, y: 0 }, 
        max: Point { x: 0, y: 0 } 
    };
    
    println!("Rendering turns to points...");

    let mut points: Vec<Point> = vec![];

    for turn in &turns { // Agent movement and point rendering
        match agent.direction {
            0 => agent.position.y -= 1,
            1 => agent.position.x += 1,
            2 => agent.position.y += 1,
            3 => agent.position.x -= 1,
            _ => panic!("Agent is facing invalid direction.")
        }
        
        points.push(Point { 
            x: agent.position.x.clone(), 
            y: agent.position.y.clone() 
        });

        if agent.position.x < agent.min.x {
            agent.min.x = agent.position.x.clone();
        } else if agent.position.x > agent.max.x {
            agent.max.x = agent.position.x.clone();
        }

        if agent.position.y < agent.min.y {
            agent.min.y = agent.position.y.clone();
        } else if agent.position.y > agent.max.y {
            agent.max.y = agent.position.y.clone();
        }
        
        match turn {
            true => agent.direction += 1,
            false => agent.direction -= 1
        }

        if agent.direction > 3 {
            agent.direction = 0;
        } else if agent.direction < 0 {
            agent.direction = 3;
        }
    }
    
    println!("Points rendered.");

    let dimensions: Point = Point { // Calculated dimensions of image
        x: agent.max.x.abs() + agent.min.x.abs() + 1, // +1 to avoid going out of image by 1
        y: agent.max.y.abs() + agent.min.y.abs() + 1
    };

    println!("Image dimensions calculated. ({} x {})", dimensions.x, dimensions.y);

    let mut image: Vec<u8> = Vec::with_capacity((dimensions.x*dimensions.y) as usize); // Image data as vector
    for _ in 0..dimensions.x*dimensions.y { // Initialise
        image.push(0);
    }

    let mut i: u32 = 0;
    while i < points.len() as u32 {
        points[i as usize].x += agent.min.x * -1; // Offset points to screen space
        points[i as usize].y += agent.min.y * -1;
        image[(points[i as usize].x+(dimensions.x*points[i as usize].y)) as usize] = ((i as f32/points.len() as f32)*255 as f32).round() as u8; // Set pixels from points
        i += 1;
    }

    println!("Image rendered to vector.");

    renderer::save_to_tga(dimensions.x, dimensions.y, image);

    println!("Image saved to output.tga!");
}

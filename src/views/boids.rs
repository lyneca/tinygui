use crate::buttons::ButtonSet;
use crate::screen::Screen;
use crate::shape::{Bitmap, Drawable, Line, Rect, Text};
use crate::view::{UpdateResult, UpdateResult::*, View, ViewSpawner};
use na::{distance, Point2, Rotation2, Vector2};
use nalgebra as na;
use rand::Rng;
use std::f32::consts::PI;

const BOID_DISTANCE: f32 = 10.0;
const BOID_VIEW_ANGLE: f32 = 2.0 * PI / 4.0;
const SPEED: f32 = 1.0;
const MAX_SPEED: f32 = 3.0;
const DESIRED_SEPARATION: f32 = 3.0;
const SEPARATION: f32 = 0.4;
const ALIGNMENT: f32 = 0.5;
const COHESION: f32 = 0.9;
const OBSTACLES: f32 = 10.0;

#[derive(Clone)]
struct Obstacle {
    position: Point2<f32>,
}

impl Obstacle {
    fn new(x: i32, y: i32) -> Obstacle {
        Obstacle {
            position: Point2::new(x as f32, y as f32),
        }
    }

    fn draw(&self, screen: &mut Screen) {
        Rect::new(4, 4)
            .at(self.position.x as i32, self.position.y as i32)
            .stroke(Some(1))
            .fill(Some(1))
            .draw(screen);
    }
}

#[derive(Clone)]
struct Boid {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
}

impl Boid {
    fn new(x: f32, y: f32) -> Boid {
        let mut rng = rand::thread_rng();
        Boid {
            position: Point2::new(x, y),
            velocity: Rotation2::new(rng.gen::<f32>() * 2.0 * PI) * Vector2::new(1.0, 1.0),
            acceleration: Vector2::new(0.0, 0.0),
        }
    }

    fn is_same_as(&self, other: &Boid) -> bool {
        return self.position == other.position && self.velocity == other.velocity;
    }

    fn nearby_boids(&self, boids: &Vec<Boid>) -> Vec<Boid> {
        boids
            .iter()
            .filter(|boid| !boid.is_same_as(self))
            .filter(|boid| distance(&boid.position, &self.position) < BOID_DISTANCE)
            .filter(|boid| {
                self.velocity.angle(&(boid.position - self.position)).abs() <= BOID_VIEW_ANGLE
            })
            .cloned()
            .collect()
    }

    fn add_force(&mut self, force: Vector2<f32>, multiplier: f32) {
        self.acceleration += force * multiplier;
    }

    fn separation(&mut self, boids: &Vec<Boid>) {
        let mut force = Vector2::new(0.0, 0.0);
        let in_range: Vec<&Boid> = boids
            .iter()
            .filter(|boid| (boid.position - self.position).magnitude() < DESIRED_SEPARATION)
            .collect();
        if in_range.len() == 0 {
            return;
        }
        for boid in &in_range {
            let diff = self.position - boid.position;
            force += diff.normalize() / (diff.magnitude() * 3.0);
        }
        force /= in_range.len() as f32;
        self.add_force(force.normalize() * SPEED - self.velocity, SEPARATION);
    }

    fn alignment(&mut self, boids: &Vec<Boid>) {
        let mut force = Vector2::new(0.0, 0.0);
        for boid in boids {
            force += boid.velocity;
        }
        force /= boids.len() as f32;
        self.add_force(force.normalize() * SPEED - self.velocity, ALIGNMENT);
    }

    fn cohesion(&mut self, boids: &Vec<Boid>) {
        let average = boids.iter().fold((0.0, 0.0), |acc, x| {
            (acc.0 + x.position.x, acc.1 + x.position.y)
        });
        let force = Point2::new(average.0, average.1) - self.position;
        self.add_force(force.normalize() * SPEED - self.velocity, COHESION);
    }

    fn obstacles(&mut self, obstacles: &Vec<Obstacle>) {
        let mut force = Vector2::new(0.0, 0.0);
        let nearby: Vec<Obstacle> = obstacles
            .iter()
            .filter(|obs| (self.position - obs.position).magnitude() < OBSTACLES)
            .cloned()
            .collect();
        if nearby.len() == 0 {
            return;
        }
        for obstacle in &nearby {
            let diff = self.position - obstacle.position;
            force += diff.normalize() / diff.magnitude();
        }
        self.add_force(
            (force / nearby.len() as f32).normalize() * SPEED - self.velocity,
            0.5,
        );
    }

    fn update(&mut self, boids: &Vec<Boid>, obstacles: &Vec<Obstacle>) {
        let nearby = self.nearby_boids(boids);
        if nearby.len() > 0 {
            self.separation(&nearby);
            self.alignment(&nearby);
            self.cohesion(&nearby);
        }
        self.obstacles(&obstacles);
        self.velocity += self.acceleration;
        if self.velocity.magnitude() > MAX_SPEED {
            self.velocity = self.velocity.normalize() * MAX_SPEED;
        }
        self.position += self.velocity;
        self.acceleration *= 0.3;
        if self.position.x < 0.0 {
            self.position.x = 127.0;
        }
        if self.position.x > 127.0 {
            self.position.x = 0.0;
        }
        if self.position.y < 0.0 {
            self.position.y = 63.0;
        }
        if self.position.y > 63.0 {
            self.position.y = 0.0;
        }
    }

    fn draw(&self, screen: &mut Screen) {
        Rect::new(0, 0)
            .at(self.position.x as i32, self.position.y as i32)
            .draw(screen)
    }
}

pub struct BoidsViewBuilder {}
impl ViewSpawner for BoidsViewBuilder {
    fn spawn(&self) -> Box<dyn View> {
        Box::new(BoidsView::new())
    }
}

pub struct BoidsView {
    boids: Vec<Boid>,
    obstacles: Vec<Obstacle>,
}

impl BoidsView {
    pub fn new() -> BoidsView {
        let mut rng = rand::thread_rng();
        BoidsView {
            boids: (0..20)
                .map(|_| Boid::new(rng.gen::<f32>() * 128.0, rng.gen::<f32>() * 64.0))
                .collect(),
            obstacles: (0..3)
                .map(|_| {
                    Obstacle::new(
                        (rng.gen::<f32>() * 128.0) as i32,
                        (rng.gen::<f32>() * 64.0) as i32,
                    )
                })
                .collect(),
        }
    }
}

impl View for BoidsView {
    fn update(&mut self, buttons: &mut ButtonSet) -> Option<UpdateResult> {
        if buttons.b.was_pressed() {
            return Some(Back);
        }
        let last_boids = self.boids.clone();
        for boid in &mut self.boids {
            boid.update(&last_boids, &self.obstacles);
        }
        None
    }

    fn render(&self, screen: &mut Screen) {
        // draw boundary rectangle

        self.boids.iter().for_each(|boid| boid.draw(screen));
        self.obstacles.iter().for_each(|obs| obs.draw(screen));

        // Rect::new(screen.get_width() - 1, screen.get_height() - 1)
        // .at(0, 0)
        // .draw(screen);
    }
}

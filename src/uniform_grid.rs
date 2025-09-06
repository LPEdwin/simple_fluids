use std::collections::HashSet;

use crate::{
    core::{Particle, Rectangle},
    vector2::Vector2,
};

pub struct UniformGrid {
    cells: Vec<HashSet<usize>>,
    cell_width: f64,
    cell_height: f64,
    boundary: Rectangle,
    n_col: usize,
    n_row: usize,
}

impl UniformGrid {
    pub fn new(boundary: Rectangle, particles: &[Particle]) -> UniformGrid {
        let width = boundary.width();
        let height = boundary.height();

        let max_radius = particles
            .iter()
            .map(|p| p.radius)
            .max_by(|a, b| a.partial_cmp(b).unwrap());

        let cell_size_min: f64 = match max_radius {
            Some(r) => 2.0 * r,
            None => width.min(height),
        };

        let n_col = (width / cell_size_min).floor().max(1.0) as usize;
        let n_row = (height / cell_size_min).floor().max(1.0) as usize;

        let cell_width = width / n_col as f64;
        let cell_height = height / n_row as f64;

        let cells: Vec<HashSet<usize>> = vec![HashSet::new(); n_col * n_row];

        UniformGrid {
            cells,
            cell_width,
            cell_height,
            boundary,
            n_col,
            n_row,
        }
    }

    pub fn with_cell_size(boundary: Rectangle, cell_size: f64) -> UniformGrid {
        let width = boundary.width();
        let height = boundary.height();

        let n_col = (width / cell_size).floor().max(1.0) as usize;
        let n_row = (height / cell_size).floor().max(1.0) as usize;

        let cell_width = width / n_col as f64;
        let cell_height = height / n_row as f64;

        let cells = vec![HashSet::new(); n_col * n_row];

        UniformGrid {
            cells,
            cell_width,
            cell_height,
            boundary,
            n_col,
            n_row,
        }
    }

    pub fn get_close_colliders(&self, position: Vector2) -> Vec<usize> {
        let (col, row) = self.get_cell_indices(position);

        let mut indices = HashSet::new();

        for dc in -1..=1 {
            for dr in -1..=1 {
                if let Some(idx) = self.get_cell_index_safe(col as isize + dc, row as isize + dr) {
                    indices.extend(&self.cells[idx]);
                }
            }
        }

        indices.into_iter().collect()
    }

    pub fn add_particle(&mut self, index: usize, particle: &Particle) {
        let (col, row) = self.get_cell_indices(particle.position);
        let cell_index = self.get_cell_index(col, row);
        self.cells[cell_index].insert(index);
    }

    fn get_cell_indices(&self, position: Vector2) -> (usize, usize) {
        let col = ((position.x - self.boundary.min.x) / self.cell_width).floor() as usize;
        let row = ((position.y - self.boundary.min.y) / self.cell_height).floor() as usize;

        let col = col.clamp(0, self.n_col - 1) as usize;
        let row = row.clamp(0, self.n_row - 1) as usize;

        (col, row)
    }

    fn get_cell_index_safe(&self, col: isize, row: isize) -> Option<usize> {
        if col < 0 || row < 0 {
            return None;
        }
        let col = col as usize;
        let row = row as usize;
        if col < self.n_col && row < self.n_row {
            Some(self.get_cell_index(col, row))
        } else {
            None
        }
    }

    fn get_cell_index(&self, col: usize, row: usize) -> usize {
        col * self.n_row + row
    }

    /// Trys to find a none overlapping position and returns it.
    /// Returns error if not possible.
    pub fn try_get_none_overlaping_position(
        &mut self,
        particle_radius: f64,
        particles: &Vec<Particle>,
        max_attempts_per_particle: usize,
    ) -> Result<Vector2, String> {
        if particle_radius > self.cell_height {
            return Err("Radius is greater than the grids.".to_string());
        }

        for _ in 0..max_attempts_per_particle {
            let position = Vector2::random_min_max(
                self.boundary.min + particle_radius,
                self.boundary.max - particle_radius,
            );
            let neighbours = self.get_close_colliders(position);
            let mut overlaps = false;
            for idx in neighbours {
                let dist_sq = (position - particles[idx].position).length_squared();
                if dist_sq < (particle_radius + particles[idx].radius).powi(2) {
                    overlaps = true;
                    break;
                }
            }
            if !overlaps {
                return Ok(position);
            }
        }

        Err("Couldn't be placed.".to_string())
    }
}

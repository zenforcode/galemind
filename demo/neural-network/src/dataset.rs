//! Dataset utilities for generating synthetic image data
//! 
//! This module provides utilities for creating synthetic datasets that simulate
//! realistic image processing workloads for testing adaptive batching.

use anyhow::Result;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use tracing::{info, debug};

use crate::model::ImageRequest;

/// Synthetic image dataset
#[derive(Debug, Clone)]
pub struct ImageDataset {
    pub images: Vec<Vec<f32>>,
    pub labels: Vec<usize>,
    pub width: u32,
    pub height: u32,
    pub channels: u32,
}

/// Generate a synthetic dataset with realistic image patterns
pub fn generate_synthetic_dataset(num_images: usize, image_size: u32) -> Result<ImageDataset> {
    info!("Generating synthetic dataset with {} images of size {}x{}", 
          num_images, image_size, image_size);
    
    let mut rng = StdRng::seed_from_u64(42); // Deterministic for reproducibility
    let channels = 3; // RGB
    let pixels_per_image = (image_size * image_size * channels) as usize;
    
    let mut images = Vec::with_capacity(num_images);
    let mut labels = Vec::with_capacity(num_images);
    
    for i in 0..num_images {
        let image_type = i % 5; // 5 different synthetic patterns
        let image_data = generate_synthetic_image(image_size, channels, image_type, &mut rng);
        let label = image_type; // Label corresponds to image pattern type
        
        images.push(image_data);
        labels.push(label);
        
        if i % 100 == 0 && i > 0 {
            debug!("Generated {} images", i);
        }
    }
    
    info!("Dataset generation complete");
    
    Ok(ImageDataset {
        images,
        labels,
        width: image_size,
        height: image_size,
        channels,
    })
}

/// Generate a single synthetic image with specific patterns
fn generate_synthetic_image(size: u32, channels: u32, pattern_type: usize, rng: &mut StdRng) -> Vec<f32> {
    let total_pixels = (size * size * channels) as usize;
    let mut image = vec![0.0f32; total_pixels];
    
    match pattern_type {
        0 => generate_gradient_pattern(&mut image, size, channels, rng),
        1 => generate_checkerboard_pattern(&mut image, size, channels, rng),
        2 => generate_circular_pattern(&mut image, size, channels, rng),
        3 => generate_noise_pattern(&mut image, size, channels, rng),
        4 => generate_striped_pattern(&mut image, size, channels, rng),
        _ => generate_random_pattern(&mut image, size, channels, rng),
    }
    
    // Normalize to [0, 1] range (typical for neural network input)
    normalize_image(&mut image);
    
    image
}

/// Generate gradient pattern (smooth transitions)
fn generate_gradient_pattern(image: &mut [f32], size: u32, channels: u32, rng: &mut StdRng) {
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;
    let max_distance = (center_x * center_x + center_y * center_y).sqrt();
    
    for y in 0..size {
        for x in 0..size {
            let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
            let normalized_distance = distance / max_distance;
            
            for c in 0..channels {
                let idx = ((y * size + x) * channels + c) as usize;
                
                // Different gradient for each channel
                let value = match c {
                    0 => normalized_distance, // Red channel
                    1 => 1.0 - normalized_distance, // Green channel
                    2 => (normalized_distance * 2.0).min(1.0), // Blue channel
                    _ => 0.5,
                };
                
                image[idx] = value + rng.gen::<f32>() * 0.1 - 0.05; // Add small noise
            }
        }
    }
}

/// Generate checkerboard pattern
fn generate_checkerboard_pattern(image: &mut [f32], size: u32, channels: u32, rng: &mut StdRng) {
    let checker_size = (size / 8).max(1); // 8x8 checkerboard
    
    for y in 0..size {
        for x in 0..size {
            let checker_x = x / checker_size;
            let checker_y = y / checker_size;
            let is_white = (checker_x + checker_y) % 2 == 0;
            
            for c in 0..channels {
                let idx = ((y * size + x) * channels + c) as usize;
                
                let base_value = if is_white { 0.8 } else { 0.2 };
                let channel_modifier = match c {
                    0 => 1.0,   // Red
                    1 => 0.8,   // Green
                    2 => 0.9,   // Blue
                    _ => 1.0,
                };
                
                image[idx] = base_value * channel_modifier + rng.gen::<f32>() * 0.1 - 0.05;
            }
        }
    }
}

/// Generate circular pattern (concentric circles)
fn generate_circular_pattern(image: &mut [f32], size: u32, channels: u32, rng: &mut StdRng) {
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;
    let max_radius = (size as f32 / 2.0).min(center_x).min(center_y);
    
    for y in 0..size {
        for x in 0..size {
            let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
            let ring_number = ((distance / max_radius) * 10.0) as usize % 3;
            
            for c in 0..channels {
                let idx = ((y * size + x) * channels + c) as usize;
                
                let value = match ring_number {
                    0 => 0.8,  // Bright ring
                    1 => 0.5,  // Medium ring
                    2 => 0.2,  // Dark ring
                    _ => 0.5,
                };
                
                let channel_modifier = if c == ring_number { 1.2 } else { 0.8 };
                image[idx] = (value * channel_modifier).min(1.0) + rng.gen::<f32>() * 0.08 - 0.04;
            }
        }
    }
}

/// Generate noise pattern (controlled random noise)
fn generate_noise_pattern(image: &mut [f32], size: u32, channels: u32, rng: &mut StdRng) {
    // Generate Perlin-like noise pattern
    let scale = size as f32 / 16.0;
    
    for y in 0..size {
        for x in 0..size {
            let noise_x = x as f32 / scale;
            let noise_y = y as f32 / scale;
            
            for c in 0..channels {
                let idx = ((y * size + x) * channels + c) as usize;
                
                // Simple noise function (in real scenarios, you'd use proper Perlin noise)
                let noise_value = (noise_x.sin() * noise_y.cos() + 
                                 (noise_x * 2.0).cos() * (noise_y * 1.5).sin()) * 0.5 + 0.5;
                
                let channel_noise = rng.gen::<f32>() * 0.3;
                image[idx] = noise_value * 0.7 + channel_noise * 0.3;
            }
        }
    }
}

/// Generate striped pattern
fn generate_striped_pattern(image: &mut [f32], size: u32, channels: u32, rng: &mut StdRng) {
    let stripe_width = (size / 12).max(1);
    
    for y in 0..size {
        for x in 0..size {
            let stripe_x = x / stripe_width;
            let stripe_y = y / stripe_width;
            let is_horizontal_stripe = stripe_y % 2 == 0;
            let is_vertical_stripe = stripe_x % 2 == 0;
            
            for c in 0..channels {
                let idx = ((y * size + x) * channels + c) as usize;
                
                let value = match c {
                    0 => if is_horizontal_stripe { 0.8 } else { 0.3 },  // Red horizontal stripes
                    1 => if is_vertical_stripe { 0.8 } else { 0.3 },    // Green vertical stripes
                    2 => if (is_horizontal_stripe && is_vertical_stripe) { 0.9 } else { 0.2 }, // Blue intersections
                    _ => 0.5,
                };
                
                image[idx] = value + rng.gen::<f32>() * 0.1 - 0.05;
            }
        }
    }
}

/// Generate random pattern
fn generate_random_pattern(image: &mut [f32], _size: u32, _channels: u32, rng: &mut StdRng) {
    for pixel in image.iter_mut() {
        *pixel = rng.gen::<f32>();
    }
}

/// Normalize image to [0, 1] range and apply standard normalization
fn normalize_image(image: &mut [f32]) {
    // Clamp to [0, 1] range
    for pixel in image.iter_mut() {
        *pixel = pixel.clamp(0.0, 1.0);
    }
    
    // Apply ImageNet-like normalization (mean subtraction and std scaling)
    let means = [0.485, 0.456, 0.406]; // ImageNet means for RGB
    let stds = [0.229, 0.224, 0.225];  // ImageNet stds for RGB
    
    let channels = 3;
    for (i, pixel) in image.iter_mut().enumerate() {
        let channel = i % channels;
        *pixel = (*pixel - means[channel]) / stds[channel];
    }
}

/// Create a dataset with variable image sizes (simulating real-world scenarios)
pub fn generate_variable_size_dataset(
    num_images: usize, 
    min_size: u32, 
    max_size: u32
) -> Result<Vec<ImageRequest>> {
    info!("Generating variable-size dataset with {} images, sizes {}x{} to {}x{}", 
          num_images, min_size, min_size, max_size, max_size);
    
    let mut rng = StdRng::seed_from_u64(123);
    let mut requests = Vec::with_capacity(num_images);
    
    for i in 0..num_images {
        // Generate random size within range
        let size = rng.gen_range(min_size..=max_size);
        
        // Round to nearest multiple of 32 for realistic CNN input sizes
        let size = ((size + 31) / 32) * 32;
        
        let image_data = generate_synthetic_image(size, 3, i % 5, &mut rng);
        
        requests.push(ImageRequest {
            id: i as u64,
            image_data,
            width: size,
            height: size,
            channels: 3,
        });
        
        if i % 50 == 0 && i > 0 {
            debug!("Generated {} variable-size images", i);
        }
    }
    
    info!("Variable-size dataset generation complete");
    Ok(requests)
}

/// Generate realistic batch patterns for testing
pub fn generate_realistic_workload_pattern(base_requests: Vec<ImageRequest>) -> Vec<(ImageRequest, u64)> {
    info!("Generating realistic workload timing patterns");
    
    let mut rng = StdRng::seed_from_u64(456);
    let mut timed_requests = Vec::new();
    
    let mut current_time = 0u64;
    
    for (i, request) in base_requests.into_iter().enumerate() {
        // Simulate realistic arrival patterns
        let delay = match i % 20 {
            0..=5 => rng.gen_range(10..50),     // Burst period - rapid arrivals
            6..=10 => rng.gen_range(50..150),   // Normal period
            11..=15 => rng.gen_range(150..300), // Slow period
            16..=17 => rng.gen_range(5..20),    // Another burst
            _ => rng.gen_range(200..500),       // Very slow period
        };
        
        current_time += delay;
        timed_requests.push((request, current_time));
    }
    
    info!("Generated timing pattern with {} requests over {}ms", 
          timed_requests.len(), current_time);
    
    timed_requests
}

/// Dataset statistics for analysis
#[derive(Debug)]
pub struct DatasetStats {
    pub num_images: usize,
    pub total_pixels: usize,
    pub avg_pixels_per_image: f64,
    pub min_size: (u32, u32),
    pub max_size: (u32, u32),
    pub total_memory_mb: f64,
}

impl ImageDataset {
    /// Calculate dataset statistics
    pub fn stats(&self) -> DatasetStats {
        let total_pixels: usize = self.images.iter()
            .map(|img| img.len())
            .sum();
        
        let avg_pixels = total_pixels as f64 / self.images.len() as f64;
        let memory_mb = (total_pixels * std::mem::size_of::<f32>()) as f64 / (1024.0 * 1024.0);
        
        DatasetStats {
            num_images: self.images.len(),
            total_pixels,
            avg_pixels_per_image: avg_pixels,
            min_size: (self.width, self.height),
            max_size: (self.width, self.height),
            total_memory_mb: memory_mb,
        }
    }
}

impl DatasetStats {
    /// Print formatted statistics
    pub fn print(&self) {
        info!("Dataset Statistics:");
        info!("  Number of images: {}", self.num_images);
        info!("  Total pixels: {}", self.total_pixels);
        info!("  Average pixels per image: {:.0}", self.avg_pixels_per_image);
        info!("  Image size range: {}x{} to {}x{}", 
              self.min_size.0, self.min_size.1,
              self.max_size.0, self.max_size.1);
        info!("  Total memory usage: {:.2} MB", self.total_memory_mb);
    }
}

use crate::{renderer::thread::Builder, ringbuffer::RingBuffer};
use futures::{future::join_all, join, poll, stream, FutureExt, StreamExt};
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    error::Error,
    fs::File,
    future::Future,
    io::{self, Write},
    ops::Deref,
    result,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread::{self, JoinHandle, ScopedJoinHandle},
    time::SystemTime,
};

use palette::{Clamp, Srgba};

use crate::{
    camera::Camera,
    color::{color::color_to_u8_srgba, Color},
    interval::Interval,
    material::scatter,
    progress_bar::ProgressBar,
    ray::{Hittable, HittableList, Ray},
};

pub fn render(
    world: &mut HittableList,
    camera: &mut Camera,
    render_file_path: &str,
) -> Result<File, io::Error> {
    camera.initialize();

    let (image_width, image_height) = camera.get_image_xy();

    let time_start = SystemTime::now();

    let mut image_ppm: String = String::new();
    image_ppm += &format!("P3\n{} {}\n255\n", image_width, image_height).to_string();

    render_inner(world, camera, &mut image_ppm);
    println!("Render finished!");

    let time_now = SystemTime::now();
    let since_the_epoch = time_now
        .duration_since(time_start)
        .expect("Time went backwards");
    println!("Render took: {:?} seconds", since_the_epoch.as_secs_f32());

    println!("Saving to file {}...", render_file_path);
    let mut render_file = File::create(render_file_path)?;
    render_file.write_all(image_ppm.as_bytes()).unwrap();

    Ok(render_file)
}

fn render_inner_thread(world: &HittableList, camera: &Camera, x: i32, y: i32) -> Color {
    let mut sum_texel_color: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    for _aa in 0..camera.samples_per_pixel {
        let ray: Ray = camera.get_ray(x, y);

        let texel_color: Color = ray_color(&ray, camera.max_ray_per_pixel, &world);
        sum_texel_color += texel_color;
    }

    sum_texel_color
}

fn render_inner_multithread_old(
    world: &HittableList,
    camera: &Camera,
    image_string: &mut String,
    progress_bar: &mut ProgressBar,
) {
    let (image_width, image_height) = camera.get_image_xy();
    let total_ray_pixel_tasks: i32 = image_width * image_height;

    thread::scope(|s: &thread::Scope<'_, '_>| {
        let num_threads_to_spawn: i32 = total_ray_pixel_tasks;
        let mut all_thread_handles: Vec<ScopedJoinHandle<Color>> =
            Vec::with_capacity(num_threads_to_spawn as usize);

        for i in 0..num_threads_to_spawn {
            let thread_id = i;
            let thread_x: i32 = thread_id % image_width;
            let thread_y = thread_id / image_width;
            let thread_handle = s.spawn(move || {
                let texel_color = render_inner_thread(world, camera, thread_x, thread_y);
                return texel_color;
            });
            all_thread_handles.push(thread_handle);
        }

        let mut thread_results: Vec<Color> = Vec::new();
        for thread_handle in all_thread_handles {
            let thread_result = thread_handle.join().unwrap();
            thread_results.push(thread_result);
        }

        for (i, result) in thread_results.iter().enumerate() {
            write_color(image_string, *result / camera.samples_per_pixel as f32);

            if i as i32 % progress_bar.calc_increment() as i32 == 0 {
                progress_bar.print_progress_percent();
                progress_bar.inc();
            }
        }
    });
}

async fn render_inner_multithread(
    world: Arc<HittableList>,
    camera: Arc<Camera>,
    image_string: &mut String,
    progress_bar: &mut ProgressBar,
) {
    // let (image_width, image_height) = camera.get_image_xy();

    // let mut pixel_futures = Vec::with_capacity((image_height * image_width) as usize);
    // for y in 0..image_height {
    //     for x in 0..image_width {
    //         let pixel_future = PixelFuture::new(x, y, world.clone(), camera.clone());
    //         pixel_futures.push(pixel_future);
    //     }
    // }

    // let results = join_all(pixel_futures).await;
    // for (i, res) in results.iter().enumerate() {
    //     write_color(image_string, *res);

    //     if i as i32 % progress_bar.calc_increment() as i32 == 0 {
    //         progress_bar.print_progress_percent();
    //         progress_bar.inc();
    //     }
    // }

    // Spawn thread that fills stream

    let (image_width, image_height) = camera.get_image_xy();
    let total_pixel_futures = image_width * image_height;
    type PixelFutureRingBuffer = RingBuffer<PixelFuture, 160>;
    let futures_ring_buffer: Arc<Mutex<PixelFutureRingBuffer>> =
        Arc::new(Mutex::new(PixelFutureRingBuffer::new()));

    let thread_ring_buffer = futures_ring_buffer.clone();
    std::thread::spawn(move || {
        let (image_width, image_height) = camera.get_image_xy();

        for y in 0..image_height {
            for x in 0..image_width {
                let pixel_future = PixelFuture::new(x, y, world.clone(), camera.clone());

                loop {
                    let lock = thread_ring_buffer.lock();
                    let mut ring_buf = lock.unwrap();
                    if ring_buf.space_left() >= 1 {
                        ring_buf.push(pixel_future);
                        break;
                    }
                }
            }
        }
    });

    let mut streamed_pixelfuture_counter = 0;
    while streamed_pixelfuture_counter < total_pixel_futures {
        let lock = futures_ring_buffer.lock();
        let mut ring_buf = lock.unwrap();

        let mut num_front_ready: usize = 0;
        let mut last_was_ready = true;
        for (i, fut) in ring_buf.iter_mut().enumerate() {
            let poll_res = poll!(fut);
            match poll_res {
                Poll::Ready(_) => {
                    if last_was_ready {
                        num_front_ready += 1;
                    }
                }
                Poll::Pending => last_was_ready = false,
            }
        }

        for i in 0..num_front_ready {
            let res = ring_buf.pop_front().unwrap().now_or_never().unwrap();

            streamed_pixelfuture_counter += 1;
            write_color(image_string, res);

            if i as i32 % progress_bar.calc_increment() as i32 == 0 {
                progress_bar.print_progress_percent();
                progress_bar.inc();
            }
        }
    }
}

pub fn render_inner(world: &HittableList, camera: &Camera, image_string: &mut String) {
    let (image_width, image_height) = camera.get_image_xy();
    let mut progress_bar: ProgressBar =
        ProgressBar::new((image_width * image_height) as f64, 20 as usize);

    const MULTITHREAD_ENABLE: bool = true;
    if MULTITHREAD_ENABLE {
        const OLD_MULTITHREAD_CODE: bool = false;
        if OLD_MULTITHREAD_CODE {
            render_inner_multithread_old(world, camera, image_string, &mut progress_bar);
        } else {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let world_copy = world.clone();
            let camera_copy = camera.clone();
            let res = rt.block_on(async {
                let world_arc_copy = Arc::new(world_copy);
                let camera_arc_copy = Arc::new(camera_copy);
                render_inner_multithread(
                    world_arc_copy,
                    camera_arc_copy,
                    image_string,
                    &mut progress_bar,
                )
                .await
            });
        }
    } else {
        for y in 0..image_height {
            for x in 0..image_width {
                let mut sum_texel_color: Color = Color::new(0.0, 0.0, 0.0, 1.0);
                for _aa in 0..camera.samples_per_pixel {
                    let ray: Ray = camera.get_ray(x, y);

                    let texel_color: Color = ray_color(&ray, camera.max_ray_per_pixel, &world);
                    sum_texel_color += texel_color;
                }
                write_color(
                    image_string,
                    sum_texel_color / camera.samples_per_pixel as f32,
                );

                if (x + y * image_width) % progress_bar.calc_increment() as i32 == 0 {
                    progress_bar.print_progress_percent();
                    progress_bar.inc();
                }
            }
        }
    }

    assert!(progress_bar.is_finished());
}

fn write_color(accum_string_file: &mut String, texel_color: Color) {
    let mut scaled_texel_color = Color {
        color: palette::rgb::Rgb {
            red: texel_color.red,
            green: texel_color.green,
            blue: texel_color.blue,
            standard: std::marker::PhantomData,
        },
        alpha: 1.0,
    };

    let gamma_corrected: Srgba = scaled_texel_color.clamp().into();
    let texel_color_u8 = color_to_u8_srgba(&gamma_corrected);

    let ir = texel_color_u8[0];
    let ig = texel_color_u8[1];
    let ib = texel_color_u8[2];

    *accum_string_file += &ir.to_string();
    *accum_string_file += &' '.to_string();
    *accum_string_file += &ig.to_string();
    *accum_string_file += &' '.to_string();
    *accum_string_file += &ib.to_string();
    *accum_string_file += &'\n'.to_string();
}

fn ray_color(ray: &Ray, depth: i32, world: &HittableList) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0, 0.0);
    }

    if let Some(hit_result) = world.hit_all(
        ray,
        Interval {
            min: 0.0001,
            max: f32::INFINITY,
        },
    ) {
        let mut scattererd: Ray = Ray::default();
        let mut diffuse: Color = Color::new(0.0, 0.0, 0.0, 1.0);
        let mut emissive: Color = Color::new(0.0, 0.0, 0.0, 1.0);
        if scatter(
            hit_result.material_id,
            ray,
            &hit_result,
            &mut diffuse,
            &mut emissive,
            &mut scattererd,
        ) {
            return emissive + diffuse * ray_color(&scattererd, depth - 1, world);
        }
        return diffuse + emissive;
    }

    // BG
    let unit_dir = ray.direction.normalize();
    let a = (unit_dir.y + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0, 1.0) * a
}

struct PixelFutureState {
    pixel_result: Option<Color>,
    pixel_x: i32,
    pixel_y: i32,
    world: Arc<HittableList>,
    camera: Arc<Camera>,
    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    waker: Option<Waker>,
}

pub struct PixelFuture {
    shared_state: Arc<Mutex<PixelFutureState>>,
}

impl Future for PixelFuture {
    type Output = Color;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.pixel_result.is_some() {
            Poll::Ready(shared_state.pixel_result.unwrap())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl PixelFuture {
    pub fn new(pixel_x: i32, pixel_y: i32, world: Arc<HittableList>, camera: Arc<Camera>) -> Self {
        let shared_state = Arc::new(Mutex::new(PixelFutureState {
            pixel_result: None,
            waker: None,
            pixel_x,
            pixel_y,
            world,
            camera,
        }));

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            let mut shared_state = thread_shared_state.lock().unwrap();

            let mut sum_texel_color: Color = Color::new(0.0, 0.0, 0.0, 1.0);
            for _aa in 0..shared_state.camera.samples_per_pixel {
                let ray: Ray = shared_state
                    .camera
                    .get_ray(shared_state.pixel_x, shared_state.pixel_y);
                let color: Color = ray_color(
                    &ray,
                    shared_state.camera.max_ray_per_pixel,
                    shared_state.world.borrow(),
                );
                sum_texel_color += color;
            }

            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            shared_state.pixel_result =
                Some(sum_texel_color / shared_state.camera.samples_per_pixel as f32);
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        Self { shared_state }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;
    use io::ErrorKind;

    use crate::{setup_world0, setup_world1};

    use super::*;

    #[test]
    #[ignore]
    fn test_renderer_render() {
        let mut camera: Camera = Camera::default();
        camera.aspect_ratio = 16.0 / 9.0;
        camera.image_width = 500;
        camera.fov = 40.0;
        camera.samples_per_pixel = 10;
        camera.max_ray_per_pixel = 10;
        camera.position = Vec3::new(-30.0, 6.0, -20.0);
        let look_at_position = Vec3::new(0.0, 0.0, 0.0);
        camera.look_at(look_at_position, Vec3::new(0.0, 1.0, 0.0));

        camera.defocus_angle = 0.6 * 0.5;
        camera.focus_dist = (camera.position - look_at_position).length();

        let mut world0: HittableList = HittableList::new();
        setup_world0(&mut world0);
        let mut world1: HittableList = HittableList::new();
        setup_world1(&mut world1);
        world0.merge(world1);

        let mut world = world0;

        let render_file_path = "../img/render.ppm";
        let result = render(&mut world, &mut camera, render_file_path);

        // Ignore the file errors
        if result.is_err() {
            match result.err() {
                Some(e) => {
                    let err_kind = e.kind();
                    if err_kind != ErrorKind::NotFound {
                        panic!();
                    }
                }
                None => {}
            }
        }
    }
}

extern crate vulkano;
extern crate image;

use std::sync::Arc;

use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer};
use vulkano::sync::GpuFuture;
use vulkano::pipeline::ComputePipeline;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::format::{Format};
use vulkano::image::{Dimensions, StorageImage};

use image::{ImageBuffer, Rgba};


fn main()  -> std::io::Result<()> {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("failed to create instance");
    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");
    for family in physical.queue_families() {
        println!("Found a queue family with {:?} queue(s)", family.queues_count());
    }
    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions {
            khr_storage_buffer_storage_class: true,
            ..DeviceExtensions::none()
        },
        [(queue_family, 0.5)].iter().cloned())
            .expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    let image = StorageImage::new(device.clone(),
                                  Dimensions::Dim2d { width: 1280, height: 720},
                                  Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    mod cs {
        vulkano_shaders::shader!{
        ty: "compute",
        src: "
        #version 450

        layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

        layout(set = 0, binding = 0, rgba8) uniform writeonly image2D img;

        void main() {
            vec2 norm_coordinates = gl_GlobalInvocationID.xy;
            norm_coordinates.y = (imageSize(img).y - 1.0) - norm_coordinates.y;
            norm_coordinates = norm_coordinates / vec2(imageSize(img));
            vec4 to_write = vec4(0.0, 0.0, 0.0, 1.0);
            //vec2 c = (norm_coordinates - vec2(0.5)) * 2.0 - vec2(1.0, 0.0);

            float aspect_ratio = 16.0 / 9.0;
            float viewport_height = 2.0;
            float viewport_width = aspect_ratio * viewport_height;
            float focal_length = 1.0;

            vec3 origin = vec3(0.0, 0.0, 0.0);
            vec3 horizontal = vec3(viewport_width, 0.0, 0.0);
            vec3 vertical = vec3(0.0, viewport_height, 0.0);
            vec3 lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);

            vec3 direction = lower_left_corner - origin
                + norm_coordinates.x * horizontal + norm_coordinates.y * vertical;
            vec3 unit_direction = direction / direction.length();


            vec3 center = vec3(0.0, 0.0, -1.0);
            float radius = 0.5;

            vec3 oc = origin - center;
            float a = dot(direction, direction);
            float half_b = dot(oc, direction);
            float c = dot(oc, oc) - radius * radius;
            float discriminant = half_b * half_b - a * c;

            if (discriminant > 0.0) {
                float t = (-half_b - sqrt(discriminant)) / a;
                vec3 ray_at_t = origin + (t * direction);
                vec3 normal = ray_at_t - center;
                vec3 unit_normal = normal / normal.length();
                //vec3 colors = 0.5 * (unit_normal + 1.0);
                vec3 colors = 0.5 * ((ray_at_t - center) + vec3(1.0, 1.0, 1.0));
                to_write = vec4(colors, 1.0);
                //to_write = (1.0 - t) * vec4(1.0, 1.0, 1.0, 1.0) + t * vec4(0.5, 0.7, 1.0, 1.0);
            }
            else {
                float t = 0.5 * (unit_direction.y + 1.0);
                to_write = (1.0 - t) * vec4(1.0, 1.0, 1.0, 1.0) + t * vec4(0.5, 0.7, 1.0, 1.0);
            }


            imageStore(img, ivec2(gl_GlobalInvocationID.xy), to_write);
        }

        "
        }
    }

    let shader = cs::Shader::load(device.clone())
        .expect("failed to create shader module");


    let compute_pipeline = Arc::new(
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
            .expect("failed to create compute pipeline"),
    );

    let set = Arc::new(
        PersistentDescriptorSet::start(
            compute_pipeline
                .layout()
                .descriptor_set_layout(0)
                .unwrap()
                .clone(),
            )
            .add_image(image.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let buf = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        (0..1280 * 720 * 4).map(|_| 0u8),
    )
        .expect("failed to create buffer");

    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder
        .dispatch([1280 / 8, 720 / 8, 1], compute_pipeline.clone(), set.clone(), ())
        .unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone())
        .unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1280, 720, &buffer_content[..]).unwrap();

    image.save("./output/image.png").unwrap();

    println!("Everything Succeeded");

    Ok(())
}

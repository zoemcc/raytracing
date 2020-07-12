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
use vulkano::format::{Format, ClearValue};
use vulkano::image::{Dimensions, StorageImage};

use image::{ImageBuffer, Rgba};


struct BasicStruct {
    a: u32,
    b: bool,
}


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


    let data = BasicStruct { a: 5, b: true };
    let buffer = CpuAccessibleBuffer::from_data(device.clone(),
                                                BufferUsage::all(), false, data)
        .expect("failed to create buffer");



    let mut content = buffer.write().unwrap();

    content.a *= 2;
    content.b = false;


    let iter = (0 .. 128).map(|_| 5u8);
    let buffer = CpuAccessibleBuffer::from_iter(device.clone(),
    BufferUsage::all(), false, iter).unwrap();

    let mut content = buffer.write().unwrap();

    content[12] = 83;
    content[7] = 3;

    let source_content = 0 .. 64;
    let source = CpuAccessibleBuffer::from_iter(device.clone(),
                                                BufferUsage::all(), false,
                                                source_content)
        .expect("failed to create buffer");


    let dest_content = (0 .. 64).map(|_| 0);
    let dest = CpuAccessibleBuffer::from_iter(device.clone(),
                                                BufferUsage::all(), false,
                                              dest_content)
        .expect("failed to create buffer");

    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder.copy_buffer(source.clone(), dest.clone()).unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let src_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();
    //println!("{:#?}", &*dest_content);
    assert_eq!(&*src_content, &*dest_content);

    mod cs {
        vulkano_shaders::shader!{
        ty: "compute",
        src: "
        #version 450

        layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

        layout(set = 0, binding = 0) buffer Data {
            uint data[];
        } buf;

        void main() {
            uint idx = gl_GlobalInvocationID.x;
            buf.data[idx] *= 12;
        }
        "
        }
    }

    let shader = cs::Shader::load(device.clone())
        .expect("failed to create shader module");


    let compute_pipeline = Arc::new(ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                                        .expect("failed to create compute pipeline"));

    let data_iter = 0..65536;
    let data_buffer =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                       false, data_iter)
        .expect("failed to create buffer");

    let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
    let set = Arc::new(PersistentDescriptorSet::start(layout.clone())
        .add_buffer(data_buffer.clone()).unwrap()
        .build().unwrap()
    );


    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder.dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ()).unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Everything Succeeded");


    let image = StorageImage::new(device.clone(),
                                  Dimensions::Dim2d { width: 1024, height: 1024},
    Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    let buf =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                       false,
                                       (0 .. 1024 * 1024 * 4).map(|_| 0u8))
            .expect("failed to create buffer");

    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder
        .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0])).unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone()).unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();

    image.save("./output/image.png").unwrap();


    Ok(())
}

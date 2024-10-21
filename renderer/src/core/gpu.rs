use super::Surface;
use ash::vk;
use std::{
    collections::BTreeMap,
    error::Error,
    ffi::{c_char, CStr},
    result::Result,
};

// TODO: maybe structure Error as a proper enum

/// If ok returns the gpu and the index of the graphics family
pub fn select_gpu(
    instance: &ash::Instance,
    surface: &Surface,
    extensions: &[*const c_char],
) -> Result<(vk::PhysicalDevice, u32), Box<dyn Error>> {
    let mut scoreboard: BTreeMap<i32, (vk::PhysicalDevice, u32)> = BTreeMap::new();

    let gpu_list = unsafe { instance.enumerate_physical_devices()? };

    if gpu_list.is_empty() {
        return Err("System doesn't have any physical device".into());
    }

    for gpu in gpu_list {
        let props = unsafe { instance.get_physical_device_properties(gpu) };
        let gpu_name = props.device_name_as_c_str()?.to_str()?;
        log::trace!("Checking device: {}", gpu_name);

        let graphics_index = match is_suitable(instance, gpu, extensions, surface)? {
            None => {
                log::trace!("Device is not suitable");
                continue;
            }
            Some(val) => val,
        };

        let score = rate(&props);
        scoreboard.insert(score, (gpu, graphics_index));
    }

    if scoreboard.is_empty() {
        return Err("No suitable physical device found".into());
    }

    let chosen = scoreboard.last_key_value().unwrap().1.to_owned();
    Ok(chosen)
}

fn is_suitable(
    instance: &ash::Instance,
    gpu: vk::PhysicalDevice,
    extensions: &[*const c_char],
    surface: &Surface,
) -> Result<Option<u32>, Box<dyn Error>> {
    // check that gpu supports all the required extensions
    let supported_extensions = unsafe { instance.enumerate_device_extension_properties(gpu)? };
    for required in extensions {
        let required = unsafe { CStr::from_ptr(*required) };

        let mut found = false;
        for supported in &supported_extensions {
            let supported = supported.extension_name_as_c_str()?;
            if supported == required {
                found = true;
                log::trace!("Device extension \"{}\" is supported", required.to_str()?);
                break;
            }
        }

        if !found {
            log::error!(
                "Device extension \"{}\" is not supported",
                required.to_str()?
            );
            return Ok(None);
        }
    }

    // TODO: check that gpu supports swapchain

    // check that gpu has a graphics queue family that can present to the surface
    let queue_props = unsafe { instance.get_physical_device_queue_family_properties(gpu) };
    for (index, props) in queue_props.iter().enumerate() {
        let support_graphics = props.queue_flags.contains(vk::QueueFlags::GRAPHICS);
        let support_presenting = surface.support_presenting(gpu, index as u32)?;

        if support_graphics && support_presenting {
            log::trace!("Device supports a graphics queue that can present to the surface");
            return Ok(Some(index as u32));
        }
    }

    Ok(None)
}

fn rate(props: &vk::PhysicalDeviceProperties) -> i32 {
    let mut score = match props.device_type {
        vk::PhysicalDeviceType::DISCRETE_GPU => 1000,
        vk::PhysicalDeviceType::INTEGRATED_GPU => 100,
        vk::PhysicalDeviceType::VIRTUAL_GPU => 50,
        vk::PhysicalDeviceType::CPU => 10,
        vk::PhysicalDeviceType::OTHER => 1,
        _ => unreachable!("Added new device type"),
    };
    score += props.limits.max_image_dimension2_d as i32;
    score
}

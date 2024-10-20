use std::ffi::{c_char, CStr, CString};

use ash::{self, ext, vk};

pub struct InstanceSpec {
    pub app_name: CString,
    pub extensions: Vec<*const c_char>,
    pub layers: Vec<*const c_char>,
    pub validation: bool,
}

pub struct Instance {
    _entry: ash::Entry,
    instance: ash::Instance,
    dbg_loader: Option<ext::debug_utils::Instance>,
    messenger: vk::DebugUtilsMessengerEXT,
}

impl Instance {
    pub fn new(spec: InstanceSpec) -> Result<Self, vk::Result> {
        let entry = ash::Entry::linked();

        let app_info = vk::ApplicationInfo::default()
            .api_version(vk::make_api_version(0, 1, 3, 0))
            .application_name(&spec.app_name);

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(&spec.layers)
            .enabled_extension_names(&spec.extensions);

        let mut severity = vk::DebugUtilsMessageSeverityFlagsEXT::default();
        severity |= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR;
        severity |= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING;
        //severity |= vk::DebugUtilsMessageSeverityFlagsEXT::INFO;
        //severity |= vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE;

        let mut type_ = vk::DebugUtilsMessageTypeFlagsEXT::default();
        type_ |= vk::DebugUtilsMessageTypeFlagsEXT::GENERAL;
        type_ |= vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION;
        type_ |= vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE;
        type_ |= vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING;

        let mut dbg_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(severity)
            .message_type(type_)
            .pfn_user_callback(Some(debug_callback));

        if spec.validation {
            let _ = create_info.push_next(&mut dbg_info);
        };

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        let (dbg_loader, messenger) = if spec.validation {
            let loader = ext::debug_utils::Instance::new(&entry, &instance);
            let messenger = unsafe { loader.create_debug_utils_messenger(&dbg_info, None)? };
            (Some(loader), messenger)
        } else {
            (None, vk::DebugUtilsMessengerEXT::null())
        };

        Ok(Self {
            _entry: entry,
            instance,
            dbg_loader,
            messenger,
        })
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if let Some(dbg_loader) = &self.dbg_loader {
            unsafe { dbg_loader.destroy_debug_utils_messenger(self.messenger, None) };
        }
        unsafe { self.instance.destroy_instance(None) }
    }
}

unsafe extern "system" fn debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message = if callback_data.p_message.is_null() {
        std::borrow::Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            log::trace!(target: "renderer", "{}", message)
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            log::info!(target: "renderer" , "{}", message)
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            log::warn!(target: "renderer", "{}", message)
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            log::error!(target: "renderer", "{}", message)
        }
        _ => unreachable!("Khronos added new Debug Messenger flags"),
    }

    vk::FALSE
}

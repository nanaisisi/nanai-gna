/// High-level GNA library components ported from `gna-lib`.

pub mod buffer_map;
pub use buffer_map::BufferMap;

pub mod buffer_map_impl;
pub use buffer_map_impl::BufferMapImpl;

pub mod compiled_model;
pub use compiled_model::CompiledModel;

pub mod request_configuration;
pub use request_configuration::RequestConfiguration;

pub mod request;
pub use request::Request;

pub mod request_builder;
pub use request_builder::RequestBuilder;

pub mod layer;
pub use layer::Layer;

pub mod layer_configuration;
pub use layer_configuration::LayerConfiguration;

pub mod transform;
pub use transform::BaseTransform;

pub mod transform_map;
pub use transform_map::TransformMap;

pub mod kernels;
pub use kernels::*;

pub mod tensor;
pub use tensor::Tensor;

pub mod model_wrapper;
pub use model_wrapper::ModelWrapper;

pub mod software_model;
pub use software_model::{SoftwareModel, SoftwareOnlyModel};

pub mod memory;
pub use memory::*;

// Mirror the original layout: expose gna-api under gna_lib::gna_api
pub mod gna_api;
pub use gna_api::*;

pub mod operation_config;
pub use operation_config::OperationConfig;

pub mod model_error;
pub use model_error::ModelError;

// Additional stubs generated from gna-lib/original
pub mod acceleration_detector;
pub mod activation_function;
pub mod activation_helper;
pub mod active_list;
pub mod affine_functions;
pub mod affine_layer_capabilities;
pub mod affine_layers;
pub mod api_wrapper;
pub mod auxiliary_capabilities;
pub mod bias;
pub mod buffer_config_validator;
pub mod capabilities;
pub mod component;
pub mod convolutional_functions;
pub mod convolutional_functions2d;
pub mod convolutional_layer;
pub mod convolutional_layer2d;
pub mod convolutional_layer2d_capabilities;
pub mod copy_layer;
pub mod data_mode;
pub mod device;
pub mod device_layer_support;
pub mod device_manager;
pub mod driver_interface;
pub mod expect;
pub mod export_device;
pub mod external_buffer;
pub mod gmm_layer;
pub mod gmm_layer_capabilities;
pub mod gna_config;
pub mod gna_types;
pub mod hardware_capabilities;
pub mod hardware_layer;
pub mod hardware_model;
pub mod hardware_model_no_mmu;
pub mod hardware_model_scorable;
pub mod hardware_model_sue1;
pub mod hardware_request;
pub mod hw_module_interface;
pub mod hybrid_device;
pub mod hybrid_model;
pub mod iscorable;
pub mod layer_capabilities;
pub mod layer_descriptor;
pub mod layer_input;
pub mod layer_output;
pub mod layout;
pub mod linux_driver_interface;
pub mod logger;
pub mod memory_container;
pub mod model_export_config;
pub mod parameter_limits;
pub mod pooling_functions;
pub mod pooling_functions2d;
pub mod pooling_mode;
pub mod profiler_configuration;
pub mod recurrent_function;
pub mod recurrent_layer;
pub mod request_handler;
pub mod shape;
pub mod sub_model;
pub mod string_helper;
pub mod thread_pool;
pub mod threshold_parameters;
pub mod transpose_layer;
pub mod validator;
pub mod weight;

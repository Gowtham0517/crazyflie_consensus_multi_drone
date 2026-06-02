#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__GoTo_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__GoTo_Request__init(msg: *mut GoTo_Request) -> bool;
    fn crazyflie_interfaces__srv__GoTo_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoTo_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__GoTo_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoTo_Request>);
    fn crazyflie_interfaces__srv__GoTo_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoTo_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GoTo_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__GoTo_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoTo_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_mask: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub relative: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: geometry_msgs::msg::rmw::Point,

    /// deg
    pub yaw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::rmw::Duration,

}



impl Default for GoTo_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__GoTo_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__GoTo_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoTo_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__GoTo_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__GoTo_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__GoTo_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoTo_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoTo_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/GoTo_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__GoTo_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__GoTo_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__GoTo_Response__init(msg: *mut GoTo_Response) -> bool;
    fn crazyflie_interfaces__srv__GoTo_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoTo_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__GoTo_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoTo_Response>);
    fn crazyflie_interfaces__srv__GoTo_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoTo_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GoTo_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__GoTo_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoTo_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GoTo_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__GoTo_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__GoTo_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoTo_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__GoTo_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__GoTo_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__GoTo_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoTo_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoTo_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/GoTo_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__GoTo_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Land_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__Land_Request__init(msg: *mut Land_Request) -> bool;
    fn crazyflie_interfaces__srv__Land_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__Land_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_Request>);
    fn crazyflie_interfaces__srv__Land_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__Land_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_mask: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub height: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::rmw::Duration,

}



impl Default for Land_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__Land_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__Land_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Land_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Land_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Land_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/Land_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Land_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Land_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__Land_Response__init(msg: *mut Land_Response) -> bool;
    fn crazyflie_interfaces__srv__Land_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Land_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__Land_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Land_Response>);
    fn crazyflie_interfaces__srv__Land_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Land_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Land_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__Land_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Land_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__Land_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__Land_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Land_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Land_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Land_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Land_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Land_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Land_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/Land_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Land_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__NotifySetpointsStop_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__NotifySetpointsStop_Request__init(msg: *mut NotifySetpointsStop_Request) -> bool;
    fn crazyflie_interfaces__srv__NotifySetpointsStop_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NotifySetpointsStop_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__NotifySetpointsStop_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NotifySetpointsStop_Request>);
    fn crazyflie_interfaces__srv__NotifySetpointsStop_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NotifySetpointsStop_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NotifySetpointsStop_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__NotifySetpointsStop_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NotifySetpointsStop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_mask: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub remain_valid_millisecs: u32,

}



impl Default for NotifySetpointsStop_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__NotifySetpointsStop_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__NotifySetpointsStop_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NotifySetpointsStop_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__NotifySetpointsStop_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__NotifySetpointsStop_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__NotifySetpointsStop_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NotifySetpointsStop_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NotifySetpointsStop_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/NotifySetpointsStop_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__NotifySetpointsStop_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__NotifySetpointsStop_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__NotifySetpointsStop_Response__init(msg: *mut NotifySetpointsStop_Response) -> bool;
    fn crazyflie_interfaces__srv__NotifySetpointsStop_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NotifySetpointsStop_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__NotifySetpointsStop_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NotifySetpointsStop_Response>);
    fn crazyflie_interfaces__srv__NotifySetpointsStop_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NotifySetpointsStop_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NotifySetpointsStop_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__NotifySetpointsStop_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NotifySetpointsStop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for NotifySetpointsStop_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__NotifySetpointsStop_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__NotifySetpointsStop_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NotifySetpointsStop_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__NotifySetpointsStop_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__NotifySetpointsStop_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__NotifySetpointsStop_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NotifySetpointsStop_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NotifySetpointsStop_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/NotifySetpointsStop_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__NotifySetpointsStop_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__SetGroupMask_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__SetGroupMask_Request__init(msg: *mut SetGroupMask_Request) -> bool;
    fn crazyflie_interfaces__srv__SetGroupMask_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGroupMask_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__SetGroupMask_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGroupMask_Request>);
    fn crazyflie_interfaces__srv__SetGroupMask_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGroupMask_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGroupMask_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__SetGroupMask_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGroupMask_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_mask: u8,

}



impl Default for SetGroupMask_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__SetGroupMask_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__SetGroupMask_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGroupMask_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__SetGroupMask_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__SetGroupMask_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__SetGroupMask_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGroupMask_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGroupMask_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/SetGroupMask_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__SetGroupMask_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__SetGroupMask_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__SetGroupMask_Response__init(msg: *mut SetGroupMask_Response) -> bool;
    fn crazyflie_interfaces__srv__SetGroupMask_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGroupMask_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__SetGroupMask_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGroupMask_Response>);
    fn crazyflie_interfaces__srv__SetGroupMask_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGroupMask_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGroupMask_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__SetGroupMask_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGroupMask_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SetGroupMask_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__SetGroupMask_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__SetGroupMask_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGroupMask_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__SetGroupMask_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__SetGroupMask_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__SetGroupMask_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGroupMask_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGroupMask_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/SetGroupMask_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__SetGroupMask_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__StartTrajectory_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__StartTrajectory_Request__init(msg: *mut StartTrajectory_Request) -> bool;
    fn crazyflie_interfaces__srv__StartTrajectory_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__StartTrajectory_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Request>);
    fn crazyflie_interfaces__srv__StartTrajectory_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartTrajectory_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__StartTrajectory_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartTrajectory_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_mask: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timescale: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reversed: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub relative: bool,

}



impl Default for StartTrajectory_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__StartTrajectory_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__StartTrajectory_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartTrajectory_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__StartTrajectory_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__StartTrajectory_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__StartTrajectory_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartTrajectory_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartTrajectory_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/StartTrajectory_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__StartTrajectory_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__StartTrajectory_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__StartTrajectory_Response__init(msg: *mut StartTrajectory_Response) -> bool;
    fn crazyflie_interfaces__srv__StartTrajectory_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__StartTrajectory_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Response>);
    fn crazyflie_interfaces__srv__StartTrajectory_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartTrajectory_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__StartTrajectory_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartTrajectory_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StartTrajectory_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__StartTrajectory_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__StartTrajectory_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartTrajectory_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__StartTrajectory_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__StartTrajectory_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__StartTrajectory_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartTrajectory_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartTrajectory_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/StartTrajectory_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__StartTrajectory_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Stop_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__Stop_Request__init(msg: *mut Stop_Request) -> bool;
    fn crazyflie_interfaces__srv__Stop_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stop_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__Stop_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stop_Request>);
    fn crazyflie_interfaces__srv__Stop_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stop_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Stop_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__Stop_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_mask: u8,

}



impl Default for Stop_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__Stop_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__Stop_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stop_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Stop_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Stop_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Stop_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stop_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stop_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/Stop_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Stop_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Stop_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__Stop_Response__init(msg: *mut Stop_Response) -> bool;
    fn crazyflie_interfaces__srv__Stop_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stop_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__Stop_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stop_Response>);
    fn crazyflie_interfaces__srv__Stop_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stop_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Stop_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__Stop_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Stop_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__Stop_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__Stop_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stop_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Stop_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Stop_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Stop_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stop_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stop_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/Stop_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Stop_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Takeoff_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__Takeoff_Request__init(msg: *mut Takeoff_Request) -> bool;
    fn crazyflie_interfaces__srv__Takeoff_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__Takeoff_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Request>);
    fn crazyflie_interfaces__srv__Takeoff_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__Takeoff_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_mask: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub height: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::rmw::Duration,

}



impl Default for Takeoff_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__Takeoff_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__Takeoff_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Takeoff_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Takeoff_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Takeoff_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/Takeoff_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Takeoff_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Takeoff_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__Takeoff_Response__init(msg: *mut Takeoff_Response) -> bool;
    fn crazyflie_interfaces__srv__Takeoff_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__Takeoff_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Response>);
    fn crazyflie_interfaces__srv__Takeoff_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Takeoff_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Takeoff_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__Takeoff_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Takeoff_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__Takeoff_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__Takeoff_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Takeoff_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Takeoff_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Takeoff_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Takeoff_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Takeoff_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Takeoff_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/Takeoff_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Takeoff_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__UpdateParams_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__UpdateParams_Request__init(msg: *mut UpdateParams_Request) -> bool;
    fn crazyflie_interfaces__srv__UpdateParams_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UpdateParams_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__UpdateParams_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UpdateParams_Request>);
    fn crazyflie_interfaces__srv__UpdateParams_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UpdateParams_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UpdateParams_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__UpdateParams_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdateParams_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub params: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for UpdateParams_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__UpdateParams_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__UpdateParams_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UpdateParams_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UpdateParams_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UpdateParams_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UpdateParams_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UpdateParams_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UpdateParams_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/UpdateParams_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__UpdateParams_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__UpdateParams_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__UpdateParams_Response__init(msg: *mut UpdateParams_Response) -> bool;
    fn crazyflie_interfaces__srv__UpdateParams_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UpdateParams_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__UpdateParams_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UpdateParams_Response>);
    fn crazyflie_interfaces__srv__UpdateParams_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UpdateParams_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UpdateParams_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__UpdateParams_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdateParams_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for UpdateParams_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__UpdateParams_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__UpdateParams_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UpdateParams_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UpdateParams_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UpdateParams_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UpdateParams_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UpdateParams_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UpdateParams_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/UpdateParams_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__UpdateParams_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__UploadTrajectory_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__UploadTrajectory_Request__init(msg: *mut UploadTrajectory_Request) -> bool;
    fn crazyflie_interfaces__srv__UploadTrajectory_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UploadTrajectory_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__UploadTrajectory_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UploadTrajectory_Request>);
    fn crazyflie_interfaces__srv__UploadTrajectory_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UploadTrajectory_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UploadTrajectory_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__UploadTrajectory_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadTrajectory_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub piece_offset: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pieces: rosidl_runtime_rs::Sequence<super::super::msg::rmw::TrajectoryPolynomialPiece>,

}



impl Default for UploadTrajectory_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__UploadTrajectory_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__UploadTrajectory_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UploadTrajectory_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UploadTrajectory_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UploadTrajectory_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UploadTrajectory_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UploadTrajectory_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UploadTrajectory_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/UploadTrajectory_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__UploadTrajectory_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__UploadTrajectory_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__UploadTrajectory_Response__init(msg: *mut UploadTrajectory_Response) -> bool;
    fn crazyflie_interfaces__srv__UploadTrajectory_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UploadTrajectory_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__UploadTrajectory_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UploadTrajectory_Response>);
    fn crazyflie_interfaces__srv__UploadTrajectory_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UploadTrajectory_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UploadTrajectory_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__UploadTrajectory_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadTrajectory_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for UploadTrajectory_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__UploadTrajectory_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__UploadTrajectory_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UploadTrajectory_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UploadTrajectory_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UploadTrajectory_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__UploadTrajectory_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UploadTrajectory_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UploadTrajectory_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/UploadTrajectory_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__UploadTrajectory_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__RemoveLogging_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__RemoveLogging_Request__init(msg: *mut RemoveLogging_Request) -> bool;
    fn crazyflie_interfaces__srv__RemoveLogging_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RemoveLogging_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__RemoveLogging_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RemoveLogging_Request>);
    fn crazyflie_interfaces__srv__RemoveLogging_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RemoveLogging_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RemoveLogging_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__RemoveLogging_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoveLogging_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub topic_name: rosidl_runtime_rs::String,

}



impl Default for RemoveLogging_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__RemoveLogging_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__RemoveLogging_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RemoveLogging_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__RemoveLogging_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__RemoveLogging_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__RemoveLogging_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RemoveLogging_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RemoveLogging_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/RemoveLogging_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__RemoveLogging_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__RemoveLogging_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__RemoveLogging_Response__init(msg: *mut RemoveLogging_Response) -> bool;
    fn crazyflie_interfaces__srv__RemoveLogging_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RemoveLogging_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__RemoveLogging_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RemoveLogging_Response>);
    fn crazyflie_interfaces__srv__RemoveLogging_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RemoveLogging_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RemoveLogging_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__RemoveLogging_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoveLogging_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for RemoveLogging_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__RemoveLogging_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__RemoveLogging_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RemoveLogging_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__RemoveLogging_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__RemoveLogging_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__RemoveLogging_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RemoveLogging_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RemoveLogging_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/RemoveLogging_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__RemoveLogging_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__AddLogging_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__AddLogging_Request__init(msg: *mut AddLogging_Request) -> bool;
    fn crazyflie_interfaces__srv__AddLogging_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddLogging_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__AddLogging_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddLogging_Request>);
    fn crazyflie_interfaces__srv__AddLogging_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddLogging_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AddLogging_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__AddLogging_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddLogging_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub topic_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frequency: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vars: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for AddLogging_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__AddLogging_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__AddLogging_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddLogging_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__AddLogging_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__AddLogging_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__AddLogging_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddLogging_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddLogging_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/AddLogging_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__AddLogging_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__AddLogging_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__AddLogging_Response__init(msg: *mut AddLogging_Response) -> bool;
    fn crazyflie_interfaces__srv__AddLogging_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddLogging_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__AddLogging_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddLogging_Response>);
    fn crazyflie_interfaces__srv__AddLogging_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddLogging_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AddLogging_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__AddLogging_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddLogging_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for AddLogging_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__AddLogging_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__AddLogging_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddLogging_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__AddLogging_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__AddLogging_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__AddLogging_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddLogging_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddLogging_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/AddLogging_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__AddLogging_Response() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Arm_Request() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__Arm_Request__init(msg: *mut Arm_Request) -> bool;
    fn crazyflie_interfaces__srv__Arm_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Arm_Request>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__Arm_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Arm_Request>);
    fn crazyflie_interfaces__srv__Arm_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Arm_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Arm_Request>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__Arm_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arm_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub arm: bool,

}



impl Default for Arm_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__Arm_Request__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__Arm_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Arm_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Arm_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Arm_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Arm_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Arm_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Arm_Request where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/Arm_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Arm_Request() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Arm_Response() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__srv__Arm_Response__init(msg: *mut Arm_Response) -> bool;
    fn crazyflie_interfaces__srv__Arm_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Arm_Response>, size: usize) -> bool;
    fn crazyflie_interfaces__srv__Arm_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Arm_Response>);
    fn crazyflie_interfaces__srv__Arm_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Arm_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Arm_Response>) -> bool;
}

// Corresponds to crazyflie_interfaces__srv__Arm_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arm_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Arm_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__srv__Arm_Response__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__srv__Arm_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Arm_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Arm_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Arm_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__srv__Arm_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Arm_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Arm_Response where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/srv/Arm_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__srv__Arm_Response() }
  }
}






#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__GoTo() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__GoTo
#[allow(missing_docs, non_camel_case_types)]
pub struct GoTo;

impl rosidl_runtime_rs::Service for GoTo {
    type Request = GoTo_Request;
    type Response = GoTo_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__GoTo() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__Land() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__Land
#[allow(missing_docs, non_camel_case_types)]
pub struct Land;

impl rosidl_runtime_rs::Service for Land {
    type Request = Land_Request;
    type Response = Land_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__Land() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__NotifySetpointsStop() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__NotifySetpointsStop
#[allow(missing_docs, non_camel_case_types)]
pub struct NotifySetpointsStop;

impl rosidl_runtime_rs::Service for NotifySetpointsStop {
    type Request = NotifySetpointsStop_Request;
    type Response = NotifySetpointsStop_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__NotifySetpointsStop() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__SetGroupMask() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__SetGroupMask
#[allow(missing_docs, non_camel_case_types)]
pub struct SetGroupMask;

impl rosidl_runtime_rs::Service for SetGroupMask {
    type Request = SetGroupMask_Request;
    type Response = SetGroupMask_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__SetGroupMask() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__StartTrajectory() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__StartTrajectory
#[allow(missing_docs, non_camel_case_types)]
pub struct StartTrajectory;

impl rosidl_runtime_rs::Service for StartTrajectory {
    type Request = StartTrajectory_Request;
    type Response = StartTrajectory_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__StartTrajectory() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__Stop() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__Stop
#[allow(missing_docs, non_camel_case_types)]
pub struct Stop;

impl rosidl_runtime_rs::Service for Stop {
    type Request = Stop_Request;
    type Response = Stop_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__Stop() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__Takeoff() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__Takeoff
#[allow(missing_docs, non_camel_case_types)]
pub struct Takeoff;

impl rosidl_runtime_rs::Service for Takeoff {
    type Request = Takeoff_Request;
    type Response = Takeoff_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__Takeoff() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__UpdateParams() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__UpdateParams
#[allow(missing_docs, non_camel_case_types)]
pub struct UpdateParams;

impl rosidl_runtime_rs::Service for UpdateParams {
    type Request = UpdateParams_Request;
    type Response = UpdateParams_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__UpdateParams() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__UploadTrajectory() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__UploadTrajectory
#[allow(missing_docs, non_camel_case_types)]
pub struct UploadTrajectory;

impl rosidl_runtime_rs::Service for UploadTrajectory {
    type Request = UploadTrajectory_Request;
    type Response = UploadTrajectory_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__UploadTrajectory() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__RemoveLogging() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__RemoveLogging
#[allow(missing_docs, non_camel_case_types)]
pub struct RemoveLogging;

impl rosidl_runtime_rs::Service for RemoveLogging {
    type Request = RemoveLogging_Request;
    type Response = RemoveLogging_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__RemoveLogging() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__AddLogging() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__AddLogging
#[allow(missing_docs, non_camel_case_types)]
pub struct AddLogging;

impl rosidl_runtime_rs::Service for AddLogging {
    type Request = AddLogging_Request;
    type Response = AddLogging_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__AddLogging() }
    }
}




#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__Arm() -> *const std::ffi::c_void;
}

// Corresponds to crazyflie_interfaces__srv__Arm
#[allow(missing_docs, non_camel_case_types)]
pub struct Arm;

impl rosidl_runtime_rs::Service for Arm {
    type Request = Arm_Request;
    type Response = Arm_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__crazyflie_interfaces__srv__Arm() }
    }
}



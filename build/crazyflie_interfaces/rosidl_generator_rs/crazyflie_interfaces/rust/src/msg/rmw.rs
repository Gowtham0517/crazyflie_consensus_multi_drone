#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__ConnectionStatistics() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__ConnectionStatistics__init(msg: *mut ConnectionStatistics) -> bool;
    fn crazyflie_interfaces__msg__ConnectionStatistics__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConnectionStatistics>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__ConnectionStatistics__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConnectionStatistics>);
    fn crazyflie_interfaces__msg__ConnectionStatistics__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConnectionStatistics>, out_seq: *mut rosidl_runtime_rs::Sequence<ConnectionStatistics>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__ConnectionStatistics
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConnectionStatistics {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uri: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sent_count: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sent_ping_count: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub receive_count: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub enqueued_count: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ack_count: u64,

}



impl Default for ConnectionStatistics {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__ConnectionStatistics__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__ConnectionStatistics__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConnectionStatistics {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__ConnectionStatistics__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__ConnectionStatistics__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__ConnectionStatistics__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConnectionStatistics {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConnectionStatistics where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/ConnectionStatistics";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__ConnectionStatistics() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__ConnectionStatisticsArray() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__ConnectionStatisticsArray__init(msg: *mut ConnectionStatisticsArray) -> bool;
    fn crazyflie_interfaces__msg__ConnectionStatisticsArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConnectionStatisticsArray>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__ConnectionStatisticsArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConnectionStatisticsArray>);
    fn crazyflie_interfaces__msg__ConnectionStatisticsArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConnectionStatisticsArray>, out_seq: *mut rosidl_runtime_rs::Sequence<ConnectionStatisticsArray>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__ConnectionStatisticsArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConnectionStatisticsArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stats: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConnectionStatistics>,

}



impl Default for ConnectionStatisticsArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__ConnectionStatisticsArray__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__ConnectionStatisticsArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConnectionStatisticsArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__ConnectionStatisticsArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__ConnectionStatisticsArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__ConnectionStatisticsArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConnectionStatisticsArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConnectionStatisticsArray where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/ConnectionStatisticsArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__ConnectionStatisticsArray() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__FullState() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__FullState__init(msg: *mut FullState) -> bool;
    fn crazyflie_interfaces__msg__FullState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FullState>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__FullState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FullState>);
    fn crazyflie_interfaces__msg__FullState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FullState>, out_seq: *mut rosidl_runtime_rs::Sequence<FullState>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__FullState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FullState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub twist: geometry_msgs::msg::rmw::Twist,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acc: geometry_msgs::msg::rmw::Vector3,

}



impl Default for FullState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__FullState__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__FullState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FullState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__FullState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__FullState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__FullState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FullState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FullState where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/FullState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__FullState() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__LogDataGeneric() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__LogDataGeneric__init(msg: *mut LogDataGeneric) -> bool;
    fn crazyflie_interfaces__msg__LogDataGeneric__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LogDataGeneric>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__LogDataGeneric__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LogDataGeneric>);
    fn crazyflie_interfaces__msg__LogDataGeneric__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LogDataGeneric>, out_seq: *mut rosidl_runtime_rs::Sequence<LogDataGeneric>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__LogDataGeneric
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LogDataGeneric {
    /// Header including the ROS 2 timestamp when the log data was received
    pub header: std_msgs::msg::rmw::Header,

    /// on-board timestamp from the STM32 (in ms)
    pub timestamp: u32,

    /// converted values, in the order as specified for the log block
    pub values: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for LogDataGeneric {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__LogDataGeneric__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__LogDataGeneric__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LogDataGeneric {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogDataGeneric__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogDataGeneric__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogDataGeneric__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LogDataGeneric {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LogDataGeneric where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/LogDataGeneric";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__LogDataGeneric() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Hover() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__Hover__init(msg: *mut Hover) -> bool;
    fn crazyflie_interfaces__msg__Hover__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Hover>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__Hover__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Hover>);
    fn crazyflie_interfaces__msg__Hover__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Hover>, out_seq: *mut rosidl_runtime_rs::Sequence<Hover>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__Hover
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Hover {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vx: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vy: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_rate: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z_distance: f32,

}



impl Default for Hover {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__Hover__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__Hover__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Hover {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Hover__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Hover__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Hover__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Hover {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Hover where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/Hover";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Hover() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__LogBlock() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__LogBlock__init(msg: *mut LogBlock) -> bool;
    fn crazyflie_interfaces__msg__LogBlock__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LogBlock>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__LogBlock__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LogBlock>);
    fn crazyflie_interfaces__msg__LogBlock__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LogBlock>, out_seq: *mut rosidl_runtime_rs::Sequence<LogBlock>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__LogBlock
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LogBlock {

    // This member is not documented.
    #[allow(missing_docs)]
    pub topic_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frequency: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub variables: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for LogBlock {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__LogBlock__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__LogBlock__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LogBlock {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogBlock__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogBlock__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogBlock__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LogBlock {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LogBlock where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/LogBlock";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__LogBlock() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Position() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__Position__init(msg: *mut Position) -> bool;
    fn crazyflie_interfaces__msg__Position__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Position>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__Position__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Position>);
    fn crazyflie_interfaces__msg__Position__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Position>, out_seq: *mut rosidl_runtime_rs::Sequence<Position>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__Position
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Position {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f32,

}



impl Default for Position {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__Position__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__Position__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Position {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Position__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Position__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Position__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Position {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Position where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/Position";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Position() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Status() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__Status__init(msg: *mut Status) -> bool;
    fn crazyflie_interfaces__msg__Status__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Status>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__Status__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Status>);
    fn crazyflie_interfaces__msg__Status__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Status>, out_seq: *mut rosidl_runtime_rs::Sequence<Status>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__Status
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Status {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// general status
    /// Bitfield, see SUPERVISOR_INFO for individual bits
    pub supervisor_info: u16,

    /// battery related
    /// internal battery voltage
    pub battery_voltage: f32,

    /// See PM_STATE_* for potential values
    pub pm_state: u8,

    /// radio related
    /// latest radio signal strength indicator
    pub rssi: u8,

    /// number of received broadcast packets during the last period
    pub num_rx_broadcast: u32,

    /// number of broadcast packets sent during the last period
    pub num_tx_broadcast: u32,

    /// number of received unicast packets during the last period
    pub num_rx_unicast: u32,

    /// number of unicast packets sent during the last period
    pub num_tx_unicast: u32,

    /// latency (in ms) for unicast packets
    pub latency_unicast: u16,

}

impl Status {
    /// Constants
    pub const SUPERVISOR_INFO_CAN_BE_ARMED: u16 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SUPERVISOR_INFO_IS_ARMED: u16 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SUPERVISOR_INFO_AUTO_ARM: u16 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SUPERVISOR_INFO_CAN_FLY: u16 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SUPERVISOR_INFO_IS_FLYING: u16 = 16;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SUPERVISOR_INFO_IS_TUMBLED: u16 = 32;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SUPERVISOR_INFO_IS_LOCKED: u16 = 64;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PM_STATE_BATTERY: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PM_STATE_CHARGING: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PM_STATE_CHARGED: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PM_STATE_LOW_POWER: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PM_STATE_SHUTDOWN: u8 = 4;

}


impl Default for Status {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__Status__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__Status__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Status {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Status__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Status__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Status__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Status {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Status where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/Status";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Status() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__TrajectoryPolynomialPiece() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__TrajectoryPolynomialPiece__init(msg: *mut TrajectoryPolynomialPiece) -> bool;
    fn crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPolynomialPiece>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPolynomialPiece>);
    fn crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryPolynomialPiece>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPolynomialPiece>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__TrajectoryPolynomialPiece
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryPolynomialPiece {

    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_x: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_y: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_z: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_yaw: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::rmw::Duration,

}



impl Default for TrajectoryPolynomialPiece {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__TrajectoryPolynomialPiece__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__TrajectoryPolynomialPiece__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryPolynomialPiece {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryPolynomialPiece {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryPolynomialPiece where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/TrajectoryPolynomialPiece";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__TrajectoryPolynomialPiece() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__VelocityWorld() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__VelocityWorld__init(msg: *mut VelocityWorld) -> bool;
    fn crazyflie_interfaces__msg__VelocityWorld__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VelocityWorld>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__VelocityWorld__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VelocityWorld>);
    fn crazyflie_interfaces__msg__VelocityWorld__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VelocityWorld>, out_seq: *mut rosidl_runtime_rs::Sequence<VelocityWorld>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__VelocityWorld
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelocityWorld {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vel: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_rate: f32,

}



impl Default for VelocityWorld {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__VelocityWorld__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__VelocityWorld__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VelocityWorld {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__VelocityWorld__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__VelocityWorld__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__VelocityWorld__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VelocityWorld {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VelocityWorld where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/VelocityWorld";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__VelocityWorld() }
  }
}



#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to crazyflie_interfaces__srv__GoTo_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub goal: geometry_msgs::msg::Point,

    /// deg
    pub yaw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::Duration,

}



impl Default for GoTo_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GoTo_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GoTo_Request {
  type RmwMsg = super::srv::rmw::GoTo_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_mask: msg.group_mask,
        relative: msg.relative,
        goal: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
        yaw: msg.yaw,
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.duration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group_mask: msg.group_mask,
      relative: msg.relative,
        goal: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      yaw: msg.yaw,
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.duration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_mask: msg.group_mask,
      relative: msg.relative,
      goal: geometry_msgs::msg::Point::from_rmw_message(msg.goal),
      yaw: msg.yaw,
      duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.duration),
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__GoTo_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoTo_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GoTo_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GoTo_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GoTo_Response {
  type RmwMsg = super::srv::rmw::GoTo_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__Land_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub duration: builtin_interfaces::msg::Duration,

}



impl Default for Land_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Land_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Land_Request {
  type RmwMsg = super::srv::rmw::Land_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_mask: msg.group_mask,
        height: msg.height,
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.duration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group_mask: msg.group_mask,
      height: msg.height,
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.duration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_mask: msg.group_mask,
      height: msg.height,
      duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.duration),
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__Land_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Land_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Land_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Land_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Land_Response {
  type RmwMsg = super::srv::rmw::Land_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__NotifySetpointsStop_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::NotifySetpointsStop_Request::default())
  }
}

impl rosidl_runtime_rs::Message for NotifySetpointsStop_Request {
  type RmwMsg = super::srv::rmw::NotifySetpointsStop_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_mask: msg.group_mask,
        remain_valid_millisecs: msg.remain_valid_millisecs,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group_mask: msg.group_mask,
      remain_valid_millisecs: msg.remain_valid_millisecs,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_mask: msg.group_mask,
      remain_valid_millisecs: msg.remain_valid_millisecs,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__NotifySetpointsStop_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NotifySetpointsStop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for NotifySetpointsStop_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::NotifySetpointsStop_Response::default())
  }
}

impl rosidl_runtime_rs::Message for NotifySetpointsStop_Response {
  type RmwMsg = super::srv::rmw::NotifySetpointsStop_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__SetGroupMask_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGroupMask_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_mask: u8,

}



impl Default for SetGroupMask_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGroupMask_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetGroupMask_Request {
  type RmwMsg = super::srv::rmw::SetGroupMask_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_mask: msg.group_mask,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group_mask: msg.group_mask,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_mask: msg.group_mask,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__SetGroupMask_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGroupMask_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SetGroupMask_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGroupMask_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetGroupMask_Response {
  type RmwMsg = super::srv::rmw::SetGroupMask_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__StartTrajectory_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartTrajectory_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StartTrajectory_Request {
  type RmwMsg = super::srv::rmw::StartTrajectory_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_mask: msg.group_mask,
        trajectory_id: msg.trajectory_id,
        timescale: msg.timescale,
        reversed: msg.reversed,
        relative: msg.relative,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group_mask: msg.group_mask,
      trajectory_id: msg.trajectory_id,
      timescale: msg.timescale,
      reversed: msg.reversed,
      relative: msg.relative,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_mask: msg.group_mask,
      trajectory_id: msg.trajectory_id,
      timescale: msg.timescale,
      reversed: msg.reversed,
      relative: msg.relative,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__StartTrajectory_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartTrajectory_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StartTrajectory_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartTrajectory_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StartTrajectory_Response {
  type RmwMsg = super::srv::rmw::StartTrajectory_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__Stop_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_mask: u8,

}



impl Default for Stop_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Stop_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Stop_Request {
  type RmwMsg = super::srv::rmw::Stop_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_mask: msg.group_mask,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group_mask: msg.group_mask,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_mask: msg.group_mask,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__Stop_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Stop_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Stop_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Stop_Response {
  type RmwMsg = super::srv::rmw::Stop_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__Takeoff_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub duration: builtin_interfaces::msg::Duration,

}



impl Default for Takeoff_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Takeoff_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Takeoff_Request {
  type RmwMsg = super::srv::rmw::Takeoff_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_mask: msg.group_mask,
        height: msg.height,
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.duration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group_mask: msg.group_mask,
      height: msg.height,
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.duration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_mask: msg.group_mask,
      height: msg.height,
      duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.duration),
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__Takeoff_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Takeoff_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Takeoff_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Takeoff_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Takeoff_Response {
  type RmwMsg = super::srv::rmw::Takeoff_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__UpdateParams_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdateParams_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub params: Vec<std::string::String>,

}



impl Default for UpdateParams_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UpdateParams_Request::default())
  }
}

impl rosidl_runtime_rs::Message for UpdateParams_Request {
  type RmwMsg = super::srv::rmw::UpdateParams_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        params: msg.params
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        params: msg.params
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      params: msg.params
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__UpdateParams_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdateParams_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for UpdateParams_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UpdateParams_Response::default())
  }
}

impl rosidl_runtime_rs::Message for UpdateParams_Response {
  type RmwMsg = super::srv::rmw::UpdateParams_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__UploadTrajectory_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub pieces: Vec<super::msg::TrajectoryPolynomialPiece>,

}



impl Default for UploadTrajectory_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UploadTrajectory_Request::default())
  }
}

impl rosidl_runtime_rs::Message for UploadTrajectory_Request {
  type RmwMsg = super::srv::rmw::UploadTrajectory_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory_id: msg.trajectory_id,
        piece_offset: msg.piece_offset,
        pieces: msg.pieces
          .into_iter()
          .map(|elem| super::msg::TrajectoryPolynomialPiece::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      trajectory_id: msg.trajectory_id,
      piece_offset: msg.piece_offset,
        pieces: msg.pieces
          .iter()
          .map(|elem| super::msg::TrajectoryPolynomialPiece::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory_id: msg.trajectory_id,
      piece_offset: msg.piece_offset,
      pieces: msg.pieces
          .into_iter()
          .map(super::msg::TrajectoryPolynomialPiece::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__UploadTrajectory_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadTrajectory_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for UploadTrajectory_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UploadTrajectory_Response::default())
  }
}

impl rosidl_runtime_rs::Message for UploadTrajectory_Response {
  type RmwMsg = super::srv::rmw::UploadTrajectory_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__RemoveLogging_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoveLogging_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub topic_name: std::string::String,

}



impl Default for RemoveLogging_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RemoveLogging_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RemoveLogging_Request {
  type RmwMsg = super::srv::rmw::RemoveLogging_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic_name: msg.topic_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic_name: msg.topic_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      topic_name: msg.topic_name.to_string(),
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__RemoveLogging_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoveLogging_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for RemoveLogging_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RemoveLogging_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RemoveLogging_Response {
  type RmwMsg = super::srv::rmw::RemoveLogging_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__AddLogging_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddLogging_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub topic_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frequency: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vars: Vec<std::string::String>,

}



impl Default for AddLogging_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddLogging_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AddLogging_Request {
  type RmwMsg = super::srv::rmw::AddLogging_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic_name: msg.topic_name.as_str().into(),
        frequency: msg.frequency,
        vars: msg.vars
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic_name: msg.topic_name.as_str().into(),
      frequency: msg.frequency,
        vars: msg.vars
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      topic_name: msg.topic_name.to_string(),
      frequency: msg.frequency,
      vars: msg.vars
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__AddLogging_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddLogging_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for AddLogging_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddLogging_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AddLogging_Response {
  type RmwMsg = super::srv::rmw::AddLogging_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__Arm_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arm_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub arm: bool,

}



impl Default for Arm_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Arm_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Arm_Request {
  type RmwMsg = super::srv::rmw::Arm_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        arm: msg.arm,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      arm: msg.arm,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      arm: msg.arm,
    }
  }
}


// Corresponds to crazyflie_interfaces__srv__Arm_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arm_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Arm_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Arm_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Arm_Response {
  type RmwMsg = super::srv::rmw::Arm_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
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



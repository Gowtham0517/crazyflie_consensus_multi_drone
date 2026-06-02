#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to crazyflie_interfaces__msg__ConnectionStatistics

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConnectionStatistics {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uri: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConnectionStatistics::default())
  }
}

impl rosidl_runtime_rs::Message for ConnectionStatistics {
  type RmwMsg = super::msg::rmw::ConnectionStatistics;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uri: msg.uri.as_str().into(),
        sent_count: msg.sent_count,
        sent_ping_count: msg.sent_ping_count,
        receive_count: msg.receive_count,
        enqueued_count: msg.enqueued_count,
        ack_count: msg.ack_count,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uri: msg.uri.as_str().into(),
      sent_count: msg.sent_count,
      sent_ping_count: msg.sent_ping_count,
      receive_count: msg.receive_count,
      enqueued_count: msg.enqueued_count,
      ack_count: msg.ack_count,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      uri: msg.uri.to_string(),
      sent_count: msg.sent_count,
      sent_ping_count: msg.sent_ping_count,
      receive_count: msg.receive_count,
      enqueued_count: msg.enqueued_count,
      ack_count: msg.ack_count,
    }
  }
}


// Corresponds to crazyflie_interfaces__msg__ConnectionStatisticsArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConnectionStatisticsArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stats: Vec<super::msg::ConnectionStatistics>,

}



impl Default for ConnectionStatisticsArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConnectionStatisticsArray::default())
  }
}

impl rosidl_runtime_rs::Message for ConnectionStatisticsArray {
  type RmwMsg = super::msg::rmw::ConnectionStatisticsArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        stats: msg.stats
          .into_iter()
          .map(|elem| super::msg::ConnectionStatistics::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        stats: msg.stats
          .iter()
          .map(|elem| super::msg::ConnectionStatistics::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      stats: msg.stats
          .into_iter()
          .map(super::msg::ConnectionStatistics::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to crazyflie_interfaces__msg__FullState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FullState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub twist: geometry_msgs::msg::Twist,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acc: geometry_msgs::msg::Vector3,

}



impl Default for FullState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FullState::default())
  }
}

impl rosidl_runtime_rs::Message for FullState {
  type RmwMsg = super::msg::rmw::FullState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.twist)).into_owned(),
        acc: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.acc)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        twist: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.twist)).into_owned(),
        acc: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.acc)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      twist: geometry_msgs::msg::Twist::from_rmw_message(msg.twist),
      acc: geometry_msgs::msg::Vector3::from_rmw_message(msg.acc),
    }
  }
}


// Corresponds to crazyflie_interfaces__msg__LogDataGeneric

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LogDataGeneric {
    /// Header including the ROS 2 timestamp when the log data was received
    pub header: std_msgs::msg::Header,

    /// on-board timestamp from the STM32 (in ms)
    pub timestamp: u32,

    /// converted values, in the order as specified for the log block
    pub values: Vec<f32>,

}



impl Default for LogDataGeneric {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LogDataGeneric::default())
  }
}

impl rosidl_runtime_rs::Message for LogDataGeneric {
  type RmwMsg = super::msg::rmw::LogDataGeneric;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        timestamp: msg.timestamp,
        values: msg.values.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      timestamp: msg.timestamp,
        values: msg.values.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      timestamp: msg.timestamp,
      values: msg.values
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to crazyflie_interfaces__msg__Hover

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Hover {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Hover::default())
  }
}

impl rosidl_runtime_rs::Message for Hover {
  type RmwMsg = super::msg::rmw::Hover;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        vx: msg.vx,
        vy: msg.vy,
        yaw_rate: msg.yaw_rate,
        z_distance: msg.z_distance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      vx: msg.vx,
      vy: msg.vy,
      yaw_rate: msg.yaw_rate,
      z_distance: msg.z_distance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      vx: msg.vx,
      vy: msg.vy,
      yaw_rate: msg.yaw_rate,
      z_distance: msg.z_distance,
    }
  }
}


// Corresponds to crazyflie_interfaces__msg__LogBlock

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LogBlock {

    // This member is not documented.
    #[allow(missing_docs)]
    pub topic_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frequency: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub variables: Vec<std::string::String>,

}



impl Default for LogBlock {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LogBlock::default())
  }
}

impl rosidl_runtime_rs::Message for LogBlock {
  type RmwMsg = super::msg::rmw::LogBlock;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic_name: msg.topic_name.as_str().into(),
        frequency: msg.frequency,
        variables: msg.variables
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic_name: msg.topic_name.as_str().into(),
      frequency: msg.frequency,
        variables: msg.variables
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
      variables: msg.variables
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to crazyflie_interfaces__msg__Position

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Position {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Position::default())
  }
}

impl rosidl_runtime_rs::Message for Position {
  type RmwMsg = super::msg::rmw::Position;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        x: msg.x,
        y: msg.y,
        z: msg.z,
        yaw: msg.yaw,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      x: msg.x,
      y: msg.y,
      z: msg.z,
      yaw: msg.yaw,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      x: msg.x,
      y: msg.y,
      z: msg.z,
      yaw: msg.yaw,
    }
  }
}


// Corresponds to crazyflie_interfaces__msg__Status

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Status {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Status::default())
  }
}

impl rosidl_runtime_rs::Message for Status {
  type RmwMsg = super::msg::rmw::Status;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        supervisor_info: msg.supervisor_info,
        battery_voltage: msg.battery_voltage,
        pm_state: msg.pm_state,
        rssi: msg.rssi,
        num_rx_broadcast: msg.num_rx_broadcast,
        num_tx_broadcast: msg.num_tx_broadcast,
        num_rx_unicast: msg.num_rx_unicast,
        num_tx_unicast: msg.num_tx_unicast,
        latency_unicast: msg.latency_unicast,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      supervisor_info: msg.supervisor_info,
      battery_voltage: msg.battery_voltage,
      pm_state: msg.pm_state,
      rssi: msg.rssi,
      num_rx_broadcast: msg.num_rx_broadcast,
      num_tx_broadcast: msg.num_tx_broadcast,
      num_rx_unicast: msg.num_rx_unicast,
      num_tx_unicast: msg.num_tx_unicast,
      latency_unicast: msg.latency_unicast,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      supervisor_info: msg.supervisor_info,
      battery_voltage: msg.battery_voltage,
      pm_state: msg.pm_state,
      rssi: msg.rssi,
      num_rx_broadcast: msg.num_rx_broadcast,
      num_tx_broadcast: msg.num_tx_broadcast,
      num_rx_unicast: msg.num_rx_unicast,
      num_tx_unicast: msg.num_tx_unicast,
      latency_unicast: msg.latency_unicast,
    }
  }
}


// Corresponds to crazyflie_interfaces__msg__TrajectoryPolynomialPiece

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryPolynomialPiece {

    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_x: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_y: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_z: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_yaw: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::Duration,

}



impl Default for TrajectoryPolynomialPiece {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrajectoryPolynomialPiece::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectoryPolynomialPiece {
  type RmwMsg = super::msg::rmw::TrajectoryPolynomialPiece;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        poly_x: msg.poly_x.into(),
        poly_y: msg.poly_y.into(),
        poly_z: msg.poly_z.into(),
        poly_yaw: msg.poly_yaw.into(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.duration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        poly_x: msg.poly_x.as_slice().into(),
        poly_y: msg.poly_y.as_slice().into(),
        poly_z: msg.poly_z.as_slice().into(),
        poly_yaw: msg.poly_yaw.as_slice().into(),
        duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.duration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      poly_x: msg.poly_x
          .into_iter()
          .collect(),
      poly_y: msg.poly_y
          .into_iter()
          .collect(),
      poly_z: msg.poly_z
          .into_iter()
          .collect(),
      poly_yaw: msg.poly_yaw
          .into_iter()
          .collect(),
      duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.duration),
    }
  }
}


// Corresponds to crazyflie_interfaces__msg__VelocityWorld

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelocityWorld {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vel: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_rate: f32,

}



impl Default for VelocityWorld {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::VelocityWorld::default())
  }
}

impl rosidl_runtime_rs::Message for VelocityWorld {
  type RmwMsg = super::msg::rmw::VelocityWorld;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        vel: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.vel)).into_owned(),
        yaw_rate: msg.yaw_rate,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        vel: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.vel)).into_owned(),
      yaw_rate: msg.yaw_rate,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      vel: geometry_msgs::msg::Vector3::from_rmw_message(msg.vel),
      yaw_rate: msg.yaw_rate,
    }
  }
}



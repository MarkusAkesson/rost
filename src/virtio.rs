
pub mod queue

const VENDOR_ID: u32 = 0x1AF4;
const DEVICE_ID: u32 = 0x103F;

pub enum SubsytemID {
    NetworkCard = 01,
    BlockDevice = 02,
    Console = 03,
    EntropySource =  04,
    MemoryBallooning = 05,
    IOMemory = 06,
    RPMSG = 07,
    SCSIHost = 08,
    NinePTransport  = 09,
    MAC_WLAN = 10,
}

pub enum IORegisterOffset {
    DeviceFeatures = 0x02,
    GuestFeatures = 0x04,
    QueueAddress = 0x08,
    QueueSize = 0x0C,
    QueueNotify = 10,
    DeviceStatus = 12,
    ISRStatus = 13,
}

pub enum StatusFlags {
    Ack = 0x1,
    Loaded = 0x2,
    Ready = 0x4,
    Error = 0x64,
    Failed = 0x128,
}

pub mod Device {
    pub mod Network {
        pub enum RegisterOffset {
            MACAddr1 = 0x14,
            MACAddr2 = 0x15,
            MACAddr3 = 0x16,
            MACAddr4 = 0x17,
            MACAddr5 = 0x18,
            MACAddr6 = 0x19,
            Status = 0x1a1
        }
    }

    pub mod Block {
        pub enum RegisterOffset {
            TotalSectorCount = 0x14,
            MaxSegmentSize = 0x1x,
            MaxSegmentCount = 0x20,
            CylinderCount = 0x24,
            HeadCount = 0x26,
            SectorCount = 0x27,
            BlockLength = 0x28,
        }
    }
}

pub trait VirtIoDevice {
    /// The virtio device type
    fn device_type(&self) -> SubsystemID;
    
    /// Number of supported queues
    fn num_queues(&self) -> u32;

    /// Return reference to queue at index `index` or None for a index out of range
    fn queue(&self, index: u32) -> Option<()>;
}

/// DMA invalid channel number. 
pub const DMA_INVALID_CHANNEL: u8 = 0xff;

/// DMA Priority Level.
pub enum DmaPriorityLevel {
    Level0,
    Level1,
    Level2,
    Level3,
}

/// DMA input actions.
pub enum DmaEventInputAction {
    /// No action.
    NoAct,
    /// Normal transfer and periodic transfer trigger.
    Trig,
    /// Conditional transfer trigger.
    CTrig,
    /// Conditional block transfer.
    CBlock,
    /// Channel suspend operation.
    Suspend,
    /// Channel resume operation.
    Resume,
    /// Skip next block suspend action.
    SSkip,
}

/// Address increment step size.
/// These bits select the address increment step size.
/// The setting applies to source or destination address,
/// depending on STEPSEL setting.
pub enum DmaAddressIncrementStepSize {
    /// The address is incremented by (beat size * 1).
    StepSize1,
    /// The address is incremented by (beat size * 2).
    StepSize2,
    /// The address is incremented by (beat size * 4).
    StepSize4,
    /// The address is incremented by (beat size * 8).
    StepSize8,
    /// The address is incremented by (beat size * 16).
    StepSize16,
    /// The address is incremented by (beat size * 32).
    StepSize32,
    /// The address is incremented by (beat size * 64).
    StepSize64,
    /// The address is incremented by (beat size * 128).
    StepSize128,
}

/// DMA step selection. This bit determines whether the step size setting
/// is applied to source or destination address.
pub enum DmaStepSelection {
	/// Step size settings apply to the destination address.
	Dst,
	/// Step size settings apply to the source address.
	Src,
}


/// The basic transfer unit in DMAC is a beat, which is defined as a
/// single bus access. Its size is configurable and applies to both read
/// and write.
pub enum DmaBeatSize {
    /// 8-bit access
    Byte,
    /// 16-bit access
    Hword,
    /// 32-bit access
    Word,
}

/// Block action definitions.
pub enum DmaBlockAction {
    /// No action
    NoAct,
    /// Channel in normal operation and sets transfer complete interrupt flag
    /// after block transfer.
    Int,
    /// Trigger channel suspend after block transfer and sets channel
    /// suspend interrupt flag once the channel is suspended
    Suspend,
    /// Sets transfer complete interrupt flag after a block transfer and
    /// trigger channel suspend. The channel suspend interrupt flag will be set
    /// once the channel is suspended. 
    Both,
}

/// Event output selection.
pub enum DmaEventOutputSelection {
    /// Event generation disable.
    Disable,
    /// Event strobe when block transfer complete.
    Block,
    /// Event output reserved.
    Reserved,
    /// Event strobe when beat transfer complete.
    Beat,
}

/// DMA trigger action type.
pub enum DmaTransferTriggerAction {
    /// Perform a block transfer when triggered.
    Block /* = DMAC_CHCTRLB_TRIGACT_BLOCK_Val */,
    /// Perform a beat transfer when triggered.
    Beat /* = DMAC_CHCTRLB_TRIGACT_BEAT_Val */,
    /// Perform a transaction when triggered.
    Transaction /* = DMAC_CHCTRLB_TRIGACT_TRANSACTION_Val */,
}

/// Callback types for DMA callback driver.
pub enum DmaCallbackType {
    /// Callback for any of transfer errors. 
    /// A transfer error is flagged if a bus error is detected during an AHB access
    /// or when the DMAC fetches an invalid descriptor.
    Error,
    /// Callback for transfer complete.
    Done,
    /// Callback for channel suspend.
    Suspend,
    /// Number of available callbacks.
    N,
}

/// DMA transfer descriptor configuration. When the source or destination address
/// increment is enabled, the addresses stored into the configuration structure
/// must correspond to the end of the transfer.
pub struct DmaDescriptorConfig {
    descriptor_valid: bool,
    event_output_selection: DmaEventOutputSelection,
    block_action: DmaBlockAction,
    beat_size: DmaBeatSize,
    src_increment_enable: bool,
    dst_increment_enable: bool,
    step_selection: DmaStepSelection,
    step_size: DmaAddressIncrementStepSize,
    block_transfer_count: u16,
    source_address: u32,
    destination_address: u32,
    next_descriptor_address: u32,
}
    
/// Configurations for DMA events.
pub struct DmaEventsConfig {
    input_action: DmaEventInputAction,
    event_output_enable: bool,
}

/// DMA configurations for transfer.
pub struct DmaResourceConfig {
    priority: DmaPriorityLevel,
    peripheral_trigger: u8,
    trigger_action: DmaTransferTriggerAction,
    event_config: DmaEventsConfig,
}


enum StatusCode {
    OK,
    Busy,
    Uninitialized,
    Suspend,
    ErrIO,
    ErrNotFound,
    ErrInvalidArg,
}

pub type DmaCallback = fn(DmaResource);

/// Structure for DMA transfer resource.
pub struct DmaResource {
    channel_id: u8,
    //TODO figure out the correct number
    callback: [DmaCallback; 3],
    callback_enable: u8,
    job_status: StatusCode, 
    transferred_size: u32,
    descriptor: DmacDescriptor,
}

bitfield!{
    struct BtCtrlType(u16); 
    impl Debug;
    pub valid, _: 0;
    pub evosel, set_evosel: 2,1;
    pub blockact, set_blockact: 4,3;
    pub beatsize, set_beatsize: 9,8;
    pub srcinc, set_srcinc: 10;
    pub dstinc, set_dstinc: 11;
    pub stepsel, set_stepsel: 12;
    pub stepsize, set_stepsize: 15,13;
}

type BtCntType = u16;
type SrcAddrType = u32;
type DstAddrType = u32;
type DescAddrType = u32;

pub struct DmacDescriptor {
   btctrl: BtCtrlType,
   btcnt: BtCntType,
   src_addr: SrcAddrType,
   dst_addr: DstAddrType,
   desc_addr: DescAddrType,
}

// End of dma.h, start of dma.c
const CONF_MAX_USED_CHANNEL_NUM: usize = 2;

struct DmaModule {
    dma_init: bool,
    allocated_channels: u32,
    free_channels: u8,
}
        
static mut dma_inst: DmaModule = DmaModule {
    dma_init: false,
    allocated_channels: 0,
    free_channels: CONF_MAX_USED_CHANNEL_NUM as u8,
};

const MAX_JOB_RESUME_COUNT: u16 = 10000;
const DMA_CHANNEL_MASK: u8 = 0x1f;

// requires .hsram to be defined in memory.x

#[link_section = ".hsram"]
static mut descriptor_section: 
    Option<[DmacDescriptor; CONF_MAX_USED_CHANNEL_NUM]> = None; 

#[link_section = ".hsram"]
static mut _write_back_section: 
    Option<[DmacDescriptor; CONF_MAX_USED_CHANNEL_NUM]> = None; 

static mut _dma_active_resource:
    Option<[&DmaResource; CONF_MAX_USED_CHANNEL_NUM]> = None;

static mut g_chan_interrupt_flag: 
    [u8; CONF_MAX_USED_CHANNEL_NUM] = [0; CONF_MAX_USED_CHANNEL_NUM];

fn system_interrupt_enter_critical_section() {
    unimplemented!()
}

fn system_interrupt_leave_critical_section() {
    unimplemented!()
}

fn _dma_find_first_free_channel_and_allocate() -> u8 {
    let count: u8;
    let tmp: u32;
    let allocated: bool = false;

    system_interrupt_enter_critical_section();

    unsafe {
        tmp = dma_inst.allocated_channels;
    }

    for count in 0..CONF_MAX_USED_CHANNEL_NUM {
        if !(tmp & 0x00000001) == 0 {
            // If free channel found, set as allocated and return number
            dma_inst.allocated_channels |= 1 << count;
            dma_inst.free_channels -= 1;
            allocated = true;
            break;
        }
        tmp = tmp >> 1;
    }

    system_interrupt_leave_critical_section();

    if !allocated {
        return DMA_INVALID_CHANNEL;
    } else {
        return count;
    }
}

fn _dma_release_channel(channel: u8) {
    dma_inst.allocated_channels &= !(1 << channel);
    dma_inst.free_channels += 1;
}

fn _dma_set_config(resource: DmaResource, resource_config: DmaResourceConfig) {
    system_interrupt_enter_critical_section();
    target_device::dmac::chid::write(|w| w.id(resource.channel_id));
    unsafe { target_device::dmac::swtrigctrl::modify(|r, w| {
        w.bits(r.bits() & !(1 << resource.channel_id) as u32)
    });}

    unsafe {
        target_device::dmac::chctrlb::write(|w| {
            w.lvl.bits(resource_config.priority);
            w.trigsrc.bits(resource_config.peripheral_trigger);
            w.trigact.bits(resource_config.trigger_action);

            if resource_config.event_config.input_action as usize != 0 {
                w.evie().set_bit();
                w.evact().bits(resource_config.event_config.input_action);
            }

            if resource_config.event_config.event_output_enable {
                w.evoe.set_bit();
            }
        });
    }
    system_interrupt_leave_critical_section()
}

fn DMAC_Handler() {
    let active_channel: u8;
    let resource: DmaResource;
    let isr: u8;
    let write_size: u32;
    let total_size: u32;

    system_interrupt_enter_critical_section();

    // Get pending channel
    active_channel = target_device::dmac::intpend::read().id();

    //Get active DMA resource based on channel
    unsafe {
        if _dma_active_resource.is_none() {
            resource =  *(_dma_active_resource.unwrap()[active_channel as usize]);
        }
        // Select the active channel
        target_device::dmac::chid::write(|w| w.bits(resource.channel_id));
    }
    isr = target_device::dmac::chintflag::read().bits();

    // Calculate block transfer size of the DMA transfer
    total_size = descriptor_section.unwrap()[resource.channel_id as usize].btcnt.into();
    write_size = _write_back_section.unwrap()[resource.channel_id as usize].btcnt.into();
    resource.transferred_size = total_size - write_size;

    // DMA channel interrupt handler
    if isr & target_device::dmac::chintenclr::read().terr().bit() != 0 {
        // Clear transfer error flag
        target_device::dmac::chintflag.modify(|r, w|
            w.bits(target_device::dmac::chintenclr.read().terr()));

        // Set IO Error status
        resource.job_status = StatusCode::ErrIO;

        // Execute the callback function
        if resource.callback_enable & (1 << DmaCallbackType::Error as usize) != 0 {
            resource.callback[DmaCallbackType::Error as usize](resource);
        }
    } else if isr & target_device::dmac::chintenclr::read().tcmpl().bit() != 0 {
        // Clear the transfer complete flag
        target_device::dmac::chintflag::write(|w| 
            target_device::dmac::chintenclr::read().tcmpl().bit());

        // Set job status
        resource.job_status = StatusCode::OK;
        
        // Execute the callback function
        if resource.callback_enable & (1 << DmaCallbackType::Done as usize) != 0 {
            resource.callback[DmaCallbackType::Done as usize](resource);
        }

    } else if isr & target_device::dmac::chintenclr::read().susp().bit() != 0 {
        // Clear the channel supsend flag
        target_device::dmac::chintflag::write(|w| 
            target_device::dmac::chintenclr::read().susp().bit());

        // Set job status
        resource.job_status = StatusCode::Suspend;
        
        // Execute the callback function
        if resource.callback_enable & (1 << DmaCallbackType::Suspend as usize) != 0 {
            resource.callback[DmaCallbackType::Suspend as usize](resource);
        }
    }
    system_interrupt_leave_critical_section();
}

fn dma_get_config_defaults() -> DmaResourceConfig {
    let event_config = DmaEventsConfig {
        input_action: DmaEventInputAction::NoAct,
        event_output_enable: false,
    };
    return DmaResourceConfig {
        priority: DmaPriorityLevel::Level0,
        peripheral_trigger: 0,
        trigger_action: DmaTransferTriggerAction::Transaction,
        event_config: event_config,
    }
}

fn dma_allocate(resource: DmaResource, config: DmaResourceConfig) -> StatusCode {
    let new_channel: u8;

    system_interrupt_enter_critical_section();

    if dma_inst.dma_init {
        // Initialize clocks for DMA
        // TODO: SAMD51 support
        target_device::pm::ahbmask::write(|w| w.dmac_());
        target_device::pm::apbbmask::write(|w| w.dmac_());

        // Perform a software reset before enable DMA controller
        target_device::dmac::ctrl::write(|w| {
            w.dmaenable().clear_bit();
            w.swrst().set_bit()
        });

        // Setup descriptor base address and write back section base address
        target_device::dmac::baseaddr::write(|w| 
            w.bits(descriptor_section.unwrap()) as u32);
        target_device::dmac::wrbaddr::write(|w| 
            w.bits(_write_back_section.unwrap()) as u32);

        // Enable all priority levels at the same time
        target_device::dmac::ctrl::write(|w| {
            w.dmaenable(); 
            w.lvlen0();
            w.lvlen1();
            w.lvlen2();
            w.lvlen3()
        });
        dma_inst.dma_init = true;
    }

    // Find the proper channel
    new_channel = _dma_find_first_free_channel_and_allocate();
    
    // If no channel is available, return not found
    if new_channel == DMA_INVALID_CHANNEL {
        system_interrupt_leave_critical_section();
        return StatusCode::ErrNotFound;
    }

    // Set the channel
    resource.channel_id = new_channel;

    // Perform a reset for the allocated channel
    unsafe {
        target_device::dmac::chid::write(|w| w.id.bits(resource.channel_id))
    }
    target_device::dmac::chctrla::write(|w| {
        w.dmaenable().clear_bit();
        w.swrst().set_bit()
    });

    // Configure the DMA control, channel registers and descriptors here
    _dma_set_config(resource, config);

    // resource->descriptor = NULL; maybe we need to turn this into an option?
    
    // Log the DMA resource into the internal DMA resource pool
    unsafe {
        _dma_active_resource.unwrap()[resource.channel_id as usize] = &resource;
    }

    system_interrupt_leave_critical_section();
    
    StatusCode::OK
}

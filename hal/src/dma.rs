pub const DMA_INVALID_CHANNEL: u8 = 0xff;

pub enum DmaPriorityLevel {
    Level0,
    Level1,
    Level2,
    Level3,
}

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

pub enum DmaTransferTriggerAction {
    /// Perform a block transfer when triggered.
    Block /* = DMAC_CHCTRLB_TRIGACT_BLOCK_Val */,
    /// Perform a beat transfer when triggered.
    Beat /* = DMAC_CHCTRLB_TRIGACT_BEAT_Val */,
    /// Perform a transaction when triggered.
    Transaction /* = DMAC_CHCTRLB_TRIGACT_TRANSACTION_Val */,
}

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

type DmaCallback<'a> = DmaResource<'a>;

pub enum StatusCode {
    OK,
    Busy,
    Uninitialized,
}

/// Structure for DMA transfer resource.
pub struct DmaResource<'a> {
    channel_id: u8,
    //TODO figure out the correct number
    callback: &'a [DmaCallback<'a>; 3],
    callback_enable: u8,
    job_status: StatusCode, 
    transferred_size: u32,
    //descriptor: &'a DmacDecriptor,
}


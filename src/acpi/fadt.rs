use super::sdt::Sdt;
use super::gas::Gas;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Fadt
{
    pub header: Sdt,
    pub firmware_ctrl: u32,
    pub dsdt: u32,
    pub reserved: u8,
    pub preferred_pm_profile: u8,
    pub sci_int: u16,
    pub smi_cmd: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4bios_req: u8,
    pub pstate_cnt: u8,
    pub pm1a_evt_blk: u32,
    pub pm1b_evt_blk: u32,
    pub pm1a_cnt_blk: u32,
    pub pm1b_cnt_blk: u32,
    pub pm2_cnt_blk: u32,
    pub pm_tmr_blk: u32,
    pub gpe0_blk: u32,
    pub gpe1_blk: u32,
    pub pm1_evt_len: u8,
    pub pm1_cnt_len: u8,
    pub pm2_cnt_len: u8,
    pub pm_tmr_len: u8,
    pub gpe0_blk_len: u8,
    pub gpe1_blk_len: u8,
    pub gpe1_base: u8,
    pub cst_cnt: u8,
    pub p_lvl2_lat: u16,
    pub p_lvl3_lat: u16,
    pub flush_size: u16,
    pub flush_stride: u16,
    pub duty_offset: u8,
    pub duty_width: u8,
    pub day_alrm: u8,
    pub mon_alrm: u8,
    pub century: u8,
    pub iapc_boot_arch: u16,
    pub reserved1: u8,
    pub flags: u32,
    pub reset_reg: Gas,
    pub reset_value: u8,
    pub arm_boot_arch: u16,
    pub fadt_minor_version: u8,
    pub x_firmware_ctrl: u64,
    pub x_dsdt: u64,
    pub x_pm1a_evt_blk: Gas,
    pub x_pm1b_evt_blk: Gas,
    pub x_pm1a_cnt_blk: Gas,
    pub x_pm1b_cnt_blk: Gas,
    pub x_pm2_cnt_blk: Gas,
    pub x_pm_tmr_blk: Gas,
    pub x_gpe0_blk: Gas,
    pub x_gpe1_blk: Gas,
    pub sleep_control_reg: Gas,
    pub sleep_status_reg: Gas,
    pub hypervisor_vendor_identity: u64,
}

impl Fadt
{
    pub fn new(sdt: &'static Sdt) -> &mut Fadt
    {
        unsafe {&mut*(sdt as *const Sdt as *mut Fadt)}
    }
}
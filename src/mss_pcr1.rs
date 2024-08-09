#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmprotset0: Pmprotset0,
    pmprotset1: Pmprotset1,
    _reserved2: [u8; 0x08],
    pmprotclr0: Pmprotclr0,
    pmprotclr1: Pmprotclr1,
    _reserved4: [u8; 0x08],
    pprotset_0: Pprotset0,
    pprotset_1: Pprotset1,
    pprotset_2: Pprotset2,
    pprotset_3: Pprotset3,
    _reserved8: [u8; 0x10],
    pprotclr0: Pprotclr0,
    pprotclr1: Pprotclr1,
    pprotclr2: Pprotclr2,
    pprotclr3: Pprotclr3,
    _reserved12: [u8; 0x10],
    pcspwrdwnset0: Pcspwrdwnset0,
    pcspwrdwnset1: Pcspwrdwnset1,
    _reserved14: [u8; 0x08],
    pcspwrdwnclr0: Pcspwrdwnclr0,
    pcspwrdwnclr1: Pcspwrdwnclr1,
    _reserved16: [u8; 0x08],
    pspwrdwnset0: Pspwrdwnset0,
    pspwrdwnset1: Pspwrdwnset1,
    pspwrdwnset2: Pspwrdwnset2,
    pspwrdwnset3: Pspwrdwnset3,
    _reserved20: [u8; 0x10],
    pspwrdwnclr0: Pspwrdwnclr0,
    pspwrdwnclr1: Pspwrdwnclr1,
    pspwrdwnclr2: Pspwrdwnclr2,
    pspwrdwnclr3: Pspwrdwnclr3,
    _reserved24: [u8; 0x10],
    pdpwrdwnset: Pdpwrdwnset,
    pdpwrdwnclr: Pdpwrdwnclr,
    _reserved26: [u8; 0x0138],
    mstidwrena: Mstidwrena,
    mstidena: Mstidena,
    mstiddiagctrl: Mstiddiagctrl,
    _reserved29: [u8; 0xf4],
    ps0mstid_l: Ps0mstidL,
    ps0mstid_h: Ps0mstidH,
    ps1mstid_l: Ps1mstidL,
    ps1mstid_h: Ps1mstidH,
    ps2mstid_l: Ps2mstidL,
    ps2mstid_h: Ps2mstidH,
    ps3mstid_l: Ps3mstidL,
    ps3mstid_h: Ps3mstidH,
    ps4mstid_l: Ps4mstidL,
    ps4mstid_h: Ps4mstidH,
    ps5mstid_l: Ps5mstidL,
    ps5mstid_h: Ps5mstidH,
    ps6mstid_l: Ps6mstidL,
    ps6mstid_h: Ps6mstidH,
    ps7mstid_l: Ps7mstidL,
    ps7mstid_h: Ps7mstidH,
    ps8mstid_l: Ps8mstidL,
    ps8mstid_h: Ps8mstidH,
    ps9mstid_l: Ps9mstidL,
    ps9mstid_h: Ps9mstidH,
    ps10mstid_l: Ps10mstidL,
    ps10mstid_h: Ps10mstidH,
    ps11mstid_l: Ps11mstidL,
    ps11mstid_h: Ps11mstidH,
    ps12mstid_l: Ps12mstidL,
    ps12mstid_h: Ps12mstidH,
    ps13mstid_l: Ps13mstidL,
    ps13mstid_h: Ps13mstidH,
    ps14mstid_l: Ps14mstidL,
    ps14mstid_h: Ps14mstidH,
    ps15mstid_l: Ps15mstidL,
    ps15mstid_h: Ps15mstidH,
    ps16mstid_l: Ps16mstidL,
    ps16mstid_h: Ps16mstidH,
    ps17mstid_l: Ps17mstidL,
    ps17mstid_h: Ps17mstidH,
    ps18mstid_l: Ps18mstidL,
    ps18mstid_h: Ps18mstidH,
    ps19mstid_l: Ps19mstidL,
    ps19mstid_h: Ps19mstidH,
    ps20mstid_l: Ps20mstidL,
    ps20mstid_h: Ps20mstidH,
    ps21mstid_l: Ps21mstidL,
    ps21mstid_h: Ps21mstidH,
    ps22mstid_l: Ps22mstidL,
    ps22mstid_h: Ps22mstidH,
    ps23mstid_l: Ps23mstidL,
    ps23mstid_h: Ps23mstidH,
    ps24mstid_l: Ps24mstidL,
    ps24mstid_h: Ps24mstidH,
    ps25mstid_l: Ps25mstidL,
    ps25mstid_h: Ps25mstidH,
    ps26mstid_l: Ps26mstidL,
    ps26mstid_h: Ps26mstidH,
    ps27mstid_l: Ps27mstidL,
    ps27mstid_h: Ps27mstidH,
    ps28mstid_l: Ps28mstidL,
    ps28mstid_h: Ps28mstidH,
    ps29mstid_l: Ps29mstidL,
    ps29mstid_h: Ps29mstidH,
    ps30mstid_l: Ps30mstidL,
    ps30mstid_h: Ps30mstidH,
    ps31mstid_l: Ps31mstidL,
    ps31mstid_h: Ps31mstidH,
    pps0mstid_l: Pps0mstidL,
    pps0mstid_h: Pps0mstidH,
    pps1mstid_l: Pps1mstidL,
    pps1mstid_h: Pps1mstidH,
    pps2mstid_l: Pps2mstidL,
    pps2mstid_h: Pps2mstidH,
    pps3mstid_l: Pps3mstidL,
    pps3mstid_h: Pps3mstidH,
    pps4mstid_l: Pps4mstidL,
    pps4mstid_h: Pps4mstidH,
    pps5mstid_l: Pps5mstidL,
    pps5mstid_h: Pps5mstidH,
    pps6mstid_l: Pps6mstidL,
    pps6mstid_h: Pps6mstidH,
    pps7mstid_l: Pps7mstidL,
    pps7mstid_h: Pps7mstidH,
    ppse0mstid_l: Ppse0mstidL,
    ppse0mstid_h: Ppse0mstidH,
    ppse1mstid_l: Ppse1mstidL,
    ppse1mstid_h: Ppse1mstidH,
    ppse2mstid_l: Ppse2mstidL,
    ppse2mstid_h: Ppse2mstidH,
    ppse3mstid_l: Ppse3mstidL,
    ppse3mstid_h: Ppse3mstidH,
    ppse4mstid_l: Ppse4mstidL,
    ppse4mstid_h: Ppse4mstidH,
    ppse5mstid_l: Ppse5mstidL,
    ppse5mstid_h: Ppse5mstidH,
    ppse6mstid_l: Ppse6mstidL,
    ppse6mstid_h: Ppse6mstidH,
    ppse7mstid_l: Ppse7mstidL,
    ppse7mstid_h: Ppse7mstidH,
    ppse8mstid_l: Ppse8mstidL,
    ppse8mstid_h: Ppse8mstidH,
    ppse9mstid_l: Ppse9mstidL,
    ppse9mstid_h: Ppse9mstidH,
    ppse10mstid_l: Ppse10mstidL,
    ppse10mstid_h: Ppse10mstidH,
    ppse11mstid_l: Ppse11mstidL,
    ppse11mstid_h: Ppse11mstidH,
    ppse12mstid_l: Ppse12mstidL,
    ppse12mstid_h: Ppse12mstidH,
    ppse13mstid_l: Ppse13mstidL,
    ppse13mstid_h: Ppse13mstidH,
    ppse14mstid_l: Ppse14mstidL,
    ppse14mstid_h: Ppse14mstidH,
    ppse15mstid_l: Ppse15mstidL,
    ppse15mstid_h: Ppse15mstidH,
    ppse16mstid_l: Ppse16mstidL,
    ppse16mstid_h: Ppse16mstidH,
    ppse17mstid_l: Ppse17mstidL,
    ppse17mstid_h: Ppse17mstidH,
    ppse18mstid_l: Ppse18mstidL,
    ppse18mstid_h: Ppse18mstidH,
    ppse19mstid_l: Ppse19mstidL,
    ppse19mstid_h: Ppse19mstidH,
    ppse20mstid_l: Ppse20mstidL,
    ppse20mstid_h: Ppse20mstidH,
    ppse21mstid_l: Ppse21mstidL,
    ppse21mstid_h: Ppse21mstidH,
    ppse22mstid_l: Ppse22mstidL,
    ppse22mstid_h: Ppse22mstidH,
    ppse23mstid_l: Ppse23mstidL,
    ppse23mstid_h: Ppse23mstidH,
    ppse24mstid_l: Ppse24mstidL,
    ppse24mstid_h: Ppse24mstidH,
    ppse25mstid_l: Ppse25mstidL,
    ppse25mstid_h: Ppse25mstidH,
    ppse26mstid_l: Ppse26mstidL,
    ppse26mstid_h: Ppse26mstidH,
    ppse27mstid_l: Ppse27mstidL,
    ppse27mstid_h: Ppse27mstidH,
    ppse28mstid_l: Ppse28mstidL,
    ppse28mstid_h: Ppse28mstidH,
    ppse29mstid_l: Ppse29mstidL,
    ppse29mstid_h: Ppse29mstidH,
    ppse30mstid_l: Ppse30mstidL,
    ppse30mstid_h: Ppse30mstidH,
    ppse31mstid_l: Ppse31mstidL,
    ppse31mstid_h: Ppse31mstidH,
    pcs0mstid: Pcs0mstid,
    pcs1mstid: Pcs1mstid,
    pcs2mstid: Pcs2mstid,
    pcs3mstid: Pcs3mstid,
    pcs4mstid: Pcs4mstid,
    pcs5mstid: Pcs5mstid,
    pcs6mstid: Pcs6mstid,
    pcs7mstid: Pcs7mstid,
    pcs8mstid: Pcs8mstid,
    pcs9mstid: Pcs9mstid,
    pcs10mstid: Pcs10mstid,
    pcs11mstid: Pcs11mstid,
    pcs12mstid: Pcs12mstid,
    pcs13mstid: Pcs13mstid,
    pcs14mstid: Pcs14mstid,
    pcs15mstid: Pcs15mstid,
    pcs16mstid: Pcs16mstid,
    pcs17mstid: Pcs17mstid,
    pcs18mstid: Pcs18mstid,
    pcs19mstid: Pcs19mstid,
    pcs20mstid: Pcs20mstid,
    pcs21mstid: Pcs21mstid,
    pcs22mstid: Pcs22mstid,
    pcs23mstid: Pcs23mstid,
    pcs24mstid: Pcs24mstid,
    pcs25mstid: Pcs25mstid,
    pcs26mstid: Pcs26mstid,
    pcs27mstid: Pcs27mstid,
    pcs28mstid: Pcs28mstid,
    pcs29mstid: Pcs29mstid,
    pcs30mstid: Pcs30mstid,
    pcs31mstid: Pcs31mstid,
    ppcs0mstid: Ppcs0mstid,
    ppcs1mstid: Ppcs1mstid,
    ppcs2mstid: Ppcs2mstid,
    ppcs3mstid: Ppcs3mstid,
    ppcs4mstid: Ppcs4mstid,
    ppcs5mstid: Ppcs5mstid,
    ppcs6mstid: Ppcs6mstid,
    ppcs7mstid: Ppcs7mstid,
    pcrextmstid: Pcrextmstid,
}
impl RegisterBlock {
    #[doc = "0x00 - Set-only register to protect PCS frames 0 to 31"]
    #[inline(always)]
    pub const fn pmprotset0(&self) -> &Pmprotset0 {
        &self.pmprotset0
    }
    #[doc = "0x04 - Set-only register to protect PCS frames 32 to 63"]
    #[inline(always)]
    pub const fn pmprotset1(&self) -> &Pmprotset1 {
        &self.pmprotset1
    }
    #[doc = "0x10 - Clear-only register to protect PCS frames 0 to 31"]
    #[inline(always)]
    pub const fn pmprotclr0(&self) -> &Pmprotclr0 {
        &self.pmprotclr0
    }
    #[doc = "0x14 - Clear-only register to protect PCS frames 32 to 63"]
    #[inline(always)]
    pub const fn pmprotclr1(&self) -> &Pmprotclr1 {
        &self.pmprotclr1
    }
    #[doc = "0x20 - Set-only register to protect the 32 quadrants of PS0 to PS7"]
    #[inline(always)]
    pub const fn pprotset_0(&self) -> &Pprotset0 {
        &self.pprotset_0
    }
    #[doc = "0x24 - Set-only register to protect the 32 quadrants of PS8 to PS15"]
    #[inline(always)]
    pub const fn pprotset_1(&self) -> &Pprotset1 {
        &self.pprotset_1
    }
    #[doc = "0x28 - Set-only register to protect the 32 quadrants of PS16 to PS23"]
    #[inline(always)]
    pub const fn pprotset_2(&self) -> &Pprotset2 {
        &self.pprotset_2
    }
    #[doc = "0x2c - Set-only register to protect the 32 quadrants of PS24 to PS31"]
    #[inline(always)]
    pub const fn pprotset_3(&self) -> &Pprotset3 {
        &self.pprotset_3
    }
    #[doc = "0x40 - Clear-only register to protect the 32 quadrants of PS0 to PS7"]
    #[inline(always)]
    pub const fn pprotclr0(&self) -> &Pprotclr0 {
        &self.pprotclr0
    }
    #[doc = "0x44 - Clear-only register to protect the 32 quadrants of PS8 to PS15"]
    #[inline(always)]
    pub const fn pprotclr1(&self) -> &Pprotclr1 {
        &self.pprotclr1
    }
    #[doc = "0x48 - Clear-only register to protect the 32 quadrants of PS16 to PS23"]
    #[inline(always)]
    pub const fn pprotclr2(&self) -> &Pprotclr2 {
        &self.pprotclr2
    }
    #[doc = "0x4c - Clear-only register to protect the 32 quadrants of PS24 to PS31"]
    #[inline(always)]
    pub const fn pprotclr3(&self) -> &Pprotclr3 {
        &self.pprotclr3
    }
    #[doc = "0x60 - Set-only register to powerdown independent (non-shared) PCS frames 0 to 31"]
    #[inline(always)]
    pub const fn pcspwrdwnset0(&self) -> &Pcspwrdwnset0 {
        &self.pcspwrdwnset0
    }
    #[doc = "0x64 - Set-only register to powerdown independent (non-shared) PCS frames 32 to 63"]
    #[inline(always)]
    pub const fn pcspwrdwnset1(&self) -> &Pcspwrdwnset1 {
        &self.pcspwrdwnset1
    }
    #[doc = "0x70 - Clear-only register to deassert powerdown bits of independent (non-shared) PCS frames 0 to 31"]
    #[inline(always)]
    pub const fn pcspwrdwnclr0(&self) -> &Pcspwrdwnclr0 {
        &self.pcspwrdwnclr0
    }
    #[doc = "0x74 - Clear-only register to deassert powerdown bits of independent (non-shared) PCS frames 32 to 63"]
    #[inline(always)]
    pub const fn pcspwrdwnclr1(&self) -> &Pcspwrdwnclr1 {
        &self.pcspwrdwnclr1
    }
    #[doc = "0x80 - Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS0 to PS7"]
    #[inline(always)]
    pub const fn pspwrdwnset0(&self) -> &Pspwrdwnset0 {
        &self.pspwrdwnset0
    }
    #[doc = "0x84 - Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS8 to PS15"]
    #[inline(always)]
    pub const fn pspwrdwnset1(&self) -> &Pspwrdwnset1 {
        &self.pspwrdwnset1
    }
    #[doc = "0x88 - Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS16 to PS23"]
    #[inline(always)]
    pub const fn pspwrdwnset2(&self) -> &Pspwrdwnset2 {
        &self.pspwrdwnset2
    }
    #[doc = "0x8c - Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS24 to PS31"]
    #[inline(always)]
    pub const fn pspwrdwnset3(&self) -> &Pspwrdwnset3 {
        &self.pspwrdwnset3
    }
    #[doc = "0xa0 - Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS0 to PS7"]
    #[inline(always)]
    pub const fn pspwrdwnclr0(&self) -> &Pspwrdwnclr0 {
        &self.pspwrdwnclr0
    }
    #[doc = "0xa4 - Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS8 to PS15"]
    #[inline(always)]
    pub const fn pspwrdwnclr1(&self) -> &Pspwrdwnclr1 {
        &self.pspwrdwnclr1
    }
    #[doc = "0xa8 - Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS16 to PS23"]
    #[inline(always)]
    pub const fn pspwrdwnclr2(&self) -> &Pspwrdwnclr2 {
        &self.pspwrdwnclr2
    }
    #[doc = "0xac - Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS24 to PS31"]
    #[inline(always)]
    pub const fn pspwrdwnclr3(&self) -> &Pspwrdwnclr3 {
        &self.pspwrdwnclr3
    }
    #[doc = "0xc0 - Set-only register to powerdown the debug frame"]
    #[inline(always)]
    pub const fn pdpwrdwnset(&self) -> &Pdpwrdwnset {
        &self.pdpwrdwnset
    }
    #[doc = "0xc4 - Clear-only register to deassert the debug frameΓÇÖs powerdown bit"]
    #[inline(always)]
    pub const fn pdpwrdwnclr(&self) -> &Pdpwrdwnclr {
        &self.pdpwrdwnclr
    }
    #[doc = "0x200 - MasterID Protection Write Enable Register"]
    #[inline(always)]
    pub const fn mstidwrena(&self) -> &Mstidwrena {
        &self.mstidwrena
    }
    #[doc = "0x204 - MasterID Protection Enable Register"]
    #[inline(always)]
    pub const fn mstidena(&self) -> &Mstidena {
        &self.mstidena
    }
    #[doc = "0x208 - MasterID Diagnostic Control Register"]
    #[inline(always)]
    pub const fn mstiddiagctrl(&self) -> &Mstiddiagctrl {
        &self.mstiddiagctrl
    }
    #[doc = "0x300 - Peripheral Frame Master-ID Protection Register0_L"]
    #[inline(always)]
    pub const fn ps0mstid_l(&self) -> &Ps0mstidL {
        &self.ps0mstid_l
    }
    #[doc = "0x304 - Peripheral Frame Master-ID Protection Register0_H"]
    #[inline(always)]
    pub const fn ps0mstid_h(&self) -> &Ps0mstidH {
        &self.ps0mstid_h
    }
    #[doc = "0x308 - Peripheral Frame Master-ID Protection Register1_L"]
    #[inline(always)]
    pub const fn ps1mstid_l(&self) -> &Ps1mstidL {
        &self.ps1mstid_l
    }
    #[doc = "0x30c - Peripheral Frame Master-ID Protection Register1_H"]
    #[inline(always)]
    pub const fn ps1mstid_h(&self) -> &Ps1mstidH {
        &self.ps1mstid_h
    }
    #[doc = "0x310 - Peripheral Frame Master-ID Protection Register2_L"]
    #[inline(always)]
    pub const fn ps2mstid_l(&self) -> &Ps2mstidL {
        &self.ps2mstid_l
    }
    #[doc = "0x314 - Peripheral Frame Master-ID Protection Register2_H"]
    #[inline(always)]
    pub const fn ps2mstid_h(&self) -> &Ps2mstidH {
        &self.ps2mstid_h
    }
    #[doc = "0x318 - Peripheral Frame Master-ID Protection Register3_L"]
    #[inline(always)]
    pub const fn ps3mstid_l(&self) -> &Ps3mstidL {
        &self.ps3mstid_l
    }
    #[doc = "0x31c - Peripheral Frame Master-ID Protection Register3_H"]
    #[inline(always)]
    pub const fn ps3mstid_h(&self) -> &Ps3mstidH {
        &self.ps3mstid_h
    }
    #[doc = "0x320 - Peripheral Frame Master-ID Protection Register4_L"]
    #[inline(always)]
    pub const fn ps4mstid_l(&self) -> &Ps4mstidL {
        &self.ps4mstid_l
    }
    #[doc = "0x324 - Peripheral Frame Master-ID Protection Register4_H"]
    #[inline(always)]
    pub const fn ps4mstid_h(&self) -> &Ps4mstidH {
        &self.ps4mstid_h
    }
    #[doc = "0x328 - Peripheral Frame Master-ID Protection Register5_L"]
    #[inline(always)]
    pub const fn ps5mstid_l(&self) -> &Ps5mstidL {
        &self.ps5mstid_l
    }
    #[doc = "0x32c - Peripheral Frame Master-ID Protection Register5_H"]
    #[inline(always)]
    pub const fn ps5mstid_h(&self) -> &Ps5mstidH {
        &self.ps5mstid_h
    }
    #[doc = "0x330 - Peripheral Frame Master-ID Protection Register6_L"]
    #[inline(always)]
    pub const fn ps6mstid_l(&self) -> &Ps6mstidL {
        &self.ps6mstid_l
    }
    #[doc = "0x334 - Peripheral Frame Master-ID Protection Register6_H"]
    #[inline(always)]
    pub const fn ps6mstid_h(&self) -> &Ps6mstidH {
        &self.ps6mstid_h
    }
    #[doc = "0x338 - Peripheral Frame Master-ID Protection Register7_L"]
    #[inline(always)]
    pub const fn ps7mstid_l(&self) -> &Ps7mstidL {
        &self.ps7mstid_l
    }
    #[doc = "0x33c - Peripheral Frame Master-ID Protection Register7_H"]
    #[inline(always)]
    pub const fn ps7mstid_h(&self) -> &Ps7mstidH {
        &self.ps7mstid_h
    }
    #[doc = "0x340 - Peripheral Frame Master-ID Protection Register8_L"]
    #[inline(always)]
    pub const fn ps8mstid_l(&self) -> &Ps8mstidL {
        &self.ps8mstid_l
    }
    #[doc = "0x344 - Peripheral Frame Master-ID Protection Register8_H"]
    #[inline(always)]
    pub const fn ps8mstid_h(&self) -> &Ps8mstidH {
        &self.ps8mstid_h
    }
    #[doc = "0x348 - Peripheral Frame Master-ID Protection Register9_L"]
    #[inline(always)]
    pub const fn ps9mstid_l(&self) -> &Ps9mstidL {
        &self.ps9mstid_l
    }
    #[doc = "0x34c - Peripheral Frame Master-ID Protection Register9_H"]
    #[inline(always)]
    pub const fn ps9mstid_h(&self) -> &Ps9mstidH {
        &self.ps9mstid_h
    }
    #[doc = "0x350 - Peripheral Frame Master-ID Protection Register10_L"]
    #[inline(always)]
    pub const fn ps10mstid_l(&self) -> &Ps10mstidL {
        &self.ps10mstid_l
    }
    #[doc = "0x354 - Peripheral Frame Master-ID Protection Register10_H"]
    #[inline(always)]
    pub const fn ps10mstid_h(&self) -> &Ps10mstidH {
        &self.ps10mstid_h
    }
    #[doc = "0x358 - Peripheral Frame Master-ID Protection Register11_L"]
    #[inline(always)]
    pub const fn ps11mstid_l(&self) -> &Ps11mstidL {
        &self.ps11mstid_l
    }
    #[doc = "0x35c - Peripheral Frame Master-ID Protection Register11_H"]
    #[inline(always)]
    pub const fn ps11mstid_h(&self) -> &Ps11mstidH {
        &self.ps11mstid_h
    }
    #[doc = "0x360 - Peripheral Frame Master-ID Protection Register12_L"]
    #[inline(always)]
    pub const fn ps12mstid_l(&self) -> &Ps12mstidL {
        &self.ps12mstid_l
    }
    #[doc = "0x364 - Peripheral Frame Master-ID Protection Register12_H"]
    #[inline(always)]
    pub const fn ps12mstid_h(&self) -> &Ps12mstidH {
        &self.ps12mstid_h
    }
    #[doc = "0x368 - Peripheral Frame Master-ID Protection Register13_L"]
    #[inline(always)]
    pub const fn ps13mstid_l(&self) -> &Ps13mstidL {
        &self.ps13mstid_l
    }
    #[doc = "0x36c - Peripheral Frame Master-ID Protection Register13_H"]
    #[inline(always)]
    pub const fn ps13mstid_h(&self) -> &Ps13mstidH {
        &self.ps13mstid_h
    }
    #[doc = "0x370 - Peripheral Frame Master-ID Protection Register14_L"]
    #[inline(always)]
    pub const fn ps14mstid_l(&self) -> &Ps14mstidL {
        &self.ps14mstid_l
    }
    #[doc = "0x374 - Peripheral Frame Master-ID Protection Register14_H"]
    #[inline(always)]
    pub const fn ps14mstid_h(&self) -> &Ps14mstidH {
        &self.ps14mstid_h
    }
    #[doc = "0x378 - Peripheral Frame Master-ID Protection Register15_L"]
    #[inline(always)]
    pub const fn ps15mstid_l(&self) -> &Ps15mstidL {
        &self.ps15mstid_l
    }
    #[doc = "0x37c - Peripheral Frame Master-ID Protection Register15_H"]
    #[inline(always)]
    pub const fn ps15mstid_h(&self) -> &Ps15mstidH {
        &self.ps15mstid_h
    }
    #[doc = "0x380 - Peripheral Frame Master-ID Protection Register16_L"]
    #[inline(always)]
    pub const fn ps16mstid_l(&self) -> &Ps16mstidL {
        &self.ps16mstid_l
    }
    #[doc = "0x384 - Peripheral Frame Master-ID Protection Register16_H"]
    #[inline(always)]
    pub const fn ps16mstid_h(&self) -> &Ps16mstidH {
        &self.ps16mstid_h
    }
    #[doc = "0x388 - Peripheral Frame Master-ID Protection Register17_L"]
    #[inline(always)]
    pub const fn ps17mstid_l(&self) -> &Ps17mstidL {
        &self.ps17mstid_l
    }
    #[doc = "0x38c - Peripheral Frame Master-ID Protection Register17_H"]
    #[inline(always)]
    pub const fn ps17mstid_h(&self) -> &Ps17mstidH {
        &self.ps17mstid_h
    }
    #[doc = "0x390 - Peripheral Frame Master-ID Protection Register18_L"]
    #[inline(always)]
    pub const fn ps18mstid_l(&self) -> &Ps18mstidL {
        &self.ps18mstid_l
    }
    #[doc = "0x394 - Peripheral Frame Master-ID Protection Register18_H"]
    #[inline(always)]
    pub const fn ps18mstid_h(&self) -> &Ps18mstidH {
        &self.ps18mstid_h
    }
    #[doc = "0x398 - Peripheral Frame Master-ID Protection Register19_L"]
    #[inline(always)]
    pub const fn ps19mstid_l(&self) -> &Ps19mstidL {
        &self.ps19mstid_l
    }
    #[doc = "0x39c - Peripheral Frame Master-ID Protection Register19_H"]
    #[inline(always)]
    pub const fn ps19mstid_h(&self) -> &Ps19mstidH {
        &self.ps19mstid_h
    }
    #[doc = "0x3a0 - Peripheral Frame Master-ID Protection Register20_L"]
    #[inline(always)]
    pub const fn ps20mstid_l(&self) -> &Ps20mstidL {
        &self.ps20mstid_l
    }
    #[doc = "0x3a4 - Peripheral Frame Master-ID Protection Register20_H"]
    #[inline(always)]
    pub const fn ps20mstid_h(&self) -> &Ps20mstidH {
        &self.ps20mstid_h
    }
    #[doc = "0x3a8 - Peripheral Frame Master-ID Protection Register21_L"]
    #[inline(always)]
    pub const fn ps21mstid_l(&self) -> &Ps21mstidL {
        &self.ps21mstid_l
    }
    #[doc = "0x3ac - Peripheral Frame Master-ID Protection Register21_H"]
    #[inline(always)]
    pub const fn ps21mstid_h(&self) -> &Ps21mstidH {
        &self.ps21mstid_h
    }
    #[doc = "0x3b0 - Peripheral Frame Master-ID Protection Register22_L"]
    #[inline(always)]
    pub const fn ps22mstid_l(&self) -> &Ps22mstidL {
        &self.ps22mstid_l
    }
    #[doc = "0x3b4 - Peripheral Frame Master-ID Protection Register22_H"]
    #[inline(always)]
    pub const fn ps22mstid_h(&self) -> &Ps22mstidH {
        &self.ps22mstid_h
    }
    #[doc = "0x3b8 - Peripheral Frame Master-ID Protection Register23_L"]
    #[inline(always)]
    pub const fn ps23mstid_l(&self) -> &Ps23mstidL {
        &self.ps23mstid_l
    }
    #[doc = "0x3bc - Peripheral Frame Master-ID Protection Register23_H"]
    #[inline(always)]
    pub const fn ps23mstid_h(&self) -> &Ps23mstidH {
        &self.ps23mstid_h
    }
    #[doc = "0x3c0 - Peripheral Frame Master-ID Protection Register24_L"]
    #[inline(always)]
    pub const fn ps24mstid_l(&self) -> &Ps24mstidL {
        &self.ps24mstid_l
    }
    #[doc = "0x3c4 - Peripheral Frame Master-ID Protection Register24_H"]
    #[inline(always)]
    pub const fn ps24mstid_h(&self) -> &Ps24mstidH {
        &self.ps24mstid_h
    }
    #[doc = "0x3c8 - Peripheral Frame Master-ID Protection Register25_L"]
    #[inline(always)]
    pub const fn ps25mstid_l(&self) -> &Ps25mstidL {
        &self.ps25mstid_l
    }
    #[doc = "0x3cc - Peripheral Frame Master-ID Protection Register25_H"]
    #[inline(always)]
    pub const fn ps25mstid_h(&self) -> &Ps25mstidH {
        &self.ps25mstid_h
    }
    #[doc = "0x3d0 - Peripheral Frame Master-ID Protection Register26_L"]
    #[inline(always)]
    pub const fn ps26mstid_l(&self) -> &Ps26mstidL {
        &self.ps26mstid_l
    }
    #[doc = "0x3d4 - Peripheral Frame Master-ID Protection Register26_H"]
    #[inline(always)]
    pub const fn ps26mstid_h(&self) -> &Ps26mstidH {
        &self.ps26mstid_h
    }
    #[doc = "0x3d8 - Peripheral Frame Master-ID Protection Register27_L"]
    #[inline(always)]
    pub const fn ps27mstid_l(&self) -> &Ps27mstidL {
        &self.ps27mstid_l
    }
    #[doc = "0x3dc - Peripheral Frame Master-ID Protection Register27_H"]
    #[inline(always)]
    pub const fn ps27mstid_h(&self) -> &Ps27mstidH {
        &self.ps27mstid_h
    }
    #[doc = "0x3e0 - Peripheral Frame Master-ID Protection Register28_L"]
    #[inline(always)]
    pub const fn ps28mstid_l(&self) -> &Ps28mstidL {
        &self.ps28mstid_l
    }
    #[doc = "0x3e4 - Peripheral Frame Master-ID Protection Register28_H"]
    #[inline(always)]
    pub const fn ps28mstid_h(&self) -> &Ps28mstidH {
        &self.ps28mstid_h
    }
    #[doc = "0x3e8 - Peripheral Frame Master-ID Protection Register29_L"]
    #[inline(always)]
    pub const fn ps29mstid_l(&self) -> &Ps29mstidL {
        &self.ps29mstid_l
    }
    #[doc = "0x3ec - Peripheral Frame Master-ID Protection Register29_H"]
    #[inline(always)]
    pub const fn ps29mstid_h(&self) -> &Ps29mstidH {
        &self.ps29mstid_h
    }
    #[doc = "0x3f0 - Peripheral Frame Master-ID Protection Register30_L"]
    #[inline(always)]
    pub const fn ps30mstid_l(&self) -> &Ps30mstidL {
        &self.ps30mstid_l
    }
    #[doc = "0x3f4 - Peripheral Frame Master-ID Protection Register30_H"]
    #[inline(always)]
    pub const fn ps30mstid_h(&self) -> &Ps30mstidH {
        &self.ps30mstid_h
    }
    #[doc = "0x3f8 - Peripheral Frame Master-ID Protection Register31_L"]
    #[inline(always)]
    pub const fn ps31mstid_l(&self) -> &Ps31mstidL {
        &self.ps31mstid_l
    }
    #[doc = "0x3fc - Peripheral Frame Master-ID Protection Register31_H"]
    #[inline(always)]
    pub const fn ps31mstid_h(&self) -> &Ps31mstidH {
        &self.ps31mstid_h
    }
    #[doc = "0x400 - Privileged Peripheral Frame Master-ID Protection Register0_L"]
    #[inline(always)]
    pub const fn pps0mstid_l(&self) -> &Pps0mstidL {
        &self.pps0mstid_l
    }
    #[doc = "0x404 - Privileged Peripheral Frame Master-ID Protection Register0_H"]
    #[inline(always)]
    pub const fn pps0mstid_h(&self) -> &Pps0mstidH {
        &self.pps0mstid_h
    }
    #[doc = "0x408 - Privileged Peripheral Frame Master-ID Protection Register1_L"]
    #[inline(always)]
    pub const fn pps1mstid_l(&self) -> &Pps1mstidL {
        &self.pps1mstid_l
    }
    #[doc = "0x40c - Privileged Peripheral Frame Master-ID Protection Register1_H"]
    #[inline(always)]
    pub const fn pps1mstid_h(&self) -> &Pps1mstidH {
        &self.pps1mstid_h
    }
    #[doc = "0x410 - Privileged Peripheral Frame Master-ID Protection Register2_L"]
    #[inline(always)]
    pub const fn pps2mstid_l(&self) -> &Pps2mstidL {
        &self.pps2mstid_l
    }
    #[doc = "0x414 - Privileged Peripheral Frame Master-ID Protection Register2_H"]
    #[inline(always)]
    pub const fn pps2mstid_h(&self) -> &Pps2mstidH {
        &self.pps2mstid_h
    }
    #[doc = "0x418 - Privileged Peripheral Frame Master-ID Protection Register3_L"]
    #[inline(always)]
    pub const fn pps3mstid_l(&self) -> &Pps3mstidL {
        &self.pps3mstid_l
    }
    #[doc = "0x41c - Privileged Peripheral Frame Master-ID Protection Register3_H"]
    #[inline(always)]
    pub const fn pps3mstid_h(&self) -> &Pps3mstidH {
        &self.pps3mstid_h
    }
    #[doc = "0x420 - Privileged Peripheral Frame Master-ID Protection Register4_L"]
    #[inline(always)]
    pub const fn pps4mstid_l(&self) -> &Pps4mstidL {
        &self.pps4mstid_l
    }
    #[doc = "0x424 - Privileged Peripheral Frame Master-ID Protection Register4_H"]
    #[inline(always)]
    pub const fn pps4mstid_h(&self) -> &Pps4mstidH {
        &self.pps4mstid_h
    }
    #[doc = "0x428 - Privileged Peripheral Frame Master-ID Protection Register5_L"]
    #[inline(always)]
    pub const fn pps5mstid_l(&self) -> &Pps5mstidL {
        &self.pps5mstid_l
    }
    #[doc = "0x42c - Privileged Peripheral Frame Master-ID Protection Register5_H"]
    #[inline(always)]
    pub const fn pps5mstid_h(&self) -> &Pps5mstidH {
        &self.pps5mstid_h
    }
    #[doc = "0x430 - Privileged Peripheral Frame Master-ID Protection Register6_L"]
    #[inline(always)]
    pub const fn pps6mstid_l(&self) -> &Pps6mstidL {
        &self.pps6mstid_l
    }
    #[doc = "0x434 - Privileged Peripheral Frame Master-ID Protection Register6_H"]
    #[inline(always)]
    pub const fn pps6mstid_h(&self) -> &Pps6mstidH {
        &self.pps6mstid_h
    }
    #[doc = "0x438 - Privileged Peripheral Frame Master-ID Protection Register7_L"]
    #[inline(always)]
    pub const fn pps7mstid_l(&self) -> &Pps7mstidL {
        &self.pps7mstid_l
    }
    #[doc = "0x43c - Privileged Peripheral Frame Master-ID Protection Register7_H"]
    #[inline(always)]
    pub const fn pps7mstid_h(&self) -> &Pps7mstidH {
        &self.pps7mstid_h
    }
    #[doc = "0x440 - Privileged Peripheral Extended Frame Master-ID Protection Register0_L"]
    #[inline(always)]
    pub const fn ppse0mstid_l(&self) -> &Ppse0mstidL {
        &self.ppse0mstid_l
    }
    #[doc = "0x444 - Privileged Peripheral Extended Frame Master-ID Protection Register0_H"]
    #[inline(always)]
    pub const fn ppse0mstid_h(&self) -> &Ppse0mstidH {
        &self.ppse0mstid_h
    }
    #[doc = "0x448 - Privileged Peripheral Extended Frame Master-ID Protection Register1_L"]
    #[inline(always)]
    pub const fn ppse1mstid_l(&self) -> &Ppse1mstidL {
        &self.ppse1mstid_l
    }
    #[doc = "0x44c - Privileged Peripheral Extended Frame Master-ID Protection Register1_H"]
    #[inline(always)]
    pub const fn ppse1mstid_h(&self) -> &Ppse1mstidH {
        &self.ppse1mstid_h
    }
    #[doc = "0x450 - Privileged Peripheral Extended Frame Master-ID Protection Register2_L"]
    #[inline(always)]
    pub const fn ppse2mstid_l(&self) -> &Ppse2mstidL {
        &self.ppse2mstid_l
    }
    #[doc = "0x454 - Privileged Peripheral Extended Frame Master-ID Protection Register2_H"]
    #[inline(always)]
    pub const fn ppse2mstid_h(&self) -> &Ppse2mstidH {
        &self.ppse2mstid_h
    }
    #[doc = "0x458 - Privileged Peripheral Extended Frame Master-ID Protection Register3_L"]
    #[inline(always)]
    pub const fn ppse3mstid_l(&self) -> &Ppse3mstidL {
        &self.ppse3mstid_l
    }
    #[doc = "0x45c - Privileged Peripheral Extended Frame Master-ID Protection Register3_H"]
    #[inline(always)]
    pub const fn ppse3mstid_h(&self) -> &Ppse3mstidH {
        &self.ppse3mstid_h
    }
    #[doc = "0x460 - Privileged Peripheral Extended Frame Master-ID Protection Register4_L"]
    #[inline(always)]
    pub const fn ppse4mstid_l(&self) -> &Ppse4mstidL {
        &self.ppse4mstid_l
    }
    #[doc = "0x464 - Privileged Peripheral Extended Frame Master-ID Protection Register4_H"]
    #[inline(always)]
    pub const fn ppse4mstid_h(&self) -> &Ppse4mstidH {
        &self.ppse4mstid_h
    }
    #[doc = "0x468 - Privileged Peripheral Extended Frame Master-ID Protection Register5_L"]
    #[inline(always)]
    pub const fn ppse5mstid_l(&self) -> &Ppse5mstidL {
        &self.ppse5mstid_l
    }
    #[doc = "0x46c - Privileged Peripheral Extended Frame Master-ID Protection Register5_H"]
    #[inline(always)]
    pub const fn ppse5mstid_h(&self) -> &Ppse5mstidH {
        &self.ppse5mstid_h
    }
    #[doc = "0x470 - Privileged Peripheral Extended Frame Master-ID Protection Register6_L"]
    #[inline(always)]
    pub const fn ppse6mstid_l(&self) -> &Ppse6mstidL {
        &self.ppse6mstid_l
    }
    #[doc = "0x474 - Privileged Peripheral Extended Frame Master-ID Protection Register6_H"]
    #[inline(always)]
    pub const fn ppse6mstid_h(&self) -> &Ppse6mstidH {
        &self.ppse6mstid_h
    }
    #[doc = "0x478 - Privileged Peripheral Extended Frame Master-ID Protection Register7_L"]
    #[inline(always)]
    pub const fn ppse7mstid_l(&self) -> &Ppse7mstidL {
        &self.ppse7mstid_l
    }
    #[doc = "0x47c - Privileged Peripheral Extended Frame Master-ID Protection Register7_H"]
    #[inline(always)]
    pub const fn ppse7mstid_h(&self) -> &Ppse7mstidH {
        &self.ppse7mstid_h
    }
    #[doc = "0x480 - Privileged Peripheral Extended Frame Master-ID Protection Register8_L"]
    #[inline(always)]
    pub const fn ppse8mstid_l(&self) -> &Ppse8mstidL {
        &self.ppse8mstid_l
    }
    #[doc = "0x484 - Privileged Peripheral Extended Frame Master-ID Protection Register8_H"]
    #[inline(always)]
    pub const fn ppse8mstid_h(&self) -> &Ppse8mstidH {
        &self.ppse8mstid_h
    }
    #[doc = "0x488 - Privileged Peripheral Extended Frame Master-ID Protection Register9_L"]
    #[inline(always)]
    pub const fn ppse9mstid_l(&self) -> &Ppse9mstidL {
        &self.ppse9mstid_l
    }
    #[doc = "0x48c - Privileged Peripheral Extended Frame Master-ID Protection Register9_H"]
    #[inline(always)]
    pub const fn ppse9mstid_h(&self) -> &Ppse9mstidH {
        &self.ppse9mstid_h
    }
    #[doc = "0x490 - Privileged Peripheral Extended Frame Master-ID Protection Register10_L"]
    #[inline(always)]
    pub const fn ppse10mstid_l(&self) -> &Ppse10mstidL {
        &self.ppse10mstid_l
    }
    #[doc = "0x494 - Privileged Peripheral Extended Frame Master-ID Protection Register10_H"]
    #[inline(always)]
    pub const fn ppse10mstid_h(&self) -> &Ppse10mstidH {
        &self.ppse10mstid_h
    }
    #[doc = "0x498 - Privileged Peripheral Extended Frame Master-ID Protection Register11_L"]
    #[inline(always)]
    pub const fn ppse11mstid_l(&self) -> &Ppse11mstidL {
        &self.ppse11mstid_l
    }
    #[doc = "0x49c - Privileged Peripheral Extended Frame Master-ID Protection Register11_H"]
    #[inline(always)]
    pub const fn ppse11mstid_h(&self) -> &Ppse11mstidH {
        &self.ppse11mstid_h
    }
    #[doc = "0x4a0 - Privileged Peripheral Extended Frame Master-ID Protection Register12_L"]
    #[inline(always)]
    pub const fn ppse12mstid_l(&self) -> &Ppse12mstidL {
        &self.ppse12mstid_l
    }
    #[doc = "0x4a4 - Privileged Peripheral Extended Frame Master-ID Protection Register12_H"]
    #[inline(always)]
    pub const fn ppse12mstid_h(&self) -> &Ppse12mstidH {
        &self.ppse12mstid_h
    }
    #[doc = "0x4a8 - Privileged Peripheral Extended Frame Master-ID Protection Register13_L"]
    #[inline(always)]
    pub const fn ppse13mstid_l(&self) -> &Ppse13mstidL {
        &self.ppse13mstid_l
    }
    #[doc = "0x4ac - Privileged Peripheral Extended Frame Master-ID Protection Register13_H"]
    #[inline(always)]
    pub const fn ppse13mstid_h(&self) -> &Ppse13mstidH {
        &self.ppse13mstid_h
    }
    #[doc = "0x4b0 - Privileged Peripheral Extended Frame Master-ID Protection Register14_L"]
    #[inline(always)]
    pub const fn ppse14mstid_l(&self) -> &Ppse14mstidL {
        &self.ppse14mstid_l
    }
    #[doc = "0x4b4 - Privileged Peripheral Extended Frame Master-ID Protection Register14_H"]
    #[inline(always)]
    pub const fn ppse14mstid_h(&self) -> &Ppse14mstidH {
        &self.ppse14mstid_h
    }
    #[doc = "0x4b8 - Privileged Peripheral Extended Frame Master-ID Protection Register15_L"]
    #[inline(always)]
    pub const fn ppse15mstid_l(&self) -> &Ppse15mstidL {
        &self.ppse15mstid_l
    }
    #[doc = "0x4bc - Privileged Peripheral Extended Frame Master-ID Protection Register15_H"]
    #[inline(always)]
    pub const fn ppse15mstid_h(&self) -> &Ppse15mstidH {
        &self.ppse15mstid_h
    }
    #[doc = "0x4c0 - Privileged Peripheral Extended Frame Master-ID Protection Register16_L"]
    #[inline(always)]
    pub const fn ppse16mstid_l(&self) -> &Ppse16mstidL {
        &self.ppse16mstid_l
    }
    #[doc = "0x4c4 - Privileged Peripheral Extended Frame Master-ID Protection Register16_H"]
    #[inline(always)]
    pub const fn ppse16mstid_h(&self) -> &Ppse16mstidH {
        &self.ppse16mstid_h
    }
    #[doc = "0x4c8 - Privileged Peripheral Extended Frame Master-ID Protection Register17_L"]
    #[inline(always)]
    pub const fn ppse17mstid_l(&self) -> &Ppse17mstidL {
        &self.ppse17mstid_l
    }
    #[doc = "0x4cc - Privileged Peripheral Extended Frame Master-ID Protection Register17_H"]
    #[inline(always)]
    pub const fn ppse17mstid_h(&self) -> &Ppse17mstidH {
        &self.ppse17mstid_h
    }
    #[doc = "0x4d0 - Privileged Peripheral Extended Frame Master-ID Protection Register18_L"]
    #[inline(always)]
    pub const fn ppse18mstid_l(&self) -> &Ppse18mstidL {
        &self.ppse18mstid_l
    }
    #[doc = "0x4d4 - Privileged Peripheral Extended Frame Master-ID Protection Register18_H"]
    #[inline(always)]
    pub const fn ppse18mstid_h(&self) -> &Ppse18mstidH {
        &self.ppse18mstid_h
    }
    #[doc = "0x4d8 - Privileged Peripheral Extended Frame Master-ID Protection Register19_L"]
    #[inline(always)]
    pub const fn ppse19mstid_l(&self) -> &Ppse19mstidL {
        &self.ppse19mstid_l
    }
    #[doc = "0x4dc - Privileged Peripheral Extended Frame Master-ID Protection Register19_H"]
    #[inline(always)]
    pub const fn ppse19mstid_h(&self) -> &Ppse19mstidH {
        &self.ppse19mstid_h
    }
    #[doc = "0x4e0 - Privileged Peripheral Extended Frame Master-ID Protection Register20_L"]
    #[inline(always)]
    pub const fn ppse20mstid_l(&self) -> &Ppse20mstidL {
        &self.ppse20mstid_l
    }
    #[doc = "0x4e4 - Privileged Peripheral Extended Frame Master-ID Protection Register20_H"]
    #[inline(always)]
    pub const fn ppse20mstid_h(&self) -> &Ppse20mstidH {
        &self.ppse20mstid_h
    }
    #[doc = "0x4e8 - Privileged Peripheral Extended Frame Master-ID Protection Register21_L"]
    #[inline(always)]
    pub const fn ppse21mstid_l(&self) -> &Ppse21mstidL {
        &self.ppse21mstid_l
    }
    #[doc = "0x4ec - Privileged Peripheral Extended Frame Master-ID Protection Register21_H"]
    #[inline(always)]
    pub const fn ppse21mstid_h(&self) -> &Ppse21mstidH {
        &self.ppse21mstid_h
    }
    #[doc = "0x4f0 - Privileged Peripheral Extended Frame Master-ID Protection Register22_L"]
    #[inline(always)]
    pub const fn ppse22mstid_l(&self) -> &Ppse22mstidL {
        &self.ppse22mstid_l
    }
    #[doc = "0x4f4 - Privileged Peripheral Extended Frame Master-ID Protection Register22_H"]
    #[inline(always)]
    pub const fn ppse22mstid_h(&self) -> &Ppse22mstidH {
        &self.ppse22mstid_h
    }
    #[doc = "0x4f8 - Privileged Peripheral Extended Frame Master-ID Protection Register23_L"]
    #[inline(always)]
    pub const fn ppse23mstid_l(&self) -> &Ppse23mstidL {
        &self.ppse23mstid_l
    }
    #[doc = "0x4fc - Privileged Peripheral Extended Frame Master-ID Protection Register23_H"]
    #[inline(always)]
    pub const fn ppse23mstid_h(&self) -> &Ppse23mstidH {
        &self.ppse23mstid_h
    }
    #[doc = "0x500 - Privileged Peripheral Extended Frame Master-ID Protection Register24_L"]
    #[inline(always)]
    pub const fn ppse24mstid_l(&self) -> &Ppse24mstidL {
        &self.ppse24mstid_l
    }
    #[doc = "0x504 - Privileged Peripheral Extended Frame Master-ID Protection Register24_H"]
    #[inline(always)]
    pub const fn ppse24mstid_h(&self) -> &Ppse24mstidH {
        &self.ppse24mstid_h
    }
    #[doc = "0x508 - Privileged Peripheral Extended Frame Master-ID Protection Register25_L"]
    #[inline(always)]
    pub const fn ppse25mstid_l(&self) -> &Ppse25mstidL {
        &self.ppse25mstid_l
    }
    #[doc = "0x50c - Privileged Peripheral Extended Frame Master-ID Protection Register25_H"]
    #[inline(always)]
    pub const fn ppse25mstid_h(&self) -> &Ppse25mstidH {
        &self.ppse25mstid_h
    }
    #[doc = "0x510 - Privileged Peripheral Extended Frame Master-ID Protection Register26_L"]
    #[inline(always)]
    pub const fn ppse26mstid_l(&self) -> &Ppse26mstidL {
        &self.ppse26mstid_l
    }
    #[doc = "0x514 - Privileged Peripheral Extended Frame Master-ID Protection Register26_H"]
    #[inline(always)]
    pub const fn ppse26mstid_h(&self) -> &Ppse26mstidH {
        &self.ppse26mstid_h
    }
    #[doc = "0x518 - Privileged Peripheral Extended Frame Master-ID Protection Register27_L"]
    #[inline(always)]
    pub const fn ppse27mstid_l(&self) -> &Ppse27mstidL {
        &self.ppse27mstid_l
    }
    #[doc = "0x51c - Privileged Peripheral Extended Frame Master-ID Protection Register27_H"]
    #[inline(always)]
    pub const fn ppse27mstid_h(&self) -> &Ppse27mstidH {
        &self.ppse27mstid_h
    }
    #[doc = "0x520 - Privileged Peripheral Extended Frame Master-ID Protection Register28_L"]
    #[inline(always)]
    pub const fn ppse28mstid_l(&self) -> &Ppse28mstidL {
        &self.ppse28mstid_l
    }
    #[doc = "0x524 - Privileged Peripheral Extended Frame Master-ID Protection Register28_H"]
    #[inline(always)]
    pub const fn ppse28mstid_h(&self) -> &Ppse28mstidH {
        &self.ppse28mstid_h
    }
    #[doc = "0x528 - Privileged Peripheral Extended Frame Master-ID Protection Register29_L"]
    #[inline(always)]
    pub const fn ppse29mstid_l(&self) -> &Ppse29mstidL {
        &self.ppse29mstid_l
    }
    #[doc = "0x52c - Privileged Peripheral Extended Frame Master-ID Protection Register29_H"]
    #[inline(always)]
    pub const fn ppse29mstid_h(&self) -> &Ppse29mstidH {
        &self.ppse29mstid_h
    }
    #[doc = "0x530 - Privileged Peripheral Extended Frame Master-ID Protection Register30_L"]
    #[inline(always)]
    pub const fn ppse30mstid_l(&self) -> &Ppse30mstidL {
        &self.ppse30mstid_l
    }
    #[doc = "0x534 - Privileged Peripheral Extended Frame Master-ID Protection Register30_H"]
    #[inline(always)]
    pub const fn ppse30mstid_h(&self) -> &Ppse30mstidH {
        &self.ppse30mstid_h
    }
    #[doc = "0x538 - Privileged Peripheral Extended Frame Master-ID Protection Register31_L"]
    #[inline(always)]
    pub const fn ppse31mstid_l(&self) -> &Ppse31mstidL {
        &self.ppse31mstid_l
    }
    #[doc = "0x53c - Privileged Peripheral Extended Frame Master-ID Protection Register31_H"]
    #[inline(always)]
    pub const fn ppse31mstid_h(&self) -> &Ppse31mstidH {
        &self.ppse31mstid_h
    }
    #[doc = "0x540 - Memory Frame Master ID Protection Register0"]
    #[inline(always)]
    pub const fn pcs0mstid(&self) -> &Pcs0mstid {
        &self.pcs0mstid
    }
    #[doc = "0x544 - Memory Frame Master ID Protection Register1"]
    #[inline(always)]
    pub const fn pcs1mstid(&self) -> &Pcs1mstid {
        &self.pcs1mstid
    }
    #[doc = "0x548 - Memory Frame Master ID Protection Register2"]
    #[inline(always)]
    pub const fn pcs2mstid(&self) -> &Pcs2mstid {
        &self.pcs2mstid
    }
    #[doc = "0x54c - Memory Frame Master ID Protection Register3"]
    #[inline(always)]
    pub const fn pcs3mstid(&self) -> &Pcs3mstid {
        &self.pcs3mstid
    }
    #[doc = "0x550 - Memory Frame Master ID Protection Register4"]
    #[inline(always)]
    pub const fn pcs4mstid(&self) -> &Pcs4mstid {
        &self.pcs4mstid
    }
    #[doc = "0x554 - Memory Frame Master ID Protection Register5"]
    #[inline(always)]
    pub const fn pcs5mstid(&self) -> &Pcs5mstid {
        &self.pcs5mstid
    }
    #[doc = "0x558 - Memory Frame Master ID Protection Register6"]
    #[inline(always)]
    pub const fn pcs6mstid(&self) -> &Pcs6mstid {
        &self.pcs6mstid
    }
    #[doc = "0x55c - Memory Frame Master ID Protection Register7"]
    #[inline(always)]
    pub const fn pcs7mstid(&self) -> &Pcs7mstid {
        &self.pcs7mstid
    }
    #[doc = "0x560 - Memory Frame Master ID Protection Register8"]
    #[inline(always)]
    pub const fn pcs8mstid(&self) -> &Pcs8mstid {
        &self.pcs8mstid
    }
    #[doc = "0x564 - Memory Frame Master ID Protection Register9"]
    #[inline(always)]
    pub const fn pcs9mstid(&self) -> &Pcs9mstid {
        &self.pcs9mstid
    }
    #[doc = "0x568 - Memory Frame Master ID Protection Register10"]
    #[inline(always)]
    pub const fn pcs10mstid(&self) -> &Pcs10mstid {
        &self.pcs10mstid
    }
    #[doc = "0x56c - Memory Frame Master ID Protection Register11"]
    #[inline(always)]
    pub const fn pcs11mstid(&self) -> &Pcs11mstid {
        &self.pcs11mstid
    }
    #[doc = "0x570 - Memory Frame Master ID Protection Register12"]
    #[inline(always)]
    pub const fn pcs12mstid(&self) -> &Pcs12mstid {
        &self.pcs12mstid
    }
    #[doc = "0x574 - Memory Frame Master ID Protection Register13"]
    #[inline(always)]
    pub const fn pcs13mstid(&self) -> &Pcs13mstid {
        &self.pcs13mstid
    }
    #[doc = "0x578 - Memory Frame Master ID Protection Register14"]
    #[inline(always)]
    pub const fn pcs14mstid(&self) -> &Pcs14mstid {
        &self.pcs14mstid
    }
    #[doc = "0x57c - Memory Frame Master ID Protection Register15"]
    #[inline(always)]
    pub const fn pcs15mstid(&self) -> &Pcs15mstid {
        &self.pcs15mstid
    }
    #[doc = "0x580 - Memory Frame Master ID Protection Register16"]
    #[inline(always)]
    pub const fn pcs16mstid(&self) -> &Pcs16mstid {
        &self.pcs16mstid
    }
    #[doc = "0x584 - Memory Frame Master ID Protection Register17"]
    #[inline(always)]
    pub const fn pcs17mstid(&self) -> &Pcs17mstid {
        &self.pcs17mstid
    }
    #[doc = "0x588 - Memory Frame Master ID Protection Register18"]
    #[inline(always)]
    pub const fn pcs18mstid(&self) -> &Pcs18mstid {
        &self.pcs18mstid
    }
    #[doc = "0x58c - Memory Frame Master ID Protection Register19"]
    #[inline(always)]
    pub const fn pcs19mstid(&self) -> &Pcs19mstid {
        &self.pcs19mstid
    }
    #[doc = "0x590 - Memory Frame Master ID Protection Register20"]
    #[inline(always)]
    pub const fn pcs20mstid(&self) -> &Pcs20mstid {
        &self.pcs20mstid
    }
    #[doc = "0x594 - Memory Frame Master ID Protection Register21"]
    #[inline(always)]
    pub const fn pcs21mstid(&self) -> &Pcs21mstid {
        &self.pcs21mstid
    }
    #[doc = "0x598 - Memory Frame Master ID Protection Register22"]
    #[inline(always)]
    pub const fn pcs22mstid(&self) -> &Pcs22mstid {
        &self.pcs22mstid
    }
    #[doc = "0x59c - Memory Frame Master ID Protection Register23"]
    #[inline(always)]
    pub const fn pcs23mstid(&self) -> &Pcs23mstid {
        &self.pcs23mstid
    }
    #[doc = "0x5a0 - Memory Frame Master ID Protection Register24"]
    #[inline(always)]
    pub const fn pcs24mstid(&self) -> &Pcs24mstid {
        &self.pcs24mstid
    }
    #[doc = "0x5a4 - Memory Frame Master ID Protection Register25"]
    #[inline(always)]
    pub const fn pcs25mstid(&self) -> &Pcs25mstid {
        &self.pcs25mstid
    }
    #[doc = "0x5a8 - Memory Frame Master ID Protection Register26"]
    #[inline(always)]
    pub const fn pcs26mstid(&self) -> &Pcs26mstid {
        &self.pcs26mstid
    }
    #[doc = "0x5ac - Memory Frame Master ID Protection Register27"]
    #[inline(always)]
    pub const fn pcs27mstid(&self) -> &Pcs27mstid {
        &self.pcs27mstid
    }
    #[doc = "0x5b0 - Memory Frame Master ID Protection Register28"]
    #[inline(always)]
    pub const fn pcs28mstid(&self) -> &Pcs28mstid {
        &self.pcs28mstid
    }
    #[doc = "0x5b4 - Memory Frame Master ID Protection Register29"]
    #[inline(always)]
    pub const fn pcs29mstid(&self) -> &Pcs29mstid {
        &self.pcs29mstid
    }
    #[doc = "0x5b8 - Memory Frame Master ID Protection Register30"]
    #[inline(always)]
    pub const fn pcs30mstid(&self) -> &Pcs30mstid {
        &self.pcs30mstid
    }
    #[doc = "0x5bc - Memory Frame Master ID Protection Register31"]
    #[inline(always)]
    pub const fn pcs31mstid(&self) -> &Pcs31mstid {
        &self.pcs31mstid
    }
    #[doc = "0x5c0 - Memory Frame Master ID Protection Register32"]
    #[inline(always)]
    pub const fn ppcs0mstid(&self) -> &Ppcs0mstid {
        &self.ppcs0mstid
    }
    #[doc = "0x5c4 - Memory Frame Master ID Protection Register33"]
    #[inline(always)]
    pub const fn ppcs1mstid(&self) -> &Ppcs1mstid {
        &self.ppcs1mstid
    }
    #[doc = "0x5c8 - Memory Frame Master ID Protection Register34"]
    #[inline(always)]
    pub const fn ppcs2mstid(&self) -> &Ppcs2mstid {
        &self.ppcs2mstid
    }
    #[doc = "0x5cc - Memory Frame Master ID Protection Register35"]
    #[inline(always)]
    pub const fn ppcs3mstid(&self) -> &Ppcs3mstid {
        &self.ppcs3mstid
    }
    #[doc = "0x5d0 - Memory Frame Master ID Protection Register36"]
    #[inline(always)]
    pub const fn ppcs4mstid(&self) -> &Ppcs4mstid {
        &self.ppcs4mstid
    }
    #[doc = "0x5d4 - Memory Frame Master ID Protection Register37"]
    #[inline(always)]
    pub const fn ppcs5mstid(&self) -> &Ppcs5mstid {
        &self.ppcs5mstid
    }
    #[doc = "0x5d8 - Memory Frame Master ID Protection Register38"]
    #[inline(always)]
    pub const fn ppcs6mstid(&self) -> &Ppcs6mstid {
        &self.ppcs6mstid
    }
    #[doc = "0x5dc - Memory Frame Master ID Protection Register39"]
    #[inline(always)]
    pub const fn ppcs7mstid(&self) -> &Ppcs7mstid {
        &self.ppcs7mstid
    }
    #[doc = "0x5e0 - Master-ID Protection Register for external PCR"]
    #[inline(always)]
    pub const fn pcrextmstid(&self) -> &Pcrextmstid {
        &self.pcrextmstid
    }
}
#[doc = "PMPROTSET0 (rw) register accessor: Set-only register to protect PCS frames 0 to 31\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprotset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprotset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmprotset0`]
module"]
#[doc(alias = "PMPROTSET0")]
pub type Pmprotset0 = crate::Reg<pmprotset0::Pmprotset0Spec>;
#[doc = "Set-only register to protect PCS frames 0 to 31"]
pub mod pmprotset0;
#[doc = "PMPROTSET1 (rw) register accessor: Set-only register to protect PCS frames 32 to 63\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprotset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprotset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmprotset1`]
module"]
#[doc(alias = "PMPROTSET1")]
pub type Pmprotset1 = crate::Reg<pmprotset1::Pmprotset1Spec>;
#[doc = "Set-only register to protect PCS frames 32 to 63"]
pub mod pmprotset1;
#[doc = "PMPROTCLR0 (rw) register accessor: Clear-only register to protect PCS frames 0 to 31\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprotclr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprotclr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmprotclr0`]
module"]
#[doc(alias = "PMPROTCLR0")]
pub type Pmprotclr0 = crate::Reg<pmprotclr0::Pmprotclr0Spec>;
#[doc = "Clear-only register to protect PCS frames 0 to 31"]
pub mod pmprotclr0;
#[doc = "PMPROTCLR1 (rw) register accessor: Clear-only register to protect PCS frames 32 to 63\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprotclr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprotclr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmprotclr1`]
module"]
#[doc(alias = "PMPROTCLR1")]
pub type Pmprotclr1 = crate::Reg<pmprotclr1::Pmprotclr1Spec>;
#[doc = "Clear-only register to protect PCS frames 32 to 63"]
pub mod pmprotclr1;
#[doc = "PPROTSET_0 (rw) register accessor: Set-only register to protect the 32 quadrants of PS0 to PS7\n\nYou can [`read`](crate::Reg::read) this register and get [`pprotset_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pprotset_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pprotset_0`]
module"]
#[doc(alias = "PPROTSET_0")]
pub type Pprotset0 = crate::Reg<pprotset_0::Pprotset0Spec>;
#[doc = "Set-only register to protect the 32 quadrants of PS0 to PS7"]
pub mod pprotset_0;
#[doc = "PPROTSET_1 (rw) register accessor: Set-only register to protect the 32 quadrants of PS8 to PS15\n\nYou can [`read`](crate::Reg::read) this register and get [`pprotset_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pprotset_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pprotset_1`]
module"]
#[doc(alias = "PPROTSET_1")]
pub type Pprotset1 = crate::Reg<pprotset_1::Pprotset1Spec>;
#[doc = "Set-only register to protect the 32 quadrants of PS8 to PS15"]
pub mod pprotset_1;
#[doc = "PPROTSET_2 (rw) register accessor: Set-only register to protect the 32 quadrants of PS16 to PS23\n\nYou can [`read`](crate::Reg::read) this register and get [`pprotset_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pprotset_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pprotset_2`]
module"]
#[doc(alias = "PPROTSET_2")]
pub type Pprotset2 = crate::Reg<pprotset_2::Pprotset2Spec>;
#[doc = "Set-only register to protect the 32 quadrants of PS16 to PS23"]
pub mod pprotset_2;
#[doc = "PPROTSET_3 (rw) register accessor: Set-only register to protect the 32 quadrants of PS24 to PS31\n\nYou can [`read`](crate::Reg::read) this register and get [`pprotset_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pprotset_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pprotset_3`]
module"]
#[doc(alias = "PPROTSET_3")]
pub type Pprotset3 = crate::Reg<pprotset_3::Pprotset3Spec>;
#[doc = "Set-only register to protect the 32 quadrants of PS24 to PS31"]
pub mod pprotset_3;
#[doc = "PPROTCLR0 (rw) register accessor: Clear-only register to protect the 32 quadrants of PS0 to PS7\n\nYou can [`read`](crate::Reg::read) this register and get [`pprotclr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pprotclr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pprotclr0`]
module"]
#[doc(alias = "PPROTCLR0")]
pub type Pprotclr0 = crate::Reg<pprotclr0::Pprotclr0Spec>;
#[doc = "Clear-only register to protect the 32 quadrants of PS0 to PS7"]
pub mod pprotclr0;
#[doc = "PPROTCLR1 (rw) register accessor: Clear-only register to protect the 32 quadrants of PS8 to PS15\n\nYou can [`read`](crate::Reg::read) this register and get [`pprotclr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pprotclr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pprotclr1`]
module"]
#[doc(alias = "PPROTCLR1")]
pub type Pprotclr1 = crate::Reg<pprotclr1::Pprotclr1Spec>;
#[doc = "Clear-only register to protect the 32 quadrants of PS8 to PS15"]
pub mod pprotclr1;
#[doc = "PPROTCLR2 (rw) register accessor: Clear-only register to protect the 32 quadrants of PS16 to PS23\n\nYou can [`read`](crate::Reg::read) this register and get [`pprotclr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pprotclr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pprotclr2`]
module"]
#[doc(alias = "PPROTCLR2")]
pub type Pprotclr2 = crate::Reg<pprotclr2::Pprotclr2Spec>;
#[doc = "Clear-only register to protect the 32 quadrants of PS16 to PS23"]
pub mod pprotclr2;
#[doc = "PPROTCLR3 (rw) register accessor: Clear-only register to protect the 32 quadrants of PS24 to PS31\n\nYou can [`read`](crate::Reg::read) this register and get [`pprotclr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pprotclr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pprotclr3`]
module"]
#[doc(alias = "PPROTCLR3")]
pub type Pprotclr3 = crate::Reg<pprotclr3::Pprotclr3Spec>;
#[doc = "Clear-only register to protect the 32 quadrants of PS24 to PS31"]
pub mod pprotclr3;
#[doc = "PCSPWRDWNSET0 (rw) register accessor: Set-only register to powerdown independent (non-shared) PCS frames 0 to 31\n\nYou can [`read`](crate::Reg::read) this register and get [`pcspwrdwnset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcspwrdwnset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcspwrdwnset0`]
module"]
#[doc(alias = "PCSPWRDWNSET0")]
pub type Pcspwrdwnset0 = crate::Reg<pcspwrdwnset0::Pcspwrdwnset0Spec>;
#[doc = "Set-only register to powerdown independent (non-shared) PCS frames 0 to 31"]
pub mod pcspwrdwnset0;
#[doc = "PCSPWRDWNSET1 (rw) register accessor: Set-only register to powerdown independent (non-shared) PCS frames 32 to 63\n\nYou can [`read`](crate::Reg::read) this register and get [`pcspwrdwnset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcspwrdwnset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcspwrdwnset1`]
module"]
#[doc(alias = "PCSPWRDWNSET1")]
pub type Pcspwrdwnset1 = crate::Reg<pcspwrdwnset1::Pcspwrdwnset1Spec>;
#[doc = "Set-only register to powerdown independent (non-shared) PCS frames 32 to 63"]
pub mod pcspwrdwnset1;
#[doc = "PCSPWRDWNCLR0 (rw) register accessor: Clear-only register to deassert powerdown bits of independent (non-shared) PCS frames 0 to 31\n\nYou can [`read`](crate::Reg::read) this register and get [`pcspwrdwnclr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcspwrdwnclr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcspwrdwnclr0`]
module"]
#[doc(alias = "PCSPWRDWNCLR0")]
pub type Pcspwrdwnclr0 = crate::Reg<pcspwrdwnclr0::Pcspwrdwnclr0Spec>;
#[doc = "Clear-only register to deassert powerdown bits of independent (non-shared) PCS frames 0 to 31"]
pub mod pcspwrdwnclr0;
#[doc = "PCSPWRDWNCLR1 (rw) register accessor: Clear-only register to deassert powerdown bits of independent (non-shared) PCS frames 32 to 63\n\nYou can [`read`](crate::Reg::read) this register and get [`pcspwrdwnclr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcspwrdwnclr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcspwrdwnclr1`]
module"]
#[doc(alias = "PCSPWRDWNCLR1")]
pub type Pcspwrdwnclr1 = crate::Reg<pcspwrdwnclr1::Pcspwrdwnclr1Spec>;
#[doc = "Clear-only register to deassert powerdown bits of independent (non-shared) PCS frames 32 to 63"]
pub mod pcspwrdwnclr1;
#[doc = "PSPWRDWNSET0 (rw) register accessor: Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS0 to PS7\n\nYou can [`read`](crate::Reg::read) this register and get [`pspwrdwnset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspwrdwnset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspwrdwnset0`]
module"]
#[doc(alias = "PSPWRDWNSET0")]
pub type Pspwrdwnset0 = crate::Reg<pspwrdwnset0::Pspwrdwnset0Spec>;
#[doc = "Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS0 to PS7"]
pub mod pspwrdwnset0;
#[doc = "PSPWRDWNSET1 (rw) register accessor: Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS8 to PS15\n\nYou can [`read`](crate::Reg::read) this register and get [`pspwrdwnset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspwrdwnset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspwrdwnset1`]
module"]
#[doc(alias = "PSPWRDWNSET1")]
pub type Pspwrdwnset1 = crate::Reg<pspwrdwnset1::Pspwrdwnset1Spec>;
#[doc = "Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS8 to PS15"]
pub mod pspwrdwnset1;
#[doc = "PSPWRDWNSET2 (rw) register accessor: Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS16 to PS23\n\nYou can [`read`](crate::Reg::read) this register and get [`pspwrdwnset2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspwrdwnset2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspwrdwnset2`]
module"]
#[doc(alias = "PSPWRDWNSET2")]
pub type Pspwrdwnset2 = crate::Reg<pspwrdwnset2::Pspwrdwnset2Spec>;
#[doc = "Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS16 to PS23"]
pub mod pspwrdwnset2;
#[doc = "PSPWRDWNSET3 (rw) register accessor: Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS24 to PS31\n\nYou can [`read`](crate::Reg::read) this register and get [`pspwrdwnset3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspwrdwnset3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspwrdwnset3`]
module"]
#[doc(alias = "PSPWRDWNSET3")]
pub type Pspwrdwnset3 = crate::Reg<pspwrdwnset3::Pspwrdwnset3Spec>;
#[doc = "Set-only register to powerdown the applicable peripherals in the 32 quadrants of PS24 to PS31"]
pub mod pspwrdwnset3;
#[doc = "PSPWRDWNCLR0 (rw) register accessor: Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS0 to PS7\n\nYou can [`read`](crate::Reg::read) this register and get [`pspwrdwnclr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspwrdwnclr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspwrdwnclr0`]
module"]
#[doc(alias = "PSPWRDWNCLR0")]
pub type Pspwrdwnclr0 = crate::Reg<pspwrdwnclr0::Pspwrdwnclr0Spec>;
#[doc = "Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS0 to PS7"]
pub mod pspwrdwnclr0;
#[doc = "PSPWRDWNCLR1 (rw) register accessor: Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS8 to PS15\n\nYou can [`read`](crate::Reg::read) this register and get [`pspwrdwnclr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspwrdwnclr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspwrdwnclr1`]
module"]
#[doc(alias = "PSPWRDWNCLR1")]
pub type Pspwrdwnclr1 = crate::Reg<pspwrdwnclr1::Pspwrdwnclr1Spec>;
#[doc = "Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS8 to PS15"]
pub mod pspwrdwnclr1;
#[doc = "PSPWRDWNCLR2 (rw) register accessor: Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS16 to PS23\n\nYou can [`read`](crate::Reg::read) this register and get [`pspwrdwnclr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspwrdwnclr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspwrdwnclr2`]
module"]
#[doc(alias = "PSPWRDWNCLR2")]
pub type Pspwrdwnclr2 = crate::Reg<pspwrdwnclr2::Pspwrdwnclr2Spec>;
#[doc = "Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS16 to PS23"]
pub mod pspwrdwnclr2;
#[doc = "PSPWRDWNCLR3 (rw) register accessor: Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS24 to PS31\n\nYou can [`read`](crate::Reg::read) this register and get [`pspwrdwnclr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspwrdwnclr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspwrdwnclr3`]
module"]
#[doc(alias = "PSPWRDWNCLR3")]
pub type Pspwrdwnclr3 = crate::Reg<pspwrdwnclr3::Pspwrdwnclr3Spec>;
#[doc = "Clear-only register to deassert powerdown bits of the applicable peripherals in the 32 quadrants of PS24 to PS31"]
pub mod pspwrdwnclr3;
#[doc = "PDPWRDWNSET (rw) register accessor: Set-only register to powerdown the debug frame\n\nYou can [`read`](crate::Reg::read) this register and get [`pdpwrdwnset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdpwrdwnset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdpwrdwnset`]
module"]
#[doc(alias = "PDPWRDWNSET")]
pub type Pdpwrdwnset = crate::Reg<pdpwrdwnset::PdpwrdwnsetSpec>;
#[doc = "Set-only register to powerdown the debug frame"]
pub mod pdpwrdwnset;
#[doc = "PDPWRDWNCLR (rw) register accessor: Clear-only register to deassert the debug frameΓÇÖs powerdown bit\n\nYou can [`read`](crate::Reg::read) this register and get [`pdpwrdwnclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdpwrdwnclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdpwrdwnclr`]
module"]
#[doc(alias = "PDPWRDWNCLR")]
pub type Pdpwrdwnclr = crate::Reg<pdpwrdwnclr::PdpwrdwnclrSpec>;
#[doc = "Clear-only register to deassert the debug frameΓÇÖs powerdown bit"]
pub mod pdpwrdwnclr;
#[doc = "MSTIDWRENA (rw) register accessor: MasterID Protection Write Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstidwrena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstidwrena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstidwrena`]
module"]
#[doc(alias = "MSTIDWRENA")]
pub type Mstidwrena = crate::Reg<mstidwrena::MstidwrenaSpec>;
#[doc = "MasterID Protection Write Enable Register"]
pub mod mstidwrena;
#[doc = "MSTIDENA (rw) register accessor: MasterID Protection Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstidena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstidena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstidena`]
module"]
#[doc(alias = "MSTIDENA")]
pub type Mstidena = crate::Reg<mstidena::MstidenaSpec>;
#[doc = "MasterID Protection Enable Register"]
pub mod mstidena;
#[doc = "MSTIDDIAGCTRL (rw) register accessor: MasterID Diagnostic Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstiddiagctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstiddiagctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstiddiagctrl`]
module"]
#[doc(alias = "MSTIDDIAGCTRL")]
pub type Mstiddiagctrl = crate::Reg<mstiddiagctrl::MstiddiagctrlSpec>;
#[doc = "MasterID Diagnostic Control Register"]
pub mod mstiddiagctrl;
#[doc = "PS0MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register0_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps0mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps0mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps0mstid_l`]
module"]
#[doc(alias = "PS0MSTID_L")]
pub type Ps0mstidL = crate::Reg<ps0mstid_l::Ps0mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register0_L"]
pub mod ps0mstid_l;
#[doc = "PS0MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register0_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps0mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps0mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps0mstid_h`]
module"]
#[doc(alias = "PS0MSTID_H")]
pub type Ps0mstidH = crate::Reg<ps0mstid_h::Ps0mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register0_H"]
pub mod ps0mstid_h;
#[doc = "PS1MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register1_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps1mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps1mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps1mstid_l`]
module"]
#[doc(alias = "PS1MSTID_L")]
pub type Ps1mstidL = crate::Reg<ps1mstid_l::Ps1mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register1_L"]
pub mod ps1mstid_l;
#[doc = "PS1MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register1_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps1mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps1mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps1mstid_h`]
module"]
#[doc(alias = "PS1MSTID_H")]
pub type Ps1mstidH = crate::Reg<ps1mstid_h::Ps1mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register1_H"]
pub mod ps1mstid_h;
#[doc = "PS2MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register2_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps2mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps2mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps2mstid_l`]
module"]
#[doc(alias = "PS2MSTID_L")]
pub type Ps2mstidL = crate::Reg<ps2mstid_l::Ps2mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register2_L"]
pub mod ps2mstid_l;
#[doc = "PS2MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register2_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps2mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps2mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps2mstid_h`]
module"]
#[doc(alias = "PS2MSTID_H")]
pub type Ps2mstidH = crate::Reg<ps2mstid_h::Ps2mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register2_H"]
pub mod ps2mstid_h;
#[doc = "PS3MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register3_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps3mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps3mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps3mstid_l`]
module"]
#[doc(alias = "PS3MSTID_L")]
pub type Ps3mstidL = crate::Reg<ps3mstid_l::Ps3mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register3_L"]
pub mod ps3mstid_l;
#[doc = "PS3MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register3_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps3mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps3mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps3mstid_h`]
module"]
#[doc(alias = "PS3MSTID_H")]
pub type Ps3mstidH = crate::Reg<ps3mstid_h::Ps3mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register3_H"]
pub mod ps3mstid_h;
#[doc = "PS4MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register4_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps4mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps4mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps4mstid_l`]
module"]
#[doc(alias = "PS4MSTID_L")]
pub type Ps4mstidL = crate::Reg<ps4mstid_l::Ps4mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register4_L"]
pub mod ps4mstid_l;
#[doc = "PS4MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register4_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps4mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps4mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps4mstid_h`]
module"]
#[doc(alias = "PS4MSTID_H")]
pub type Ps4mstidH = crate::Reg<ps4mstid_h::Ps4mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register4_H"]
pub mod ps4mstid_h;
#[doc = "PS5MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register5_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps5mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps5mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps5mstid_l`]
module"]
#[doc(alias = "PS5MSTID_L")]
pub type Ps5mstidL = crate::Reg<ps5mstid_l::Ps5mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register5_L"]
pub mod ps5mstid_l;
#[doc = "PS5MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register5_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps5mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps5mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps5mstid_h`]
module"]
#[doc(alias = "PS5MSTID_H")]
pub type Ps5mstidH = crate::Reg<ps5mstid_h::Ps5mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register5_H"]
pub mod ps5mstid_h;
#[doc = "PS6MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register6_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps6mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps6mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps6mstid_l`]
module"]
#[doc(alias = "PS6MSTID_L")]
pub type Ps6mstidL = crate::Reg<ps6mstid_l::Ps6mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register6_L"]
pub mod ps6mstid_l;
#[doc = "PS6MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register6_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps6mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps6mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps6mstid_h`]
module"]
#[doc(alias = "PS6MSTID_H")]
pub type Ps6mstidH = crate::Reg<ps6mstid_h::Ps6mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register6_H"]
pub mod ps6mstid_h;
#[doc = "PS7MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register7_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps7mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps7mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps7mstid_l`]
module"]
#[doc(alias = "PS7MSTID_L")]
pub type Ps7mstidL = crate::Reg<ps7mstid_l::Ps7mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register7_L"]
pub mod ps7mstid_l;
#[doc = "PS7MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register7_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps7mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps7mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps7mstid_h`]
module"]
#[doc(alias = "PS7MSTID_H")]
pub type Ps7mstidH = crate::Reg<ps7mstid_h::Ps7mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register7_H"]
pub mod ps7mstid_h;
#[doc = "PS8MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register8_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps8mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps8mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps8mstid_l`]
module"]
#[doc(alias = "PS8MSTID_L")]
pub type Ps8mstidL = crate::Reg<ps8mstid_l::Ps8mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register8_L"]
pub mod ps8mstid_l;
#[doc = "PS8MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register8_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps8mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps8mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps8mstid_h`]
module"]
#[doc(alias = "PS8MSTID_H")]
pub type Ps8mstidH = crate::Reg<ps8mstid_h::Ps8mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register8_H"]
pub mod ps8mstid_h;
#[doc = "PS9MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register9_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps9mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps9mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps9mstid_l`]
module"]
#[doc(alias = "PS9MSTID_L")]
pub type Ps9mstidL = crate::Reg<ps9mstid_l::Ps9mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register9_L"]
pub mod ps9mstid_l;
#[doc = "PS9MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register9_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps9mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps9mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps9mstid_h`]
module"]
#[doc(alias = "PS9MSTID_H")]
pub type Ps9mstidH = crate::Reg<ps9mstid_h::Ps9mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register9_H"]
pub mod ps9mstid_h;
#[doc = "PS10MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register10_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps10mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps10mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps10mstid_l`]
module"]
#[doc(alias = "PS10MSTID_L")]
pub type Ps10mstidL = crate::Reg<ps10mstid_l::Ps10mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register10_L"]
pub mod ps10mstid_l;
#[doc = "PS10MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register10_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps10mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps10mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps10mstid_h`]
module"]
#[doc(alias = "PS10MSTID_H")]
pub type Ps10mstidH = crate::Reg<ps10mstid_h::Ps10mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register10_H"]
pub mod ps10mstid_h;
#[doc = "PS11MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register11_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps11mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps11mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps11mstid_l`]
module"]
#[doc(alias = "PS11MSTID_L")]
pub type Ps11mstidL = crate::Reg<ps11mstid_l::Ps11mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register11_L"]
pub mod ps11mstid_l;
#[doc = "PS11MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register11_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps11mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps11mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps11mstid_h`]
module"]
#[doc(alias = "PS11MSTID_H")]
pub type Ps11mstidH = crate::Reg<ps11mstid_h::Ps11mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register11_H"]
pub mod ps11mstid_h;
#[doc = "PS12MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register12_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps12mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps12mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps12mstid_l`]
module"]
#[doc(alias = "PS12MSTID_L")]
pub type Ps12mstidL = crate::Reg<ps12mstid_l::Ps12mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register12_L"]
pub mod ps12mstid_l;
#[doc = "PS12MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register12_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps12mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps12mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps12mstid_h`]
module"]
#[doc(alias = "PS12MSTID_H")]
pub type Ps12mstidH = crate::Reg<ps12mstid_h::Ps12mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register12_H"]
pub mod ps12mstid_h;
#[doc = "PS13MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register13_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps13mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps13mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps13mstid_l`]
module"]
#[doc(alias = "PS13MSTID_L")]
pub type Ps13mstidL = crate::Reg<ps13mstid_l::Ps13mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register13_L"]
pub mod ps13mstid_l;
#[doc = "PS13MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register13_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps13mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps13mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps13mstid_h`]
module"]
#[doc(alias = "PS13MSTID_H")]
pub type Ps13mstidH = crate::Reg<ps13mstid_h::Ps13mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register13_H"]
pub mod ps13mstid_h;
#[doc = "PS14MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register14_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps14mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps14mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps14mstid_l`]
module"]
#[doc(alias = "PS14MSTID_L")]
pub type Ps14mstidL = crate::Reg<ps14mstid_l::Ps14mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register14_L"]
pub mod ps14mstid_l;
#[doc = "PS14MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register14_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps14mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps14mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps14mstid_h`]
module"]
#[doc(alias = "PS14MSTID_H")]
pub type Ps14mstidH = crate::Reg<ps14mstid_h::Ps14mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register14_H"]
pub mod ps14mstid_h;
#[doc = "PS15MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register15_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps15mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps15mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps15mstid_l`]
module"]
#[doc(alias = "PS15MSTID_L")]
pub type Ps15mstidL = crate::Reg<ps15mstid_l::Ps15mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register15_L"]
pub mod ps15mstid_l;
#[doc = "PS15MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register15_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps15mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps15mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps15mstid_h`]
module"]
#[doc(alias = "PS15MSTID_H")]
pub type Ps15mstidH = crate::Reg<ps15mstid_h::Ps15mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register15_H"]
pub mod ps15mstid_h;
#[doc = "PS16MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register16_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps16mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps16mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps16mstid_l`]
module"]
#[doc(alias = "PS16MSTID_L")]
pub type Ps16mstidL = crate::Reg<ps16mstid_l::Ps16mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register16_L"]
pub mod ps16mstid_l;
#[doc = "PS16MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register16_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps16mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps16mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps16mstid_h`]
module"]
#[doc(alias = "PS16MSTID_H")]
pub type Ps16mstidH = crate::Reg<ps16mstid_h::Ps16mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register16_H"]
pub mod ps16mstid_h;
#[doc = "PS17MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register17_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps17mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps17mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps17mstid_l`]
module"]
#[doc(alias = "PS17MSTID_L")]
pub type Ps17mstidL = crate::Reg<ps17mstid_l::Ps17mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register17_L"]
pub mod ps17mstid_l;
#[doc = "PS17MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register17_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps17mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps17mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps17mstid_h`]
module"]
#[doc(alias = "PS17MSTID_H")]
pub type Ps17mstidH = crate::Reg<ps17mstid_h::Ps17mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register17_H"]
pub mod ps17mstid_h;
#[doc = "PS18MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register18_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps18mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps18mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps18mstid_l`]
module"]
#[doc(alias = "PS18MSTID_L")]
pub type Ps18mstidL = crate::Reg<ps18mstid_l::Ps18mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register18_L"]
pub mod ps18mstid_l;
#[doc = "PS18MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register18_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps18mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps18mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps18mstid_h`]
module"]
#[doc(alias = "PS18MSTID_H")]
pub type Ps18mstidH = crate::Reg<ps18mstid_h::Ps18mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register18_H"]
pub mod ps18mstid_h;
#[doc = "PS19MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register19_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps19mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps19mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps19mstid_l`]
module"]
#[doc(alias = "PS19MSTID_L")]
pub type Ps19mstidL = crate::Reg<ps19mstid_l::Ps19mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register19_L"]
pub mod ps19mstid_l;
#[doc = "PS19MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register19_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps19mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps19mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps19mstid_h`]
module"]
#[doc(alias = "PS19MSTID_H")]
pub type Ps19mstidH = crate::Reg<ps19mstid_h::Ps19mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register19_H"]
pub mod ps19mstid_h;
#[doc = "PS20MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register20_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps20mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps20mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps20mstid_l`]
module"]
#[doc(alias = "PS20MSTID_L")]
pub type Ps20mstidL = crate::Reg<ps20mstid_l::Ps20mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register20_L"]
pub mod ps20mstid_l;
#[doc = "PS20MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register20_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps20mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps20mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps20mstid_h`]
module"]
#[doc(alias = "PS20MSTID_H")]
pub type Ps20mstidH = crate::Reg<ps20mstid_h::Ps20mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register20_H"]
pub mod ps20mstid_h;
#[doc = "PS21MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register21_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps21mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps21mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps21mstid_l`]
module"]
#[doc(alias = "PS21MSTID_L")]
pub type Ps21mstidL = crate::Reg<ps21mstid_l::Ps21mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register21_L"]
pub mod ps21mstid_l;
#[doc = "PS21MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register21_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps21mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps21mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps21mstid_h`]
module"]
#[doc(alias = "PS21MSTID_H")]
pub type Ps21mstidH = crate::Reg<ps21mstid_h::Ps21mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register21_H"]
pub mod ps21mstid_h;
#[doc = "PS22MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register22_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps22mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps22mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps22mstid_l`]
module"]
#[doc(alias = "PS22MSTID_L")]
pub type Ps22mstidL = crate::Reg<ps22mstid_l::Ps22mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register22_L"]
pub mod ps22mstid_l;
#[doc = "PS22MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register22_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps22mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps22mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps22mstid_h`]
module"]
#[doc(alias = "PS22MSTID_H")]
pub type Ps22mstidH = crate::Reg<ps22mstid_h::Ps22mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register22_H"]
pub mod ps22mstid_h;
#[doc = "PS23MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register23_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps23mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps23mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps23mstid_l`]
module"]
#[doc(alias = "PS23MSTID_L")]
pub type Ps23mstidL = crate::Reg<ps23mstid_l::Ps23mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register23_L"]
pub mod ps23mstid_l;
#[doc = "PS23MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register23_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps23mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps23mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps23mstid_h`]
module"]
#[doc(alias = "PS23MSTID_H")]
pub type Ps23mstidH = crate::Reg<ps23mstid_h::Ps23mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register23_H"]
pub mod ps23mstid_h;
#[doc = "PS24MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register24_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps24mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps24mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps24mstid_l`]
module"]
#[doc(alias = "PS24MSTID_L")]
pub type Ps24mstidL = crate::Reg<ps24mstid_l::Ps24mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register24_L"]
pub mod ps24mstid_l;
#[doc = "PS24MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register24_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps24mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps24mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps24mstid_h`]
module"]
#[doc(alias = "PS24MSTID_H")]
pub type Ps24mstidH = crate::Reg<ps24mstid_h::Ps24mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register24_H"]
pub mod ps24mstid_h;
#[doc = "PS25MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register25_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps25mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps25mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps25mstid_l`]
module"]
#[doc(alias = "PS25MSTID_L")]
pub type Ps25mstidL = crate::Reg<ps25mstid_l::Ps25mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register25_L"]
pub mod ps25mstid_l;
#[doc = "PS25MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register25_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps25mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps25mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps25mstid_h`]
module"]
#[doc(alias = "PS25MSTID_H")]
pub type Ps25mstidH = crate::Reg<ps25mstid_h::Ps25mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register25_H"]
pub mod ps25mstid_h;
#[doc = "PS26MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register26_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps26mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps26mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps26mstid_l`]
module"]
#[doc(alias = "PS26MSTID_L")]
pub type Ps26mstidL = crate::Reg<ps26mstid_l::Ps26mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register26_L"]
pub mod ps26mstid_l;
#[doc = "PS26MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register26_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps26mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps26mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps26mstid_h`]
module"]
#[doc(alias = "PS26MSTID_H")]
pub type Ps26mstidH = crate::Reg<ps26mstid_h::Ps26mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register26_H"]
pub mod ps26mstid_h;
#[doc = "PS27MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register27_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps27mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps27mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps27mstid_l`]
module"]
#[doc(alias = "PS27MSTID_L")]
pub type Ps27mstidL = crate::Reg<ps27mstid_l::Ps27mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register27_L"]
pub mod ps27mstid_l;
#[doc = "PS27MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register27_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps27mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps27mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps27mstid_h`]
module"]
#[doc(alias = "PS27MSTID_H")]
pub type Ps27mstidH = crate::Reg<ps27mstid_h::Ps27mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register27_H"]
pub mod ps27mstid_h;
#[doc = "PS28MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register28_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps28mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps28mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps28mstid_l`]
module"]
#[doc(alias = "PS28MSTID_L")]
pub type Ps28mstidL = crate::Reg<ps28mstid_l::Ps28mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register28_L"]
pub mod ps28mstid_l;
#[doc = "PS28MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register28_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps28mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps28mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps28mstid_h`]
module"]
#[doc(alias = "PS28MSTID_H")]
pub type Ps28mstidH = crate::Reg<ps28mstid_h::Ps28mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register28_H"]
pub mod ps28mstid_h;
#[doc = "PS29MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register29_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps29mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps29mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps29mstid_l`]
module"]
#[doc(alias = "PS29MSTID_L")]
pub type Ps29mstidL = crate::Reg<ps29mstid_l::Ps29mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register29_L"]
pub mod ps29mstid_l;
#[doc = "PS29MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register29_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps29mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps29mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps29mstid_h`]
module"]
#[doc(alias = "PS29MSTID_H")]
pub type Ps29mstidH = crate::Reg<ps29mstid_h::Ps29mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register29_H"]
pub mod ps29mstid_h;
#[doc = "PS30MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register30_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps30mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps30mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps30mstid_l`]
module"]
#[doc(alias = "PS30MSTID_L")]
pub type Ps30mstidL = crate::Reg<ps30mstid_l::Ps30mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register30_L"]
pub mod ps30mstid_l;
#[doc = "PS30MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register30_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps30mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps30mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps30mstid_h`]
module"]
#[doc(alias = "PS30MSTID_H")]
pub type Ps30mstidH = crate::Reg<ps30mstid_h::Ps30mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register30_H"]
pub mod ps30mstid_h;
#[doc = "PS31MSTID_L (rw) register accessor: Peripheral Frame Master-ID Protection Register31_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ps31mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps31mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps31mstid_l`]
module"]
#[doc(alias = "PS31MSTID_L")]
pub type Ps31mstidL = crate::Reg<ps31mstid_l::Ps31mstidLSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register31_L"]
pub mod ps31mstid_l;
#[doc = "PS31MSTID_H (rw) register accessor: Peripheral Frame Master-ID Protection Register31_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ps31mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps31mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps31mstid_h`]
module"]
#[doc(alias = "PS31MSTID_H")]
pub type Ps31mstidH = crate::Reg<ps31mstid_h::Ps31mstidHSpec>;
#[doc = "Peripheral Frame Master-ID Protection Register31_H"]
pub mod ps31mstid_h;
#[doc = "PPS0MSTID_L (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register0_L\n\nYou can [`read`](crate::Reg::read) this register and get [`pps0mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps0mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps0mstid_l`]
module"]
#[doc(alias = "PPS0MSTID_L")]
pub type Pps0mstidL = crate::Reg<pps0mstid_l::Pps0mstidLSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register0_L"]
pub mod pps0mstid_l;
#[doc = "PPS0MSTID_H (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register0_H\n\nYou can [`read`](crate::Reg::read) this register and get [`pps0mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps0mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps0mstid_h`]
module"]
#[doc(alias = "PPS0MSTID_H")]
pub type Pps0mstidH = crate::Reg<pps0mstid_h::Pps0mstidHSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register0_H"]
pub mod pps0mstid_h;
#[doc = "PPS1MSTID_L (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register1_L\n\nYou can [`read`](crate::Reg::read) this register and get [`pps1mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps1mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps1mstid_l`]
module"]
#[doc(alias = "PPS1MSTID_L")]
pub type Pps1mstidL = crate::Reg<pps1mstid_l::Pps1mstidLSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register1_L"]
pub mod pps1mstid_l;
#[doc = "PPS1MSTID_H (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register1_H\n\nYou can [`read`](crate::Reg::read) this register and get [`pps1mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps1mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps1mstid_h`]
module"]
#[doc(alias = "PPS1MSTID_H")]
pub type Pps1mstidH = crate::Reg<pps1mstid_h::Pps1mstidHSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register1_H"]
pub mod pps1mstid_h;
#[doc = "PPS2MSTID_L (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register2_L\n\nYou can [`read`](crate::Reg::read) this register and get [`pps2mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps2mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps2mstid_l`]
module"]
#[doc(alias = "PPS2MSTID_L")]
pub type Pps2mstidL = crate::Reg<pps2mstid_l::Pps2mstidLSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register2_L"]
pub mod pps2mstid_l;
#[doc = "PPS2MSTID_H (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register2_H\n\nYou can [`read`](crate::Reg::read) this register and get [`pps2mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps2mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps2mstid_h`]
module"]
#[doc(alias = "PPS2MSTID_H")]
pub type Pps2mstidH = crate::Reg<pps2mstid_h::Pps2mstidHSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register2_H"]
pub mod pps2mstid_h;
#[doc = "PPS3MSTID_L (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register3_L\n\nYou can [`read`](crate::Reg::read) this register and get [`pps3mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps3mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps3mstid_l`]
module"]
#[doc(alias = "PPS3MSTID_L")]
pub type Pps3mstidL = crate::Reg<pps3mstid_l::Pps3mstidLSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register3_L"]
pub mod pps3mstid_l;
#[doc = "PPS3MSTID_H (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register3_H\n\nYou can [`read`](crate::Reg::read) this register and get [`pps3mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps3mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps3mstid_h`]
module"]
#[doc(alias = "PPS3MSTID_H")]
pub type Pps3mstidH = crate::Reg<pps3mstid_h::Pps3mstidHSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register3_H"]
pub mod pps3mstid_h;
#[doc = "PPS4MSTID_L (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register4_L\n\nYou can [`read`](crate::Reg::read) this register and get [`pps4mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps4mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps4mstid_l`]
module"]
#[doc(alias = "PPS4MSTID_L")]
pub type Pps4mstidL = crate::Reg<pps4mstid_l::Pps4mstidLSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register4_L"]
pub mod pps4mstid_l;
#[doc = "PPS4MSTID_H (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register4_H\n\nYou can [`read`](crate::Reg::read) this register and get [`pps4mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps4mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps4mstid_h`]
module"]
#[doc(alias = "PPS4MSTID_H")]
pub type Pps4mstidH = crate::Reg<pps4mstid_h::Pps4mstidHSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register4_H"]
pub mod pps4mstid_h;
#[doc = "PPS5MSTID_L (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register5_L\n\nYou can [`read`](crate::Reg::read) this register and get [`pps5mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps5mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps5mstid_l`]
module"]
#[doc(alias = "PPS5MSTID_L")]
pub type Pps5mstidL = crate::Reg<pps5mstid_l::Pps5mstidLSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register5_L"]
pub mod pps5mstid_l;
#[doc = "PPS5MSTID_H (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register5_H\n\nYou can [`read`](crate::Reg::read) this register and get [`pps5mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps5mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps5mstid_h`]
module"]
#[doc(alias = "PPS5MSTID_H")]
pub type Pps5mstidH = crate::Reg<pps5mstid_h::Pps5mstidHSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register5_H"]
pub mod pps5mstid_h;
#[doc = "PPS6MSTID_L (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register6_L\n\nYou can [`read`](crate::Reg::read) this register and get [`pps6mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps6mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps6mstid_l`]
module"]
#[doc(alias = "PPS6MSTID_L")]
pub type Pps6mstidL = crate::Reg<pps6mstid_l::Pps6mstidLSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register6_L"]
pub mod pps6mstid_l;
#[doc = "PPS6MSTID_H (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register6_H\n\nYou can [`read`](crate::Reg::read) this register and get [`pps6mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps6mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps6mstid_h`]
module"]
#[doc(alias = "PPS6MSTID_H")]
pub type Pps6mstidH = crate::Reg<pps6mstid_h::Pps6mstidHSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register6_H"]
pub mod pps6mstid_h;
#[doc = "PPS7MSTID_L (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register7_L\n\nYou can [`read`](crate::Reg::read) this register and get [`pps7mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps7mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps7mstid_l`]
module"]
#[doc(alias = "PPS7MSTID_L")]
pub type Pps7mstidL = crate::Reg<pps7mstid_l::Pps7mstidLSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register7_L"]
pub mod pps7mstid_l;
#[doc = "PPS7MSTID_H (rw) register accessor: Privileged Peripheral Frame Master-ID Protection Register7_H\n\nYou can [`read`](crate::Reg::read) this register and get [`pps7mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps7mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps7mstid_h`]
module"]
#[doc(alias = "PPS7MSTID_H")]
pub type Pps7mstidH = crate::Reg<pps7mstid_h::Pps7mstidHSpec>;
#[doc = "Privileged Peripheral Frame Master-ID Protection Register7_H"]
pub mod pps7mstid_h;
#[doc = "PPSE0MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register0_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse0mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse0mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse0mstid_l`]
module"]
#[doc(alias = "PPSE0MSTID_L")]
pub type Ppse0mstidL = crate::Reg<ppse0mstid_l::Ppse0mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register0_L"]
pub mod ppse0mstid_l;
#[doc = "PPSE0MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register0_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse0mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse0mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse0mstid_h`]
module"]
#[doc(alias = "PPSE0MSTID_H")]
pub type Ppse0mstidH = crate::Reg<ppse0mstid_h::Ppse0mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register0_H"]
pub mod ppse0mstid_h;
#[doc = "PPSE1MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register1_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse1mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse1mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse1mstid_l`]
module"]
#[doc(alias = "PPSE1MSTID_L")]
pub type Ppse1mstidL = crate::Reg<ppse1mstid_l::Ppse1mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register1_L"]
pub mod ppse1mstid_l;
#[doc = "PPSE1MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register1_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse1mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse1mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse1mstid_h`]
module"]
#[doc(alias = "PPSE1MSTID_H")]
pub type Ppse1mstidH = crate::Reg<ppse1mstid_h::Ppse1mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register1_H"]
pub mod ppse1mstid_h;
#[doc = "PPSE2MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register2_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse2mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse2mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse2mstid_l`]
module"]
#[doc(alias = "PPSE2MSTID_L")]
pub type Ppse2mstidL = crate::Reg<ppse2mstid_l::Ppse2mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register2_L"]
pub mod ppse2mstid_l;
#[doc = "PPSE2MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register2_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse2mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse2mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse2mstid_h`]
module"]
#[doc(alias = "PPSE2MSTID_H")]
pub type Ppse2mstidH = crate::Reg<ppse2mstid_h::Ppse2mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register2_H"]
pub mod ppse2mstid_h;
#[doc = "PPSE3MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register3_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse3mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse3mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse3mstid_l`]
module"]
#[doc(alias = "PPSE3MSTID_L")]
pub type Ppse3mstidL = crate::Reg<ppse3mstid_l::Ppse3mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register3_L"]
pub mod ppse3mstid_l;
#[doc = "PPSE3MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register3_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse3mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse3mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse3mstid_h`]
module"]
#[doc(alias = "PPSE3MSTID_H")]
pub type Ppse3mstidH = crate::Reg<ppse3mstid_h::Ppse3mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register3_H"]
pub mod ppse3mstid_h;
#[doc = "PPSE4MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register4_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse4mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse4mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse4mstid_l`]
module"]
#[doc(alias = "PPSE4MSTID_L")]
pub type Ppse4mstidL = crate::Reg<ppse4mstid_l::Ppse4mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register4_L"]
pub mod ppse4mstid_l;
#[doc = "PPSE4MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register4_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse4mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse4mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse4mstid_h`]
module"]
#[doc(alias = "PPSE4MSTID_H")]
pub type Ppse4mstidH = crate::Reg<ppse4mstid_h::Ppse4mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register4_H"]
pub mod ppse4mstid_h;
#[doc = "PPSE5MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register5_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse5mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse5mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse5mstid_l`]
module"]
#[doc(alias = "PPSE5MSTID_L")]
pub type Ppse5mstidL = crate::Reg<ppse5mstid_l::Ppse5mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register5_L"]
pub mod ppse5mstid_l;
#[doc = "PPSE5MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register5_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse5mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse5mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse5mstid_h`]
module"]
#[doc(alias = "PPSE5MSTID_H")]
pub type Ppse5mstidH = crate::Reg<ppse5mstid_h::Ppse5mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register5_H"]
pub mod ppse5mstid_h;
#[doc = "PPSE6MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register6_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse6mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse6mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse6mstid_l`]
module"]
#[doc(alias = "PPSE6MSTID_L")]
pub type Ppse6mstidL = crate::Reg<ppse6mstid_l::Ppse6mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register6_L"]
pub mod ppse6mstid_l;
#[doc = "PPSE6MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register6_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse6mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse6mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse6mstid_h`]
module"]
#[doc(alias = "PPSE6MSTID_H")]
pub type Ppse6mstidH = crate::Reg<ppse6mstid_h::Ppse6mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register6_H"]
pub mod ppse6mstid_h;
#[doc = "PPSE7MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register7_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse7mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse7mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse7mstid_l`]
module"]
#[doc(alias = "PPSE7MSTID_L")]
pub type Ppse7mstidL = crate::Reg<ppse7mstid_l::Ppse7mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register7_L"]
pub mod ppse7mstid_l;
#[doc = "PPSE7MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register7_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse7mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse7mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse7mstid_h`]
module"]
#[doc(alias = "PPSE7MSTID_H")]
pub type Ppse7mstidH = crate::Reg<ppse7mstid_h::Ppse7mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register7_H"]
pub mod ppse7mstid_h;
#[doc = "PPSE8MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register8_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse8mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse8mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse8mstid_l`]
module"]
#[doc(alias = "PPSE8MSTID_L")]
pub type Ppse8mstidL = crate::Reg<ppse8mstid_l::Ppse8mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register8_L"]
pub mod ppse8mstid_l;
#[doc = "PPSE8MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register8_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse8mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse8mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse8mstid_h`]
module"]
#[doc(alias = "PPSE8MSTID_H")]
pub type Ppse8mstidH = crate::Reg<ppse8mstid_h::Ppse8mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register8_H"]
pub mod ppse8mstid_h;
#[doc = "PPSE9MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register9_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse9mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse9mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse9mstid_l`]
module"]
#[doc(alias = "PPSE9MSTID_L")]
pub type Ppse9mstidL = crate::Reg<ppse9mstid_l::Ppse9mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register9_L"]
pub mod ppse9mstid_l;
#[doc = "PPSE9MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register9_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse9mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse9mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse9mstid_h`]
module"]
#[doc(alias = "PPSE9MSTID_H")]
pub type Ppse9mstidH = crate::Reg<ppse9mstid_h::Ppse9mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register9_H"]
pub mod ppse9mstid_h;
#[doc = "PPSE10MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register10_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse10mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse10mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse10mstid_l`]
module"]
#[doc(alias = "PPSE10MSTID_L")]
pub type Ppse10mstidL = crate::Reg<ppse10mstid_l::Ppse10mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register10_L"]
pub mod ppse10mstid_l;
#[doc = "PPSE10MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register10_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse10mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse10mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse10mstid_h`]
module"]
#[doc(alias = "PPSE10MSTID_H")]
pub type Ppse10mstidH = crate::Reg<ppse10mstid_h::Ppse10mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register10_H"]
pub mod ppse10mstid_h;
#[doc = "PPSE11MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register11_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse11mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse11mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse11mstid_l`]
module"]
#[doc(alias = "PPSE11MSTID_L")]
pub type Ppse11mstidL = crate::Reg<ppse11mstid_l::Ppse11mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register11_L"]
pub mod ppse11mstid_l;
#[doc = "PPSE11MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register11_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse11mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse11mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse11mstid_h`]
module"]
#[doc(alias = "PPSE11MSTID_H")]
pub type Ppse11mstidH = crate::Reg<ppse11mstid_h::Ppse11mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register11_H"]
pub mod ppse11mstid_h;
#[doc = "PPSE12MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register12_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse12mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse12mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse12mstid_l`]
module"]
#[doc(alias = "PPSE12MSTID_L")]
pub type Ppse12mstidL = crate::Reg<ppse12mstid_l::Ppse12mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register12_L"]
pub mod ppse12mstid_l;
#[doc = "PPSE12MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register12_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse12mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse12mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse12mstid_h`]
module"]
#[doc(alias = "PPSE12MSTID_H")]
pub type Ppse12mstidH = crate::Reg<ppse12mstid_h::Ppse12mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register12_H"]
pub mod ppse12mstid_h;
#[doc = "PPSE13MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register13_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse13mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse13mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse13mstid_l`]
module"]
#[doc(alias = "PPSE13MSTID_L")]
pub type Ppse13mstidL = crate::Reg<ppse13mstid_l::Ppse13mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register13_L"]
pub mod ppse13mstid_l;
#[doc = "PPSE13MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register13_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse13mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse13mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse13mstid_h`]
module"]
#[doc(alias = "PPSE13MSTID_H")]
pub type Ppse13mstidH = crate::Reg<ppse13mstid_h::Ppse13mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register13_H"]
pub mod ppse13mstid_h;
#[doc = "PPSE14MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register14_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse14mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse14mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse14mstid_l`]
module"]
#[doc(alias = "PPSE14MSTID_L")]
pub type Ppse14mstidL = crate::Reg<ppse14mstid_l::Ppse14mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register14_L"]
pub mod ppse14mstid_l;
#[doc = "PPSE14MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register14_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse14mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse14mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse14mstid_h`]
module"]
#[doc(alias = "PPSE14MSTID_H")]
pub type Ppse14mstidH = crate::Reg<ppse14mstid_h::Ppse14mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register14_H"]
pub mod ppse14mstid_h;
#[doc = "PPSE15MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register15_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse15mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse15mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse15mstid_l`]
module"]
#[doc(alias = "PPSE15MSTID_L")]
pub type Ppse15mstidL = crate::Reg<ppse15mstid_l::Ppse15mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register15_L"]
pub mod ppse15mstid_l;
#[doc = "PPSE15MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register15_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse15mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse15mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse15mstid_h`]
module"]
#[doc(alias = "PPSE15MSTID_H")]
pub type Ppse15mstidH = crate::Reg<ppse15mstid_h::Ppse15mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register15_H"]
pub mod ppse15mstid_h;
#[doc = "PPSE16MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register16_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse16mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse16mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse16mstid_l`]
module"]
#[doc(alias = "PPSE16MSTID_L")]
pub type Ppse16mstidL = crate::Reg<ppse16mstid_l::Ppse16mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register16_L"]
pub mod ppse16mstid_l;
#[doc = "PPSE16MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register16_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse16mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse16mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse16mstid_h`]
module"]
#[doc(alias = "PPSE16MSTID_H")]
pub type Ppse16mstidH = crate::Reg<ppse16mstid_h::Ppse16mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register16_H"]
pub mod ppse16mstid_h;
#[doc = "PPSE17MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register17_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse17mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse17mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse17mstid_l`]
module"]
#[doc(alias = "PPSE17MSTID_L")]
pub type Ppse17mstidL = crate::Reg<ppse17mstid_l::Ppse17mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register17_L"]
pub mod ppse17mstid_l;
#[doc = "PPSE17MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register17_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse17mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse17mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse17mstid_h`]
module"]
#[doc(alias = "PPSE17MSTID_H")]
pub type Ppse17mstidH = crate::Reg<ppse17mstid_h::Ppse17mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register17_H"]
pub mod ppse17mstid_h;
#[doc = "PPSE18MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register18_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse18mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse18mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse18mstid_l`]
module"]
#[doc(alias = "PPSE18MSTID_L")]
pub type Ppse18mstidL = crate::Reg<ppse18mstid_l::Ppse18mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register18_L"]
pub mod ppse18mstid_l;
#[doc = "PPSE18MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register18_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse18mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse18mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse18mstid_h`]
module"]
#[doc(alias = "PPSE18MSTID_H")]
pub type Ppse18mstidH = crate::Reg<ppse18mstid_h::Ppse18mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register18_H"]
pub mod ppse18mstid_h;
#[doc = "PPSE19MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register19_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse19mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse19mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse19mstid_l`]
module"]
#[doc(alias = "PPSE19MSTID_L")]
pub type Ppse19mstidL = crate::Reg<ppse19mstid_l::Ppse19mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register19_L"]
pub mod ppse19mstid_l;
#[doc = "PPSE19MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register19_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse19mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse19mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse19mstid_h`]
module"]
#[doc(alias = "PPSE19MSTID_H")]
pub type Ppse19mstidH = crate::Reg<ppse19mstid_h::Ppse19mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register19_H"]
pub mod ppse19mstid_h;
#[doc = "PPSE20MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register20_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse20mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse20mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse20mstid_l`]
module"]
#[doc(alias = "PPSE20MSTID_L")]
pub type Ppse20mstidL = crate::Reg<ppse20mstid_l::Ppse20mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register20_L"]
pub mod ppse20mstid_l;
#[doc = "PPSE20MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register20_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse20mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse20mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse20mstid_h`]
module"]
#[doc(alias = "PPSE20MSTID_H")]
pub type Ppse20mstidH = crate::Reg<ppse20mstid_h::Ppse20mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register20_H"]
pub mod ppse20mstid_h;
#[doc = "PPSE21MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register21_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse21mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse21mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse21mstid_l`]
module"]
#[doc(alias = "PPSE21MSTID_L")]
pub type Ppse21mstidL = crate::Reg<ppse21mstid_l::Ppse21mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register21_L"]
pub mod ppse21mstid_l;
#[doc = "PPSE21MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register21_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse21mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse21mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse21mstid_h`]
module"]
#[doc(alias = "PPSE21MSTID_H")]
pub type Ppse21mstidH = crate::Reg<ppse21mstid_h::Ppse21mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register21_H"]
pub mod ppse21mstid_h;
#[doc = "PPSE22MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register22_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse22mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse22mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse22mstid_l`]
module"]
#[doc(alias = "PPSE22MSTID_L")]
pub type Ppse22mstidL = crate::Reg<ppse22mstid_l::Ppse22mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register22_L"]
pub mod ppse22mstid_l;
#[doc = "PPSE22MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register22_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse22mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse22mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse22mstid_h`]
module"]
#[doc(alias = "PPSE22MSTID_H")]
pub type Ppse22mstidH = crate::Reg<ppse22mstid_h::Ppse22mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register22_H"]
pub mod ppse22mstid_h;
#[doc = "PPSE23MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register23_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse23mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse23mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse23mstid_l`]
module"]
#[doc(alias = "PPSE23MSTID_L")]
pub type Ppse23mstidL = crate::Reg<ppse23mstid_l::Ppse23mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register23_L"]
pub mod ppse23mstid_l;
#[doc = "PPSE23MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register23_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse23mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse23mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse23mstid_h`]
module"]
#[doc(alias = "PPSE23MSTID_H")]
pub type Ppse23mstidH = crate::Reg<ppse23mstid_h::Ppse23mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register23_H"]
pub mod ppse23mstid_h;
#[doc = "PPSE24MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register24_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse24mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse24mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse24mstid_l`]
module"]
#[doc(alias = "PPSE24MSTID_L")]
pub type Ppse24mstidL = crate::Reg<ppse24mstid_l::Ppse24mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register24_L"]
pub mod ppse24mstid_l;
#[doc = "PPSE24MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register24_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse24mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse24mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse24mstid_h`]
module"]
#[doc(alias = "PPSE24MSTID_H")]
pub type Ppse24mstidH = crate::Reg<ppse24mstid_h::Ppse24mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register24_H"]
pub mod ppse24mstid_h;
#[doc = "PPSE25MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register25_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse25mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse25mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse25mstid_l`]
module"]
#[doc(alias = "PPSE25MSTID_L")]
pub type Ppse25mstidL = crate::Reg<ppse25mstid_l::Ppse25mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register25_L"]
pub mod ppse25mstid_l;
#[doc = "PPSE25MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register25_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse25mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse25mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse25mstid_h`]
module"]
#[doc(alias = "PPSE25MSTID_H")]
pub type Ppse25mstidH = crate::Reg<ppse25mstid_h::Ppse25mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register25_H"]
pub mod ppse25mstid_h;
#[doc = "PPSE26MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register26_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse26mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse26mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse26mstid_l`]
module"]
#[doc(alias = "PPSE26MSTID_L")]
pub type Ppse26mstidL = crate::Reg<ppse26mstid_l::Ppse26mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register26_L"]
pub mod ppse26mstid_l;
#[doc = "PPSE26MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register26_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse26mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse26mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse26mstid_h`]
module"]
#[doc(alias = "PPSE26MSTID_H")]
pub type Ppse26mstidH = crate::Reg<ppse26mstid_h::Ppse26mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register26_H"]
pub mod ppse26mstid_h;
#[doc = "PPSE27MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register27_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse27mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse27mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse27mstid_l`]
module"]
#[doc(alias = "PPSE27MSTID_L")]
pub type Ppse27mstidL = crate::Reg<ppse27mstid_l::Ppse27mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register27_L"]
pub mod ppse27mstid_l;
#[doc = "PPSE27MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register27_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse27mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse27mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse27mstid_h`]
module"]
#[doc(alias = "PPSE27MSTID_H")]
pub type Ppse27mstidH = crate::Reg<ppse27mstid_h::Ppse27mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register27_H"]
pub mod ppse27mstid_h;
#[doc = "PPSE28MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register28_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse28mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse28mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse28mstid_l`]
module"]
#[doc(alias = "PPSE28MSTID_L")]
pub type Ppse28mstidL = crate::Reg<ppse28mstid_l::Ppse28mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register28_L"]
pub mod ppse28mstid_l;
#[doc = "PPSE28MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register28_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse28mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse28mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse28mstid_h`]
module"]
#[doc(alias = "PPSE28MSTID_H")]
pub type Ppse28mstidH = crate::Reg<ppse28mstid_h::Ppse28mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register28_H"]
pub mod ppse28mstid_h;
#[doc = "PPSE29MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register29_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse29mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse29mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse29mstid_l`]
module"]
#[doc(alias = "PPSE29MSTID_L")]
pub type Ppse29mstidL = crate::Reg<ppse29mstid_l::Ppse29mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register29_L"]
pub mod ppse29mstid_l;
#[doc = "PPSE29MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register29_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse29mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse29mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse29mstid_h`]
module"]
#[doc(alias = "PPSE29MSTID_H")]
pub type Ppse29mstidH = crate::Reg<ppse29mstid_h::Ppse29mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register29_H"]
pub mod ppse29mstid_h;
#[doc = "PPSE30MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register30_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse30mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse30mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse30mstid_l`]
module"]
#[doc(alias = "PPSE30MSTID_L")]
pub type Ppse30mstidL = crate::Reg<ppse30mstid_l::Ppse30mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register30_L"]
pub mod ppse30mstid_l;
#[doc = "PPSE30MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register30_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse30mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse30mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse30mstid_h`]
module"]
#[doc(alias = "PPSE30MSTID_H")]
pub type Ppse30mstidH = crate::Reg<ppse30mstid_h::Ppse30mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register30_H"]
pub mod ppse30mstid_h;
#[doc = "PPSE31MSTID_L (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register31_L\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse31mstid_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse31mstid_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse31mstid_l`]
module"]
#[doc(alias = "PPSE31MSTID_L")]
pub type Ppse31mstidL = crate::Reg<ppse31mstid_l::Ppse31mstidLSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register31_L"]
pub mod ppse31mstid_l;
#[doc = "PPSE31MSTID_H (rw) register accessor: Privileged Peripheral Extended Frame Master-ID Protection Register31_H\n\nYou can [`read`](crate::Reg::read) this register and get [`ppse31mstid_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppse31mstid_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppse31mstid_h`]
module"]
#[doc(alias = "PPSE31MSTID_H")]
pub type Ppse31mstidH = crate::Reg<ppse31mstid_h::Ppse31mstidHSpec>;
#[doc = "Privileged Peripheral Extended Frame Master-ID Protection Register31_H"]
pub mod ppse31mstid_h;
#[doc = "PCS0MSTID (rw) register accessor: Memory Frame Master ID Protection Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs0mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs0mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs0mstid`]
module"]
#[doc(alias = "PCS0MSTID")]
pub type Pcs0mstid = crate::Reg<pcs0mstid::Pcs0mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register0"]
pub mod pcs0mstid;
#[doc = "PCS1MSTID (rw) register accessor: Memory Frame Master ID Protection Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs1mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs1mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs1mstid`]
module"]
#[doc(alias = "PCS1MSTID")]
pub type Pcs1mstid = crate::Reg<pcs1mstid::Pcs1mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register1"]
pub mod pcs1mstid;
#[doc = "PCS2MSTID (rw) register accessor: Memory Frame Master ID Protection Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs2mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs2mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs2mstid`]
module"]
#[doc(alias = "PCS2MSTID")]
pub type Pcs2mstid = crate::Reg<pcs2mstid::Pcs2mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register2"]
pub mod pcs2mstid;
#[doc = "PCS3MSTID (rw) register accessor: Memory Frame Master ID Protection Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs3mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs3mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs3mstid`]
module"]
#[doc(alias = "PCS3MSTID")]
pub type Pcs3mstid = crate::Reg<pcs3mstid::Pcs3mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register3"]
pub mod pcs3mstid;
#[doc = "PCS4MSTID (rw) register accessor: Memory Frame Master ID Protection Register4\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs4mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs4mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs4mstid`]
module"]
#[doc(alias = "PCS4MSTID")]
pub type Pcs4mstid = crate::Reg<pcs4mstid::Pcs4mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register4"]
pub mod pcs4mstid;
#[doc = "PCS5MSTID (rw) register accessor: Memory Frame Master ID Protection Register5\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs5mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs5mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs5mstid`]
module"]
#[doc(alias = "PCS5MSTID")]
pub type Pcs5mstid = crate::Reg<pcs5mstid::Pcs5mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register5"]
pub mod pcs5mstid;
#[doc = "PCS6MSTID (rw) register accessor: Memory Frame Master ID Protection Register6\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs6mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs6mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs6mstid`]
module"]
#[doc(alias = "PCS6MSTID")]
pub type Pcs6mstid = crate::Reg<pcs6mstid::Pcs6mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register6"]
pub mod pcs6mstid;
#[doc = "PCS7MSTID (rw) register accessor: Memory Frame Master ID Protection Register7\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs7mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs7mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs7mstid`]
module"]
#[doc(alias = "PCS7MSTID")]
pub type Pcs7mstid = crate::Reg<pcs7mstid::Pcs7mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register7"]
pub mod pcs7mstid;
#[doc = "PCS8MSTID (rw) register accessor: Memory Frame Master ID Protection Register8\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs8mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs8mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs8mstid`]
module"]
#[doc(alias = "PCS8MSTID")]
pub type Pcs8mstid = crate::Reg<pcs8mstid::Pcs8mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register8"]
pub mod pcs8mstid;
#[doc = "PCS9MSTID (rw) register accessor: Memory Frame Master ID Protection Register9\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs9mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs9mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs9mstid`]
module"]
#[doc(alias = "PCS9MSTID")]
pub type Pcs9mstid = crate::Reg<pcs9mstid::Pcs9mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register9"]
pub mod pcs9mstid;
#[doc = "PCS10MSTID (rw) register accessor: Memory Frame Master ID Protection Register10\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs10mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs10mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs10mstid`]
module"]
#[doc(alias = "PCS10MSTID")]
pub type Pcs10mstid = crate::Reg<pcs10mstid::Pcs10mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register10"]
pub mod pcs10mstid;
#[doc = "PCS11MSTID (rw) register accessor: Memory Frame Master ID Protection Register11\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs11mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs11mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs11mstid`]
module"]
#[doc(alias = "PCS11MSTID")]
pub type Pcs11mstid = crate::Reg<pcs11mstid::Pcs11mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register11"]
pub mod pcs11mstid;
#[doc = "PCS12MSTID (rw) register accessor: Memory Frame Master ID Protection Register12\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs12mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs12mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs12mstid`]
module"]
#[doc(alias = "PCS12MSTID")]
pub type Pcs12mstid = crate::Reg<pcs12mstid::Pcs12mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register12"]
pub mod pcs12mstid;
#[doc = "PCS13MSTID (rw) register accessor: Memory Frame Master ID Protection Register13\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs13mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs13mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs13mstid`]
module"]
#[doc(alias = "PCS13MSTID")]
pub type Pcs13mstid = crate::Reg<pcs13mstid::Pcs13mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register13"]
pub mod pcs13mstid;
#[doc = "PCS14MSTID (rw) register accessor: Memory Frame Master ID Protection Register14\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs14mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs14mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs14mstid`]
module"]
#[doc(alias = "PCS14MSTID")]
pub type Pcs14mstid = crate::Reg<pcs14mstid::Pcs14mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register14"]
pub mod pcs14mstid;
#[doc = "PCS15MSTID (rw) register accessor: Memory Frame Master ID Protection Register15\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs15mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs15mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs15mstid`]
module"]
#[doc(alias = "PCS15MSTID")]
pub type Pcs15mstid = crate::Reg<pcs15mstid::Pcs15mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register15"]
pub mod pcs15mstid;
#[doc = "PCS16MSTID (rw) register accessor: Memory Frame Master ID Protection Register16\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs16mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs16mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs16mstid`]
module"]
#[doc(alias = "PCS16MSTID")]
pub type Pcs16mstid = crate::Reg<pcs16mstid::Pcs16mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register16"]
pub mod pcs16mstid;
#[doc = "PCS17MSTID (rw) register accessor: Memory Frame Master ID Protection Register17\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs17mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs17mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs17mstid`]
module"]
#[doc(alias = "PCS17MSTID")]
pub type Pcs17mstid = crate::Reg<pcs17mstid::Pcs17mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register17"]
pub mod pcs17mstid;
#[doc = "PCS18MSTID (rw) register accessor: Memory Frame Master ID Protection Register18\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs18mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs18mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs18mstid`]
module"]
#[doc(alias = "PCS18MSTID")]
pub type Pcs18mstid = crate::Reg<pcs18mstid::Pcs18mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register18"]
pub mod pcs18mstid;
#[doc = "PCS19MSTID (rw) register accessor: Memory Frame Master ID Protection Register19\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs19mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs19mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs19mstid`]
module"]
#[doc(alias = "PCS19MSTID")]
pub type Pcs19mstid = crate::Reg<pcs19mstid::Pcs19mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register19"]
pub mod pcs19mstid;
#[doc = "PCS20MSTID (rw) register accessor: Memory Frame Master ID Protection Register20\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs20mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs20mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs20mstid`]
module"]
#[doc(alias = "PCS20MSTID")]
pub type Pcs20mstid = crate::Reg<pcs20mstid::Pcs20mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register20"]
pub mod pcs20mstid;
#[doc = "PCS21MSTID (rw) register accessor: Memory Frame Master ID Protection Register21\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs21mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs21mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs21mstid`]
module"]
#[doc(alias = "PCS21MSTID")]
pub type Pcs21mstid = crate::Reg<pcs21mstid::Pcs21mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register21"]
pub mod pcs21mstid;
#[doc = "PCS22MSTID (rw) register accessor: Memory Frame Master ID Protection Register22\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs22mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs22mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs22mstid`]
module"]
#[doc(alias = "PCS22MSTID")]
pub type Pcs22mstid = crate::Reg<pcs22mstid::Pcs22mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register22"]
pub mod pcs22mstid;
#[doc = "PCS23MSTID (rw) register accessor: Memory Frame Master ID Protection Register23\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs23mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs23mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs23mstid`]
module"]
#[doc(alias = "PCS23MSTID")]
pub type Pcs23mstid = crate::Reg<pcs23mstid::Pcs23mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register23"]
pub mod pcs23mstid;
#[doc = "PCS24MSTID (rw) register accessor: Memory Frame Master ID Protection Register24\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs24mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs24mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs24mstid`]
module"]
#[doc(alias = "PCS24MSTID")]
pub type Pcs24mstid = crate::Reg<pcs24mstid::Pcs24mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register24"]
pub mod pcs24mstid;
#[doc = "PCS25MSTID (rw) register accessor: Memory Frame Master ID Protection Register25\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs25mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs25mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs25mstid`]
module"]
#[doc(alias = "PCS25MSTID")]
pub type Pcs25mstid = crate::Reg<pcs25mstid::Pcs25mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register25"]
pub mod pcs25mstid;
#[doc = "PCS26MSTID (rw) register accessor: Memory Frame Master ID Protection Register26\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs26mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs26mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs26mstid`]
module"]
#[doc(alias = "PCS26MSTID")]
pub type Pcs26mstid = crate::Reg<pcs26mstid::Pcs26mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register26"]
pub mod pcs26mstid;
#[doc = "PCS27MSTID (rw) register accessor: Memory Frame Master ID Protection Register27\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs27mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs27mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs27mstid`]
module"]
#[doc(alias = "PCS27MSTID")]
pub type Pcs27mstid = crate::Reg<pcs27mstid::Pcs27mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register27"]
pub mod pcs27mstid;
#[doc = "PCS28MSTID (rw) register accessor: Memory Frame Master ID Protection Register28\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs28mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs28mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs28mstid`]
module"]
#[doc(alias = "PCS28MSTID")]
pub type Pcs28mstid = crate::Reg<pcs28mstid::Pcs28mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register28"]
pub mod pcs28mstid;
#[doc = "PCS29MSTID (rw) register accessor: Memory Frame Master ID Protection Register29\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs29mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs29mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs29mstid`]
module"]
#[doc(alias = "PCS29MSTID")]
pub type Pcs29mstid = crate::Reg<pcs29mstid::Pcs29mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register29"]
pub mod pcs29mstid;
#[doc = "PCS30MSTID (rw) register accessor: Memory Frame Master ID Protection Register30\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs30mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs30mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs30mstid`]
module"]
#[doc(alias = "PCS30MSTID")]
pub type Pcs30mstid = crate::Reg<pcs30mstid::Pcs30mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register30"]
pub mod pcs30mstid;
#[doc = "PCS31MSTID (rw) register accessor: Memory Frame Master ID Protection Register31\n\nYou can [`read`](crate::Reg::read) this register and get [`pcs31mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcs31mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcs31mstid`]
module"]
#[doc(alias = "PCS31MSTID")]
pub type Pcs31mstid = crate::Reg<pcs31mstid::Pcs31mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register31"]
pub mod pcs31mstid;
#[doc = "PPCS0MSTID (rw) register accessor: Memory Frame Master ID Protection Register32\n\nYou can [`read`](crate::Reg::read) this register and get [`ppcs0mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppcs0mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcs0mstid`]
module"]
#[doc(alias = "PPCS0MSTID")]
pub type Ppcs0mstid = crate::Reg<ppcs0mstid::Ppcs0mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register32"]
pub mod ppcs0mstid;
#[doc = "PPCS1MSTID (rw) register accessor: Memory Frame Master ID Protection Register33\n\nYou can [`read`](crate::Reg::read) this register and get [`ppcs1mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppcs1mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcs1mstid`]
module"]
#[doc(alias = "PPCS1MSTID")]
pub type Ppcs1mstid = crate::Reg<ppcs1mstid::Ppcs1mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register33"]
pub mod ppcs1mstid;
#[doc = "PPCS2MSTID (rw) register accessor: Memory Frame Master ID Protection Register34\n\nYou can [`read`](crate::Reg::read) this register and get [`ppcs2mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppcs2mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcs2mstid`]
module"]
#[doc(alias = "PPCS2MSTID")]
pub type Ppcs2mstid = crate::Reg<ppcs2mstid::Ppcs2mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register34"]
pub mod ppcs2mstid;
#[doc = "PPCS3MSTID (rw) register accessor: Memory Frame Master ID Protection Register35\n\nYou can [`read`](crate::Reg::read) this register and get [`ppcs3mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppcs3mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcs3mstid`]
module"]
#[doc(alias = "PPCS3MSTID")]
pub type Ppcs3mstid = crate::Reg<ppcs3mstid::Ppcs3mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register35"]
pub mod ppcs3mstid;
#[doc = "PPCS4MSTID (rw) register accessor: Memory Frame Master ID Protection Register36\n\nYou can [`read`](crate::Reg::read) this register and get [`ppcs4mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppcs4mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcs4mstid`]
module"]
#[doc(alias = "PPCS4MSTID")]
pub type Ppcs4mstid = crate::Reg<ppcs4mstid::Ppcs4mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register36"]
pub mod ppcs4mstid;
#[doc = "PPCS5MSTID (rw) register accessor: Memory Frame Master ID Protection Register37\n\nYou can [`read`](crate::Reg::read) this register and get [`ppcs5mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppcs5mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcs5mstid`]
module"]
#[doc(alias = "PPCS5MSTID")]
pub type Ppcs5mstid = crate::Reg<ppcs5mstid::Ppcs5mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register37"]
pub mod ppcs5mstid;
#[doc = "PPCS6MSTID (rw) register accessor: Memory Frame Master ID Protection Register38\n\nYou can [`read`](crate::Reg::read) this register and get [`ppcs6mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppcs6mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcs6mstid`]
module"]
#[doc(alias = "PPCS6MSTID")]
pub type Ppcs6mstid = crate::Reg<ppcs6mstid::Ppcs6mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register38"]
pub mod ppcs6mstid;
#[doc = "PPCS7MSTID (rw) register accessor: Memory Frame Master ID Protection Register39\n\nYou can [`read`](crate::Reg::read) this register and get [`ppcs7mstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppcs7mstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcs7mstid`]
module"]
#[doc(alias = "PPCS7MSTID")]
pub type Ppcs7mstid = crate::Reg<ppcs7mstid::Ppcs7mstidSpec>;
#[doc = "Memory Frame Master ID Protection Register39"]
pub mod ppcs7mstid;
#[doc = "PCREXTMSTID (rw) register accessor: Master-ID Protection Register for external PCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrextmstid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrextmstid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrextmstid`]
module"]
#[doc(alias = "PCREXTMSTID")]
pub type Pcrextmstid = crate::Reg<pcrextmstid::PcrextmstidSpec>;
#[doc = "Master-ID Protection Register for external PCR"]
pub mod pcrextmstid;

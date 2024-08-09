#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spigcr0: Spigcr0,
    spigcr1: Spigcr1,
    spiint0: Spiint0,
    spilvl: Spilvl,
    spiflg: Spiflg,
    spipc0: Spipc0,
    spipc1: Spipc1,
    spipc2: Spipc2,
    spipc3: Spipc3,
    spipc4: Spipc4,
    spipc5: Spipc5,
    spipc6: Spipc6,
    _reserved12: [u8; 0x08],
    spidat0: Spidat0,
    spidat1: Spidat1,
    spibuf: Spibuf,
    spiemu: Spiemu,
    spidelay: Spidelay,
    spidef: Spidef,
    spifmt0: Spifmt0,
    spifmt1: Spifmt1,
    spifmt2: Spifmt2,
    spifmt3: Spifmt3,
    tgintvect0: Tgintvect0,
    tgintvect1: Tgintvect1,
    spipc9: Spipc9,
    spipmctrl: Spipmctrl,
    mibspie: Mibspie,
    tgitenst: Tgitenst,
    tgitencr: Tgitencr,
    tgitlvst: Tgitlvst,
    tgitlvcr: Tgitlvcr,
    tgintflag: Tgintflag,
    _reserved32: [u8; 0x08],
    tickcnt: Tickcnt,
    ltgpend: Ltgpend,
    tg0ctrl: Tg0ctrl,
    tg1ctrl: Tg1ctrl,
    tg2ctrl: Tg2ctrl,
    tg3ctrl: Tg3ctrl,
    tg4ctrl: Tg4ctrl,
    tg5ctrl: Tg5ctrl,
    tg6ctrl: Tg6ctrl,
    tg7ctrl: Tg7ctrl,
    _reserved42: [u8; 0x20],
    dma0ctrl: Dma0ctrl,
    dma1ctrl: Dma1ctrl,
    dma2ctrl: Dma2ctrl,
    dma3ctrl: Dma3ctrl,
    dma4ctrl: Dma4ctrl,
    _reserved47: [u8; 0x0c],
    icount0: Icount0,
    icount1: Icount1,
    icount2: Icount2,
    _reserved50: [u8; 0x02],
    icount3: Icount3,
    icount4: Icount4,
    _reserved52: [u8; 0x0c],
    dmacntlen: Dmacntlen,
    _reserved53: [u8; 0x04],
    par_ecc_ctrl: ParEccCtrl,
    par_ecc_stat: ParEccStat,
    uerraddr1: Uerraddr1,
    uerraddr0: Uerraddr0,
    rxovrn_buf_addr: RxovrnBufAddr,
    iolpbktstcr: Iolpbktstcr,
    extended_prescale1: ExtendedPrescale1,
    extended_prescale2: ExtendedPrescale2,
    eccdiag_ctrl: EccdiagCtrl,
    eccdiag_stat: EccdiagStat,
    sberraddr1: Sberraddr1,
    sberraddr0: Sberraddr0,
    _reserved65: [u8; 0xac],
    spirev: Spirev,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI / MibSPI Global Control Register 0"]
    #[inline(always)]
    pub const fn spigcr0(&self) -> &Spigcr0 {
        &self.spigcr0
    }
    #[doc = "0x04 - SPI / MibSPI Global control register 1"]
    #[inline(always)]
    pub const fn spigcr1(&self) -> &Spigcr1 {
        &self.spigcr1
    }
    #[doc = "0x08 - SPI / MibSPI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn spiint0(&self) -> &Spiint0 {
        &self.spiint0
    }
    #[doc = "0x0c - SPI / MibSPI Interrupt Level Register"]
    #[inline(always)]
    pub const fn spilvl(&self) -> &Spilvl {
        &self.spilvl
    }
    #[doc = "0x10 - SPI / MibSPI Flag Register"]
    #[inline(always)]
    pub const fn spiflg(&self) -> &Spiflg {
        &self.spiflg
    }
    #[doc = "0x14 - SPI / MibSPI Pin Control Register 0 (SPIPC0) - SPIFUN Note: Duplicate Control Bits for SIMO0 &amp; SOMI0 Bit 24 is not physically implemented. it is a mirror of Bit11. Any write to Bit 24 will be reflected on Bit11 and when Bit 24 &amp; Bit 11 simultaneously written, the value of Bit11 will control the SOMI pin. Read value of Bit 24 always reflects the Bit 11 value. This is true for the Bit 24 &amp; Bit 11 of all of SPIPC0 to SPIPC9 registers. Same is true for SIMO pin with Bit16 &amp; Bit 10 of SPIPC0 to SPIPC9 registers."]
    #[inline(always)]
    pub const fn spipc0(&self) -> &Spipc0 {
        &self.spipc0
    }
    #[doc = "0x18 - SPI / MibSPI Pin Control Register 1 (SPIPC1) - SPIDIR"]
    #[inline(always)]
    pub const fn spipc1(&self) -> &Spipc1 {
        &self.spipc1
    }
    #[doc = "0x1c - SPI / MibSPI Pin Control Register 2 (SPIPC2) - SPIDIN"]
    #[inline(always)]
    pub const fn spipc2(&self) -> &Spipc2 {
        &self.spipc2
    }
    #[doc = "0x20 - SPI / MibSPI Pin Control Register 3 (SPIPC3) - SPIDOUT"]
    #[inline(always)]
    pub const fn spipc3(&self) -> &Spipc3 {
        &self.spipc3
    }
    #[doc = "0x24 - SPI / MibSPI Pin Control Register 4 (SPIPC4) - SPIDSET"]
    #[inline(always)]
    pub const fn spipc4(&self) -> &Spipc4 {
        &self.spipc4
    }
    #[doc = "0x28 - SPI / MibSPI Pin Control Register 5 (SPIPC5) - SPIDCLR"]
    #[inline(always)]
    pub const fn spipc5(&self) -> &Spipc5 {
        &self.spipc5
    }
    #[doc = "0x2c - SPI / MibSPI Pin Control Register 6 (SPIPC6) - SPIPDR"]
    #[inline(always)]
    pub const fn spipc6(&self) -> &Spipc6 {
        &self.spipc6
    }
    #[doc = "0x38 - SPI / MibSPI Transmit Data Register 0 Note: Accessibility of SPIDAT0 The SPIDAT0 register is not accessible in Multibuffer Mode of MibSPI. It is only accessible in compatibility mode."]
    #[inline(always)]
    pub const fn spidat0(&self) -> &Spidat0 {
        &self.spidat0
    }
    #[doc = "0x3c - SPI / MibSPI Transmit Data Register 1 When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned."]
    #[inline(always)]
    pub const fn spidat1(&self) -> &Spidat1 {
        &self.spidat1
    }
    #[doc = "0x40 - SPI / MibSPI Receive Buffer Register"]
    #[inline(always)]
    pub const fn spibuf(&self) -> &Spibuf {
        &self.spibuf
    }
    #[doc = "0x44 - SPI / MibSPI Emulation Register Note: All the fields ot SPIEMU register are Read-Only. Read operation on this register under any mode will not have any impact on the status of this or any other registers."]
    #[inline(always)]
    pub const fn spiemu(&self) -> &Spiemu {
        &self.spiemu
    }
    #[doc = "0x48 - SPI / MibSPI Delay Register"]
    #[inline(always)]
    pub const fn spidelay(&self) -> &Spidelay {
        &self.spidelay
    }
    #[doc = "0x4c - SPI / MibSPI Default Chip select Register"]
    #[inline(always)]
    pub const fn spidef(&self) -> &Spidef {
        &self.spidef
    }
    #[doc = "0x50 - SPI / MibSPI Data Format Register 0"]
    #[inline(always)]
    pub const fn spifmt0(&self) -> &Spifmt0 {
        &self.spifmt0
    }
    #[doc = "0x54 - SPI / MibSPI Data Format Register 1"]
    #[inline(always)]
    pub const fn spifmt1(&self) -> &Spifmt1 {
        &self.spifmt1
    }
    #[doc = "0x58 - SPI / MibSPI Data Format Register 2"]
    #[inline(always)]
    pub const fn spifmt2(&self) -> &Spifmt2 {
        &self.spifmt2
    }
    #[doc = "0x5c - SPI / MibSPI Data Format Register 3"]
    #[inline(always)]
    pub const fn spifmt3(&self) -> &Spifmt3 {
        &self.spifmt3
    }
    #[doc = "0x60 - SPI Interrupt Vector Register 0 / MibSPI Transfer Group Interrupt Vector Register 0"]
    #[inline(always)]
    pub const fn tgintvect0(&self) -> &Tgintvect0 {
        &self.tgintvect0
    }
    #[doc = "0x64 - SPI Interrupt Vector Register 1 / MibSPI Transfer Group Interrupt Vector Register 1"]
    #[inline(always)]
    pub const fn tgintvect1(&self) -> &Tgintvect1 {
        &self.tgintvect1
    }
    #[doc = "0x68 - SPI/MibSPI Pin Control Register 9 (SPIPC9) - SPISRSEL"]
    #[inline(always)]
    pub const fn spipc9(&self) -> &Spipc9 {
        &self.spipc9
    }
    #[doc = "0x6c - SPI/MibSPI Parallel/Modulo Mode Control Register"]
    #[inline(always)]
    pub const fn spipmctrl(&self) -> &Spipmctrl {
        &self.spipmctrl
    }
    #[doc = "0x70 - MibSPI Enable Register"]
    #[inline(always)]
    pub const fn mibspie(&self) -> &Mibspie {
        &self.mibspie
    }
    #[doc = "0x74 - MibSPI Transfer Group Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn tgitenst(&self) -> &Tgitenst {
        &self.tgitenst
    }
    #[doc = "0x78 - MibSPI Transfer Group Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn tgitencr(&self) -> &Tgitencr {
        &self.tgitencr
    }
    #[doc = "0x7c - MibSPI Transfer Group Interrupt Level Set Register"]
    #[inline(always)]
    pub const fn tgitlvst(&self) -> &Tgitlvst {
        &self.tgitlvst
    }
    #[doc = "0x80 - MibSPI Transfer Group Interrupt Level Clear Register"]
    #[inline(always)]
    pub const fn tgitlvcr(&self) -> &Tgitlvcr {
        &self.tgitlvcr
    }
    #[doc = "0x84 - Transfer Group Interrupt Flag Register"]
    #[inline(always)]
    pub const fn tgintflag(&self) -> &Tgintflag {
        &self.tgintflag
    }
    #[doc = "0x90 - Tick Count Register"]
    #[inline(always)]
    pub const fn tickcnt(&self) -> &Tickcnt {
        &self.tickcnt
    }
    #[doc = "0x94 - Last Transfer Group End Pointer"]
    #[inline(always)]
    pub const fn ltgpend(&self) -> &Ltgpend {
        &self.ltgpend
    }
    #[doc = "0x98 - MibSPI Transfer Group Control Register The number of transfer groups is scalable by design up to a maximum of 16. Depending on the implementation the number of transfer groups and hence the number of transfer group control register may vary. Each transfer group can be configured via one dedicated control register. The register description below shows one exemplary control register(x) which is identical for all transfer groups. E.g. the control register for transfer group 2 is named ΓÇ£TG2CTRLΓÇ¥ and is located at address base0+98h+4*2."]
    #[inline(always)]
    pub const fn tg0ctrl(&self) -> &Tg0ctrl {
        &self.tg0ctrl
    }
    #[doc = "0x9c - MibSPI Transfer Group Control Register"]
    #[inline(always)]
    pub const fn tg1ctrl(&self) -> &Tg1ctrl {
        &self.tg1ctrl
    }
    #[doc = "0xa0 - MibSPI Transfer Group Control Register"]
    #[inline(always)]
    pub const fn tg2ctrl(&self) -> &Tg2ctrl {
        &self.tg2ctrl
    }
    #[doc = "0xa4 - MibSPI Transfer Group Control Register"]
    #[inline(always)]
    pub const fn tg3ctrl(&self) -> &Tg3ctrl {
        &self.tg3ctrl
    }
    #[doc = "0xa8 - MibSPI Transfer Group Control Register"]
    #[inline(always)]
    pub const fn tg4ctrl(&self) -> &Tg4ctrl {
        &self.tg4ctrl
    }
    #[doc = "0xac - MibSPI Transfer Group Control Register"]
    #[inline(always)]
    pub const fn tg5ctrl(&self) -> &Tg5ctrl {
        &self.tg5ctrl
    }
    #[doc = "0xb0 - MibSPI Transfer Group Control Register"]
    #[inline(always)]
    pub const fn tg6ctrl(&self) -> &Tg6ctrl {
        &self.tg6ctrl
    }
    #[doc = "0xb4 - MibSPI Transfer Group Control Register"]
    #[inline(always)]
    pub const fn tg7ctrl(&self) -> &Tg7ctrl {
        &self.tg7ctrl
    }
    #[doc = "0xd8 - MibSPI DMA Channel Control Register"]
    #[inline(always)]
    pub const fn dma0ctrl(&self) -> &Dma0ctrl {
        &self.dma0ctrl
    }
    #[doc = "0xdc - MibSPI DMA Channel Control Register"]
    #[inline(always)]
    pub const fn dma1ctrl(&self) -> &Dma1ctrl {
        &self.dma1ctrl
    }
    #[doc = "0xe0 - MibSPI DMA Channel Control Register"]
    #[inline(always)]
    pub const fn dma2ctrl(&self) -> &Dma2ctrl {
        &self.dma2ctrl
    }
    #[doc = "0xe4 - MibSPI DMA Channel Control Register"]
    #[inline(always)]
    pub const fn dma3ctrl(&self) -> &Dma3ctrl {
        &self.dma3ctrl
    }
    #[doc = "0xe8 - MibSPI DMA Channel Control Register"]
    #[inline(always)]
    pub const fn dma4ctrl(&self) -> &Dma4ctrl {
        &self.dma4ctrl
    }
    #[doc = "0xf8 - MibSPI DMAxCOUNT"]
    #[inline(always)]
    pub const fn icount0(&self) -> &Icount0 {
        &self.icount0
    }
    #[doc = "0xfc - MibSPI DMAxCOUNT"]
    #[inline(always)]
    pub const fn icount1(&self) -> &Icount1 {
        &self.icount1
    }
    #[doc = "0x100 - MibSPI DMAxCOUNT"]
    #[inline(always)]
    pub const fn icount2(&self) -> &Icount2 {
        &self.icount2
    }
    #[doc = "0x104 - MibSPI DMAxCOUNT"]
    #[inline(always)]
    pub const fn icount3(&self) -> &Icount3 {
        &self.icount3
    }
    #[doc = "0x108 - MibSPI DMAxCOUNT"]
    #[inline(always)]
    pub const fn icount4(&self) -> &Icount4 {
        &self.icount4
    }
    #[doc = "0x118 - DMA LARGE COUNT register"]
    #[inline(always)]
    pub const fn dmacntlen(&self) -> &Dmacntlen {
        &self.dmacntlen
    }
    #[doc = "0x120 - Parity/ECC Control Register"]
    #[inline(always)]
    pub const fn par_ecc_ctrl(&self) -> &ParEccCtrl {
        &self.par_ecc_ctrl
    }
    #[doc = "0x124 - Parity/ECC Status Register"]
    #[inline(always)]
    pub const fn par_ecc_stat(&self) -> &ParEccStat {
        &self.par_ecc_stat
    }
    #[doc = "0x128 - Uncorrectable Parity or double bit ECC error Address Register - RXRAM"]
    #[inline(always)]
    pub const fn uerraddr1(&self) -> &Uerraddr1 {
        &self.uerraddr1
    }
    #[doc = "0x12c - Uncorrectable Parity or double bit ECC error address register - TXRAM"]
    #[inline(always)]
    pub const fn uerraddr0(&self) -> &Uerraddr0 {
        &self.uerraddr0
    }
    #[doc = "0x130 - Receive RAM Overrun Buffer Address Register"]
    #[inline(always)]
    pub const fn rxovrn_buf_addr(&self) -> &RxovrnBufAddr {
        &self.rxovrn_buf_addr
    }
    #[doc = "0x134 - SPI/MibSPI IO Loopback Test Control Register This register controls test mode for I/O pins. It also controls whether loop-back should be digital or analog ones in this test mode. In addition it contains control bits to induce some of the error condition into the module. These are to be used for test purpose only. All the control/status bits in this register are valid only when IO LPBK TST ENA field is set to ΓÇ£1010ΓÇ¥."]
    #[inline(always)]
    pub const fn iolpbktstcr(&self) -> &Iolpbktstcr {
        &self.iolpbktstcr
    }
    #[doc = "0x138 - SPI/MibSPI Extended Prescale Register 1 (EXTENDED_PRESCALE1 for SPIFMT0 and SPIFMT1) This register provides an extended Prescale values for SPICLK generation to be able to interface with much slower SPI Slaves. This is an extension of SPIFMT0 and SPIFMT1 registers. For example, EPRESCALE_FMT1(7:0) of EXTENDED_PRESCALE1 and PRESCALE1(7:0) of SPIFMT1 register will always reflect the same contents. Similarly EPRESCALE_FMT0(7:0) and PRESCALE0(7:0) of SPIFMT0 reflect the same contents."]
    #[inline(always)]
    pub const fn extended_prescale1(&self) -> &ExtendedPrescale1 {
        &self.extended_prescale1
    }
    #[doc = "0x13c - SPI/MibSPI Extended Prescale Register 2 (EXTENDED_PRESCALE2 for SPIFMT2 and SPIFMT3) This register provides an extended Prescale values for SPICLK generation to be able to interface with much slower SPI Slaves. This register is an extension of SPIFMT2 and SPIFMT3 registers. For example, EPRESCALE_FMT2(7:0) of EXTENDED_PRESCALE2 and PRESCALE2(7:0) of SPIFMT2 register will always reflect the same contents. Similarly EPRESCALE_FMT3(7:0) and PRESCALE3(7:0) of SPIFMT3 reflect the same contents."]
    #[inline(always)]
    pub const fn extended_prescale2(&self) -> &ExtendedPrescale2 {
        &self.extended_prescale2
    }
    #[doc = "0x140 - ECC Diagnostic Control register"]
    #[inline(always)]
    pub const fn eccdiag_ctrl(&self) -> &EccdiagCtrl {
        &self.eccdiag_ctrl
    }
    #[doc = "0x144 - ECC Diagnostic Status register"]
    #[inline(always)]
    pub const fn eccdiag_stat(&self) -> &EccdiagStat {
        &self.eccdiag_stat
    }
    #[doc = "0x148 - Single Bit Error Address Register - RXRAM"]
    #[inline(always)]
    pub const fn sberraddr1(&self) -> &Sberraddr1 {
        &self.sberraddr1
    }
    #[doc = "0x14c - Single Bit ECC Error Address Register - TXRAM"]
    #[inline(always)]
    pub const fn sberraddr0(&self) -> &Sberraddr0 {
        &self.sberraddr0
    }
    #[doc = "0x1fc - SPI / MibSPI Revision ID Register"]
    #[inline(always)]
    pub const fn spirev(&self) -> &Spirev {
        &self.spirev
    }
}
#[doc = "SPIGCR0 (rw) register accessor: SPI / MibSPI Global Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spigcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spigcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spigcr0`]
module"]
#[doc(alias = "SPIGCR0")]
pub type Spigcr0 = crate::Reg<spigcr0::Spigcr0Spec>;
#[doc = "SPI / MibSPI Global Control Register 0"]
pub mod spigcr0;
#[doc = "SPIGCR1 (rw) register accessor: SPI / MibSPI Global control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spigcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spigcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spigcr1`]
module"]
#[doc(alias = "SPIGCR1")]
pub type Spigcr1 = crate::Reg<spigcr1::Spigcr1Spec>;
#[doc = "SPI / MibSPI Global control register 1"]
pub mod spigcr1;
#[doc = "SPIINT0 (rw) register accessor: SPI / MibSPI Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spiint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spiint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spiint0`]
module"]
#[doc(alias = "SPIINT0")]
pub type Spiint0 = crate::Reg<spiint0::Spiint0Spec>;
#[doc = "SPI / MibSPI Interrupt Enable Register"]
pub mod spiint0;
#[doc = "SPILVL (rw) register accessor: SPI / MibSPI Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spilvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spilvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spilvl`]
module"]
#[doc(alias = "SPILVL")]
pub type Spilvl = crate::Reg<spilvl::SpilvlSpec>;
#[doc = "SPI / MibSPI Interrupt Level Register"]
pub mod spilvl;
#[doc = "SPIFLG (rw) register accessor: SPI / MibSPI Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spiflg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spiflg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spiflg`]
module"]
#[doc(alias = "SPIFLG")]
pub type Spiflg = crate::Reg<spiflg::SpiflgSpec>;
#[doc = "SPI / MibSPI Flag Register"]
pub mod spiflg;
#[doc = "SPIPC0 (rw) register accessor: SPI / MibSPI Pin Control Register 0 (SPIPC0) - SPIFUN Note: Duplicate Control Bits for SIMO0 &amp; SOMI0 Bit 24 is not physically implemented. it is a mirror of Bit11. Any write to Bit 24 will be reflected on Bit11 and when Bit 24 &amp; Bit 11 simultaneously written, the value of Bit11 will control the SOMI pin. Read value of Bit 24 always reflects the Bit 11 value. This is true for the Bit 24 &amp; Bit 11 of all of SPIPC0 to SPIPC9 registers. Same is true for SIMO pin with Bit16 &amp; Bit 10 of SPIPC0 to SPIPC9 registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipc0`]
module"]
#[doc(alias = "SPIPC0")]
pub type Spipc0 = crate::Reg<spipc0::Spipc0Spec>;
#[doc = "SPI / MibSPI Pin Control Register 0 (SPIPC0) - SPIFUN Note: Duplicate Control Bits for SIMO0 &amp; SOMI0 Bit 24 is not physically implemented. it is a mirror of Bit11. Any write to Bit 24 will be reflected on Bit11 and when Bit 24 &amp; Bit 11 simultaneously written, the value of Bit11 will control the SOMI pin. Read value of Bit 24 always reflects the Bit 11 value. This is true for the Bit 24 &amp; Bit 11 of all of SPIPC0 to SPIPC9 registers. Same is true for SIMO pin with Bit16 &amp; Bit 10 of SPIPC0 to SPIPC9 registers."]
pub mod spipc0;
#[doc = "SPIPC1 (rw) register accessor: SPI / MibSPI Pin Control Register 1 (SPIPC1) - SPIDIR\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipc1`]
module"]
#[doc(alias = "SPIPC1")]
pub type Spipc1 = crate::Reg<spipc1::Spipc1Spec>;
#[doc = "SPI / MibSPI Pin Control Register 1 (SPIPC1) - SPIDIR"]
pub mod spipc1;
#[doc = "SPIPC2 (rw) register accessor: SPI / MibSPI Pin Control Register 2 (SPIPC2) - SPIDIN\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipc2`]
module"]
#[doc(alias = "SPIPC2")]
pub type Spipc2 = crate::Reg<spipc2::Spipc2Spec>;
#[doc = "SPI / MibSPI Pin Control Register 2 (SPIPC2) - SPIDIN"]
pub mod spipc2;
#[doc = "SPIPC3 (rw) register accessor: SPI / MibSPI Pin Control Register 3 (SPIPC3) - SPIDOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipc3`]
module"]
#[doc(alias = "SPIPC3")]
pub type Spipc3 = crate::Reg<spipc3::Spipc3Spec>;
#[doc = "SPI / MibSPI Pin Control Register 3 (SPIPC3) - SPIDOUT"]
pub mod spipc3;
#[doc = "SPIPC4 (rw) register accessor: SPI / MibSPI Pin Control Register 4 (SPIPC4) - SPIDSET\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipc4`]
module"]
#[doc(alias = "SPIPC4")]
pub type Spipc4 = crate::Reg<spipc4::Spipc4Spec>;
#[doc = "SPI / MibSPI Pin Control Register 4 (SPIPC4) - SPIDSET"]
pub mod spipc4;
#[doc = "SPIPC5 (rw) register accessor: SPI / MibSPI Pin Control Register 5 (SPIPC5) - SPIDCLR\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipc5`]
module"]
#[doc(alias = "SPIPC5")]
pub type Spipc5 = crate::Reg<spipc5::Spipc5Spec>;
#[doc = "SPI / MibSPI Pin Control Register 5 (SPIPC5) - SPIDCLR"]
pub mod spipc5;
#[doc = "SPIPC6 (rw) register accessor: SPI / MibSPI Pin Control Register 6 (SPIPC6) - SPIPDR\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipc6`]
module"]
#[doc(alias = "SPIPC6")]
pub type Spipc6 = crate::Reg<spipc6::Spipc6Spec>;
#[doc = "SPI / MibSPI Pin Control Register 6 (SPIPC6) - SPIPDR"]
pub mod spipc6;
#[doc = "SPIDAT0 (rw) register accessor: SPI / MibSPI Transmit Data Register 0 Note: Accessibility of SPIDAT0 The SPIDAT0 register is not accessible in Multibuffer Mode of MibSPI. It is only accessible in compatibility mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`spidat0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spidat0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spidat0`]
module"]
#[doc(alias = "SPIDAT0")]
pub type Spidat0 = crate::Reg<spidat0::Spidat0Spec>;
#[doc = "SPI / MibSPI Transmit Data Register 0 Note: Accessibility of SPIDAT0 The SPIDAT0 register is not accessible in Multibuffer Mode of MibSPI. It is only accessible in compatibility mode."]
pub mod spidat0;
#[doc = "SPIDAT1 (rw) register accessor: SPI / MibSPI Transmit Data Register 1 When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned.\n\nYou can [`read`](crate::Reg::read) this register and get [`spidat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spidat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spidat1`]
module"]
#[doc(alias = "SPIDAT1")]
pub type Spidat1 = crate::Reg<spidat1::Spidat1Spec>;
#[doc = "SPI / MibSPI Transmit Data Register 1 When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned."]
pub mod spidat1;
#[doc = "SPIBUF (rw) register accessor: SPI / MibSPI Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spibuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spibuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spibuf`]
module"]
#[doc(alias = "SPIBUF")]
pub type Spibuf = crate::Reg<spibuf::SpibufSpec>;
#[doc = "SPI / MibSPI Receive Buffer Register"]
pub mod spibuf;
#[doc = "SPIEMU (rw) register accessor: SPI / MibSPI Emulation Register Note: All the fields ot SPIEMU register are Read-Only. Read operation on this register under any mode will not have any impact on the status of this or any other registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`spiemu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spiemu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spiemu`]
module"]
#[doc(alias = "SPIEMU")]
pub type Spiemu = crate::Reg<spiemu::SpiemuSpec>;
#[doc = "SPI / MibSPI Emulation Register Note: All the fields ot SPIEMU register are Read-Only. Read operation on this register under any mode will not have any impact on the status of this or any other registers."]
pub mod spiemu;
#[doc = "SPIDELAY (rw) register accessor: SPI / MibSPI Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spidelay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spidelay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spidelay`]
module"]
#[doc(alias = "SPIDELAY")]
pub type Spidelay = crate::Reg<spidelay::SpidelaySpec>;
#[doc = "SPI / MibSPI Delay Register"]
pub mod spidelay;
#[doc = "SPIDEF (rw) register accessor: SPI / MibSPI Default Chip select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spidef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spidef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spidef`]
module"]
#[doc(alias = "SPIDEF")]
pub type Spidef = crate::Reg<spidef::SpidefSpec>;
#[doc = "SPI / MibSPI Default Chip select Register"]
pub mod spidef;
#[doc = "SPIFMT0 (rw) register accessor: SPI / MibSPI Data Format Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spifmt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spifmt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spifmt0`]
module"]
#[doc(alias = "SPIFMT0")]
pub type Spifmt0 = crate::Reg<spifmt0::Spifmt0Spec>;
#[doc = "SPI / MibSPI Data Format Register 0"]
pub mod spifmt0;
#[doc = "SPIFMT1 (rw) register accessor: SPI / MibSPI Data Format Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spifmt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spifmt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spifmt1`]
module"]
#[doc(alias = "SPIFMT1")]
pub type Spifmt1 = crate::Reg<spifmt1::Spifmt1Spec>;
#[doc = "SPI / MibSPI Data Format Register 1"]
pub mod spifmt1;
#[doc = "SPIFMT2 (rw) register accessor: SPI / MibSPI Data Format Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`spifmt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spifmt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spifmt2`]
module"]
#[doc(alias = "SPIFMT2")]
pub type Spifmt2 = crate::Reg<spifmt2::Spifmt2Spec>;
#[doc = "SPI / MibSPI Data Format Register 2"]
pub mod spifmt2;
#[doc = "SPIFMT3 (rw) register accessor: SPI / MibSPI Data Format Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`spifmt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spifmt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spifmt3`]
module"]
#[doc(alias = "SPIFMT3")]
pub type Spifmt3 = crate::Reg<spifmt3::Spifmt3Spec>;
#[doc = "SPI / MibSPI Data Format Register 3"]
pub mod spifmt3;
#[doc = "TGINTVECT0 (rw) register accessor: SPI Interrupt Vector Register 0 / MibSPI Transfer Group Interrupt Vector Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tgintvect0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgintvect0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgintvect0`]
module"]
#[doc(alias = "TGINTVECT0")]
pub type Tgintvect0 = crate::Reg<tgintvect0::Tgintvect0Spec>;
#[doc = "SPI Interrupt Vector Register 0 / MibSPI Transfer Group Interrupt Vector Register 0"]
pub mod tgintvect0;
#[doc = "TGINTVECT1 (rw) register accessor: SPI Interrupt Vector Register 1 / MibSPI Transfer Group Interrupt Vector Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tgintvect1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgintvect1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgintvect1`]
module"]
#[doc(alias = "TGINTVECT1")]
pub type Tgintvect1 = crate::Reg<tgintvect1::Tgintvect1Spec>;
#[doc = "SPI Interrupt Vector Register 1 / MibSPI Transfer Group Interrupt Vector Register 1"]
pub mod tgintvect1;
#[doc = "SPIPC9 (rw) register accessor: SPI/MibSPI Pin Control Register 9 (SPIPC9) - SPISRSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipc9`]
module"]
#[doc(alias = "SPIPC9")]
pub type Spipc9 = crate::Reg<spipc9::Spipc9Spec>;
#[doc = "SPI/MibSPI Pin Control Register 9 (SPIPC9) - SPISRSEL"]
pub mod spipc9;
#[doc = "SPIPMCTRL (rw) register accessor: SPI/MibSPI Parallel/Modulo Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipmctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipmctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipmctrl`]
module"]
#[doc(alias = "SPIPMCTRL")]
pub type Spipmctrl = crate::Reg<spipmctrl::SpipmctrlSpec>;
#[doc = "SPI/MibSPI Parallel/Modulo Mode Control Register"]
pub mod spipmctrl;
#[doc = "MIBSPIE (rw) register accessor: MibSPI Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mibspie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mibspie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mibspie`]
module"]
#[doc(alias = "MIBSPIE")]
pub type Mibspie = crate::Reg<mibspie::MibspieSpec>;
#[doc = "MibSPI Enable Register"]
pub mod mibspie;
#[doc = "TGITENST (rw) register accessor: MibSPI Transfer Group Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgitenst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgitenst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgitenst`]
module"]
#[doc(alias = "TGITENST")]
pub type Tgitenst = crate::Reg<tgitenst::TgitenstSpec>;
#[doc = "MibSPI Transfer Group Interrupt Enable Set Register"]
pub mod tgitenst;
#[doc = "TGITENCR (rw) register accessor: MibSPI Transfer Group Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgitencr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgitencr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgitencr`]
module"]
#[doc(alias = "TGITENCR")]
pub type Tgitencr = crate::Reg<tgitencr::TgitencrSpec>;
#[doc = "MibSPI Transfer Group Interrupt Enable Clear Register"]
pub mod tgitencr;
#[doc = "TGITLVST (rw) register accessor: MibSPI Transfer Group Interrupt Level Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgitlvst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgitlvst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgitlvst`]
module"]
#[doc(alias = "TGITLVST")]
pub type Tgitlvst = crate::Reg<tgitlvst::TgitlvstSpec>;
#[doc = "MibSPI Transfer Group Interrupt Level Set Register"]
pub mod tgitlvst;
#[doc = "TGITLVCR (rw) register accessor: MibSPI Transfer Group Interrupt Level Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgitlvcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgitlvcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgitlvcr`]
module"]
#[doc(alias = "TGITLVCR")]
pub type Tgitlvcr = crate::Reg<tgitlvcr::TgitlvcrSpec>;
#[doc = "MibSPI Transfer Group Interrupt Level Clear Register"]
pub mod tgitlvcr;
#[doc = "TGINTFLAG (rw) register accessor: Transfer Group Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgintflag`]
module"]
#[doc(alias = "TGINTFLAG")]
pub type Tgintflag = crate::Reg<tgintflag::TgintflagSpec>;
#[doc = "Transfer Group Interrupt Flag Register"]
pub mod tgintflag;
#[doc = "TICKCNT (rw) register accessor: Tick Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tickcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tickcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tickcnt`]
module"]
#[doc(alias = "TICKCNT")]
pub type Tickcnt = crate::Reg<tickcnt::TickcntSpec>;
#[doc = "Tick Count Register"]
pub mod tickcnt;
#[doc = "LTGPEND (rw) register accessor: Last Transfer Group End Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ltgpend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltgpend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltgpend`]
module"]
#[doc(alias = "LTGPEND")]
pub type Ltgpend = crate::Reg<ltgpend::LtgpendSpec>;
#[doc = "Last Transfer Group End Pointer"]
pub mod ltgpend;
#[doc = "TG0CTRL (rw) register accessor: MibSPI Transfer Group Control Register The number of transfer groups is scalable by design up to a maximum of 16. Depending on the implementation the number of transfer groups and hence the number of transfer group control register may vary. Each transfer group can be configured via one dedicated control register. The register description below shows one exemplary control register(x) which is identical for all transfer groups. E.g. the control register for transfer group 2 is named ΓÇ£TG2CTRLΓÇ¥ and is located at address base0+98h+4*2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tg0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg0ctrl`]
module"]
#[doc(alias = "TG0CTRL")]
pub type Tg0ctrl = crate::Reg<tg0ctrl::Tg0ctrlSpec>;
#[doc = "MibSPI Transfer Group Control Register The number of transfer groups is scalable by design up to a maximum of 16. Depending on the implementation the number of transfer groups and hence the number of transfer group control register may vary. Each transfer group can be configured via one dedicated control register. The register description below shows one exemplary control register(x) which is identical for all transfer groups. E.g. the control register for transfer group 2 is named ΓÇ£TG2CTRLΓÇ¥ and is located at address base0+98h+4*2."]
pub mod tg0ctrl;
#[doc = "TG1CTRL (rw) register accessor: MibSPI Transfer Group Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg1ctrl`]
module"]
#[doc(alias = "TG1CTRL")]
pub type Tg1ctrl = crate::Reg<tg1ctrl::Tg1ctrlSpec>;
#[doc = "MibSPI Transfer Group Control Register"]
pub mod tg1ctrl;
#[doc = "TG2CTRL (rw) register accessor: MibSPI Transfer Group Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg2ctrl`]
module"]
#[doc(alias = "TG2CTRL")]
pub type Tg2ctrl = crate::Reg<tg2ctrl::Tg2ctrlSpec>;
#[doc = "MibSPI Transfer Group Control Register"]
pub mod tg2ctrl;
#[doc = "TG3CTRL (rw) register accessor: MibSPI Transfer Group Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg3ctrl`]
module"]
#[doc(alias = "TG3CTRL")]
pub type Tg3ctrl = crate::Reg<tg3ctrl::Tg3ctrlSpec>;
#[doc = "MibSPI Transfer Group Control Register"]
pub mod tg3ctrl;
#[doc = "TG4CTRL (rw) register accessor: MibSPI Transfer Group Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg4ctrl`]
module"]
#[doc(alias = "TG4CTRL")]
pub type Tg4ctrl = crate::Reg<tg4ctrl::Tg4ctrlSpec>;
#[doc = "MibSPI Transfer Group Control Register"]
pub mod tg4ctrl;
#[doc = "TG5CTRL (rw) register accessor: MibSPI Transfer Group Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg5ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg5ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg5ctrl`]
module"]
#[doc(alias = "TG5CTRL")]
pub type Tg5ctrl = crate::Reg<tg5ctrl::Tg5ctrlSpec>;
#[doc = "MibSPI Transfer Group Control Register"]
pub mod tg5ctrl;
#[doc = "TG6CTRL (rw) register accessor: MibSPI Transfer Group Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg6ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg6ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg6ctrl`]
module"]
#[doc(alias = "TG6CTRL")]
pub type Tg6ctrl = crate::Reg<tg6ctrl::Tg6ctrlSpec>;
#[doc = "MibSPI Transfer Group Control Register"]
pub mod tg6ctrl;
#[doc = "TG7CTRL (rw) register accessor: MibSPI Transfer Group Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg7ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg7ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tg7ctrl`]
module"]
#[doc(alias = "TG7CTRL")]
pub type Tg7ctrl = crate::Reg<tg7ctrl::Tg7ctrlSpec>;
#[doc = "MibSPI Transfer Group Control Register"]
pub mod tg7ctrl;
#[doc = "DMA0CTRL (rw) register accessor: MibSPI DMA Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0ctrl`]
module"]
#[doc(alias = "DMA0CTRL")]
pub type Dma0ctrl = crate::Reg<dma0ctrl::Dma0ctrlSpec>;
#[doc = "MibSPI DMA Channel Control Register"]
pub mod dma0ctrl;
#[doc = "DMA1CTRL (rw) register accessor: MibSPI DMA Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1ctrl`]
module"]
#[doc(alias = "DMA1CTRL")]
pub type Dma1ctrl = crate::Reg<dma1ctrl::Dma1ctrlSpec>;
#[doc = "MibSPI DMA Channel Control Register"]
pub mod dma1ctrl;
#[doc = "DMA2CTRL (rw) register accessor: MibSPI DMA Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2ctrl`]
module"]
#[doc(alias = "DMA2CTRL")]
pub type Dma2ctrl = crate::Reg<dma2ctrl::Dma2ctrlSpec>;
#[doc = "MibSPI DMA Channel Control Register"]
pub mod dma2ctrl;
#[doc = "DMA3CTRL (rw) register accessor: MibSPI DMA Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma3ctrl`]
module"]
#[doc(alias = "DMA3CTRL")]
pub type Dma3ctrl = crate::Reg<dma3ctrl::Dma3ctrlSpec>;
#[doc = "MibSPI DMA Channel Control Register"]
pub mod dma3ctrl;
#[doc = "DMA4CTRL (rw) register accessor: MibSPI DMA Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma4ctrl`]
module"]
#[doc(alias = "DMA4CTRL")]
pub type Dma4ctrl = crate::Reg<dma4ctrl::Dma4ctrlSpec>;
#[doc = "MibSPI DMA Channel Control Register"]
pub mod dma4ctrl;
#[doc = "ICOUNT0 (rw) register accessor: MibSPI DMAxCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`icount0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icount0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icount0`]
module"]
#[doc(alias = "ICOUNT0")]
pub type Icount0 = crate::Reg<icount0::Icount0Spec>;
#[doc = "MibSPI DMAxCOUNT"]
pub mod icount0;
#[doc = "ICOUNT1 (rw) register accessor: MibSPI DMAxCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`icount1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icount1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icount1`]
module"]
#[doc(alias = "ICOUNT1")]
pub type Icount1 = crate::Reg<icount1::Icount1Spec>;
#[doc = "MibSPI DMAxCOUNT"]
pub mod icount1;
#[doc = "ICOUNT2 (rw) register accessor: MibSPI DMAxCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`icount2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icount2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icount2`]
module"]
#[doc(alias = "ICOUNT2")]
pub type Icount2 = crate::Reg<icount2::Icount2Spec>;
#[doc = "MibSPI DMAxCOUNT"]
pub mod icount2;
#[doc = "ICOUNT3 (rw) register accessor: MibSPI DMAxCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`icount3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icount3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icount3`]
module"]
#[doc(alias = "ICOUNT3")]
pub type Icount3 = crate::Reg<icount3::Icount3Spec>;
#[doc = "MibSPI DMAxCOUNT"]
pub mod icount3;
#[doc = "ICOUNT4 (rw) register accessor: MibSPI DMAxCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`icount4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icount4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icount4`]
module"]
#[doc(alias = "ICOUNT4")]
pub type Icount4 = crate::Reg<icount4::Icount4Spec>;
#[doc = "MibSPI DMAxCOUNT"]
pub mod icount4;
#[doc = "DMACNTLEN (rw) register accessor: DMA LARGE COUNT register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacntlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacntlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacntlen`]
module"]
#[doc(alias = "DMACNTLEN")]
pub type Dmacntlen = crate::Reg<dmacntlen::DmacntlenSpec>;
#[doc = "DMA LARGE COUNT register"]
pub mod dmacntlen;
#[doc = "PAR_ECC_CTRL (rw) register accessor: Parity/ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`par_ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par_ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@par_ecc_ctrl`]
module"]
#[doc(alias = "PAR_ECC_CTRL")]
pub type ParEccCtrl = crate::Reg<par_ecc_ctrl::ParEccCtrlSpec>;
#[doc = "Parity/ECC Control Register"]
pub mod par_ecc_ctrl;
#[doc = "PAR_ECC_STAT (rw) register accessor: Parity/ECC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`par_ecc_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par_ecc_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@par_ecc_stat`]
module"]
#[doc(alias = "PAR_ECC_STAT")]
pub type ParEccStat = crate::Reg<par_ecc_stat::ParEccStatSpec>;
#[doc = "Parity/ECC Status Register"]
pub mod par_ecc_stat;
#[doc = "UERRADDR1 (rw) register accessor: Uncorrectable Parity or double bit ECC error Address Register - RXRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`uerraddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uerraddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uerraddr1`]
module"]
#[doc(alias = "UERRADDR1")]
pub type Uerraddr1 = crate::Reg<uerraddr1::Uerraddr1Spec>;
#[doc = "Uncorrectable Parity or double bit ECC error Address Register - RXRAM"]
pub mod uerraddr1;
#[doc = "UERRADDR0 (rw) register accessor: Uncorrectable Parity or double bit ECC error address register - TXRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`uerraddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uerraddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uerraddr0`]
module"]
#[doc(alias = "UERRADDR0")]
pub type Uerraddr0 = crate::Reg<uerraddr0::Uerraddr0Spec>;
#[doc = "Uncorrectable Parity or double bit ECC error address register - TXRAM"]
pub mod uerraddr0;
#[doc = "RXOVRN_BUF_ADDR (rw) register accessor: Receive RAM Overrun Buffer Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxovrn_buf_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxovrn_buf_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxovrn_buf_addr`]
module"]
#[doc(alias = "RXOVRN_BUF_ADDR")]
pub type RxovrnBufAddr = crate::Reg<rxovrn_buf_addr::RxovrnBufAddrSpec>;
#[doc = "Receive RAM Overrun Buffer Address Register"]
pub mod rxovrn_buf_addr;
#[doc = "IOLPBKTSTCR (rw) register accessor: SPI/MibSPI IO Loopback Test Control Register This register controls test mode for I/O pins. It also controls whether loop-back should be digital or analog ones in this test mode. In addition it contains control bits to induce some of the error condition into the module. These are to be used for test purpose only. All the control/status bits in this register are valid only when IO LPBK TST ENA field is set to ΓÇ£1010ΓÇ¥.\n\nYou can [`read`](crate::Reg::read) this register and get [`iolpbktstcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iolpbktstcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iolpbktstcr`]
module"]
#[doc(alias = "IOLPBKTSTCR")]
pub type Iolpbktstcr = crate::Reg<iolpbktstcr::IolpbktstcrSpec>;
#[doc = "SPI/MibSPI IO Loopback Test Control Register This register controls test mode for I/O pins. It also controls whether loop-back should be digital or analog ones in this test mode. In addition it contains control bits to induce some of the error condition into the module. These are to be used for test purpose only. All the control/status bits in this register are valid only when IO LPBK TST ENA field is set to ΓÇ£1010ΓÇ¥."]
pub mod iolpbktstcr;
#[doc = "EXTENDED_PRESCALE1 (rw) register accessor: SPI/MibSPI Extended Prescale Register 1 (EXTENDED_PRESCALE1 for SPIFMT0 and SPIFMT1) This register provides an extended Prescale values for SPICLK generation to be able to interface with much slower SPI Slaves. This is an extension of SPIFMT0 and SPIFMT1 registers. For example, EPRESCALE_FMT1(7:0) of EXTENDED_PRESCALE1 and PRESCALE1(7:0) of SPIFMT1 register will always reflect the same contents. Similarly EPRESCALE_FMT0(7:0) and PRESCALE0(7:0) of SPIFMT0 reflect the same contents.\n\nYou can [`read`](crate::Reg::read) this register and get [`extended_prescale1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extended_prescale1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extended_prescale1`]
module"]
#[doc(alias = "EXTENDED_PRESCALE1")]
pub type ExtendedPrescale1 = crate::Reg<extended_prescale1::ExtendedPrescale1Spec>;
#[doc = "SPI/MibSPI Extended Prescale Register 1 (EXTENDED_PRESCALE1 for SPIFMT0 and SPIFMT1) This register provides an extended Prescale values for SPICLK generation to be able to interface with much slower SPI Slaves. This is an extension of SPIFMT0 and SPIFMT1 registers. For example, EPRESCALE_FMT1(7:0) of EXTENDED_PRESCALE1 and PRESCALE1(7:0) of SPIFMT1 register will always reflect the same contents. Similarly EPRESCALE_FMT0(7:0) and PRESCALE0(7:0) of SPIFMT0 reflect the same contents."]
pub mod extended_prescale1;
#[doc = "EXTENDED_PRESCALE2 (rw) register accessor: SPI/MibSPI Extended Prescale Register 2 (EXTENDED_PRESCALE2 for SPIFMT2 and SPIFMT3) This register provides an extended Prescale values for SPICLK generation to be able to interface with much slower SPI Slaves. This register is an extension of SPIFMT2 and SPIFMT3 registers. For example, EPRESCALE_FMT2(7:0) of EXTENDED_PRESCALE2 and PRESCALE2(7:0) of SPIFMT2 register will always reflect the same contents. Similarly EPRESCALE_FMT3(7:0) and PRESCALE3(7:0) of SPIFMT3 reflect the same contents.\n\nYou can [`read`](crate::Reg::read) this register and get [`extended_prescale2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extended_prescale2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extended_prescale2`]
module"]
#[doc(alias = "EXTENDED_PRESCALE2")]
pub type ExtendedPrescale2 = crate::Reg<extended_prescale2::ExtendedPrescale2Spec>;
#[doc = "SPI/MibSPI Extended Prescale Register 2 (EXTENDED_PRESCALE2 for SPIFMT2 and SPIFMT3) This register provides an extended Prescale values for SPICLK generation to be able to interface with much slower SPI Slaves. This register is an extension of SPIFMT2 and SPIFMT3 registers. For example, EPRESCALE_FMT2(7:0) of EXTENDED_PRESCALE2 and PRESCALE2(7:0) of SPIFMT2 register will always reflect the same contents. Similarly EPRESCALE_FMT3(7:0) and PRESCALE3(7:0) of SPIFMT3 reflect the same contents."]
pub mod extended_prescale2;
#[doc = "ECCDIAG_CTRL (rw) register accessor: ECC Diagnostic Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccdiag_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccdiag_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccdiag_ctrl`]
module"]
#[doc(alias = "ECCDIAG_CTRL")]
pub type EccdiagCtrl = crate::Reg<eccdiag_ctrl::EccdiagCtrlSpec>;
#[doc = "ECC Diagnostic Control register"]
pub mod eccdiag_ctrl;
#[doc = "ECCDIAG_STAT (rw) register accessor: ECC Diagnostic Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccdiag_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccdiag_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccdiag_stat`]
module"]
#[doc(alias = "ECCDIAG_STAT")]
pub type EccdiagStat = crate::Reg<eccdiag_stat::EccdiagStatSpec>;
#[doc = "ECC Diagnostic Status register"]
pub mod eccdiag_stat;
#[doc = "SBERRADDR1 (rw) register accessor: Single Bit Error Address Register - RXRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`sberraddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sberraddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sberraddr1`]
module"]
#[doc(alias = "SBERRADDR1")]
pub type Sberraddr1 = crate::Reg<sberraddr1::Sberraddr1Spec>;
#[doc = "Single Bit Error Address Register - RXRAM"]
pub mod sberraddr1;
#[doc = "SBERRADDR0 (rw) register accessor: Single Bit ECC Error Address Register - TXRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`sberraddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sberraddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sberraddr0`]
module"]
#[doc(alias = "SBERRADDR0")]
pub type Sberraddr0 = crate::Reg<sberraddr0::Sberraddr0Spec>;
#[doc = "Single Bit ECC Error Address Register - TXRAM"]
pub mod sberraddr0;
#[doc = "SPIREV (rw) register accessor: SPI / MibSPI Revision ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spirev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spirev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spirev`]
module"]
#[doc(alias = "SPIREV")]
pub type Spirev = crate::Reg<spirev::SpirevSpec>;
#[doc = "SPI / MibSPI Revision ID Register"]
pub mod spirev;
